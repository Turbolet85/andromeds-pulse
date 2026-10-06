
## 2026-10-06-npm-supply-chain-gate-is-green-again — the supply-chain job does install node_modules, after the gate step
**Section:** §9 CI Integration → Pipeline structure, the Supply chain row
**Change:** The row now says the gate step itself needs no `npm ci` (the license walk and `npm audit` are lockfile-only) and runs before the job's own `npm ci` + `npm run build` steps, so a red gate skips the install, the UI build and the auditable build. Was: "deliberately NO `npm ci` (… so node_modules is never installed in this job)", a statement about the job that was false: the job carries both steps between the gate and the cargo-auditable install.
**Why:** The chunk's CI readings measured it: the steps after the gate read `skipped` on the red commits and `success` on `fc4ebdf`. The doc alone was wrong and the workflow needed no change, so the playbook's pre-existing-reality rule applied it as routine. The mismatch predates this chunk. Trap for later chunks: a red at the gate step leaves the install, the build and the auditable release build unmeasured on that commit, not green.
**Kept:** The sibling sites in security-plan (the npm channel bullet and `dep-security-ci-gate`) and architecture (the xtask CLI surfaces entry) say "no `npm ci`" of the gate or the verb, which is true; they were read and left.
**Ref:** .andromeda/runs/2026-10-06T23-06-47Z-wrap/
