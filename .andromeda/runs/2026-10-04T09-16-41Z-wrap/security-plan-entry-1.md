
## 2026-10-04-corpus-key-creation-is-race-free — corpus key creation is race-free (locked create-or-read)
**Section:** §Secret Management → Runtime · §Secret Management → Production / dev separation · §Secret Management → What counts as secret → Corpus encryption key · §Data Protection → At rest → Persistent incident corpus · §Threat Model Summary → Data classification (corpus) · §Logging & Monitoring → What NEVER to log · §Security Anti-Patterns → Logging (full-path rule)
**Change:**
- Runtime: `fetch_from_os_store` is a locked create-or-read — lock dir resolved before any keyring call, exclusive `std::fs::File::lock` on the entry's content-free lock file, get → only on `NoEntry` generate → set → read back → return the read-back key → unlock; no unlocked generate-and-set path. Measured: 8 re-exec processes → 1 distinct key (prior code: 8 distinct, 7 not stored). Library unchanged (`keyring`; `std` lock; existing `blake3`). A lock-dir / open / lock failure maps to unit `KeychainError::Unavailable` / `Failed` → `KeyringUnavailable`, no path, no OS error text — the passphrase fallback only when configured, else the corpus is absent; never an unlocked or ephemeral key.
- Production / dev separation, What counts as secret, Data Protection corpus, Data classification: the once-per-machine key is ONE key under concurrent first run; the fallback trigger now reads "store unreachable or the corpus-key lock dir unusable".
- The full-path NEVER-log set gains the corpus-key lock dir / lock-file path.
**Why:** concurrent first-run processes each minted a key and the last write won, handing their peers undecryptable rows — the orphaning this custody posture exists to prevent.
**Ref:** .andromeda/runs/2026-10-04T09-16-41Z-wrap/
