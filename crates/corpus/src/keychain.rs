//! Keychain backend abstraction — chunk #68.
//!
//! Trait-injected backend per test-plan §8 (constructor-injected
//! `Arc<dyn Trait>`) so production binds to OS keychain (`keyring`
//! crate) while tests bind to an in-memory fake. Real keychain APIs
//! are NEVER called from CI tests (per test-plan §6 P5 surrogate
//! pattern — keychain access is non-interactive in headless runners).

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Mutex;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeychainError {
    #[error("keychain unavailable")]
    Unavailable,
    #[error("entry not found")]
    NotFound,
    #[error("operation failed")]
    Failed,
}

/// Identifies which keychain backend produced a key. Bounded
/// enumeration per obs-plan §11 Metrics anti-pattern (label values
/// enumerated upfront).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackendKind {
    MacosKeychain,
    LinuxSecretService,
    WindowsDpapi,
    PassphraseFallback,
    /// In-memory fake used by tests only. Never seen in production
    /// `tracing` events.
    InMemoryFake,
}

impl BackendKind {
    pub fn as_str(self) -> &'static str {
        match self {
            BackendKind::MacosKeychain => "macos_keychain",
            BackendKind::LinuxSecretService => "linux_secret_service",
            BackendKind::WindowsDpapi => "windows_dpapi",
            BackendKind::PassphraseFallback => "passphrase_fallback",
            BackendKind::InMemoryFake => "in_memory_fake",
        }
    }
}

/// Keychain backend trait — fetches (or creates on first call) the
/// 32-byte corpus encryption key for the named service identifier.
/// Implementations MUST be `Send + Sync + Debug` so callers can hold
/// `Arc<dyn KeychainBackend>` and thread it through resolver
/// constructors.
pub trait KeychainBackend: Send + Sync + Debug {
    /// Returns the persisted 32-byte key for `service_id`. If no key
    /// is present, creates and persists a new one before returning.
    fn fetch_or_create_key(&self, service_id: &str) -> Result<[u8; 32], KeychainError>;

    /// Identifies which backend produced the key (for bounded-cardinality
    /// labeling in tracing events).
    fn backend_kind(&self) -> BackendKind;
}

/// In-memory backend used by tests. Holds a single key per service
/// identifier in a Mutex-guarded HashMap. NEVER used in production
/// (the production binary constructs `OsKeychainBackend` only).
#[derive(Debug, Default)]
pub struct FakeKeychainBackend {
    keys: Mutex<HashMap<String, [u8; 32]>>,
}

impl FakeKeychainBackend {
    pub fn new() -> Self {
        Self::default()
    }

    /// Seed a specific key for a service ID. Useful for round-trip tests
    /// that need a deterministic key across multiple backend instances.
    pub fn with_seeded_key(service_id: &str, key: [u8; 32]) -> Self {
        let mut map = HashMap::new();
        map.insert(service_id.to_string(), key);
        Self {
            keys: Mutex::new(map),
        }
    }
}

impl KeychainBackend for FakeKeychainBackend {
    fn fetch_or_create_key(&self, service_id: &str) -> Result<[u8; 32], KeychainError> {
        let mut guard = self.keys.lock().map_err(|_| KeychainError::Failed)?;
        let entry = guard
            .entry(service_id.to_string())
            .or_insert_with(generate_random_key);
        Ok(*entry)
    }

    fn backend_kind(&self) -> BackendKind {
        BackendKind::InMemoryFake
    }
}

/// Environment variable supplying the fallback passphrase. Per
/// security-plan.md §Secret Management → Storage → Runtime the fallback key is
/// derived per-user from the environment; §Data Protection forbids plaintext
/// runtime state on disk, so nothing is persisted here.
pub const CORPUS_PASSPHRASE_ENV: &str = "ANDROMEDA_PULSE_CORPUS_PASSPHRASE";

/// BLAKE3 key-derivation context. Changing this string changes every derived
/// key, so it is versioned rather than edited.
const PASSPHRASE_KDF_CONTEXT: &str = "andromeda-pulse corpus cell key v1";

/// Bounded parse per security-plan.md §Input Validation — an env var is an
/// input boundary even when its content is a secret.
const MAX_PASSPHRASE_BYTES: usize = 1024;

