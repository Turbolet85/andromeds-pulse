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

/// Production OS keychain backend wrapping the `keyring` crate. Surfaces
/// the OS-native dialog on first call per platform (macOS Keychain
/// prompt / Linux Secret Service / Windows DPAPI consent) per capability
/// P-049 observable signal.
#[derive(Debug)]
pub struct OsKeychainBackend {
    service: String,
}

impl OsKeychainBackend {
    pub fn new(service: impl Into<String>) -> Self {
        Self {
            service: service.into(),
        }
    }
}

impl KeychainBackend for OsKeychainBackend {
    fn fetch_or_create_key(&self, service_id: &str) -> Result<[u8; 32], KeychainError> {
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

    fn backend_kind(&self) -> BackendKind {
        if cfg!(target_os = "macos") {
            BackendKind::MacosKeychain
        } else if cfg!(target_os = "windows") {
            BackendKind::WindowsDpapi
        } else {
            BackendKind::LinuxSecretService
        }
    }
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
