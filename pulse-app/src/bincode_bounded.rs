//! Size-bounded bincode deserialize — chunk #72 follow-up.
//!
//! Plain `bincode::deserialize` in bincode 1.3 uses `Config` (fixint
//! encoding, infinite limit). When a corrupt or maliciously-crafted byte
//! stream carries a valid-looking 8-byte usize length prefix, bincode
//! pre-allocates a `Vec` of that capacity BEFORE consuming the payload —
//! crafted bytes like `b"\x00\x01\x02 garbage"` decode the first 8 bytes as
//! ~8.9e18 and trigger an immediate OOM process abort (rather than a
//! graceful `Err`). This is dangerous on the legacy-bincode migration path
//! (`baseline_persistence::migrate_legacy_inner`) where the input file is
//! unencrypted and an attacker с filesystem write access could weaponize
//! it; less acute on the encrypted corpus load paths, where the AES-256-GCM
//! GCM tag MUST validate before bincode sees the plaintext, but still
//! relevant if key material is ever compromised.
//!
//! [`deserialize`] uses `Options::with_limit(DEFAULT_MAX_SIZE_BYTES)` к
//! cap the length-prefix value before allocation; oversized prefixes
//! fail-fast as `Err::SizeLimit`. Encoding shape matches plain
//! `bincode::serialize` (LittleEndian + FixintEncoding +
//! RejectTrailingBytes) for binary compatibility с existing on-disk
//! payloads. Centralizes the per-adapter discipline in one helper so all
//! four persistence load paths share identical bounds.

use bincode::Options;
use triage::contract::DEFAULT_MAX_SIZE_BYTES;

/// Deserialize bincode bytes с the corpus-file-size cap applied as а
/// length-prefix bound. Encoding shape matches plain `bincode::serialize`.
pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T, bincode::Error>
where
    T: serde::Deserialize<'a>,
{
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(DEFAULT_MAX_SIZE_BYTES)
        .deserialize(bytes)
}