/// Production keychain backend: OS credential store primary, passphrase
/// fallback with a warning, per capability P-049.
///
/// The fallback engages ONLY when a passphrase is configured. A transient
/// store failure must not silently switch keys — rows written under a
/// different key are exactly the defect this backend exists to prevent — so an
/// unconfigured host surfaces the store's error instead of inventing a key.
#[derive(Debug)]
pub struct OsKeychainBackend {
    service: String,
    served_by: Mutex<Option<BackendKind>>,
}

impl OsKeychainBackend {
    pub fn new(service: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            served_by: Mutex::new(None),
        }
    }

    fn platform_kind() -> BackendKind {
        if cfg!(target_os = "macos") {
            BackendKind::MacosKeychain
        } else if cfg!(target_os = "windows") {
            BackendKind::WindowsDpapi
        } else {
            BackendKind::LinuxSecretService
        }
    }

    fn record_served_by(&self, kind: BackendKind) {
        if let Ok(mut guard) = self.served_by.lock() {
            *guard = Some(kind);
        }
    }

    fn fetch_from_os_store(&self, service_id: &str) -> Result<[u8; 32], KeychainError> {
        let entry = keyring::Entry::new(&self.service, service_id)
            .map_err(|_| KeychainError::Unavailable)?;
        match entry.get_password() {
            Ok(serialized) => decode_key(&serialized),
            Err(keyring::Error::NoEntry) => {
                let key = generate_random_key();
                let encoded = encode_key(&key);
                entry
                    .set_password(&encoded)
                    .map_err(|_| KeychainError::Failed)?;
                Ok(key)
            }
            Err(_) => Err(KeychainError::Failed),
        }
    }

    /// The primary-then-fallback decision, taking both inputs by argument so
    /// each branch is exercisable without a platform credential store and
    /// without mutating the process environment.
    fn resolve_key(
        &self,
        service_id: &str,
        store_result: Result<[u8; 32], KeychainError>,
        passphrase: Option<String>,
    ) -> Result<[u8; 32], KeychainError> {
        match store_result {
            Ok(key) => {
                self.record_served_by(Self::platform_kind());
                Ok(key)
            }
            Err(store_err) => match passphrase {
                Some(passphrase) => {
                    let key = derive_key_from_passphrase(&passphrase, service_id);
                    self.record_served_by(BackendKind::PassphraseFallback);
                    tracing::warn!(
                        target: "corpus.keychain.fallback",
                        backend_kind = BackendKind::PassphraseFallback.as_str(),
                        reason = "os_credential_store_unavailable",
                        consequence = "key_custody_reduced_to_environment_passphrase",
                        "corpus key served by passphrase fallback; reduced key-custody guarantees",
                    );
                    Ok(key)
                }
                None => Err(store_err),
            },
        }
    }
}

impl KeychainBackend for OsKeychainBackend {
    fn fetch_or_create_key(&self, service_id: &str) -> Result<[u8; 32], KeychainError> {
        self.resolve_key(
            service_id,
            self.fetch_from_os_store(service_id),
            configured_passphrase(),
        )
    }

    fn backend_kind(&self) -> BackendKind {
        self.served_by
            .lock()
            .ok()
            .and_then(|guard| *guard)
            .unwrap_or_else(Self::platform_kind)
    }
}

/// Reads the fallback passphrase from the environment.
fn configured_passphrase() -> Option<String> {
    passphrase_from_raw(std::env::var(CORPUS_PASSPHRASE_ENV).ok())
}

/// The bounded parse, split from the environment read so it is testable
/// without mutating the process environment. An unset, empty, or over-long
/// value is "not configured" — the fallback is opt-in.
fn passphrase_from_raw(raw: Option<String>) -> Option<String> {
    let raw = raw?;
    if raw.is_empty() || raw.len() > MAX_PASSPHRASE_BYTES {
        return None;
    }
    Some(raw)
}

/// Derives the 32-byte cell key from the passphrase. `service_id` is folded in
/// so two corpora under one passphrase do not share a key.
fn derive_key_from_passphrase(passphrase: &str, service_id: &str) -> [u8; 32] {
    let mut material = Vec::with_capacity(passphrase.len() + service_id.len() + 1);
    material.extend_from_slice(service_id.as_bytes());
    material.push(0);
    material.extend_from_slice(passphrase.as_bytes());
    blake3::derive_key(PASSPHRASE_KDF_CONTEXT, &material)
}

fn generate_random_key() -> [u8; 32] {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes
}

