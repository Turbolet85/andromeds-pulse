
## 2026-10-09-pre-push-check-native-on-linux — the hand-run stage 5 is the verb's own test stage; the boot job's ci-gates line needs a passing smoke
**Section:** §4 Unit Test Strategy (the corpus crate bullet) · §1 Test Scope Summary (the `performance-budget: WebGPU canvas throughput` trigger row)
**Change:**
- §4: the credential-store legs clean-skip in any run whose environment carries neither `DBUS_SESSION_BUS_ADDRESS` nor `XDG_RUNTIME_DIR`; was "any run under `env -i` … (measured at that chunk's native pre-push stage 5)". The `env -i` measurement is kept as the hand-run it was. Since this chunk that stage is `cargo xtask pre-push:linux`'s own `test` stage, whose constructed environment holds neither variable, pinned by set equality; there the clean-skip is stated, not measured.
- §1: the boot job's line reads `no WebGPU adapter (no_navigator_gpu)` on a run whose smoke step passes; was "on every run that reaches the settle verdict, so the boot job's line reads …". On a run whose smoke step fails the job skips its `ci-gates` step and prints no such line, as measured on `ci#38010977166` (settle verdict `ended`, the step `skipped`, 2 adapter records in the log all the same).
**Why:** The stage the clause named by hand is now a stage of the verb. The line claim was measured false on the first run whose settle verdict read `ended`: a failed smoke step skips the step that prints the line.
**Kept:** the `--success-output immediate` / `lacks [skip]` rule; "the boot smoke reads the app past its adapter request on every run that reaches the settle verdict", which held on that run; the `ipc-rejection-wire-coverage` pending-trigger row's statement that the boot job's log holds the adapter record.
**Ref:** .andromeda/runs/2026-10-10T01-19-03Z-wrap/
