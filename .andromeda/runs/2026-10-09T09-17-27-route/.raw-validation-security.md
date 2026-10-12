# Security validation — route draft

---

## Insert
- Between `Corpus encryption at rest retired` and `Detection parity after the removals`: **"Supply-chain gate re-based on the smaller graph — advisory, licence, ban and secret-scan gates stay; ignores and carve-outs for departed crates pruned (P-083, P-085)"** (epoch: `Epoch 2`)
  Reason: per security-plan §Bootstrap phases `dep-security-ci-gate` and §Dependency Security → CI integration ("pruned rather than kept, since a stale skip hides the duplicate's return"), `Window's gates retired` names only the gate's npm and capability halves as leaving, while `deny.toml` still carries the Tauri GTK3/unic advisory ignores, the Tauri-ecosystem duplicate skips and the keyring carve-out, which no chunk prunes.
- Between `Token lifecycle by engine command` and `Checks reason by event time`: **"Channel key custody on the node — the receiver's private key readable by the engine's account only, replaceable, never logged (P-087)"** (epoch: `Epoch 3`)
  Reason: per security-plan §Secret Management ("What counts as secret", "Rotation cadence" — every key has a storage and a rotation entry) and §Data Protection → In transit ("TLS: N/A for inbound", which P-087 replaces), the encrypted channel brings the engine's first runtime private key and the draft gives a lifecycle to the token only.

## Reorder
- Move `Corpus encryption at rest retired` before `Headless engine entry point`
  Reason: per security-plan §Secret Management → Runtime, a host with neither a credential store nor a configured passphrase opens no corpus at all, and a server or CI runner has no such store (intent R3 records `KeyringUnavailable` on a runner; no workflow or script sets the passphrase today). As drafted, the headless boot and its read-back gate need the secret-class `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` placed in CI for the seven chunks before the retirement.
- Move `Door admission and bounds` before `Door queries`
  Reason: per security-plan §Threat Model Summary (MCP vector — "No network exposure" is the trust boundary P-092 replaces) and the focus guide's auth-before-user-data rule, the draft has two chunks widen what an unadmitted door returns to another host (filtered telemetry, then whole incidents) before any admission exists.

## Rewrite
- `Token lifecycle by engine command`: "made, shown once, replaced, revoked; never written to a log or a report" → "made, shown once, replaced, revoked; not recoverable from the node's stores; never written to a log or a report"
  Reason: per security-plan §Secret Management "What counts as secret" and §Data Protection → At rest, P-085 leaves the node with unencrypted stores and no credential store, so "shown once" holds only if what the engine keeps cannot yield the token back.
- `Corpus encryption at rest retired`: "stores readable on the node with no credential store" → "stores readable only by the engine's account, no credential store"
  Reason: per security-plan §Data Protection → At rest ("access control inherits OS user permissions") and §Logging & Monitoring → Access controls (mode 0700), once cell encryption leaves, the account boundary is the corpus's only at-rest control, and on a server it is not the single-user desktop the plan assumed.
- `Telemetry store on disk`: "raw telemetry kept for days, surviving restart" → "raw telemetry kept for days, owner-only, surviving restart"
  Reason: security-plan §Data Protection → At rest calls ring-buffer encryption N/A because it is in-memory with no persistence, and §Security Anti-Patterns → Data Protection bans DuckDB encryption (CVE-2025-64429), so days of telemetry on disk have only file ownership and the write-boundary scrub.
- `Door admission and bounds`: "admitted parties only" → "admitted parties only, encrypted in transit"
  Reason: per security-plan §Data Protection → In transit, P-092 has the door carry stored telemetry and the admission credential between hosts, and the draft states an encrypted channel and plaintext refusal for the receiver only.
- `Span keeps its identity`: "the service's version and environment kept" → "the service's version and environment kept, each scrubbed before storage"
  Reason: per security-plan §Security Anti-Patterns → Logging (intended posture: every client-controlled cell a producer can reach is scrubbed pre-write), span name, status message, version and environment are new client-controlled columns, and the draft states scrubbing only on the attributes chunk after it.
- `Real service watched for days`: "one of the founder's services, named by him first" → "a founder's service, named first with the personal data it carries"
  Reason: security-plan §Threat Model Summary → Compliance triggers and §Compliance Controls rest "None" on telemetry staying on the user's own machine with no PII collected. This is the first chunk where real people's data can reach and stay on another node, and the scrubber catalog covers only email, card and SSN among personal data.