fn encode_key(key: &[u8; 32]) -> String {
    // Hex encode — keyring values are strings on most backends; hex
    // round-trip is simple + length-stable. NEVER use base64 here because
    // some keyring backends sniff for embedded `=` padding and reject.
    let mut s = String::with_capacity(64);
    for b in key {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn decode_key(encoded: &str) -> Result<[u8; 32], KeychainError> {
    if encoded.len() != 64 {
        return Err(KeychainError::Failed);
    }
    let mut key = [0u8; 32];
    for (i, chunk) in encoded.as_bytes().chunks(2).enumerate() {
        if i >= 32 {
            return Err(KeychainError::Failed);
        }
        let s = std::str::from_utf8(chunk).map_err(|_| KeychainError::Failed)?;
        key[i] = u8::from_str_radix(s, 16).map_err(|_| KeychainError::Failed)?;
    }
    Ok(key)
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn fake_backend_returns_same_key_on_repeated_fetch() {
        let backend = FakeKeychainBackend::new();
        let k1 = backend
            .fetch_or_create_key("test-service")
            .expect("first fetch");
        let k2 = backend
            .fetch_or_create_key("test-service")
            .expect("second fetch");
        assert_eq!(k1, k2);
    }

    #[test]
    fn fake_backend_returns_different_keys_for_different_services() {
        let backend = FakeKeychainBackend::new();
        let k1 = backend.fetch_or_create_key("svc-a").expect("svc-a");
        let k2 = backend.fetch_or_create_key("svc-b").expect("svc-b");
        assert_ne!(k1, k2);
    }

    #[test]
    fn fake_backend_with_seeded_key_returns_seed() {
        let seed = [42u8; 32];
        let backend = FakeKeychainBackend::with_seeded_key("svc", seed);
        let fetched = backend.fetch_or_create_key("svc").expect("fetch");
        assert_eq!(fetched, seed);
    }

    #[test]
    fn resolve_key_prefers_the_os_store_when_it_serves() {
        let backend = OsKeychainBackend::new("test-service");
        let from_store = [7u8; 32];
        let resolved = backend
            .resolve_key("corpus-key", Ok(from_store), Some("a passphrase".into()))
            .expect("store serves");

        assert_eq!(resolved, from_store);
        assert_ne!(backend.backend_kind(), BackendKind::PassphraseFallback);
    }

    #[test]
    fn resolve_key_falls_back_to_passphrase_when_the_store_is_unavailable() {
        let backend = OsKeychainBackend::new("test-service");
        let resolved = backend
            .resolve_key(
                "corpus-key",
                Err(KeychainError::Unavailable),
                Some("a passphrase".into()),
            )
            .expect("fallback serves");

        assert_eq!(
            resolved,
            derive_key_from_passphrase("a passphrase", "corpus-key")
        );
        assert_eq!(backend.backend_kind(), BackendKind::PassphraseFallback);
    }

    #[test]
    fn resolve_key_surfaces_the_store_error_when_no_passphrase_is_configured() {
        let backend = OsKeychainBackend::new("test-service");
        let result = backend.resolve_key("corpus-key", Err(KeychainError::Unavailable), None);

        assert!(matches!(result, Err(KeychainError::Unavailable)));
    }

    #[test]
    fn passphrase_fallback_is_stable_across_backend_instances() {
        let first = OsKeychainBackend::new("test-service")
            .resolve_key(
                "corpus-key",
                Err(KeychainError::Unavailable),
                Some("shared".into()),
            )
            .expect("first");
        let second = OsKeychainBackend::new("test-service")
            .resolve_key(
                "corpus-key",
                Err(KeychainError::Unavailable),
                Some("shared".into()),
            )
            .expect("second");

        assert_eq!(first, second);
    }

    #[test]
    fn derived_keys_differ_by_passphrase_and_by_service_id() {
        let base = derive_key_from_passphrase("passphrase-a", "corpus-key");
        assert_ne!(
            base,
            derive_key_from_passphrase("passphrase-b", "corpus-key")
        );
        assert_ne!(
            base,
            derive_key_from_passphrase("passphrase-a", "other-key")
        );
    }

    const CHILD_SERVICE_ENV: &str = "ANDROMEDA_PULSE_TEST_KEYCHAIN_SERVICE";
    const CHILD_SINK_ENV: &str = "ANDROMEDA_PULSE_TEST_KEY_SINK";
    const CHILD_TEST_PATH: &str = "keychain::tests::child_writes_its_resolved_key";

    /// Child half of the cross-process proof. Inert unless the parent sets both
    /// env vars, so it is a trivial pass in an ordinary run.
    #[test]
    fn child_writes_its_resolved_key() {
        let (Ok(service), Ok(sink)) = (
            std::env::var(CHILD_SERVICE_ENV),
            std::env::var(CHILD_SINK_ENV),
        ) else {
            return;
        };
        let key = OsKeychainBackend::new(service)
            .fetch_or_create_key("corpus-key")
            .expect("child resolves the key");
        std::fs::write(sink, encode_key(&key)).expect("child writes the key sink");
    }

    /// The chunk's headline claim. Two instances in ONE process would not prove
    /// it — keyring's mock store was already process-global, so that check
    /// passed while the defect was live. Only a real second process does.
    ///
    /// Skips cleanly where no credential store exists (CI); the fallback branch
    /// is covered by the `resolve_key` tests above, which need no store.
    #[test]
    fn corpus_key_survives_a_real_process_boundary() {
        let service = format!(
            "andromeda-pulse-test-{}-{}",
            std::process::id(),
            "corpus-key-persistence"
        );
        let backend = OsKeychainBackend::new(&service);

        let Ok(parent_key) = backend.fetch_or_create_key("corpus-key") else {
            eprintln!("[skip] no OS credential store on this host; cross-process leg not run");
            return;
        };
        assert_ne!(
            backend.backend_kind(),
            BackendKind::PassphraseFallback,
            "this leg must exercise the real store, not the fallback"
        );

        let tmp = TempDir::new().expect("tmp");
        let sink = tmp.path().join("child-key.hex");
        let status =
            std::process::Command::new(std::env::current_exe().expect("current test binary"))
                .args(["--exact", CHILD_TEST_PATH, "--nocapture"])
                .env(CHILD_SERVICE_ENV, &service)
                .env(CHILD_SINK_ENV, &sink)
                .status();

        // Delete the entry BEFORE asserting so a failure cannot leave the
        // operator's credential store polluted.
        let cleanup =
            keyring::Entry::new(&service, "corpus-key").and_then(|e| e.delete_credential());

        let status = status.expect("child process spawns");
        assert!(status.success(), "child test binary exited non-zero");
        let child_hex = std::fs::read_to_string(&sink).expect("child wrote its key");
        let child_key = decode_key(child_hex.trim()).expect("child key decodes");

        assert_eq!(
            child_key, parent_key,
            "a key minted by one process must be readable by a later, separate process"
        );
        cleanup.expect("test-scoped credential entry removed");
    }

    #[test]
    fn passphrase_bounded_parse_rejects_empty_and_oversized() {
        assert_eq!(passphrase_from_raw(None), None);
        assert_eq!(passphrase_from_raw(Some(String::new())), None);
        assert_eq!(
            passphrase_from_raw(Some("x".repeat(MAX_PASSPHRASE_BYTES + 1))),
            None
        );

        let at_limit = "x".repeat(MAX_PASSPHRASE_BYTES);
        assert_eq!(passphrase_from_raw(Some(at_limit.clone())), Some(at_limit));
    }

    #[test]
    fn fake_backend_kind_is_in_memory_fake() {
        let backend = FakeKeychainBackend::new();
        assert_eq!(backend.backend_kind(), BackendKind::InMemoryFake);
    }

    #[test]
    fn backend_kind_as_str_bounded_enumeration() {
        assert_eq!(BackendKind::MacosKeychain.as_str(), "macos_keychain");
        assert_eq!(
            BackendKind::LinuxSecretService.as_str(),
            "linux_secret_service"
        );
        assert_eq!(BackendKind::WindowsDpapi.as_str(), "windows_dpapi");
        assert_eq!(
            BackendKind::PassphraseFallback.as_str(),
            "passphrase_fallback"
        );
        assert_eq!(BackendKind::InMemoryFake.as_str(), "in_memory_fake");
    }

    #[test]
    fn encode_decode_round_trips() {
        let key = [0xaau8; 32];
        let encoded = encode_key(&key);
        assert_eq!(encoded.len(), 64);
        let decoded = decode_key(&encoded).expect("decode");
        assert_eq!(key, decoded);
    }

    #[test]
    fn decode_rejects_wrong_length() {
        assert!(decode_key("short").is_err());
        assert!(decode_key(&"a".repeat(63)).is_err());
        assert!(decode_key(&"a".repeat(65)).is_err());
    }

    #[test]
    fn decode_rejects_non_hex() {
        assert!(decode_key(&"z".repeat(64)).is_err());
    }
}
