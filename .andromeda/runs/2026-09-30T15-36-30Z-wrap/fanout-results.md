# Fan-out results — 2026-09-30-perf-budget-gate-reads-real-samples

Seven Explore doc-agents, one batch, the verbatim prompt; no keyed contracts (`registry.py contracts` → NOT MIGRATED
for arch / tests / obs / a11y, the line dropped). Returns decoded: no HTML entities in any YAML value; no return failed
the parse; stripping removed only each return's trailing `#` commentary lines (no-drift reasoning), so no raw twin.

| doc | proposals | verdict |
|---|---|---|
| architecture | 8 | all apply |
| security-plan | 2 | all apply (escalate-severity, routine by actual class) |
| design-system | 0 | no drift (tokens n/a on every surface; status claims untouched) |
| layout-templates | 0 | no drift (no UI surface; Halo status untouched) |
| test-plan | 13 | all apply |
| obs-plan | 12 | all apply |
| a11y-plan | 0 | no drift (no interactive element; envelope unchanged) |

## architecture (8)
1. D-arch-resources · §Occupied Resources → xtask CLI surfaces — register `perf:budget` (grader, budgets, nearest-rank p99, exits 0/1/2, named frame cannot-evaluate line, `ci-gates` / `perf:load-profiles` in-process, scripts retired, lint-test Linux `--require memory,snapshot`). → **apply** (playbook :106 accurate this-chunk addition; CLI verbs with exit contracts are registered in this section by precedent).
2. D-arch-resources · same — register `perf:frame-sample` (Windows dev-host frame gate, child-only env, exits, artifact, not CI-wired per `ci#36723465727`). → **apply** (:106).
3. D-arch-resources · §Occupied Resources → Environment variables — `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` harness-SET on the frame leg's child only, never read by the product. → **apply** (:106; an env var is a registry kind).
4. D-arch-resources · xtask CLI surfaces `agent-run` contract — retire "ps1 does not mirror the recorder"; ps1 boot recorder. → **apply** (:106).
5. dependent-of 4 · `harness:status` contract — `ended` no longer null under ps1. → **apply** (group with 4).
6. dependent-of 4 · §Occupied Resources → Filesystem locations `.spawn`/`.exit` — writers are both scripts. → **apply** (group with 4).
7. D-arch-resources · §Infrastructure Patterns → CI/CD approach — cache ownership sentence (release owns `release-{os}`, 8 entries, usage + watch). → **apply** (:106).
8. D-arch-resources · same — lint-test Linux perf-samples → `perf:budget` → upload; release carries no frame boot step. → **apply** (:106).

## security-plan (2)
1. D-security-input (escalate) · §Security Anti-Patterns → Input carve-out — the `agent-run.{sh,ps1}` `.spawn`/`.exit` records join the harness-only class. → **apply** — playbook :82 routine-by-actual-class: (a) escalate condition affirmatively absent (report: harness-written, harness-read, `exit N` ASCII, no product consumer — no unvalidated boundary); (b) actual class is an accurate this-chunk addition (:106) that the plan's expected list names. Applied text scoped: the records are harness state FILES, not env vars — written into the carve-out as a sibling class, not as tool-locator vars.
2. dependent-of 1 · §Threat Model Summary → CLI input trust boundary — the restated "carve-out is unchanged". → **apply** (group with 1).

