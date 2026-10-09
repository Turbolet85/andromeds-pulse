
## 2026-10-04-corpus-key-creation-is-race-free — XDG_RUNTIME_DIR lock dir and the content-free lock file, outside the data dir
**Section:** §Input Validation → CLI / env var inputs · §Security Anti-Patterns → Input (new carve-out) · §Threat Model Summary → Attack surface → CLI input · §Data Protection → At rest — per medium (new corpus-key lock file bullet)
**Change:**
- The system `XDG_RUNTIME_DIR` (Linux) is a product-read input: trimmed; set ⇒ absolute + `canonicalize()` + is-dir, else FAIL CLOSED (`KeychainError::Unavailable`, never a fallback); unset/blank ⇒ the canonicalized temp dir; deliberately NOT confined under the data dir.
- `andromeda-pulse-corpus-key-{h16}.lock`: per-user, content-free, mode 0600, never deleted, written by the app and the MCP sidecar — the second product-written location outside the data dir after the `~/Downloads` export sink; no key material, so "no plaintext runtime state on disk" is unchanged.
- Residuals stated in the body: XDG disagreement (measured under a systemd user session only), shared-`/tmp` pre-create denial (accepted under the single-user trust boundary), passphrase degrade, no lock timeout.
**Why:** a boundary widening — a product-read `*_DIR` input admitted outside the confine-under-data-dir rule. Ratified live by the founder on 2026-10-04 (he chose the per-user lock keyed on the credential entry under `$XDG_RUNTIME_DIR`), confirmed by the operator at this wrap; the lock must be keyed on the one per-user credential entry, which racing processes share whatever data dir they resolve.
**Kept:** §Security Anti-Patterns → Universal "NEVER let the rust-toolchain drift below 1.85.0" — a true minimum-pin ban; the raise rides the route entry "The declared Rust floor matches the code".
**Ref:** .andromeda/runs/2026-10-04T09-16-41Z-wrap/
