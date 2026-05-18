// Chunk #68 scaffold: cell_encrypt / cell_decrypt are the substrate for
// chunks #70+ (incident records, digest archive) which consume them at write
// path. Tests exercise them; production write path lands in subsequent chunks.
#![allow(dead_code)]

//! Per-cell AES-256-GCM encryption — chunk #68.
//!
//! Cell-level encryption per Phase 6 user decision (default): table +
//! column metadata visible without key; cell payloads opaque. Smaller
//! dep footprint than SQLCipher; no C-side build dep. Trade-off
//! documented in chunk #68 plan §Implementation notes — if future
//! chunks need full-file opacity, swap to SQLCipher.
//!
//! Output format: `nonce (12 bytes) || ciphertext (variable, includes
//! 16-byte GCM tag)`. Nonce is freshly generated per call from
//! `OsRng`; never reused across calls с the same key.

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::RngCore;

use crate::error::Error;

/// 32-byte AES-256 key. Sourced from `KeychainBackend::fetch_or_create_key`.
/// Zeroize-on-drop deferred (the `zeroize` crate is not yet a workspace dep;
/// add in a follow-up chunk if defense-in-depth memory hygiene becomes a
/// concern). The key is loaded ONCE per `Corpus::open` call and lives for
/// the corpus lifetime.
#[derive(Clone)]
pub(crate) struct EncryptionKey([u8; 32]);

impl EncryptionKey {
    pub(crate) fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub(crate) fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl std::fmt::Debug for EncryptionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NEVER include key material in Debug output. Show only that a
        // key is present.
        f.debug_struct("EncryptionKey").field("len", &32).finish()
    }
}

const NONCE_LEN: usize = 12;

pub(crate) fn cell_encrypt(key: &EncryptionKey, plaintext: &[u8]) -> Result<Vec<u8>, Error> {
    let cipher_key = Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let cipher = Aes256Gcm::new(cipher_key);
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| Error::EncryptionFailed)?;
    let mut output = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);
    Ok(output)
}

pub(crate) fn cell_decrypt(key: &EncryptionKey, encrypted: &[u8]) -> Result<Vec<u8>, Error> {
    if encrypted.len() < NONCE_LEN {
        return Err(Error::DecryptionFailed);
    }
    let (nonce_bytes, ciphertext) = encrypted.split_at(NONCE_LEN);
    let cipher_key = Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let cipher = Aes256Gcm::new(cipher_key);
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| Error::DecryptionFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> EncryptionKey {
        EncryptionKey::new([0x42; 32])
    }

    #[test]
    fn round_trip_preserves_plaintext() {
        let key = test_key();
        let plaintext = b"hello, corpus";
        let encrypted = cell_encrypt(&key, plaintext).expect("encrypt");
        let decrypted = cell_decrypt(&key, &encrypted).expect("decrypt");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn ciphertext_includes_nonce_prefix() {
        let key = test_key();
        let plaintext = b"x";
        let encrypted = cell_encrypt(&key, plaintext).expect("encrypt");
        // 12-byte nonce + 1-byte ciphertext + 16-byte GCM tag = 29 bytes minimum.
        assert!(encrypted.len() >= NONCE_LEN + 1 + 16);
    }

    #[test]
    fn each_encryption_uses_fresh_nonce() {
        let key = test_key();
        let plaintext = b"same input";
        let e1 = cell_encrypt(&key, plaintext).expect("encrypt 1");
        let e2 = cell_encrypt(&key, plaintext).expect("encrypt 2");
        // Different nonces produce different ciphertexts even for
        // identical plaintext + identical key.
        assert_ne!(e1, e2);
    }

    #[test]
    fn decrypt_with_different_key_fails() {
        let key1 = test_key();
        let key2 = EncryptionKey::new([0x99; 32]);
        let encrypted = cell_encrypt(&key1, b"secret").expect("encrypt");
        let result = cell_decrypt(&key2, &encrypted);
        assert!(matches!(result, Err(Error::DecryptionFailed)));
    }

    #[test]
    fn decrypt_truncated_payload_fails() {
        let key = test_key();
        let too_short = vec![0u8; NONCE_LEN - 1];
        let result = cell_decrypt(&key, &too_short);
        assert!(matches!(result, Err(Error::DecryptionFailed)));
    }

    #[test]
    fn decrypt_tampered_payload_fails() {
        let key = test_key();
        let mut encrypted = cell_encrypt(&key, b"genuine").expect("encrypt");
        // Flip a byte in the ciphertext region (after the nonce).
        let last_idx = encrypted.len() - 1;
        encrypted[last_idx] ^= 0xff;
        let result = cell_decrypt(&key, &encrypted);
        assert!(matches!(result, Err(Error::DecryptionFailed)));
    }

    #[test]
    fn empty_plaintext_round_trips() {
        let key = test_key();
        let encrypted = cell_encrypt(&key, b"").expect("encrypt empty");
        let decrypted = cell_decrypt(&key, &encrypted).expect("decrypt empty");
        assert_eq!(decrypted, b"");
    }

    #[test]
    fn debug_does_not_leak_key_material() {
        let key = test_key();
        let s = format!("{key:?}");
        assert!(!s.contains("0x42"));
        assert!(!s.contains("66")); // 0x42 in decimal
        assert!(s.contains("EncryptionKey"));
    }
}