## test-plan (13)
1. D-tests-obs-harness · §3 `boot` — ps1 mirrors the recorder; its ps1 live leg ran once by hand. → **apply** (:106; obs-plan §3 carries no spawn/exit claim — bind holds).
2. dependent-of 1 · §3 `status` — `ended` real under ps1 (`exit -1`). → **apply**.
3. D-tests-coverage · §1 `perf-slo-check-arm-coverage` — DISCHARGED. → **apply** (:106; plan expected).
4. dependent-of 3 · §10 load profiles — grader, not the deleted scripts. → **apply**.
5. dependent-of 3 · §3 per-chunk gate discipline — NEUTRAL analogy re-pointed to the grader. → **apply**.
6. D-tests-coverage · §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage` — widened: the ps1 boot recorder; its ended leg measured once by hand; the row stays OPEN. → **apply** — narrows the plan's expected "ps1 boot half discharged": the row's own standard ("one-time-proof-standing-in-for-a-gate") rules a by-hand leg a proof, not a committed test; recorded as a check-5 disposition (carried, narrowed).
7. D-tests-coverage · §1 `performance-budget: WebGPU canvas throughput` — frame budget's home is the dev-host `perf:frame-sample`; CI cannot-evaluate. → **apply** (:106).
8. dependent-of 7 · §10 frame-budget note — "all CI assertions" wording. → **apply**.
9. D-tests-coverage · §3 per-chunk gate discipline — record `perf:frame-sample` (dev-host, not CI) and `perf:budget` exit contracts. → **apply** (:106).
10. D-tests-framework · §9 lint-test row — Linux perf-samples + `perf:budget` + upload. → **apply** (:106; plan expected).
11. dependent-of 10 · §10 load profiles — `[profile.perf-samples]` + the default's second exclusion. → **apply**.
12. D-tests-framework · §9 release row — own saving `release-{os}` key; no CI frame leg. → **apply** (:106; plan expected).
13. dependent-of 12 · §9 lint-test cache cell — release no longer restores lint-test. → **apply**.

## obs-plan (12)
1. D-obs-defect-narrative · §10 CI gates perf-budget bullet — VACUOUS + owner retired; shipped gate; frame dev-host; no release-build frame fail. → **apply** (:106; plan expected; owner discharged by this chunk).
2. dependent-of 1 · §10 CI gates snapshot bullet — "assert max" → nearest-rank p99; formatting-only caveat. → **apply** (relay §2: the grader's one rule).
3. dependent-of 1 · §10 budgets snapshot row — jq floor rule → nearest-rank; the metric times `format_markdown` only; owner the minted entry. → **apply** (playbook :62 APPLY-AS-MEASURED: impl half owned by the route entry minted at this wrap's P5; relay §2 + §3).
4. dependent-of 1 · §10 budgets frame row — NEUTRAL/owner narrative retired; dev-host gate; CI named line; nearest-rank; no adapter-state record (owner the minted entry). → **apply**.
5. dependent-of 1 · §10 budgets memory row — CI-enforced via the required arm; gauge rows × 256 B, no RSS gate. → **apply** (plan expected).
6. dependent-of 1 · §1 perf-budget-instruments frame row — VACUOUS + owner retired. → **apply**.
7. dependent-of 1 · §1 perf-budget-instruments snapshot row — timer scope. → **apply** (:62).
8. dependent-of 1 · §5 snapshot metric row — timer scope. → **apply** (:62).
9. dependent-of 1 · §9 log artifact row — `logs-perf-samples-*`; no frame artifact. → **apply** (plan expected, frame half superseded).
10. dependent-of 1 · §10 load-profile constraints — NEUTRAL tolerance scoped to unrequired arms. → **apply**.
11. dependent-of 1 · §11 SLO ban example — no release-build frame fail. → **apply**.
12. D-obs-instrumentation · §1 multi-platform-exporter-compat — `app.boot.gpu.check` = `wgpu_backend` only; no adapter-state record (owner the minted entry). → **apply** (:106; plan expected "§8" — the only `gpu_available` site in obs-plan is this §1 row, grep 1 hit :132).

## Validate — the six checks
1. Playbook: 35 routine (rules :106, :62, :82); 0 escalations; 0 rule collisions.
2. Cross-contradiction: none — every section is edited by one proposal or by a `dependent-of` group moving one way.
3. Intent-consistency: the frame fallback diverges from the plan's (obs, CI) acceptance and is JUSTIFIED by the operator's recorded decision (`evidence/frame-gate-decision.md`); the scope record's 9 lines (5 in-intent, 4 companion) each serve a listed step or file — none new behaviour.
4. Absence-needs-evidence: the "0 hits" claims rest on P1's grep script (`sites.sh`, per-master counts in the report); the sweep in cascade step 2 re-checks every master.
5. Expected amendments: every plan entry is matched — obs §1/§10/§9 (1–9), obs §8 → the §1 :132 row (12), test §1 ×2 (3, 6 — narrowed), test §3 (1, 2), test §9 (10, 12, 13), test §10 (4, 11), arch CI/CD (7, 8), arch xtask CLI (1, 2, 4, 5), arch env vars (3), security carve-out (1, 2). The frame-leg parts of obs §9 and test §9 are superseded by the operator's decision.
6. Disproved claims: #1 snapshot timer → obs 3, 7, 8, 2 + the P5 minted entry; #2 p99 rule → obs 2, 3, 4; #3 plan premise → report only (chunk artifact); #4 `gpu_available` → obs 12. All DISPOSED.
