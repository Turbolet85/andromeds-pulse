# security extract

## Relevance
partial — the chunk adds no runtime surface (no IPC, network, input, secret or log path). Security owns only its supply-chain license gates: `cargo deny check ... licenses` and the npm channel's license allowlist. Both read the manifest and lockfile fields this chunk edits.

## Constraints
- `cargo deny check bans licenses sources` stays a pass/fail CI gate. Any `cargo deny check` failure fails the build. The `licenses` check is load-bearing for the Homebrew tap and Scoop distribution channels, where dependency-license incidents have caused channels to be removed (per security-plan §Dependency Security → CI integration). The new `MIT OR Apache-2.0` expression on the 16 inheriting workspace members must satisfy that check. Whether the workspace's own crates are in cargo-deny's checked set today is research's question (scope item 5, PREMISE AT RISK).
- `deny.toml [licenses]` holds the SPDX allowlist for the Homebrew/Scoop channels (per security-plan §Bootstrap phases → dep-audit-tooling-install). If the own-crate check needs `Apache-2.0` or `MIT`, the allowlist must cover it. Whether it already lists both identifiers is research's question. Any change to `[licenses]` must not relax the `[bans]` posture: `multiple-versions = "deny"` is never relaxed, and the `tonic` canary is never skipped (same §).
- `cargo xtask check:npm-supply-chain` must stay green after the root package's `license` changes (exit 0, or `green-with-dispositions` with the existing exceptions only). Exit 1 means a license outside the per-class allowlist. Exit 2 means cannot-evaluate and is never a pass (per security-plan §Dependency Security → npm channel). The gate reads the committed `package-lock.json` alone, so the lockfile ROOT entry is the value the gate would see. Whether the gate enumerates the root package (the `""` entry) or only dependency entries is research's question (scope Boundaries).
- The fix must not add a new license exception to cover the project's own license change. Exceptions in `pulse-app/ui/npm-policy.json` require mandatory provenance (ghsa/package/reason/owner/closing_condition) and are pruned-not-kept. Cargo-side carve-outs are ID-scoped, owner-named and reasoned (per security-plan §Dependency Security → npm channel and → Duplicate-version carve-outs).
- Keep `cargo deny check advisories` observed as a separate invocation, never folded into the `bans licenses sources` gate command in the plan's Test Commands (per security-plan §Dependency Security → CI integration; `.claude/rules/security.md` Session Additions 2026-08-17).
- The secret-scanning CI step must stay green over the two new root text files (per security-plan §Bootstrap phases → secret-scanning-ci-gate; §Security Anti-Patterns → Secrets).

## Patterns to follow
- Red-before / green-after witness on a supply-chain gate. The scope names the Conductor precedent (cargo-deny changed to check own crates). Security's contribution: the witness subject must be one of the gates security-plan §Dependency Security already runs in CI (`cargo deny check ... licenses`, or `check:npm-supply-chain`'s license walk), so the witness is enforced every PR and not only once.
- Use the lockfile as the canonical enumeration for npm license checks. `package-lock.json` carries `license` fields, so manifest and lock must agree (per security-plan §Dependency Security → npm channel; `.claude/rules/security.md` Session Additions 2026-08-30).
- A closed gate stays a plain pass/fail. Record the post-change run's exit and verdict text, not a claimed pass (per security-plan §Dependency Security → CI integration).

## Anti-patterns to avoid
- NEVER relax `multiple-versions = "deny"`, and never skip the `tonic` canary while editing `deny.toml` (per security-plan §Security Anti-Patterns → Universal; §Bootstrap phases → dep-audit-tooling-install).
- NEVER quiet a gate over the project's own license change by adding a broad or unprovenanced exception, or by widening a per-class allowlist beyond need (per security-plan §Dependency Security → npm channel).

## Contract bindings
- Security CI gate ↔ tests §CI Integration: the `supply-chain` job in `ci.yml` runs both `cargo deny check bans licenses sources` and `cargo xtask check:npm-supply-chain`. The chunk's red/green witness lands in that job (per security-plan §Bootstrap phases → dep-security-ci-gate).
- Security ↔ arch/distribution: the `licenses` check exists for the Homebrew/Scoop `update-channels.yml` path, and those manifests may declare a license string. Whether they do is research's question (per security-plan §Dependency Security → CI integration).

## Acceptance criteria contributions
- `cargo deny check bans licenses sources` exits 0 after the change, with the command's own printed verdict recorded (per security-plan §Dependency Security → CI integration).
- `cargo xtask check:npm-supply-chain` exits 0 after the change with no new exception added to `pulse-app/ui/npm-policy.json`; the same four standing exceptions still apply (per security-plan §Dependency Security → npm channel).
- `deny.toml` `[bans]` is unchanged in `multiple-versions = "deny"` and in the `tonic` entry. A diff against chunk base `1dfca74` shows no relaxation (per security-plan §Bootstrap phases → dep-audit-tooling-install).
- The secret-scan CI check is green on the chunk's commit (per security-plan §Bootstrap phases → secret-scanning-ci-gate).
