# security extract

## Relevance
partial — the chunk writes a committed record and re-points an xtask gate and its CI step; it crosses no product trust boundary (no port, no TauRPC procedure, no product-read env var, no log emission). security-plan carries no mandate for `verify:capability-matrix` or for either capability record (the gate is absent from the plan's fail-condition roster, §Dependency Security → CI integration); what applies is the plan's rules for the files the chunk edits (`xtask/src/main.rs`, `ci.yml`) and for the controls that stand on surfaces the record may mark retired.

## Constraints
- The edit to `xtask/src/main.rs` shares a file with the `EXPECTED_PROCEDURES` pin; security-plan §API Security (TauRPC capability authorization row) requires that pin, and `staged_gate::EXPECTED_GRANTS`, to stay the enforcement for procedures and grants. The scope adds no procedure and removes no surface, so both pins are required to read unchanged against `279a477e`; whether the planned edit touches either is research's question.
- A retired disposition in the record lifts no control. security-plan §Input Validation (persisted window geometry row, the one row the plan keys to P-061) requires that boundary's validation for as long as its code stands — and the scope removes no code here.
- The same holds for the model surface: security-plan §Security Anti-Patterns → Input (the NARROWED EXCEPTION for the three L4 path vars) requires its guard while the shipped binary reads those vars, whatever the record says of the model's ids.
- And for the model's argv path: security-plan §Security Anti-Patterns → Code Patterns requires the `-p` prompt bound as the one admitted OTLP-derived argv operand while that path exists.
- security-plan §Secret Management (Runtime, and "What counts as secret" → corpus encryption key) states the passphrase-fallback posture as current truth under P-049, one of the 82 ids. Whether P-049's text falls on a surface the version removes is research's question; a retirement of it would leave a current-truth control named by a retired id, so it is a doubtful id for P4, not a silent classification. The plan names only P-049 and P-061 by number; which other ids of the 82 carry a control the plan mandates is research's question (the plan does not key its controls by capability id).
- If the chunk edits `ci.yml`: security-plan §Security Anti-Patterns → Secrets requires every third-party action pinned by 40-char commit SHA and the workflow-level `permissions:` held at `contents: read`.
- If the gate gains any new input beyond a fixed in-repo path (an env var, a path argument naming the record): security-plan §Security Anti-Patterns → Input scopes the harness/xtask-only carve-out to exactly its named members, so a new member is a boundary question brought at P4, never assumed inside the class.

## Patterns to follow
- An xtask gate wired into CI as a plain named `run:` step, with no third-party action so the SHA-pinning convention is not triggered — security-plan §Bootstrap phases (`dep-security-ci-gate`). Whether the existing `lint-test` step at `ci.yml:105-106` already has that shape is research's question.
- A three-way exit contract in which "could not evaluate" is its own arm and never a pass (0 green · 1 red · 2 cannot-evaluate) — security-plan §Dependency Security → CI integration (the npm channel bullet and the `check:staged-artifacts` clause of the fail-condition bullet). It is the plan's stated shape for a gate that reads a committed input; whether the old gate has an absent-record arm today is research's question.
- A pinned set asserted by set equality in BOTH directions, with an unpinned member red and a deleted subject red — security-plan §API Security (TauRPC capability authorization row). The record's "82 ids, each exactly once, no retired id carried by the gate" is the same shape of assertion.
- A step's shape pinned by a workflow test and listed in the soft-fail ban's gate list (no `continue-on-error`, no `||`) — security-plan §Dependency Security → CI integration (the `cargo audit` bullet). Whether `verify:capability-matrix` is in that list is research's question.

## Anti-patterns to avoid
- Reading a retired window or desktop id as leave to drop, widen or re-target a `pulse:notification` / `pulse:tray` / `pulse:updater` grant or any `core:window:*` permission in this chunk — security-plan §Security Anti-Patterns → API; the capability files are outside this chunk's scope.
- Referencing a third-party action by `@v2` or a floating tag, or raising workflow-level `permissions:` above `contents: read`, in any `ci.yml` edit — security-plan §Security Anti-Patterns → Secrets.
- Adding a dependency to `xtask` for the record's reader without the Rust supply-chain gates passing, or relaxing `multiple-versions = "deny"` to admit it — security-plan §Dependency Security → CI integration.

## Contract bindings
- security ↔ tests (CI): `capability-drift` and `check:staged-artifacts` are security's gates in the same workflow as the re-pointed `verify:capability-matrix` step (security-plan §Bootstrap phases, `dep-security-ci-gate`); the step-shape pins and the soft-fail ban list live in the tests domain's workflow tests.
- security ↔ arch: the classification of the 82 ids is arch's and the founder's, not this domain's. Where it retires P-049 or P-061, the security-plan sentences that name them (§Threat Model Summary data classification, §Data Protection, §Secret Management, §Input Validation) need an amendment at the wrap — the masters are read-only at phase.
- security ↔ the external harness: security-plan declares no trust boundary for a reader of the committed record in another project; nothing is extracted for the record's form.

## Acceptance criteria contributions
- `cargo xtask capability-drift` exits 0 and `cargo xtask check:staged-artifacts` exits 0 (`staged-clean`) on the chunk's staged content (per security-plan §Dependency Security → CI integration)
- `git diff 279a477e -- pulse-app/capabilities/` is empty, and the `EXPECTED_PROCEDURES` / `staged_gate::EXPECTED_GRANTS` pins read unchanged against `279a477e` (per security-plan §API Security, TauRPC capability authorization row)
- If `ci.yml` is in the diff against `279a477e`: every `uses:` line carries a 40-char commit SHA and the workflow-level `permissions:` reads `contents: read` (per security-plan §Security Anti-Patterns → Secrets)
- If `Cargo.lock` is in the diff against `279a477e`: `cargo audit` exits 0 and `cargo deny check bans licenses sources` passes (per security-plan §Dependency Security → CI integration)
