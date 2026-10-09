# security extract

## Relevance
partial — the chunk touches three security-owned facts: the Universal toolchain-floor ban, the clippy `-D warnings` gate, and the corpus-key lock-file contract (CARRY). It adds no new trust boundary, IPC surface, dependency or log emission.

## Constraints
- security-plan §Security Anti-Patterns → Universal bans letting the toolchain drift below 1.85.0, because Edition 2024 and its security-positive defaults need it. A raised declared floor (≥ 1.89) must stay at or above that ban. The ban's wording ("1.85.0 minimum") becomes a stale lower bound that is still true. Amending it to the new floor is a wrap-time master amendment, never a phase or implement edit (scope item 4). The same lockstep applies to `.claude/rules/security.md` §Rust toolchain.
- security-plan §Dependency Security → CI integration requires the build to fail on any clippy warning. Removing `#[allow(clippy::incompatible_msrv)]` from `fetch_with_lock_dir` is acceptable only if clippy `--workspace --all-targets --all-features -D warnings` stays green with the raised `rust-version`. Do not swap in a different suppression such as a crate-level allow or a lint-config downgrade.
- security-plan §Data Protection → "Corpus-key lock file" defines the production lock file as content-free, mode `0o600`, opened without truncate and never deleted. That contract serializes first-ever key creation. The CARRY fix (the skip arm leaves no lock file) must therefore stay test-scoped. It may clean up in the test, avoid taking the lock before the reachability probe, or use a private lock dir. It must never add a delete or unlink to the product `Corpus::open` / `fetch_with_lock_dir` path. Whether the fix stays out of product code is a question for research and the plan.
- security-plan §Data Protection → "Corpus-key lock file" and §Secret Management → Runtime say lock-dir resolution FAILS CLOSED when `XDG_RUNTIME_DIR` is set but unusable, with no temp-dir fallback. Any lock-dir redirection the CARRY test uses must not weaken or bypass that product resolution rule.
- security-plan §Dependency Security, Boundaries in scope: no dependency is added or upgraded. `cargo deny check bans licenses sources` and `cargo audit` must stay unchanged in verdict. Raising `rust-version` must not trigger a lockfile re-resolution that changes any crate version. Whether cargo's MSRV-aware resolver would re-resolve under the new floor is research's question.
- security-plan §Logging & Monitoring and §Security Anti-Patterns → Logging forbid a full lock-dir or lock-file path in logs; only the basename is allowed, and failures surface as unit `KeychainError` variants. Nothing the CARRY fix touches may start logging or printing that path. Test-only diagnostics are included, if they reach the tracing sink.

## Patterns to follow
- Wrap-only master amendments (security-plan's amendments sidecar lineage, cited in §Dependency Security): the security-plan:470 floor ban is corrected at wrap as a recorded amendment, in lockstep with the `.claude/rules/security.md` leaf.
- Suppressions follow the project's narrowest-scope practice: one function-scoped allow with a reason (the current `incompatible_msrv` site). Its removal is the fix itself; it is not replaced (§Dependency Security → CI integration lint rule).
- The lock-file invariants (content-free · `0o600` · no truncate · never deleted) are asserted by the existing concurrent witness (§Data Protection → "Corpus-key lock file"). A CARRY change must leave that witness green and unchanged in meaning.
- If a floor witness is added in CI (a P4 question), every third-party action in it, for example a toolchain installer, is SHA-pinned per §Dependency Security → CI integration and §Security Anti-Patterns → Secrets. The job gets `permissions: contents: read` and harden-runner as its first step.

## Anti-patterns to avoid
- Deleting the corpus-key lock file on a product path to fix the test residue. This breaks the "never deleted" contract in §Data Protection → "Corpus-key lock file", and a recreated file lets peers lock different inodes, which reopens the first-creation race.
- Replacing the removed allow with a broader one, or relaxing `-D warnings`, which §Dependency Security → CI integration forbids.
- Adding an unpinned or floating-tag action (for example `@stable` / `@v1`) for any floor-witness CI step, which §Security Anti-Patterns → Secrets bans.

## Contract bindings
- security ↔ arch: `app_info.rust_version` is fed by `env!("CARGO_PKG_RUST_VERSION")`, so the raise changes an arch §Standard Contracts envelope value. The arch example (`architecture.md:103`) is the arch-owned amendment. §Error Handling's ban on exposing library versions governs error responses only; the declared-floor field is an existing, already-ratified `app_info` disclosure, so this chunk adds no new one.
- security ↔ tests: the clippy `-D warnings` gate and the corpus keychain tests (the CARRY test, and the concurrent witness asserting lock-file invariants) are test-plan-owned harness pieces. Their verdicts are the security evidence.
- security ↔ CI: any floor witness job binds to the supply-chain CI rules (SHA pinning, least-privilege permissions, harden-runner) in §Dependency Security and §Security Anti-Patterns → Secrets.

## Acceptance criteria contributions
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` exits 0 after the raised `rust-version` lands, with zero `incompatible_msrv` allows left in the tree (grep count 0) and no new lint suppression added (per security-plan §Dependency Security → CI integration).
- The declared floor is ≥ 1.85.0, so the Universal ban holds (per security-plan §Security Anti-Patterns → Universal). The wrap records the floor-ban amendment at security-plan:470 and the `.claude/rules/security.md` §Rust toolchain leaf in lockstep (pass: both state the new floor, or the report records why not).
- After the CARRY test's skip arm runs (no reachable credential store, for example under `env -i`), the lock dir it used has no new `andromeda-pulse-corpus-key-*.lock` file. Every lock-file invariant assertion in the concurrent witness still passes, and no product code path unlinks the lock file (grep: no `remove_file` on the lock path outside `#[cfg(test)]`) (per security-plan §Data Protection → "Corpus-key lock file").
- `cargo deny check bans licenses sources` exits 0 and `Cargo.lock` has no version changes attributable to the floor raise (per security-plan §Dependency Security).
