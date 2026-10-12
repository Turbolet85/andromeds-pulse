
## 2026-10-09-ci-on-linux-alone — three statements of a `ci.yml` matrix retired
**Section:** §Threat Model Summary → Infrastructure → CI/CD · §Dependency Security → CI integration · §Bootstrap phases `dep-security-ci-gate`
**Change:**
- CI/CD: `ci.yml` is six jobs on `ubuntu-22.04` alone (was "matrix Linux/macOS/Windows").
- CI integration: `cargo audit` runs as a step of `ci.yml`'s `supply-chain` job (was "a step in `ci.yml` matrix").
- `dep-security-ci-gate`: the gates are wired "into `ci.yml`" (was "into `ci.yml` matrix").
**Why:** Chunk 2026-10-09-ci-on-linux-alone removed the Windows and macOS runners from the `ci` workflow; no job carries a matrix. Every gate step, every sha pin, the workflow-level `permissions: contents: read` and the audit step's token are unchanged, and the removed `release` job read no secret and named no environment.
**Kept:** The dated measurement "green on all three `lint-test` runners" in the vendored test-only channel paragraph, true as dated. The `release.yml` and `update-channels.yml` clauses of the CI/CD line.
**Ref:** .andromeda/runs/2026-10-09T15-11-06Z-wrap/
