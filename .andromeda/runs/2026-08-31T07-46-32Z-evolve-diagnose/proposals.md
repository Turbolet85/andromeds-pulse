# Evolve Diagnosis — andromeda-pulse · Epoch 4 — Polish & ship: verification (epoch-to-date) · 2026-08-31T07:46Z

> Every count below is grounded in `.andromeda/friction-log.ndjson` (Epoch-4 slice, retraction-filtered); raw query outputs are the `q-*.json` twins beside this file. Epoch-to-date: the Conductor-return entry is still markerless — its future records are not in this slice. Obligation-free throughout: accept / reject / defer / modify, no mechanism-side consequence.

## Mechanism health

- **Records:** 1096 in-epoch (564 step / 532 friction) · 39 chunks · unparseable 0 · retracted 1 (0 unresolvable; id `2026-08-23T22:09:53Z-a`) · no retraction-of-retraction.
- **Null-epoch records:** 2 (2026-08-16, andromeda-wrap-session/curation + andromeda-wrap-session/curation, chunk:null — the 0-pending adaptation-wrap era; excluded from the slice by the epoch filter, era-legitimate).
- **Step coverage:** 36/39 chunks at the exact expected 5/3/5. Deviations (3): `2026-08-21-delegated-timing-observables` {'phase': 5, 'implement': 4, 'wrap-session': 5}; `2026-08-26-cadence-runaway-blocking-pool` {'phase': 4, 'implement': 3, 'wrap-session': 5}; `2026-08-28-duplicate-span-replay-fails-loudly` {'phase': 5, 'implement': 6, 'wrap-session': 5} — one MISSING phase/plan checkpoint (cadence-runaway: a checkpoint that did not fire), two extra-run cases (delegated-timing implement 4; duplicate-span-replay implement 6 — the mid-implement pivot re-entered steps).
- **Session starts:** 42 new-session orientation records; 12 chunk-null wrap step records (adaptation / 0-pending wraps).
- **Problem-fact fill:** 210/564 step records carry ≥1 deviation fact (fill is expected <100%). **Id fill:** 850/850 post-2026-08-18 records carry ids (100%).
- **Untyped rate:** 75/532 overall (14%). Hotspots: route-resolve 12/36, smoke 8/37, orientation 7/42, fix-loop 11/63, gates 6/32 — route-resolve's third is the strongest playbook-coverage signal (see the extension candidates).
- **Calibration boundaries in range:** `id` required from 2026-08-18 (E4 spans it; 246 earlier records read against their era). Deviation scan + universal types live for the whole epoch. Two deferred problem-facts carry EMPTY notes (capture defect — see L8).
- **Self-reference note:** this diagnosis session's own 2 orientation records (2026-08-31) are in the slice by design.

## Proposals (typed patterns)

### P1 — phase/distill/contract.extract-format  (+ merged: wrap-session/reconcile/contract.proposal-format) — 27 cases · weight 27 · 24 chunk(s)
**Pattern:** The sub-agent return transport HTML-entity-escapes `<` `>` `&` inside code spans — 3–7 of 7 distiller returns per batch, in 24 of 39 chunks at phase/distill plus 3 wrap fan-outs (27 cases; 23 summed decode iterations). Every occurrence was remedied by the documented decode-on-save; the expensive halves were the per-batch re-verification and the raw-twin duty (twins were repeatedly RECONSTRUCTED by inverse-encode and judged sanctioned — see level candidate L7).
**Evidence:** all 27 cases · summed impact: iterations=25 retries=1

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-15-corpus-key-persistence | The design distiller's first return opened with '# No domain coverage' as an h1 in place of the required '# design extract' header line, and ran 3 lines (below the >=4 minimum for a no-coverage retur… | retries=1 | — |
| 2026-08-21-delegated-timing-observables | Sub-agent return bodies arrive with HTML-entity-escaped angle brackets inside code spans, so extracts naming Rust generics, the TauRPC__<router.path> mock shape, or HTML elements persist mangled unle… | — | 2026-08-21T12:12:32Z-b |
| 2026-08-22-pii-scrubber-recall | three of four substantive returns arrived with HTML-entity-encoded angle brackets and ampersands (&lt; &gt; &amp;) in code spans such as <data_dir> and 'env > config.toml'; decoded at persist time, w… | — | .andromeda/runs/2026-08-22T15-23-16Z-phase/.raw-{arch,tests,security}.md |
| 2026-08-22-log-records-identity | 4 of 4 substantive distiller returns (arch, security, tests, obs) carried HTML entities in place of literal < > and & characters; the fan-out per-extract checks (header, section order, anchors, in-do… | — | the .raw-*.md twins in that run dir preserve the entity-encoded form |
| 2026-08-23-ingestion-scrub-coverage | the sub-agent return transport HTML-escaped <, > and & in 5 of 7 extracts (arch 1 char, security 5, tests 7, obs 2, a11y 10), mangling tier sets and path templates such as >45s, <data_dir> and TauRPC… | — | .andromeda/runs/2026-08-23T06-05-00Z-phase/ |
| 2026-08-23-metrics-points-identity | The subagent return transport HTML-escaped <, > and & in 3 of the 5 substantive extracts — arch (DELETE ... WHERE ts &lt; cutoff), security (Logging &amp; Monitoring), tests (&lt;data_dir&gt;/logs/ag… | — | post-decode grep for &lt;/&gt;/&amp; across all 7 stripped extracts returns 0; … |
| 2026-08-23-metrics-points-labels | the return transport HTML-escaped angle brackets and ampersands in SIX of the seven extracts in a single batch — arch, security, design, obs, a11y, tests; only layouts came back clean. fan-out.md rec… | — | 2026-08-23T10:27:49Z-b |
| 2026-08-23-webview-self-verify | The subagent return transport HTML-escaped <, > and & in 6 of the 7 extracts (arch, security, design, tests, obs, a11y), mangling TryFrom<u16>, cargo xtask <task>, <Icon glyph=.../>, <data_dir>/logs/… | iterations=6 | post-decode grep for &lt;/&gt;/&amp; returns 0 in all 7 stripped extracts |
| 2026-08-23-integration-ux-e2e-test | FIVE of seven distiller returns failed per-extract check 6 (no entity escapes) — the return transport HTML-escaped < > and & in arch, security, design, obs and a11y; only tests and layouts came back … | iterations=5 | 2026-08-23T14:37:51Z-b |
| 2026-08-23-a11y-verification | The subagent return transport HTML-escaped <, > and & in 3 of 7 extracts (design 2 entities, arch 4, a11y 13), failing per-extract check 6; fan-out.md prescribes decode-on-save rather than re-spawn, … | — | .andromeda/runs/2026-08-23T19-45-00Z-phase/ |
| 2026-08-23-headful-leg-extension | The subagent return transport HTML-escaped <, > and & in ALL SEVEN extracts (fan-out.md check 6, previously recorded as measured x4) — mangling code spans like <table>, <tr>, <section>, <STAGE>, <dat… | iterations=7 | 2026-08-23T23:19:21Z-b |
| 2026-08-24-headful-mechanics-probe-race-disposition | Four of seven returns (arch, security, tests, a11y) arrived HTML-entity-escaped on the return transport, mangling STAGE/task/data_dir angle-bracket templates, a tabindex greater-than comparison, and … | — | 2026-08-24T19:47:22Z-b |
| 2026-08-25-demo-injector-formalized-api-surface-retire | the entity-escape check fired on 7 of 7 extracts, not sporadically — every return carried `&gt;`, plus `&amp;` (security, a11y) and `&lt;` (tests, a11y); fan-out.md records this transport behaviour a… | — | post-save probe: entities=0 across all 7 files; 7/7 sections=7, lines 39-48 |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | Named check 6 (no entity escapes) fired on 5 of 7 returns — arch, security, tests, obs, a11y all carried &lt; &gt; &amp; from the subagent return transport, mangling comparison operators (>45s, >60s)… | — | .andromeda/runs/2026-08-26T07-48-30Z-phase/ |
| 2026-08-26-cadence-runaway-blocking-pool | Entity escapes hit EXACTLY 5 of 7 returns again — arch, security, tests, obs, a11y — with the two No-domain-coverage returns clean, reproducing the predecessor chunk distribution precisely. Two runs … | — | .andromeda/runs/2026-08-26T10-30-00Z-phase/ |
| 2026-08-26-l4-runtime-security-residuals | the return transport HTML-escaped entities in 3 of 7 extracts - arch 4, security 5, tests 2, eleven total (&amp; in section names like 'Logging & Monitoring', &lt;/&gt; in 'TryFrom<u16>' and '<data_d… | — | 2026-08-26T13:52:58Z-b |
| 2026-08-26-interpretation-brief-completeness | 6 of 7 distiller returns carried HTML entity escapes (&lt;/&gt;/&amp;) in the return transport — the known check-6 class fan-out.md records as measured x4, now x6 in one batch; remedied by the docume… | — | .andromeda/runs/2026-08-26T19-30-00Z-phase/.raw-*.md |
| 2026-08-27-idle-observer-generation-damper | 5 of 7 distiller returns arrived HTML-entity-escaped (check 6: &lt;/&gt;/&amp; in arch, security, a11y, tests, obs); decoded on save with raw twins per the documented remedy, no re-spawn — fan-out.md… | iterations=5 | 2026-08-27T17:30:34Z-b |
| 2026-08-27-report-window-copy-affordance | the subagent return transport HTML-escaped <, > and & in SIX of the seven extracts (arch, security, design, a11y, tests, obs — layouts alone arrived clean), mangling CLI templates (--expect-absent <S… | — | the six .raw-*.md twins in that run dir carry the escaped forms; the stripped s… |
| 2026-08-27-incident-persist-vs-resolve-write-race | The return transport HTML-escaped <, > and & in FIVE of seven extracts (arch, security, tests, layouts, a11y) — mangling <section>, <data_dir>, TauRPC__<router.path> and every Logging & Monitoring an… | — | .andromeda/runs/2026-08-27T22-35-43Z-phase/.raw-*.md |
| 2026-08-28-ingest-consumer-initiating-freeze | The subagent return transport HTML-escaped &, < and > in 4 of the 7 extracts (security 3x &amp; in section names like Logging & Monitoring; arch 4x across the env-precedence chain and cargo xtask <ta… | — | .andromeda/runs/2026-08-28T17-05-00Z-phase/ |
| 2026-08-28-ingest-consumer-block-under-gap-resume | Validation check 6 (no entity escapes) failed on 4 of 7 returns — the transport HTML-escaped ampersand in security/obs section titles (Logging &amp; Monitoring, Isolation &amp; Load-Profile) and the … | — | .andromeda/runs/2026-08-28T19-05-00Z-phase/ |
| 2026-08-28-duplicate-span-replay-fails-loudly | 3 of 7 distiller returns arrived HTML-escaped by the return transport (13 chars total across security/tests/obs: ampersand, less-than, greater-than), mangling a Rust generic and two section titles; t… | — | raw twins .raw-security.md (7) / .raw-tests.md (1) / .raw-obs.md (5) preserve t… |
| 2026-08-29-app-registry-reconciliation | Validation check 6 (no entity escapes) failed on SIX of seven returns in a single batch — arch, security, design, tests, obs, a11y all carried &lt; / &gt; / &amp; mangling generics, path templates an… | — | .andromeda/runs/2026-08-29T11-35-47Z-phase/.raw-*.md (6 twins written, layouts … |
| 2026-08-23-integration-ux-e2e-test | TWO of the three proposal-carrying returns HTML-escaped angle brackets inside YAML string fields — a11y-plan emitted &lt;region aria-label=...&gt; and test-plan emitted &lt;STAGE&gt;, &gt; 0 and Opti… | iterations=2 | 2026-08-23T17:03:52Z-b |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | Two of the seven returns (security-plan, obs-plan) carried HTML entity escapes from the subagent return transport — &amp; / &lt; / &gt; inside section headings and code spans (e.g. §8 PII Scrubbing &… | — | .andromeda/runs/2026-08-26T09-45-00Z-wrap/fanout-results.md |
| 2026-08-27-incident-persist-vs-resolve-write-race | All seven doc-agent returns arrived HTML-entity-escaped by the return transport — &lt; &gt; &amp; inside the YAML payloads, mangling the arch proposals `<= ?2` predicate and `Result<IncidentWriteOutc… | — | 2026-08-28T16:08:55Z-b |

**Proposal:** Make entity-decode a MECHANICAL default of the fan-out persist path (scripted decode + `entities=0` probe, which several chunks already ran ad hoc), and codify reconstructed-by-inverse-encode raw twins in fan-out.md as the sanctioned form. The transport itself is harness-level — not reachable from skill text — so the skill-side goal is zero per-batch judgment cost.

### P2 — implement/fix-loop/tooling.environmental  (+ merged: implement/smoke/tooling.environmental) — 17 cases · weight 22 · 15 chunk(s)
**Pattern:** One environment family dominates: parallel-link memory pressure (rust-lld IO failure · os 1453 · 0xc000012d/os 1455) and target/ disk accumulation (up to 268G; `cargo clean` recovered 62k files/225G/87G/180G/290G across the epoch — see the six removed-cause facts), plus regen/fingerprint rebuild bursts (CARGO_INCREMENTAL, bindings). 17 cases across 15 chunks, all recovered per-instance.
**Evidence:** all 17 cases · summed impact: iterations=11 retries=10

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-workspace-key-alignment | cargo build --workspace --tests died twice on rust-lld (once at --jobs 4, once at --jobs 2) with LLVM ERROR: IO failure on output stream: no space on device; the linker crash text reads like a memory… | retries=2 | — |
| 2026-08-15-corpus-key-persistence | cargo audit — a gate this project's CI runs — fails before scanning anything: 'error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244', reproduced on a second run. The… | retries=1 | — |
| 2026-08-22-pii-scrubber-recall | disk exhaustion surfaced as an LLVM linker IO error rather than a disk error, and only during the link of the ~30-binary pulse-app test set; the free-disk.ps1 dry-run measured only ~13GB reclaimable … | retries=1 | 2026-08-22T17:54:34Z-c |
| 2026-08-23-ingestion-scrub-coverage | the bindings-clobber defect recurred again on a chunk that touches NO TauRPC surface: running the plan-mandated default-features cargo nextest run --workspace rewrote pulse-app/ui/src/bindings/index.… | iterations=1 | target/capability-drift/report.json |
| 2026-08-23-webview-self-verify | cargo nextest run --workspace failed to compile every pulse-app test target with crate wasmtime / libduckdb_sys / triage required to be available in rlib format, but was not found in this form — acro… | iterations=2 | cargo build -p pulse-app recompiled 5 crates and finished clean immediately bef… |
| 2026-08-25-demo-injector-formalized-api-surface-retire | disk exhaustion presented as a LINKER failure, matching the documented signature exactly: cargo reported `error: linking with rust-lld.exe failed: exit code 1` for one pulse-app test binary, and the … | retries=1 | background task exit 101; grep of the output found the LLVM ENOSPC line under t… |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | capability-drift went RED with 3 missing mcp.* procedures purely because the default-features nextest gate REWROTE pulse-app/ui/src/bindings/index.ts to the no-mcp shape — the chunk touches no TauRPC… | iterations=1 | cargo xtask capability-drift: drifted (3 missing) -> clean after regen |
| 2026-08-26-cadence-runaway-blocking-pool | cargo xtask capability-drift failed with the same 3 missing mcp.* procedures again — the taurpc dev-mode export_types clobber from the default-features workspace nextest; cleared by the documented --… | iterations=1 retries=1 | capability-drift exit 1 (3 missing) then exit 0 clean after regen; git diff --q… |
| 2026-08-27-idle-observer-generation-damper | background workspace nextest task externally killed mid-compile with no failure signature; cause unknown, re-run green | retries=1 | 2026-08-27T19:58:33Z-c |
| 2026-08-28-ingest-consumer-initiating-freeze | cargo fmt --check was red on the new file after the PostToolUse rustfmt hook had already formatted it - the known edition mismatch (the bare hook parses as edition 2015, cargo fmt passes the workspac… | iterations=1 | cargo fmt --check diff at gap_resume.rs:260 |
| 2026-08-29-halo-state-pulse-signature-deferred | cargo nextest workspace run failed at compile: could not exec rust-lld.exe, os error 1453 Insufficient quota, while linking the xtask test binary in a parallel link burst on the warm target; retry vi… | retries=1 | background task outputs bujkohh4z (fail) and b1f655bvc/bkv82oim2 (green retry) |
| 2026-08-29-advisory-backlog | disk-full: target/ had accumulated 268G of artifact generations across months; the wasmtime 43+46 coexistence tipped D: to 100pct and rust-lld failed with LLVM no-space-on-device; cargo clean (290GiB… | iterations=1 | scratchpad/nextest-run2.log |
| 2026-08-29-advisory-backlog | NEW PRESENTATION of the parallel-link memory-pressure family: STATUS_COMMITMENT_LIMIT 0xc000012d + os error 1455 paging-file-too-small during the pulse-app feature-graph test-binary link set, cascadi… | iterations=1 | scratchpad/mcp-legs2.log |
| 2026-08-30-dead-lib-src-test-migration | a nextest list invocation missing the session's CARGO_INCREMENTAL=0 prefix re-fingerprinted the workspace and burst-rebuilt unthrottled, hitting the known memory-pressure family (rust-lld 0xc000001d … | iterations=1 | 2026-08-30T19:54:19Z-b |
| 2026-08-23-webview-self-verify | The 300 GB D: dev drive reached 0.1 GB free during the post-fix gate re-run, after this session had driven many full and targeted rebuilds. It surfaced exactly as the standing learning predicts — rus… | iterations=2 | Get-PSDrive showed D: at 0.1 GB free; after the targeted clean the rebuild and … |
| 2026-08-23-a11y-verification | cargo xtask webview-drive SKIPPED clean at exit 0 on first invocation because ANDROMEDA_PULSE_MSEDGEDRIVER_PATH was not visible in the Bash shell, even though it IS set at the Windows User level and … | retries=1 | powershell [Environment]::GetEnvironmentVariable returned the path while bash s… |
| 2026-08-29-app-registry-reconciliation | inject_demo --sustained is structurally incapable of forming an incident: it holds a CONSTANT error rate, so the cue evaluators short/long EWMA ratio converges and never crosses its gate — measured a… | retries=1 | /tmp/leg-green2.txt — INCONCLUSIVE, sidecar listed no active incident within 24… |

**Proposal:** Partly absorbed project-side (`--jobs 4` + build-first de-race curated; cargo-clean pre-authorized). Remaining pipeline generalization: promote the remedy from remembered rules to a mechanical hygiene surface — e.g. a target/-size + free-disk line in the new-session health checks (warn at threshold) and a documented clean cadence — so the family stops being rediscovered mid-gate.

### P3 — phase/validate/contract.mechanical-check — 16 cases · weight 14 · 15 chunk(s)
**Pattern:** P5's mechanical pass fired in 15 chunks — almost every case the gate WORKING as designed. The recurring sub-causes it keeps catching: acceptance criteria with no producing invocation in Test Commands, and Expected-amendments lists naming leaves or immutable artifacts (the masters-only rule re-learned per chunk).
**Evidence:** all 16 cases · summed impact: iterations=13 extra_reads=4

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-fingerprint-feed-capture-repair | checks 3 (new-file parent resolvable) and 5 (no placeholder leak) both fired on the same two entries — the guard-test home and the repair site were written as brace placeholders because a diagnose-th… | iterations=1 | — |
| 2026-08-15-corpus-key-persistence | Mechanical check 4's REQUIRED-RESOLUTION fired on three acceptance criteria whose evidence no listed gate command can produce, and each had a DIFFERENT legitimate reason: the arm-zero evidence is ope… | — | — |
| 2026-08-22-log-records-identity | mechanical check 4 REQUIRED-RESOLUTION fired: the plan asserted an acceptance criterion for an end-to-end leg driven over the real OTLP receiver, but Test Commands listed only the app launch and no p… | iterations=1 | plan.md Test Commands, step 9 live-leg block |
| 2026-08-23-ingestion-scrub-coverage | mechanical check 4 REQUIRED-RESOLUTION fired and worked as designed: four acceptance criteria asserted artifacts under <data_dir>/logs/agent-latest.jsonl* while the listed live leg (SCENARIO=scrub-ca… | iterations=1 extra_reads=3 | andromeda-pulse-0.3.0/chunks/2026-08-23-ingestion-scrub-coverage/plan.md §Test … |
| 2026-08-23-metrics-points-identity | The plan shipped to validation with an acceptance criterion asserting canary absence in agent-latest.jsonl while Test Commands listed no invocation that produces it — the producing direct-binary boot… | iterations=1 | pre-edit Test Commands listed only cargo gates + the example producer + a bare … |
| 2026-08-23-integration-ux-e2e-test | Mechanical check 2b FAILED: the plans Expected amendments (wrap) block listed intent.md, which is an IMMUTABLE artifact rather than one of the seven spec masters, so wraps amendment flow does not and… | iterations=1 | 2026-08-23T16:04:21Z-b |
| 2026-08-23-headful-leg-extension | P4 synthesis left an unsubstituted {name}.rs placeholder in the New-files path (a conditional the plan deferred to implement: pick a test-file home if no existing one fits), which plan-template forbi… | iterations=1 | 2026-08-23T23:37:42Z-b |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | Check 4 required-resolution fired: a security acceptance criterion asserted cargo deny check bans licenses sources exits 0 and cargo deny check advisories is observed separately, while neither invoca… | iterations=1 | andromeda-pulse-0.3.0/chunks/2026-08-26-ingest-consumer-stall-under-sustained-l… |
| 2026-08-26-l4-runtime-security-residuals | P3's scope premise closure left three dangling [inferred] tags. The closure instruction says to re-read scope.md's [inferred] BULLETS and verify or correct each, and that was done - the premises sect… | extra_reads=1 | grep -c '\[inferred\]' scope.md returned 3 after the P3 closure, at lines 41, 8… |
| 2026-08-26-l4-runtime-security-residuals | the plan's 'Expected amendments (wrap)' block was authored with the five Tier-2/3 cascade leaves (rules/security.md, docs/security-summary.md, CLAUDE.md, rules/observability.md, docs/obs-summary.md) … | — | 2026-08-26T16:52:08Z-c |
| 2026-08-28-ingest-consumer-initiating-freeze | Mechanical check 4s producing-invocation rule fired on the audit PREREQ acceptance criterion: the criterion asserts the full-form cargo audit probe is discharged, but no listed Test Command produces … | — | andromeda-pulse-0.3.0/chunks/2026-08-28-ingest-consumer-initiating-freeze/plan.… |
| 2026-08-28-ingest-consumer-block-under-gap-resume | Check 2 failed on the Expected-amendments half: the plan listed .claude/rules/observability.md beside obs-plan.md §10 because BOTH carry the same false mechanism verbatim, but only the latter is a sp… | iterations=1 | plan.md Implementation notes, Expected amendments (wrap) |
| 2026-08-28-duplicate-span-replay-fails-loudly | The scripted mechanical pass reported 0 FAILs and 0 WARNs while two acceptance criteria had no producing invocation in Test Commands — the cargo deny advisories criterion naming an owned id set no li… | iterations=2 | resolved both ways the rule allows — the two cargo deny invocations added as se… |
| 2026-08-30-staged-bindings-assertion | plan.md Expected-amendments list initially named two non-master artifacts (.andromeda/playbook.md re-point + rules/security.md cascade leaf); mechanical check 2 caught it pre-review and one plan edit… | iterations=1 | 2026-08-30T11:47:50Z-b |
| 2026-08-30-acl-rejection-logging | check 4 fired on the freshly-authored plan: observability.rs is on the declared boot-smoke trigger list while Test Commands carried the smoke only as P3 prose (the check's own measured-example trap, … | iterations=1 | andromeda-pulse-0.3.0/chunks/2026-08-30-acl-rejection-logging/plan.md |
| 2026-08-30-agent-harness-teardown-truth | P4 draft tripped three mechanical checks before review: Expected-amendments named the rules/verification-harness.md LEAF as a peer amendment (masters-only rule), two placeholder-lookalike notations q… | iterations=1 | 2026-08-30T21:17:33Z-b |

**Proposal:** Move the two recurring sub-causes upstream into the P4 synthesis template as explicit authoring checks (per-criterion producing-invocation; masters-only Expected-amendments), so validate stops being their first reader. The check itself is healthy — keep it as the backstop.

### P4 — implement/fix-loop/contract.spec-reality-gap — 12 cases · weight 15 · 10 chunk(s)
**Pattern:** Fix-loop measurement falsified spec/plan premises in 10 chunks (2 soft-exits): dead surfaces, wrong causal mechanisms, recipes that do not reach the harness. Each was resolved by the designed paths (soft-exit, premise-closure, operator fork).
**Evidence:** all 12 cases · summed impact: iterations=8 extra_reads=6 soft_exit=2

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-workspace-key-alignment | the corpus AES-256-GCM key does not survive a process boundary: Cargo.lock shows keyring 3.6.3 resolving with dependencies [log, zeroize] only — no platform credential-store backend is linked, so key… | — | andromeda-pulse-0.3.0/chunks/2026-08-14-workspace-key-alignment/evidence/after-… |
| 2026-08-16-fault-identity-semantics-decided | Incident.fingerprint has two incompatible in-tree contracts. Documented at crates/triage/src/contract.rs:340 as an 'Anonymized fingerprint hash for cross-incident grouping per capability spec P-047',… | iterations=2 | nextest FAIL rows assemble_populates_corpus_matches_from_fingerprint_match @ as… |
| 2026-08-21-delegated-timing-observables | Surfaced, not authored: the P-025 rendered surface (a Halo State Pulse canvas) does not exist in production, so record_halo_hue_latency ships with no call site and is forward-inert. design-system §Br… | — | 2026-08-21T12:52:42Z-d |
| 2026-08-21-delegated-timing-observables | RESOLUTION of the gap recorded at 2026-08-21T12:52:42Z-d: the operator confirmed the orphan independently and directed option (a). P-025 now measures the severity-driven hue where severityToHueFracti… | iterations=1 | andromeda-pulse-0.3.0/chunks/2026-08-21-delegated-timing-observables/plan.md |
| 2026-08-23-integration-ux-e2e-test | The plans acceptance says the leg drives the path UNDER DETERMINISTIC-L4, and the matrix acceptance repeats it, but neither the plan nor research said HOW the mode reaches the app under test — which … | iterations=1 | 2026-08-23T16:35:26Z-c |
| 2026-08-23-a11y-verification | security-plan section Dependency Security states the owned upgradeable advisory set as SEVEN IDs; cargo deny check advisories measured EIGHT at HEAD (0189/0190/0194/0195/0204/0222/0253/0258), matchin… | — | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/plan.md#implementatio… |
| 2026-08-27-idle-observer-generation-damper | GREEN leg surfaced a pre-existing persist-vs-resolve write race: the 60s incident persist cycle list-then-write span overlaps the phase-aligned auto-resolve tick, so stale active rows clobber resolve… | extra_reads=6 | green-leg log: persist cycle 19:21:59.942 count=5 dur=74 vs observer resolution… |
| 2026-08-27-report-window-copy-affordance | the documented warm-re-embed recipe does not reach the two headful harnesses. rules/testing.md (2026-07-05 boot-smoke entry) prescribes npm run build + cargo build -p pulse-app to re-embed the fronte… | iterations=3 | three consecutive GREEN-arm legs reported copy_state error with copy_rejection … |
| 2026-08-28-ingest-consumer-block-under-gap-resume | Three durable artifacts (the working-route entry, obs-plan §10 defect 4, rules/observability.md) frame this as a consumer/connection-isolation defect triggered by an idle gap. Measured: the idle gap … | soft_exit=1 | target/gap-resume/run-56020 + run-57376 + run-41760 probe and log analysis; one… |
| 2026-08-28-duplicate-span-replay-fails-loudly | The plan selected fork requires a returning flush error to hook a connection reset onto; the duckdb 1.10502 Appender exposes only flush add_column and clear_columns over a direct FFI call to duckdb_a… | soft_exit=1 | read the vendored duckdb-1.10502.0 appender/mod.rs public API and Drop impl; su… |
| 2026-08-28-duplicate-span-replay-fails-loudly | An in-repo comment at crates/buffer/src/schema.rs 320-323 and the test-plan section 4 text that cites it both assert a duplicate-INSERT probe hangs on libduckdb-sys 1.10502, which is the stated reaso… | — | surfaced for wrap rather than authored; the appender-path hang itself is now fi… |
| 2026-08-30-diagnostics-un-muting-harness-truth-sweep | the mechanical zero-src-test guard found ~101 dead lib-src #[test] fns across 14 pulse-app files beyond the 130 the route entry named in observability.rs (heartbeat 25, window 13, diagnostics_router … | iterations=1 | /tmp/nextest.txt first-run offender list |

**Proposal:** No separate mechanism change — this group is the fix-loop face of the cross-step `contract.premise-falsified` class; see PU1 for the aggregate direction. Evidence kept here for the per-step picture.

### P5 — wrap-session/reconcile/ambiguity.playbook-no-match — 5 cases · weight 25 · 5 chunk(s)
**Pattern:** Five reconcile passes hit dispositions the drift playbook could not decide — two rules matching with opposing verdicts, a class with no rule, and two unease-escalations — costing 4 halts and 6 dialogue rounds.
**Evidence:** all 5 cases · summed impact: dialogue_rounds=6 halted=4

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-fingerprint-feed-capture-repair | the D-tests-framework group matched TWO playbook rules pointing opposite ways — line-50 routine-APPLY (accurate this-chunk correction inside a documented structure) and the 2026-06-29 routine-HANDOFF… | dialogue_rounds=2 halted=1 | — |
| 2026-08-15-corpus-key-persistence | The cargo-audit failure had no sanctioned disposition in a 10-rule playbook: it is not Foundation-sequencing (no LATER CHUNK owns it — an upstream does) and not a tooling.environmental soft-exit (the… | dialogue_rounds=1 | — |
| 2026-08-16-baseline-family-reachability | D-obs-pii fired at escalate severity on two accurate registry-completeness proposals (a new allowlist leaf + its log-level twin). The playbook had a rule matching the SUBSTANCE (2026-07-08 routine-AP… | dialogue_rounds=1 halted=1 | — |
| 2026-08-16-fault-identity-semantics-decided | One proposal matched TWO playbook rules with opposing verdicts, so the playbook could not decide it. The 2026-08-16 D-obs-pii rule states its two conditions 'must BOTH hold, else escalate'; condition… | dialogue_rounds=1 halted=1 | playbook rules dated 2026-08-16 and 2026-08-15; guard at observability.rs:5849-… |
| 2026-08-23-a11y-verification | Two of 19 proposals escalated by UNEASE rather than by rule: narrowing a11y-plan section 11 NEVER-nest-focusables ban to nested TAB STOPS, and scoping test-plan section 2 Playwright-UNADOPTED to conn… | dialogue_rounds=1 halted=1 | .andromeda/runs/2026-08-23T22-15-00Z-wrap/fanout-results.md |

**Proposal:** Each halt minted a rule at the time (drift-base grew), which is the designed loop. Direction worth judging: a standing tiebreaker for the two-rules-oppose shape (most-specific-wins, or escalate-with-both-cited) would remove the commonest recurrence.

### P6 — wrap-session/route-resolve/ambiguity.trajectory-halt — 8 cases · weight 16 · 8 chunk(s)
**Pattern:** The new-chunk-ahead trajectory class halted (or formally required a halt) in 8 chunks; in several the operator had ALREADY directed the entry in a prior directive, making the halt a one-round formality.
**Evidence:** all 8 cases · summed impact: dialogue_rounds=6 halted=1

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-16-fault-identity-semantics-decided | One trajectory halt, class new-chunk-ahead: minting an own route entry for the producer defect the chunk surfaced. Resolved in a single round. The operator had pre-supplied the disposition (own entry… | dialogue_rounds=1 halted=1 | working-route.md line 60 (new first markerless entry) vs the 6 pre-existing mar… |
| 2026-08-23-metrics-points-labels | one HALT of class new-chunk-ahead, resolved in a single round. The follow-up — no webview surface renders the labels this chunk made readable — had no owning markerless entry, so the choice was a tai… | dialogue_rounds=1 | 2026-08-23T11:35:20Z-b |
| 2026-08-23-webview-self-verify | Adding the npm-advisory-coverage entry is a NEW-CHUNK-AHEAD edit, which the gradient classes as trajectory and would normally HALT into dialogue. It did not halt, because the operator had already app… | — | fanout-results.md §Escalations row E2 |
| 2026-08-23-integration-ux-e2e-test | ONE trajectory halt, one round, class new-chunk-ahead: TEN CARRYs had no owner entry. Seven were boundaried out of this chunk because each needs a driver MECHANIC the assembled path does not use, and… | dialogue_rounds=1 | 2026-08-23T17:59:53Z-b |
| 2026-08-25-demo-injector-formalized-api-surface-retire | one trajectory halt, one round, resolved — but the interesting part is that HALF of it was already settled upstream. E1 needed no asking: the operator's resolution of the P2 R3 escalation explicitly … | dialogue_rounds=1 | P2 escalation resolution chose 'escalate wider - bundle with the argv residual'… |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | A new-chunk-ahead insertion is trajectory and normally HALTs, but the operator had already directed both the entry and what its research must own (the TRIGGER question — which conditions admit the ru… | — | 2026-08-26T10:11:45Z-b |
| 2026-08-27-incident-persist-vs-resolve-write-race | One trajectory halt, one round, class new-chunk-ahead. The DISPOSITION was already settled — the operator picked the /phase P4 option whose own text says a new route entry owns reconciliation — so th… | dialogue_rounds=1 | 2026-08-28T16:36:52Z-b |
| 2026-08-30-agent-harness-teardown-truth | markerless tail went empty mid-version (one unclaimed cap remaining) with two obligations needing a carrier; the closing chunk existed only as handoff prose, never a route entry — resolved by operato… | dialogue_rounds=1 | 2026-08-31T06:06:18Z-b |

**Proposal:** Let a recorded operator pre-direction (a directive naming the entry/disposition) satisfy the trajectory gate without a fresh halt; the gradient keeps its force for genuinely new trajectory. See also level candidate L4 (the override signature on this same rule).

### P7 — phase/plan/ambiguity.scope-question — 8 cases · weight 15 · 8 chunk(s)
**Pattern:** 8 plan dialogues; 7 were genuinely the operator's call (placement forks, verification depth, guard shape) and resolved in one round each. One bundled a decisive lean as a question.
**Evidence:** all 8 cases · summed impact: dialogue_rounds=7

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-fingerprint-feed-capture-repair | whether the feed instrumentation ships permanently or is diagnostic scaffolding was not answerable from the materials — obs-plan mandates boundary logging but does not decide permanence for a diagnos… | dialogue_rounds=1 | — |
| 2026-08-15-corpus-key-persistence | One dialogue round, two questions, both genuinely the operator's call and neither answerable from the materials. Q1 (retire-HOW): the operator directive named a two-pole fork (silent purge vs in-plac… | dialogue_rounds=1 | — |
| 2026-08-16-baseline-family-reachability | Two questions were bundled into one round; the second (verification depth) was closer to a DECISIVE lean than a true fork - the projects own curated lesson from the corpus-key chunk (an in-process te… | dialogue_rounds=1 | — |
| 2026-08-23-integration-ux-e2e-test | One dialogue round, two questions, both genuinely the operators call rather than answerable from the materials. (1) Selector convention: a11y-plan mandates accessible names for the Traces surface whi… | dialogue_rounds=1 | 2026-08-23T15:58:04Z-c |
| 2026-08-24-headful-mechanics-probe-race-disposition | Half B path selection was a genuine three-way fork (measure / guard / instrument-then-measure) that the materials could only PARTLY answer: research decisively eliminated path (a) as worded, but choo… | — | 2026-08-24T19:57:02Z-b |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | The enforcement-level question could not be settled from the materials because they point different ways: obs-plan §10 acceptance requires the condition be caught by a machine assertion that FAILS ra… | dialogue_rounds=1 | 2026-08-26T08:48:56Z-b |
| 2026-08-27-incident-persist-vs-resolve-write-race | One dialogue round, two questions. Q1 (guard placement/shape) was narrowed decisively by research to a single surviving form but the trade-off — a guarded UPDATE declines silently, so declines need a… | dialogue_rounds=1 | 2026-08-27T23:01:56Z-b |
| 2026-08-29-app-registry-reconciliation | Two questions in one round. Q1 (reconciler placement/cadence) was genuinely the operator call: the artifacts bound the DAG and the obs cost of each option but none ranks a 60s-coupled fold against a … | dialogue_rounds=1 | plan.md §Constraints & rejected approaches (both rejected placements recorded w… |

**Proposal:** None — the dialogue is doing its job at ~1 round per chunk. Minor direction: frame decisive leans as leans (recommend-first) per the one bundled case.

### P8 — implement/fix-loop/retry.fix-iterations — 6 cases · weight 19 · 6 chunk(s)
**Pattern:** Six chunks logged iteration counts at or above the protocol's typical band (17 summed; worst case 6 iterations, every one a distinct root cause in the headful harness), plus one deliberate near-soft-exit continuation.
**Evidence:** all 6 cases · summed impact: iterations=17 retries=1

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-fingerprint-feed-capture-repair | one iteration, formatting only: cargo fmt --check flagged three files the PostToolUse rustfmt hook had already run over — the hook invokes bare rustfmt per file, which parses as edition 2015 and so d… | iterations=1 | — |
| 2026-08-23-integration-ux-e2e-test | Six iterations to green, above the protocols 1-5 typical band, every one a DISTINCT root cause in the harness and none a repeat after a fix — so continuation stayed correct rather than tripping Trigg… | iterations=6 | 2026-08-23T16:35:26Z-b |
| 2026-08-23-headful-leg-extension | The extended leg needed 3 iterations to green: run 1 exposed the per-boot navigation race on a NON-main window (findings blank) plus two vacuous sub-predicates (hidden-window dialog probe, click-focu… | iterations=3 | andromeda-pulse-0.3.0/chunks/2026-08-23-headful-leg-extension/ (report will car… |
| 2026-08-24-headful-mechanics-probe-race-disposition | Three consecutive fix attempts raised DRIVE_TIMEOUT (420 to 660 to 1200) on a misdiagnosis: the leg was read as slow because two runs stopped at the same visible stage, but node block-buffers stdout … | iterations=3 | /tmp/wd-red3.out |
| 2026-08-26-l4-runtime-security-residuals | clippy::assertions_on_constants rejected assert!(MAX_PROMPT_BYTES < 32_767) inside a #[test] - a constant-vs-constant assertion. This is a DOCUMENTED trap with a documented remedy (the 2026-05-11 ses… | iterations=1 retries=1 | 2026-08-26T17:15:29Z-b |
| 2026-08-27-report-window-copy-affordance | soft-exit Trigger 1 (same failure recurring after distinct fixes) NEAR-FIRED and continuation was chosen deliberately. The identical signature — copy_state error, same stage, same assertion — recurre… | iterations=3 | 2026-08-27T21:44:35Z-d |

**Proposal:** None mechanism-side — the counts contextualize harness-heavy chunks (webview legs dominate the tail). The soft-exit triggers behaved; the one near-fire chose continuation deliberately and recorded it.

### P9 — implement/smoke/tooling.harness-friction — 7 cases · weight 11 · 7 chunk(s)
**Pattern:** Seven chunks routed around the agent-run harness: `boot` forced a cold `cargo run --release` rebuild, `status` returned a well-formed envelope about no process (exit 0 with no app), `cleanup` killed the wrapper and reported done. The smokes fell back to direct-binary launches every time.
**Evidence:** all 7 cases · summed impact: retries=10 extra_reads=4

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-workspace-key-alignment | the 5-command harness could not drive this smoke: its boot verb is cargo run --release (a cold release rebuild) while the smoke needed the warm debug binary just built, and its status verb reported a… | — | — |
| 2026-08-15-tier-1-incident-path-investigation | scripts/agent-run.sh status returned exit 0 and a well-formed status envelope describing pid 25512 with uptime_ms 0, subsystems 'initialized' and last_tick_at null, while the process actually under t… | — | — |
| 2026-08-16-fault-identity-semantics-decided | A listed gate reported green about the wrong process, live. bash scripts/agent-run.sh status exited 0 with a well-formed envelope naming pid 19008, uptime_ms 0 and last_tick_at null, while the pulse-… | extra_reads=1 | smoke data dir run/andromeda-pulse.pid = 31308 vs agent-run.sh status pid = 190… |
| 2026-08-23-ingestion-scrub-coverage | the agent-run.sh boot verb could not be used for a wire proof that needs (a) the already-built debug binary rather than a fresh release build and (b) a caller-supplied fresh data dir, since the verb … | extra_reads=1 | andromeda-pulse-0.3.0/chunks/2026-08-23-ingestion-scrub-coverage/plan.md §Test … |
| 2026-08-23-metrics-points-identity | scripts/agent-run.sh status exits 0 and emits a full health payload even when no app is running, naming a stale pid with uptime_ms 0. A gate keyed on that verb's exit code would read GREEN against a … | extra_reads=2 | status returned pid 46740 / uptime_ms 0 with both ports closed and PowerShell r… |
| 2026-08-23-a11y-verification | The shipped 7-stage webview-drive leg is FLAKY at its launch stage on this host: the main dashboard window intermittently never leaves about:blank, so neither the 20s self-mount wait nor the 30s post… | retries=9 | nine webview-drive runs this session; launch observed=false in seven of them ac… |
| 2026-08-30-acl-rejection-logging | agent-run.sh cleanup exit 0 with 'port still accepting' warnings and a surviving pulse-app child — the kill hit the wrapper PID; manual Stop-Process -Id closed it; also the 180s HARNESS_STATUS_TIMEOU… | retries=1 | boot.log exit 143 mid-rustc; cleanup.txt port warnings; orphan pid 59976 |

**Proposal:** ABSORBED BY THE PROJECT at epoch end: `harness:status` real-process verdict (2026-08-30-diagnostics-un-muting) + boot pre-build/direct-spawn + independent-probe cleanup (2026-08-30-agent-harness-teardown-truth). Nothing further proposed; residual: the ps1 arm's live drive is owed via the test-plan §1 trigger.

### P10 — wrap-session/gates/tooling.commit-mechanics — 9 cases · weight 8 · 9 chunk(s)
**Pattern:** The TauRPC bindings-clobber at the P7 light gate recurred across 9 wraps (any default-features cargo run rewrites bindings.ts to the no-mcp shape; the ordering rule then caught it each time). Eight more untyped fix-loop records carry the same event mid-implement — see extension candidate U-A.
**Evidence:** all 9 cases · summed impact: iterations=1 retries=4 dialogue_rounds=1

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-fingerprint-feed-capture-repair | the wrap's OWN P7 light gate clobbered the TauRPC bindings: cargo nextest run --workspace under default features re-ran emit_taurpc_bindings and rewrote bindings/index.ts to the no-mcp shape, turning… | retries=1 | capability-drift report.json |
| 2026-08-16-fault-identity-semantics-decided | The light gate structurally re-broke the TauRPC bindings it had to ship correct. cargo nextest run --workspace at P7 is a default-features cargo op, so it regenerated pulse-app/ui/src/bindings/index.… | iterations=1 | grep -c '"mcp":' = 0 after the light gate, 1 after regen; git show :pulse-app/u… |
| 2026-08-17-conductor-e2e-verification-closure | the light gate's workspace nextest clobbered the TauRPC bindings back to the no-mcp shape for the second time this session, exactly as the standing discipline predicts, so the regen had to run again … | — | — |
| 2026-08-23-metrics-points-identity | The TauRPC bindings clobber fired a SECOND time within this one session, at its other known trigger: the wrap's own P7 light gate is a default-features workspace nextest and runs AFTER the implement-… | retries=1 | post-light-gate grep -c '"mcp":' returned 0; after regen the worktree read 1 an… |
| 2026-08-23-integration-ux-e2e-test | The commit was deliberately HELD at P7 until an out-of-repo host action completed — the msedgedriver relocation out of a dead sessions %TEMP% scratchpad to a durable path. The reason was not the dire… | dialogue_rounds=1 | 2026-08-23T19:29:23Z-b |
| 2026-08-25-demo-injector-formalized-api-surface-retire | the bindings-clobber defect recurred and the ordering rule CAUGHT it, which is the point worth recording. The playbook's 2026-08-22 entry exists because this shipped five times past four rules-file r… | retries=1 | staged mcp count 0 with capability-drift missing [mcp.start, mcp.status, mcp.st… |
| 2026-08-26-l4-runtime-security-residuals | the light gate's own default-features nextest rewrote pulse-app/ui/src/bindings/index.ts to the no-mcp shape AFTER implement had already regenerated it, so capability-drift would have shipped red wit… | retries=1 | 2026-08-26T19:13:41Z-b |
| 2026-08-27-incident-persist-vs-resolve-write-race | The bindings clobber fired a SECOND time this session, exactly as the playbooks 2026-08-22 ordering rule predicts: the light gates own nextest --workspace rewrote pulse-app/ui/src/bindings/index.ts t… | — | 2026-08-28T16:42:18Z-b |
| 2026-08-28-ingest-consumer-block-under-gap-resume | The flip-compaction ordering constraint bit correctly and is worth recording as a positive: the audit PREREQ pin #19 lived in the freight of the line this wrap froze, so P5 had to consume it (re-pin … | — | pin #19 count in working-route: 1 before P7, 0 after; pin #20 present once on t… |

**Proposal:** ABSORBED: the playbook ordering rule (regen last, verify staged) plus `check:staged-artifacts` (2026-08-30-staged-bindings-assertion) made the staged copy the gated truth. Remaining generalization: the wrap SKILL could sequence regen-last/verify-staged natively — the cause (taurpc dev-mode export on any default-features run) will exist in any TauRPC project, not just this one.

### P11 — wrap-session/reconcile/contract.false-positive-proposal — 6 cases · weight 12 · 6 chunk(s)
**Pattern:** Six detector proposals asserted wrong facts (overclaimed universals, false premises about what a chunk introduced, a trigger premise the chunk itself falsified); each was caught by the validate-before-apply posture at the cost of dialogue rounds and reads.
**Evidence:** all 6 cases · summed impact: dialogue_rounds=4 extra_reads=1 halted=1

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-23-metrics-points-identity | The test-plan doc-agent's §3 item-6 proposal asserted that 'only the MockTraceSpan half of the mandate exists', framing the correction as PARTIAL. Re-derived first-hand under Validate check 4: MockTr… | extra_reads=1 | grep -rc over crates/ and pulse-app/: MockArrowBatch 0, MockMetricPoint 0, Mock… |
| 2026-08-23-webview-self-verify | D-security-deps proposed amending security-plan §Bootstrap phases to record that the pulse-app/ui npm ecosystem sits outside Dependabot coverage, reasoning from the spec text Add .github/dependabot.y… | dialogue_rounds=1 | fanout-results.md §Validation item 4 |
| 2026-08-23-headful-leg-extension | D-security-logging proposed retiring the INTENDED-posture and subscriber-layer sentences to accommodate the new window_label field — a category error twice over: the posture sentence governs attribut… | — | .andromeda/runs/2026-08-24T17-32-09Z-wrap/fanout-results.md |
| 2026-08-24-headful-mechanics-probe-race-disposition | D-security-logging returned a 3-proposal group (primary plus 2 dependents) claiming the chunk introduced a first-party log record class gated by bounded enumeration rather than scrub_attribute, makin… | dialogue_rounds=1 | 2026-08-24T23:46:03Z-b |
| 2026-08-28-duplicate-span-replay-fails-loudly | D-security-logging proposed rewriting the section 434 whole-batch-ERROR sentence to say the claim never held on this project Arrow appender path, generalizing this chunk spans-path hang measurement t… | dialogue_rounds=1 | git show 7608217:Cargo.lock confirms duckdb 1.10502.0 at that chunk; rejected W… |
| 2026-08-29-app-registry-reconciliation | D-tests-coverage proposed minting a durable-incidents-adapter-and-wiring-coverage trigger on the premise that the traits pins live in triage against a test double so reverting the wiring leaves every… | dialogue_rounds=1 halted=1 | fanout-results.md Escalation #6 |

**Proposal:** Keep the posture. Direction: detector returns could carry a basis pointer (file+line measured) per claim, so the orchestrator's re-derivation is one read instead of a search.

### P12 — new-session/orientation/tooling.output-cap-overflow — 14 cases · weight 4 · 1 chunk(s)
**Pattern:** 14 of 42 session starts overflowed the tool-result cap reading working-route.md (single-line multi-KB entries) or the skill-reference bundle; recovery was structural re-extraction each time (31 summed extra reads). Recurred at other steps too — see extension candidate U-F.
**Evidence:** all 14 cases · summed impact: retries=3 extra_reads=31

| chunk | what | impact | evidence |
|---|---|---|---|
| (session-scope) | grep -nE over andromeda-pulse-0.3.0/working-route.md exceeded the tool-result size cap (51.6KB persisted to file) because working-route entries are multi-hundred-char single lines; the markerless-tai… | extra_reads=2 | 2026-08-21T11:37:18Z-b |
| (session-scope) | Reading the working-route tail to derive the markerless cursor overflowed the tool-result cap twice (45.3KB then 45KB, both persisted to disk rather than returned) because v0.3.0 route entries are mu… | extra_reads=2 | 2026-08-21T15:27:16Z-b |
| (session-scope) | cat of working-route.md (36.2KB) overflowed the tool-result cap and returned a 2KB preview plus a persisted-file pointer; recovered by two python re-extraction passes over the file (boundary derivati… | extra_reads=2 retries=1 | tool-results/b2m8m3bv7.txt |
| (session-scope) | a full read of andromeda-pulse-0.3.0/working-route.md returned Output too large (41.3KB) and was truncated to a 2KB preview; recovered by structural re-extraction (awk classification of line kinds + … | extra_reads=2 | scratch tool-results/bomsrdl4q.txt (persisted overflow output) |
| (session-scope) | cat of andromeda-pulse-0.3.0/working-route.md returned 43.5KB, over the tool-result cap; the body was spilled to a persisted file and only the first 2KB previewed, which covered Epoch 1 and part of E… | extra_reads=1 | 2026-08-23T11:49:04Z-c |
| (session-scope) | Batched cat of 5 skill reference files returned 36.2KB and persisted past the tool-result cap; the persisted-file re-cat overflowed again (wasted read); recovered by per-file sed/cat of the still-nee… | extra_reads=3 | 2026-08-24T19:27:01Z-b |
| (session-scope) | working-route.md (40.7KB) exceeded the tool-result cap on full cat; recovered in one structural pass via the route grammar (frozen-marker count, markerless-tail grep, epoch headers). | extra_reads=1 | 2026-08-24T19:27:01Z-c |
| (session-scope) | cat of andromeda-pulse-0.3.0/working-route.md (64.2KB) overflowed the tool-result cap; recovered via structural re-extraction, whose first pass also admitted indented separator lines and needed one r… | extra_reads=2 retries=1 | 2026-08-27T17:04:42Z-b |
| (session-scope) | working-route.md (47.5KB) overflowed the tool-result cap on plain read; stamped markers and markerless tail recovered via grep/awk structural re-extraction | extra_reads=2 | 2026-08-29T21:11:33Z-b |
| (session-scope) | cat of working-route.md (44.6KB, single-line entries) exceeded the tool-result cap and persisted to file; position and markers re-extracted structurally with grep/sed | extra_reads=2 | 2026-08-30T00:12:46Z-c |
| (session-scope) | working-route.md reads exceeded the tool-result size cap twice (44KB cat, 31KB filtered grep) because entries are single long annotated lines; recovered with cut -c1-90 title-only extraction | extra_reads=2 | 2026-08-30T08:04:56Z-c |
| (session-scope) | single-call cat of all 5 skill references (36.1KB) overflowed the tool-result cap; the naive recovery (re-cat the persisted file) overflowed identically; sed range-extraction of the needed sections r… | extra_reads=3 | 2026-08-30T11:23:53Z-b |
| (session-scope) | the 5-file reference batch read (36.8KB) hit the tool-result persistence cap, and the persisted-file re-read hit it again; content recovered by structural re-extraction as 4 per-file reads | retries=1 extra_reads=4 | 2026-08-30T13:17:27Z-b |
| (session-scope) | one cat of the 5 new-session reference files returned 36.8KB and was persisted to a tool-results file; re-extracted via 3 sed slice reads (third slice empty - file was 555 lines) | extra_reads=3 | tool-results/b01hcze37.txt |

**Proposal:** Prescribe the structural read as the DOCUMENTED default in the new-session skill body: grammar-aware tail extraction (markers + first markerless line) and per-file reference reads. The entry-length half is project-side (see U-B long-line-edit).

### P13 — implement/fix-loop/contract.test-expectation — 6 cases · weight 8 · 6 chunk(s)
**Pattern:** Six chunks hit pre-existing tests that pinned the defect being fixed, or expectations needing strengthen-not-relax treatment; all were resolved per the discipline (strengthened, never relaxed), plus one plan-listed health check probing an inert config.
**Evidence:** all 6 cases · summed impact: iterations=6 retries=1

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-15-corpus-key-persistence | The plan's broader gate list surfaced an INHERITED red: pulse-app::e2e_p3_mcp_incident_tools::mcp_retrieve_report_previously_seen_matches_resolver_selection fails 1 of 382 under --features mcp-server… | — | — |
| 2026-08-17-incident-fingerprint-producer-repaired | a pre-existing pin asserted Incident.fingerprint == the L4Output fixture value, i.e. it encoded the exact producer defect this chunk removes; resolved by strengthening it (its cue now carries a real … | iterations=1 | — |
| 2026-08-21-delegated-timing-observables | Ran clippy once without the -D warnings the plan's Test Command specifies; it exited 0 while emitting a complex-type warning that the real gate form would have failed on. Caught by comparing the invo… | iterations=1 | 2026-08-21T12:52:42Z-e |
| 2026-08-23-a11y-verification | Four pre-existing assertions pinned the accessible name this chunk deliberately changes. Three were found by grepping the test file before running; the FOURTH surfaced only when the suite ran, becaus… | iterations=1 | pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.test.tsx |
| 2026-08-26-interpretation-brief-completeness | 3 green iterations: (1) unit_incident_producer :229 pinned the pre-union exact set on a fingerprinted cue — strengthened to the 3-element union; (2) a runner-dependent flake surfaced (producer_observ… | iterations=3 | nextest run 1: 1 fail (handle_digest_emits_prompt_version_v2_1_for_primary_tier… |
| 2026-08-30-npm-advisory-coverage | the plan-listed suite-health check `npx playwright test --list` inspects the inert default playwright.config.ts (0 tests) — the real a11y suite lives behind --config=playwright-a11y.config.ts; as wri… | retries=1 | 2026-08-30T01:10:00Z-b |

**Proposal:** None — the discipline held everywhere. The pattern documents the standing cost of pinned-defect tests; the inert-config case was amended into test-plan §1 at its chunk.

### P14 — implement/code/input.plan-step-ambiguous — 11 cases · weight 4 · 10 chunk(s)
**Pattern:** Eleven chunks had a plan step underdetermined or self-contradictory at code time — placement rules instead of locations, two options one structurally impossible, steps contradicting their own touchpoints.
**Evidence:** all 11 cases · summed impact: iterations=3 extra_reads=9

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-fingerprint-feed-capture-repair | plan step 6 mandated allowlist resolver probes without naming a location; the conventional home (observability.rs mod tests) is dead code under [lib] test = false — the baseline run confirmed zero pu… | extra_reads=2 | — |
| 2026-08-14-workspace-key-alignment | step 7 named a placement RULE (pulse-app/tests/*.rs if it reaches pulse-app internals, co-located otherwise) rather than a file, and the plans New-files list contained no test file — so the only in-b… | — | — |
| 2026-08-15-corpus-key-persistence | Plan Step 2 said the fallback derives 'a passphrase-derived key' without naming the passphrase SOURCE, and the two readings differ materially: an environment-supplied passphrase versus an auto-genera… | extra_reads=1 | — |
| 2026-08-16-fault-identity-semantics-decided | Plan step 7 said to remove the unreachable fingerprint comparison from select_corpus_matches but did not address the consequence: the current_fingerprints parameter becomes unused, and removing it wo… | extra_reads=1 | plan.md Implementation Steps step 7 vs research.md Files to modify |
| 2026-08-16-fault-identity-semantics-decided | Plan step 8 named TWO tests as depending on the dead fingerprint arm (retrieval.rs:176 and :211); THREE did. select_corpus_matches_ranks_by_recency_and_caps_at_limit (:198) also selected purely via t… | iterations=1 | retrieval.rs:198 select_corpus_matches(candidates, &["fp-a".to_string()], &[], … |
| 2026-08-17-conductor-e2e-verification-closure | plan step 5 and its own named touchpoint contradict each other: the step prose demands a real andromeda-pulse-mcp subprocess JSON-RPC call, the named file e2e_p3_mcp_incident_tools.rs is in-process d… | extra_reads=1 | — |
| 2026-08-22-log-records-identity | the plan underdetermined where the new column goes: step 4 said to add the Arrow Field in the same position as the DDL column, while step 5 said only to add the column and extend the PK, never fixing… | extra_reads=1 | plan.md Implementation Steps 4 and 5 |
| 2026-08-23-integration-ux-e2e-test | Plan Step 1 says to replace the single is_widget_hidden_transition boolean WITH A PER-STAGE PREDICATE SET, while Step 3s stage table enumerates six stages none of which is the widget close. Following… | extra_reads=1 | 2026-08-23T16:14:39Z-b |
| 2026-08-23-a11y-verification | Plan step 1 could not be followed as written, in two independent ways. (1) CONTRADICTION: it prescribed roving tabindex on <tr> while its own parenthetical banned nested focusables per a11y-plan sect… | extra_reads=2 | pulse-app/ui/src/dashboard/routes/traces/TraceTable.test.tsx |
| 2026-08-23-headful-leg-extension | plan.md Test Commands listed the standing RED arm as webview-drive --expect-absent dashboard-toggle without --no-inject; against a healthy app the named stage IS observed so the arm exits FAILURE by … | iterations=1 | 2026-08-24T16:31:18Z-b |
| 2026-08-25-demo-injector-formalized-api-surface-retire | the plan offered two options for the lint:a11y repair and the FIRST is structurally impossible, which neither phase research nor the a11y extract could have known without running it: eslint --rule bu… | iterations=1 | npm run lint:a11y after the quoting fix: "A configuration object specifies rule… |

**Proposal:** Add a P4 synthesis check: every step names a concrete location and admits a single executable interpretation (pairs with P3's upstream move). Several cases trace to research under-enumeration — see chain X1.

### P15 — phase/validate/contract.intent-divergence — 8 cases · weight 5 · 8 chunk(s)
**Pattern:** Val-1 classified 8 intent-incomplete / falsified-premise cases the P3 closure had missed — mostly working as designed (2 dialogue rounds total). Two consecutive chunks showed the same sub-shape: premise closure corrected the premise BULLET but left sections DERIVED from it standing.
**Evidence:** all 8 cases · summed impact: dialogue_rounds=2

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-fingerprint-feed-capture-repair | val-1 found a scope bullet still tagged [inferred] after P3's premise closure ran — the closure worked only the dedicated Premises section and missed an inline [inferred] in the Surfaces section; cla… | — | — |
| 2026-08-14-workspace-key-alignment | val-1 discovered a falsified scope premise that the P3 closure had missed: scope said both processes call the shared derivation, but Conductor spawns the sidecar with ANDROMEDA_PULSE_DATA_DIR propaga… | dialogue_rounds=1 | — |
| 2026-08-15-corpus-key-persistence | Validation-1 classified two plan steps as intent-incomplete rather than defect: Step 3 (read-path skip-and-count hardening) and Step 5 (repairing the pre-existing corpus.open.error allowlist gap) are… | — | — |
| 2026-08-23-integration-ux-e2e-test | Validation-1 classified INTENT-INCOMPLETE, not defect. scope.md §3 as first written absorbed five CARRYs; the plan carries two, because P3 measured the cost and the operator chose the narrower set at… | dialogue_rounds=1 | 2026-08-23T16:04:21Z-c |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | P3 scope-premise closure corrected the PREMISE bullet (obs-plan §10 is violated by viz and the Q7 fallback, not honoured) but left the section DERIVED from that premise stale: the Surfaces and contra… | — | andromeda-pulse-0.3.0/chunks/2026-08-26-ingest-consumer-stall-under-sustained-l… |
| 2026-08-26-cadence-runaway-blocking-pool | SECOND consecutive chunk where the P3 premise closure corrected the premise bullet but left a section DERIVED from it stale, and val-1 caught the residue. Last chunk it was the Surfaces table still c… | — | scope.md §Outcome vs §Premise closure |
| 2026-08-27-incident-persist-vs-resolve-write-race | Val-1 classified INTENT-INCOMPLETE, not aligned-clean: scope.md Boundaries asserted the in-app ledger is already correct and is not being redesigned. P3 had already added a premise correction elsewhe… | — | 2026-08-27T23:06:46Z-b |
| 2026-08-29-app-registry-reconciliation | Val-1 classified INTENT-INCOMPLETE, not defect: the plan carried two deliverables scope did not name — a committed cross-process CONTENT test (operator-selected at the P4 fork) and a smoke:external-r… | — | scope.md In scope item 4 (added at P5 val-1) |

**Proposal:** Have the premise-closure instruction name DERIVED sections explicitly (not just tagged bullets). Otherwise healthy.

### P16 — wrap-session/reconcile/contract.cascade-miss — 10 cases · weight 4 · 10 chunk(s)
**Pattern:** Ten chunks found amendment leaves the cascade DAG table does not name (conventions.md and docs/services/* as architecture/plan leaves; duplicate-claim sites) — each caught only by the orchestrator's own grep, costing extra reads per wrap.
**Evidence:** all 10 cases · summed impact: iterations=1 dialogue_rounds=1 extra_reads=10

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-17-conductor-e2e-verification-closure | the cascade's cross-master grep caught a curated Tier-2 entry that the amendment flow structurally cannot fix: rules/testing.md line 264 states as CURRENT that the canned L4 output hardcodes an empty… | — | .andromeda/runs/2026-08-17T20-45-59Z-wrap/fanout-results.md |
| 2026-08-21-delegated-timing-observables | A verbatim single-site replace left the SAME claim standing elsewhere twice over: .claude/rules/security.md:23 restated the retired two-enforcement-pieces pairing one line below the line just amended… | extra_reads=3 | 2026-08-21T15:02:06Z-c |
| 2026-08-22-log-records-identity | the cascade DAG table names only stack.md (plus CLAUDE.md GENERATED blocks) as architecture.md's leaves, but .claude/docs/conventions.md declares itself Extracted from .andromeda/architecture.md Conv… | dialogue_rounds=1 | run dir fanout-results.md, Orchestrator findings beyond the detectors, item 1 |
| 2026-08-23-ingestion-scrub-coverage | the cross-master cascade grep is only as good as the retired-wording pattern set the orchestrator derives from the amendments, and the first sweep under-derived it: patterns were taken from the secur… | extra_reads=2 | .andromeda/runs/2026-08-23T07-05-00Z-wrap/fanout-results.md |
| 2026-08-23-metrics-points-labels | the amendment-flow cascade TABLE names only stack.md as architecture.md leaf, but the provenance header shows TWO — conventions.md distills the arch Conventions section verbatim and carried the retir… | — | 2026-08-23T11:30:09Z-b |
| 2026-08-23-integration-ux-e2e-test | Validate check 5 caught TWO expected-amendment entries that NO detector proposed — test-plan §9 CI Integration (the E2E stage named the bare command form while the chunk gave it two flags) and a new … | extra_reads=2 | 2026-08-23T17:03:52Z-c |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | The obs detector named THREE sites restating the stall definition (§1, §3, §10 Standard+); the orchestrator grep found a FOURTH candidate at obs-plan:620 and a fifth in .claude/docs/workflow.md:49. B… | extra_reads=2 | 2026-08-26T10:07:40Z-c |
| 2026-08-26-cadence-runaway-blocking-pool | the cascade table names CLAUDE.md blocks + docs/{specialist}-summary.md + rules/* for a plan amendment, but .claude/docs/services/interpretation.md is also a leaf and it carried TWO claims this chunk… | extra_reads=1 | cross-master grep for the no-bare-interpretation wording returned obs-plan, rul… |
| 2026-08-27-report-window-copy-affordance | correcting a COUNT at a site whose ENUMERATION was independently stale produced a self-contradicting body mid-pass. The test-plan section-6 P5 Status bullet stated fifteen stages over a 13-id list th… | iterations=1 | .andromeda/runs/2026-08-27T21-51-58Z-wrap/fanout-results.md — Apply + cascade, … |
| 2026-08-30-staged-bindings-assertion | the cascade re-compute found 4 leaf sites (docs/gotchas.md:35, docs/services/ui-bridge.md:31/36/49) still carrying the per-procedure-capability-JSON claim retired from the masters at 2026-08-21-deleg… | — | 2026-08-30T12:59:31Z-b |

**Proposal:** Amend the cascade table to the measured leaf set once (conventions.md + services docs as architecture.md/plan leaves) — an Andromeda reference fix that has been re-derived at least four times.

### P17 — wrap-session/gates/tooling.light-gate-red — 4 cases · weight 10 · 4 chunk(s)
**Pattern:** Four first-run light-gate reds: the rlib-format-mismatch family twice, one designed-red-by-plan command, one load-sensitive race the identical implement run passed.
**Evidence:** all 4 cases · summed impact: iterations=3 retries=1 dialogue_rounds=1 halted=1

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-22-log-records-identity | the light gate went RED on its first run - exit 101, crate libduckdb_sys required to be available in rlib format - despite the identical command set having passed 1835/1835 under /implement barely an… | retries=1 | light-gate run 1 exit 101; remediated by named-cluster clean + build-first, run… |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | The light gate went RED on first run with the documented rlib-format-mismatch family naming DEPENDENCY crates (plist, notify_types, anyhow, serde_with, libduckdb_sys, tower, arrow_array, wasmtime_int… | halted=1 iterations=2 | cargo nextest exit 101 -> after de-race, 1950/1950 exit 0 |
| 2026-08-28-ingest-consumer-initiating-freeze | A command in the chunks plan Test Commands is RED at the light gate BY DESIGN and the wrap committed anyway - a shape P7s red-means-do-not-commit rule does not describe. cargo xtask smoke:gap-resume … | — | arm A verdict: buffer consumer stalled 450s, reason=rows_static_while_channel_q… |
| 2026-08-29-advisory-backlog | the wrap light gate reproduced a load-sensitive race the identical implement run passed: body_over_8mb_returns_413_or_4xx panicked on the mid-upload ConnectionAborted arm of DefaultBodyLimit respond-… | iterations=1 dialogue_rounds=1 | scratchpad/wrap-gates.log |

**Proposal:** Make the build-first de-race prefix the light gate's documented default on this host class; the designed-red case fed the P-shape rule already curated (never a combined designed-red invocation).

### P18 — wrap-session/curation/ambiguity.filter-borderline — 19 cases · weight 2 · 19 chunk(s)
**Pattern:** Nineteen wraps hit curation borderlines; the dominant shape is candidates scoring EXACTLY the Filter-4 threshold (0.6 = verified-by-measurement 0.4 + specific-technical-detail 0.2) — the boundary sits precisely where real candidates mass — plus Filter-1 edges against content the same wrap's cascade had just written.
**Evidence:** all 19 cases · summed impact: iterations=1 extra_reads=2 deferred=1

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-workspace-key-alignment | both Tier 3 survivors scored exactly 0.6 at Filter 4 — the threshold — from the same two signals (verified-by-measurement +0.4, specific-technical-detail-with-context +0.2); a slightly stricter readi… | — | — |
| 2026-08-15-tier-1-incident-path-investigation | Three borderline calls, all resolved but none determined by the rules alone. Filter 4: BOTH surviving candidates scored EXACTLY 0.6 (0.4 verified-by-measurement + 0.2 specific-technical-detail) again… | — | — |
| 2026-08-16-fault-identity-semantics-decided | Two Filter-1 dedup calls sat at the edge. (i) The cargo-deny candidate is adjacent to TWO existing generated-body lines in rules/security.md - the CI form already lists only bans/licenses/sources, an… | extra_reads=1 | rules/security.md §Supply chain + CI (CI form + advisory-visibility policy); CL… |
| 2026-08-17-incident-fingerprint-producer-repaired | the test-pins-the-defect candidate scored well clear on confidence but sat at a Filter-1/Filter-3 edge against the 2026-08-15 dead-fixture entry: Jaccard was under the 0.7 dedup bar so it was not a d… | extra_reads=1 | — |
| 2026-08-17-conductor-e2e-verification-closure | the ambiguous-zero candidate scored exactly 0.6 against a 0.6 threshold — measurement-verified plus specific-technical-detail and nothing else, since its repeat instances sit in EARLIER sessions and … | — | — |
| 2026-08-22-log-records-identity | the vacuous-conditional candidate scored EXACTLY 0.6 against a 0.6 threshold - explicit user correction 0.4 plus specific-technical-detail-with-context 0.2 - so it passed by equality rather than marg… | — | curation-tier-decision.md Filter 4 threshold and tiebreakers 2/4 |
| 2026-08-23-ingestion-scrub-coverage | two borderline calls, both resolved by the written tiebreakers rather than by feel. (1) The shared-extractor transform entry reads as a directive (transform at the extractor) which points Tier 2, but… | — | andromeda-pulse-0.3.0/chunks/2026-08-23-ingestion-scrub-coverage/report.md |
| 2026-08-23-metrics-points-labels | the joined-form scrub candidate hit Filter 1 against a body the CASCADE had written minutes earlier in this same wrap — rules/security.md now states the fact because the security-plan amendment re-de… | — | 2026-08-23T11:32:39Z-b |
| 2026-08-23-a11y-verification | Two candidates scored EXACTLY 0.6 — the Filter-4 threshold — so both barely qualified, and with two higher-confidence survivors the Filter-5 cap of 3 left one slot for the pair. The tie was genuinely… | — | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/report.md |
| (session-scope) | The one applied candidate scored EXACTLY at the Filter 4 threshold (0.4 verified-by-measurement + 0.2 specific-technical-detail = 0.6, apply if >= 0.6), so apply-vs-drop turned on whether a tool-mech… | — | wrap 2026-08-25 P3; the underlying event is the separator-merge friction record… |
| 2026-08-25-demo-injector-formalized-api-surface-retire | two candidates sat on Filter 1's dedup-vs-additive-facet boundary and resolved in OPPOSITE directions, which is what made the boundary feel underdetermined. The disk-exhaustion candidate cleared the … | — | Filter 1 additive-facet tie-breaker vs Filter 1 recurrence clause, curation-tie… |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | Filter 1 made two non-obvious calls in one pass, both turning on WHERE the matched text lives rather than on similarity. (1) The tick-presence-is-not-progress candidate matched text THIS WRAP had jus… | — | 2026-08-26T10:09:57Z-b |
| 2026-08-26-l4-runtime-security-residuals | a candidate whose content THIS wrap had just written into a SPEC MASTER has no documented disposition in the filter set. Filter 1's dedup sources are the three curation homes plus rule-file BODIES - … | — | 2026-08-26T19:04:13Z-b |
| 2026-08-27-idle-observer-generation-damper | the ASCII-argv-template candidate sat near the 2026-08-26 layout-chars security entry on similarity — resolved as a DISTINCT-mechanism additive entry (OS argv decode vs in-process reject-guard whitel… | iterations=1 | 2026-08-27T20:20:00Z-b |
| 2026-08-27-report-window-copy-affordance | three of the four surviving candidates scored exactly 0.6 against a 0.6 threshold (verified-by-measurement +0.4 plus specific-technical-detail +0.2), so the Filter 5 cap of 3 had to break a three-way… | deferred=1 | 2026-08-27T22:07:14Z-b |
| 2026-08-27-incident-persist-vs-resolve-write-race | Four candidates passed Filter 4, three at EXACTLY the 0.6 threshold (verified-by-measurement +0.4 plus specific-technical-detail +0.2, with no user correction and no repeat signal), against a cap of … | — | 2026-08-28T16:12:06Z-b |
| 2026-08-28-ingest-consumer-initiating-freeze | Two of three surviving candidates sat on a Filter 1 edge, both against content THIS WRAP had just written. (a) The scenario-vs-gate candidate deduped against the rule BODY the cascade had just added … | — | curation-tier-decision.md Filter 1 DEDUP-REJECT-ONLY clause |
| 2026-08-28-ingest-consumer-block-under-gap-resume | Filter 1 hit the wrap-own-cascade self-collision again: the observe-window candidate matched text THIS wrap had just written into rules/verification-harness.md BODY via the cascade. The DEDUP-REJECT-… | — | cascade body edit + Session Additions entry, same file, same wrap |
| 2026-08-29-app-registry-reconciliation | Three of four candidates scored EXACTLY 0.6 — the threshold — each from the same pair of signals (verified-by-measurement 0.4 plus specific-technical-detail-with-context 0.2). None had a user correct… | — | curation log: C1 harness-negative-finding, C3 independent-producer verdict half… |

**Proposal:** Move the threshold off the mass point (0.55 or 0.65), or add a documented exact-threshold disposition; separately codify the dedup-vs-wrap-own-cascade clause (it recurred 3×). Low per-case cost, but it is the highest-frequency judgment sink in the loop.

### P19 — wrap-session/curation/ambiguity.tier-routing — 15 cases · weight 2 · 14 chunk(s)
**Pattern:** Fifteen wraps spent judgment routing candidates between two defensible homes (testing vs verification-harness vs security; standalone vs in-place extension), with the written tiebreakers not adjudicating.
**Evidence:** all 15 cases · summed impact: retries=1 extra_reads=3

| chunk | what | impact | evidence |
|---|---|---|---|
| (session-scope) | the surviving candidate (route-entry provenance belongs in the trailing parenthetical) is path-scopable to working-route.md but no rule file covers that scope and minting one for a single convention … | — | — |
| 2026-08-14-fingerprint-feed-capture-repair | one candidate (count an observable independently rather than deriving it, so equality becomes evidence) was neither duplicate nor low-confidence but had just been written into obs-plan §5 by this wra… | — | — |
| 2026-08-14-workspace-key-alignment | Filter 1s additive-facet tiebreaker says keep an extension by amending the matched entry in place, same tier same location — but the matched entry here is a FAMILY split across two rule files (rules/… | — | — |
| 2026-08-15-corpus-key-persistence | The MSYS switch-rewriting finding resisted tier placement: it is a single imperative (length test -> Tier 1/2) about evidence integrity (close to the Tier-1 safety bar, since it produced a false 'no … | — | — |
| 2026-08-17-conductor-e2e-verification-closure | the ambiguous-zero rule had two plausible Tier-2 homes — rules/observability.md holds the sibling entry about a log target's zero being evidence only about its own writers, while rules/testing.md hol… | — | — |
| (session-scope) | The external-relay-verification learning had two tiebreakers pointing opposite ways: tiebreaker 2 prefers Tier 2 over Tier 1, while tiebreaker 3 (path-scopability) rules Tier 2 out because the rule g… | — | 2026-08-21T11:56:42Z-b |
| 2026-08-21-delegated-timing-observables | The strongest candidate this pass — no drift-base detector owns whether a documented surface/mechanism EXISTS at HEAD — is a judgment-base gap, not a project learning, and the operator declined the a… | — | 2026-08-21T15:10:33Z-b |
| 2026-08-23-webview-self-verify | One candidate had a genuinely contestable home. The measurement that Playwright CAN attach to the live Tauri app over WebView2 CDP sits against an existing rules/testing.md Session Additions entry (2… | — | test-plan §2 Agent-runnable invariants, amended this wrap |
| 2026-08-23-integration-ux-e2e-test | Two of the three surviving candidates routed to an IN-PLACE EXTENSION of an existing entry rather than a new one, and in both cases the matched entry was written by the IMMEDIATELY PRECEDING chunk — … | — | 2026-08-23T17:05:38Z-b |
| 2026-08-23-headful-leg-extension | The boundary-mechanism lesson had two defensible homes — a standalone T1 entry vs an in-place extension of the existing DURABLE-TEXT entry; chose the extension because the existing entry already carr… | — | 2026-08-24T17:49:02Z-b |
| 2026-08-24-headful-mechanics-probe-race-disposition | Four candidates competed for three slots; the pristine-baseline attribution recipe was the weakest as a standalone because it sits against an existing 2026-08-23 entry covering the same principle, so… | — | 2026-08-24T23:52:00Z-b |
| 2026-08-25-demo-injector-formalized-api-surface-retire | routing an in-place extension required finding the matched entry's home first, and my assumption was wrong: the disk-hygiene entry READS like Tier 3 reference material (a multi-paragraph routine with… | extra_reads=1 | grep -rn free-disk.ps1 found it at .claude/rules/testing.md:244, not in session… |
| 2026-08-26-cadence-runaway-blocking-pool | classified a candidate Tier 1 because the entry it extends appeared to live in CLAUDE.md; the entry is actually in .claude/rules/testing.md — the session-start context block renders CLAUDE.md and eac… | retries=1 extra_reads=2 | grep for the entry heading in CLAUDE.md returned nothing across three attempts;… |
| 2026-08-26-l4-runtime-security-residuals | the layout-char-whitelist entry had two defensible Tier-2 homes and the tiebreakers do not adjudicate between them: rules/security.md fits the SUBJECT (input-validation boundary design) but loads unc… | — | 2026-08-26T19:04:13Z-c |
| 2026-08-28-duplicate-span-replay-fails-loudly | The inconclusive-probe discipline is a testing-authoring rule and would naturally route Tier 2 to rules/testing.md, but two testing.md slots were already taken by higher-confidence candidates and it … | — | cap held at 3 with 0 deferred; the entry cross-references the rules/testing.md … |

**Proposal:** Add a one-line home-registry to curation-tier-decision.md (which rule file owns which class); it would decide the commonest contested pairs mechanically.

### P20 — wrap-session/route-resolve/contract.carry-no-owner — 5 cases · weight 5 · 5 chunk(s)
**Pattern:** Five wraps held carries/amendments with no owning route entry (deferred twice, dialogue twice) — the mint-an-owner path resolved each but at halt cost.
**Evidence:** all 5 cases · summed impact: dialogue_rounds=2 deferred=2

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-15-corpus-key-persistence | Two amendments applied in P2 named route entries as owners that did not exist yet — obs-plan section 8's muted-diagnostic backlog cited a 'Tier-1 incident-path investigation' entry, and deny.toml plu… | dialogue_rounds=1 | — |
| 2026-08-17-conductor-e2e-verification-closure | one of the two carried findings had no in-version owner entry: the incident producer's permanently-empty span_refs and timestamps_unix_nano is a production-behaviour change spanning producer, persist… | dialogue_rounds=1 | — |
| (session-scope) | Two of nine intake items had no pinnable owner after an approved batch disposition: #8 awaits an operator product call so no entry may own it yet, and #13 was to fold into a standing item whose locat… | deferred=2 | 2026-08-21T11:53:25Z-d |
| 2026-08-24-headful-mechanics-probe-race-disposition | The re-homed boot-geometry observation is harness work, but every harness-owning route entry has been consumed by this Epoch-4 run, so the nearest owner is the harness-truth sweep entry rather than a… | — | 2026-08-24T23:50:21Z-b |
| 2026-08-26-l4-runtime-security-residuals | the strict-path adopt-or-drop follow-up had no NATURAL owner entry. The CARRY grammar pins to a SPECIFIC later markerless entry, but none of the eight was about dependency hygiene: the Diagnostics sw… | — | 2026-08-26T19:06:57Z-b |

**Proposal:** Direction for judgment: sanction a standing per-version residuals pool entry that orphan carries pin to by default, so a missing owner stops forcing a mint-or-defer dialogue each time. (Changes route semantics — founder call.)

### P21 — phase/validate/contract.matrix-claim — 6 cases · weight 3 · 6 chunk(s)
**Pattern:** Six claim-gate frictions: affordance caps verified via process proxies (found and corrected — the affordance-honesty clause landed mid-epoch), an acceptance passable without the subject running, and zero-claim chunks still warranting full matrix reads.
**Evidence:** all 6 cases · summed impact: dialogue_rounds=1 extra_reads=3

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-17-conductor-e2e-verification-closure | the decline note bound for the matrix asserted that at HEAD the harness storm scenario forms NO storm and therefore no incident; the operator re-derived the arithmetic and falsified it. The claim had… | dialogue_rounds=1 extra_reads=2 | conductor scenarios/fingerprint-storm.toml phases.emission + crates/conductor-e… |
| 2026-08-21-delegated-timing-observables | At the claim gate the set of capabilities this chunk SERVES and the set it COULD link are disjoint: P-025/P-027/P-045 are v0.2.0 ids in docs/v0_2_0/capability-verification-matrix.json, while the acti… | — | 2026-08-21T12:25:55Z-b |
| 2026-08-23-a11y-verification | A zero-claim chunk still warranted a FULL matrix read. Scanning capability ACCEPTANCE TEXT rather than titles showed 10 of 22 caps touch a11y vocabulary — all already verified under other chunks, so … | extra_reads=1 | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/plan.md#implementatio… |
| 2026-08-24-headful-mechanics-probe-race-disposition | P-061 and P-062 are affordance caps (acceptance asserts a real drag / a real resize-clamp, method e2e) marked verified, but their ref fields defer the affordance-level evidence to P-075 and P-076: P-… | — | 2026-08-24T19:59:05Z-b |
| 2026-08-25-demo-injector-formalized-api-surface-retire | the cap being claimed had an acceptance that could pass without the capability subject ever running. P-077 read method by-construction with acceptance "inject_demo.rs is tracked and builds; the integ… | — | matrix P-077 notes field carries the premise correction; plan.md cites verifica… |
| 2026-08-27-report-window-copy-affordance | the matrix claim step was a structural no-op with no judgment available: 21 of 22 capabilities are already verified and the single remaining entry P-075 is method dynamic-external, owned by the exter… | — | 2026-08-27T21:11:41Z-b |

**Proposal:** The affordance-honesty clause covers the class going forward; direction: a mechanical claim-time check that the acceptance names the subject's real invocation.

### P22 — phase/plan/input.extracts-conflict — 4 cases · weight 4 · 4 chunk(s)
**Pattern:** Four plan-time conflicts between extracts/authorities that P2 aggregate check C caught late or not at all; three more untyped cases were intra-extract or extract-vs-skill-contract conflicts check C structurally cannot see (see U-G).
**Evidence:** all 4 cases · summed impact: iterations=1 dialogue_rounds=1

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-workspace-key-alignment | the security extract requires the shared derivation not preserve a raw to_string_lossy path (strict-path canonicalize+confine), but the P-079 test locks to_string_lossy of an already-canonicalized ro… | — | — |
| 2026-08-21-delegated-timing-observables | A three-way conflict that P2 aggregate check C did not catch because it spans a spec BODY, a sibling spec's AMENDMENT, and the code: layout-templates §Component describes the halo hue as driven by ER… | iterations=1 | andromeda-pulse-0.3.0/chunks/2026-08-21-delegated-timing-observables/plan.md §I… |
| 2026-08-23-integration-ux-e2e-test | The design extracts acceptance contribution required the empty-state assertion to key on the FULL mandated composition including the Observatory glyph, but the shipped EmptyState renders that glyph d… | — | 2026-08-23T15:58:04Z-b |
| 2026-08-26-l4-runtime-security-residuals | two domain authorities pointed opposite ways on the same implementation choice and P2's aggregate check C did not surface it: the security extract states strict-path is the plan's NAMED primitive so … | dialogue_rounds=1 | security.md Constraints bullet 3 vs arch.md Constraints bullet 6 |

**Proposal:** Extend aggregate check C's classes to intra-extract tension and extract-vs-skill-contract disagreement.

### P23 — implement/code/input.research-files-wrong — 4 cases · weight 3 · 4 chunk(s)
**Pattern:** Four modify-lists under-enumerated the real edit surface (callers of reshaped fns, boundary files carrying transit types).
**Evidence:** all 4 cases · summed impact: dialogue_rounds=1 extra_reads=7

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-fingerprint-feed-capture-repair | crates/buffer/src/contract.rs was required by plan step 4 but absent from Files-to-modify — buffer.tick fields transit BufferHeartbeat rather than being read from BufferState directly; judged in-scop… | extra_reads=1 | — |
| 2026-08-16-baseline-family-reachability | Research listed the modify-set from a graph query on the CONSTANT, so it never enumerated the callers of the FUNCTION whose signature the plan directs changing. Two production callers were missing - … | dialogue_rounds=1 extra_reads=2 | — |
| 2026-08-23-a11y-verification | The research boundary list was incomplete in a way only writing the code exposed: the token focus ring needs tokens.css (a :focus-visible rule cannot be an inline style, and the acceptance criterion … | extra_reads=2 | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/plan.md#codebase-touc… |
| 2026-08-26-interpretation-brief-completeness | Research/plan under-enumerated the builder caller set: integration_real_llama_cli.rs:131 calls build_primary_tier_prompt (env-gated test the graph query did not surface as a caller row), and the plan… | extra_reads=2 | rustc E0061 at integration_real_llama_cli.rs:131; e2e_p3 :562-566 comment stati… |

**Proposal:** Research playbook: require a graph caller-enumeration for every signature-changing symbol before the boundary list closes (chain X1's producer-side fix).

### P24 — phase/plan/input.research-thin — 3 cases · weight 4 · 3 chunk(s)
**Pattern:** Three plans were starved by research omissions — the live-leg producer never inspected, a needed convention unrecorded, the decisive external-integration fork unexamined.
**Evidence:** all 3 cases · summed impact: iterations=1 dialogue_rounds=1 extra_reads=2

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-workspace-key-alignment | research framed the launch-mode fork as cwd-based-derivation vs env-passthrough and did not examine how the external Conductor instrument spawns the sidecar; reading Conductors architecture doc at sy… | extra_reads=1 dialogue_rounds=1 | — |
| 2026-08-23-metrics-points-labels | research established the change surface fully but never looked at the live-leg producer, so the first Test Commands draft made two independent errors that would both have reached /implement as a brok… | iterations=1 | 2026-08-23T10:39:28Z-b |
| 2026-08-23-a11y-verification | P3 research.md recorded a Conventions-to-follow section but omitted the visually-hidden text convention, which the operator-selected aria-describedby design in step 2 depends on. Caught only by probi… | extra_reads=1 | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/plan.md#implementatio… |

**Proposal:** Research checklist: when the plan will name a live leg, the leg's invocation form is a required research object (X1).

### P25 — phase/plan/retry.synthesis-rework — 3 cases · weight 4 · 3 chunk(s)
**Pattern:** Three first-draft plans needed rework for self-inflicted defects (criterion without a producer; a matrix id that does not exist).
**Evidence:** all 3 cases · summed impact: iterations=3

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-15-tier-1-incident-path-investigation | First-pass plan carried an a11y acceptance criterion (zero new violation tuples) with no producing command in Test Commands, because the tests extract's 'webview gates excluded on zero pulse-app/ui d… | iterations=1 | — |
| 2026-08-27-report-window-copy-affordance | P4 authored Test Commands that omitted two producers its own acceptance criteria depend on: no smoke/status line despite pulse-app/capabilities/*.json being on test-plan section 3 declared boot-smoke… | iterations=1 | 2026-08-27T21:06:01Z-b |
| 2026-08-30-npm-advisory-coverage | first plan draft cited verification-matrix.json#P-083 — an id that does not exist (matrix holds 22 caps, none claimable by this chunk); invented during authoring, self-caught immediately post-write a… | iterations=1 | 2026-08-30T00:44:17Z-b |

**Proposal:** Covered by P3's upstream checks; the matrix-id case suggests the P4 template link ids to the read matrix rather than free-typing.

### P26 — wrap-session/reconcile/input.report-insufficient — 4 cases · weight 3 · 4 chunk(s)
**Pattern:** Four reports missed entries or precision that reconcile's detectors then caught (a missing disproved-claim, a wrong number attributed to a default).
**Evidence:** all 4 cases · summed impact: iterations=2

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-21-delegated-timing-observables | The report Coverage-of-new-surfaces bullet graded the webview mark sites tests-unit while frame-metrics.ts five new exports executed in no test (vi.mock replaced the module at its only call site) and… | iterations=1 | 2026-08-21T15:02:06Z-b |
| 2026-08-28-ingest-consumer-initiating-freeze | The P1 report authored a Spec-claims-disproved-by-measurement bullet with four entries and MISSED a fifth that the chunks own measurement falsified: test-plan §3s Direct-binary smoke variant asserts … | — | test-plan §3 Direct-binary smoke variant; arm A verdict in the chunk report Out… |
| 2026-08-28-ingest-consumer-block-under-gap-resume | The obs fan-out returned proposals:[] while obs-plan §10 defect 4 carried three claims the chunk measured false. Not a detector defect: none of D-obs-instrumentation / D-obs-stack / D-obs-pii has an … | — | fan-out returns: obs proposals:[]; applied amendment obs-plan §10 defect 4 + le… |
| 2026-08-30-acl-rejection-logging | the report's Deviation 4 wrote 'outruns the 180s default' where 180 was the session's own HARNESS_STATUS_TIMEOUT override — a detector faithfully proposed amending test-plan's (correct) 10s default t… | iterations=1 | .andromeda/runs/2026-08-30T17-19-47Z-wrap/.raw-fanout-test-plan.md (orchestrato… |

**Proposal:** Generalize the 2026-08-27 lesson: the report's disproved-claims and deviation bullets get a mechanical re-derivation pass against the chunk's own measurements before the fan-out reads them.

### P27 — wrap-session/report/contract.detector-fact-gap — 9 cases · weight 1 · 9 chunk(s)
**Pattern:** Nine wraps found that facts the detectors bind to have no home in the stock report bullets (harness/status-shape changes, scrub shapes, external-repo claims, disposition-less disproofs).
**Evidence:** all 9 cases · summed impact: extra_reads=5

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-fingerprint-feed-capture-repair | D-tests-obs-harness binds to whether the harness / status shape / log format changed, and this chunk moved BOTH (UTF-8 exports in agent-run.sh/.ps1; three added buffer.tick fields plus a new app.boot… | — | — |
| 2026-08-15-corpus-key-persistence | The stock 'Spec claims disproved by measurement' bullet had to carry two materially different kinds at once: three genuine spec/comment claims this chunk falsified and now owes amendments for, AND a … | — | — |
| 2026-08-17-conductor-e2e-verification-closure | the falsified claim this wrap was directed to sweep is stated only in the external harness repo, which is read-only from here, so no Pulse-side spec body can carry the correction and no drift detecto… | extra_reads=2 | — |
| 2026-08-22-log-records-identity | the Spec-claims-disproved-by-measurement bullet had to carry six entries, two of which are not spec claims in the strict sense: one is a working-route ENTRY premise (the two-tables misreading, alread… | — | report.md Changes, Spec claims disproved by measurement, entries 4 and 5 |
| 2026-08-23-metrics-points-labels | D-security-logging binds to three facts the stock Changes bullets have no home for — which columns scrub, WHICH SHAPE the scrub takes, and the stated FAILURE MODE of that treatment. This chunk moved … | — | 2026-08-23T11:20:32Z-b |
| 2026-08-24-headful-mechanics-probe-race-disposition | The operator supplied a correction at wrap time (a sub-part folded into a declined stage inheriting that stage decline reason) that no Changes bullet family naturally holds — it is neither a symbol n… | — | 2026-08-24T22:42:57Z-b |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | The Spec-claims-disproved-by-measurement bullet assumes its entries have a DISPOSABLE target — it is defined as recording what still needs disposition. This chunk three most consequential falsificati… | — | 2026-08-26T09:54:04Z-b |
| 2026-08-28-duplicate-span-replay-fails-loudly | The drift-base detector at line 61 binds to a stated failure mode of a store or log boundary, and this chunk changed exactly that — a constraint-violating Appender::flush went from blocking unbounded… | extra_reads=1 | the template own instruction to scan drift-base check fields before writing Cha… |
| 2026-08-29-app-registry-reconciliation | D-obs-defect-narrative (minted last wrap) binds to the Spec claims disproved by measurement bullet, whose own template definition assumes a DISPOSABLE target — what still needs disposition. This chun… | extra_reads=2 | report.md Spec claims disproved by measurement, first bullet |

**Proposal:** Add the missing fact-slots to the report template — the detectors' input contract IS the report; each gap found here was a detector reading between bullets.

### P28 — phase/research/input.extract-signal-gap — 6 cases · weight 1 · 6 chunk(s)
**Pattern:** Six research passes found the decisive fact at a location no extract signalled (found by unsignalled repo-wide greps); one muted diagnostic blocked a question outright.
**Evidence:** all 6 cases · summed impact: extra_reads=12

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-23-metrics-points-labels | the a11y extract cited its harness coordinates as tests-a11y/helpers/mock-tauri.ts and tests-a11y/helpers/v02-fixtures.ts, paths that do not resolve from the repo root — the harness lives under pulse… | extra_reads=2 | .andromeda/runs/2026-08-23T10-14-00Z-phase/tree-query-2026-08-23-metrics-points… |
| 2026-08-23-webview-self-verify | The obs extract asserted as fact that the observability allowlist enumerates no tray, ui, window or app.boot.webview.* entry, and built an acceptance bullet requiring the chunk to add a new target pl… | extra_reads=1 | research.md §Premise corrections to the extracts, items 1 and 2 |
| 2026-08-25-demo-injector-formalized-api-surface-retire | the tests extract treated the capability verification matrix as ONE artifact and asked whether P-077 has an entry to concretize, implying the `cargo xtask verify:capability-matrix` gate would cover i… | extra_reads=2 | xtask/src/main.rs:650-653 joins docs/v0_2_0/capability-verification-matrix.json |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | The decisive finding was NOT at any extract-signalled location. Every extract that named the obs-plan §10 isolation invariant pointed at the buffer/triage paths that honour it (retention.rs:73, basel… | extra_reads=5 | .andromeda/runs/2026-08-26T07-48-30Z-phase/tree-query-2026-08-26-ingest-consume… |
| 2026-08-26-cadence-runaway-blocking-pool | A muted diagnostic blocked a research question outright rather than merely slowing it. obs named triage.cue.tick -> cues_suppressed / bypass_triggered as still in the §8 muted backlog; confirmed live… | — | green-with-fix leg triage.cue.tick record |
| 2026-08-27-report-window-copy-affordance | the single most decisive research finding — that a live region for the copy outcome ALREADY SHIPS — lives in pulse-app/ui/src/report/Report.tsx, a file NO extract named. The a11y extract raised exact… | extra_reads=2 | .andromeda/runs/2026-08-27T20-41-09Z-phase/tree-query-2026-08-27-report-window-… |

**Proposal:** Keep repo-wide greps mandatory (already the discipline); direction: distillers could emit a sections-silent-on-X signal so silence is distinguishable from absence.

### P29 — implement/code/tooling.hook-friction — 4 cases · weight 1 · 4 chunk(s)
**Pattern:** Four PostToolUse hook events: edition-2015 rustfmt parse gaps, stale contradictory diagnostics against untouched files, one rule pointing the wrong way once.
**Evidence:** all 4 cases · summed impact: extra_reads=2

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-workspace-key-alignment | the PostToolUse rustfmt hook parses as edition 2015 and left two files (workspace-detector/src/contract.rs, mcp-server/src/bin/andromeda-pulse-mcp.rs) that edition-2024 cargo fmt then reformatted; co… | — | — |
| 2026-08-17-conductor-e2e-verification-closure | PostToolUse diagnostics fired twice with mutually contradictory stale content about files never touched this chunk — first expected-7-arguments-found-8 at crates/triage/src/pattern/storm.rs call site… | extra_reads=2 | — |
| 2026-08-23-integration-ux-e2e-test | The code-writing disciplines mid-burst diagnostics are expected noise rule pointed the wrong way once. A PostToolUse rustc dead_code warning on is_widget_hidden_transition was, by that rule, exactly … | — | 2026-08-23T16:14:39Z-c |
| 2026-08-27-incident-persist-vs-resolve-write-race | Two known hook behaviours during the multi-file signature change. (a) PostToolUse diagnostics fired repeatedly against mid-edit state, including one STALE report claiming persistence.rs:301 still car… | — | 2026-08-27T23:16:27Z-c |

**Proposal:** Known host behavior, low cost; direction: a conventions note on which hook diagnostics are expected noise mid-burst (partially exists) and a debounce if available.

### P30 — wrap-session/gates/tooling.gate-deferral — 4 cases · weight 1 · 4 chunk(s)
**Pattern:** Four sanctioned deferral events at gates (audit-pin probes, proportional live-leg deferrals) — designed behavior, recorded by this type as intended.
**Evidence:** all 4 cases · summed impact: deferred=3

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-15-corpus-key-persistence | cargo audit was probed at the light gate per the deferral ratified earlier this same wrap and failed identically — 'error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0… | deferred=1 | — |
| 2026-08-15-tier-1-incident-path-investigation | The ratified cargo-audit standing deferral was re-pinned for the SECOND consecutive wrap. Probed as its terms require: cargo audit still cannot load the RustSec DB (parse error: duplicate advisory ID… | deferred=1 | — |
| 2026-08-23-a11y-verification | The chunk plan lists cargo xtask webview-drive as a Test Command, and it is a LIVE leg that this session measured at roughly a 29 percent pass rate. Re-running it as the light gate would more likely … | deferred=1 | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/report.md |
| 2026-08-28-duplicate-span-replay-fails-loudly | The light gate re-ran every plan Test Command literally except the three smoke:gap-resume arms, whose combined runtime is roughly 35 minutes of real app boots; the skill carve-out for re-proving a by… | — | mtime comparison against target/release/pulse-app.exe returned zero newer track… |

**Proposal:** None — visibility only; the deferral machinery behaved (see also L8 for closure tracking).

### P31 — phase/take-up/input.working-entry-thin — 4 cases · weight 1 · 4 chunk(s)
**Pattern:** Four take-ups needed extra reads because the entry omitted context the reader cannot cheaply re-derive (prior-version cap ids, the first table of a two-table class, a signature phrased as an existing reading).
**Evidence:** all 4 cases · summed impact: extra_reads=3

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-16-baseline-family-reachability | The working entry names TWO baseline-derived families (silence, activity-floor) while the route-adaptation dialogue one wrap earlier named a THIRD (error-baseline-spike) as this chunk immediate consu… | — | — |
| 2026-08-21-delegated-timing-observables | The working entry names P-025/P-027/P-045 as the chunk targets without recording that those ids belong to the PRIOR version's matrix (docs/v0_2_0/capability-verification-matrix.json); the active v0.3… | extra_reads=2 | 2026-08-21T12:04:40Z-b |
| 2026-08-22-log-records-identity | the lineage clause (the 2026-08-15 identity class landing on a second table, intake #11) names neither the FIRST table nor where the class was measured, so folding it into scope required reconstructi… | — | scope.md §Folded annotations, Lineage bullet |
| 2026-08-29-app-registry-reconciliation | The working entry offers declined_count on triage.incident.persist as the defect signature, phrased as an existing visible reading; the predecessor chunk report (a different artifact, reached via the… | extra_reads=1 | scope.md caveat block under the mechanism table |

**Proposal:** Route-resolve authoring guidance: name coordinates the NEXT reader cannot re-derive cheaply (ids' version, both tables, whether a signature exists yet).

### P32 — phase/take-up/input.carry-context-gap — 4 cases · weight 1 · 4 chunk(s)
**Pattern:** Four carries contradicted themselves or lacked the arithmetic that changes what they demand (the interval-point count; a guard site in a file with no such seam).
**Evidence:** all 4 cases · summed impact: extra_reads=6

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-23-integration-ux-e2e-test | The taken-up entry carries two annotations that contradict each other on the same obligation: a NOTE stating the cargo-audit PREREQ re-pinned AWAY and is no longer this entry-s obligation, and a late… | extra_reads=2 | 2026-08-23T14:25:14Z-b |
| 2026-08-23-headful-leg-extension | The entry CONTEXT names the extension seam as a STAGES table with per-stage obs/dom predicates without saying which side of the two-sided harness owns it; grepping STAGES in the driver file a reader … | extra_reads=1 | 2026-08-23T23:11:06Z-b |
| 2026-08-24-headful-mechanics-probe-race-disposition | The navigation-race CARRY dictates the guard site as a re-navigate-on-show guard in pulse-app/src/window.rs, but that file handles only CloseRequested/Moved/Resized and Tauri 2 WindowEvent exposes no… | extra_reads=2 | 2026-08-24T19:38:13Z-b |
| 2026-08-26-cadence-runaway-blocking-pool | The folded cargo-audit PREREQ carried its full history but NOT the arithmetic that changes what it demands. Its text says next interval point 46 and records the last pin as a skip; deciding whether T… | extra_reads=1 | 2026-08-26T10:37:48Z-b |

**Proposal:** Same authoring-checklist direction; the interval-arithmetic case is now curated (confirm the session count at each wrap).

### P33 — phase/distill/input.spec-source-gap — 3 cases · weight 1 · 3 chunk(s)
**Pattern:** Three distills surfaced contradictions INSIDE a master (stale deferral state, crate missing from its own enumerations) — the distiller doing its job.
**Evidence:** all 3 cases · summed impact: deferred=1

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-workspace-key-alignment | the obs distiller surfaced an unresolved contradiction INSIDE obs-plan.md: §5 Vector 5 requires workspace-detector output not be logged as-is, while §4 P7 and the §8 workspace-detector allowlist name… | — | .andromeda/runs/2026-08-14T22-35-07Z-phase/obs.md |
| 2026-08-16-baseline-family-reachability | The tests distiller reported that crate triage appears in NONE of the three places test-plan.md enumerates library crates (pyramid Unit row, what-unit-tests-cover, test-file-location) - yet triage is… | — | .andromeda/runs/2026-08-16T16-30-00Z-phase/tests.md |
| 2026-08-28-ingest-consumer-block-under-gap-resume | The security distiller found the cargo-audit standing-deferral state stale in its own master: security-plan body and the newest sidecar entry both read next-probe-at-session-52, while session 52 alre… | deferred=1 | andromeda-pulse-0.3.0/chunks/2026-08-28-ingest-consumer-block-under-gap-resume/… |

**Proposal:** None — routes to wrap amendments as designed; kept for the record.

### P34 — implement/smoke/tooling.subprocess-bounds — 3 cases · weight 1 · 3 chunk(s)
**Pattern:** Three smokes paid interpretation cost on bounded-subprocess semantics (SIGTERM grace never honored → SIGKILL, rc=124 at the injector bound, Stop-Process exit codes read as failure).
**Evidence:** all 3 cases · summed impact: —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-16-baseline-family-reachability | pulse-app did not exit on SIGTERM within a 10s grace window in either run; SIGKILL was required both times. Teardown still released 4317/4318 and left zero orphan processes, so the escalation is surv… | — | — |
| 2026-08-17-conductor-e2e-verification-closure | two bounded-subprocess signals fired and both needed interpretation rather than being failures: the injector exited 124 because its 90s bound expired mid-storm, which is the designed outcome for a de… | — | — |
| 2026-08-22-log-records-identity | terminating the backgrounded app with Stop-Process -Force made its task-notification report status failed, exit code 127 - which reads exactly like a launch failure and would invert the smoke verdict… | — | smoke data dir logs/agent-latest.jsonl.2026-08-22, 8 app.boot lines |

**Proposal:** Largely closed by the teardown-truth chunk's probe-based verdicts; the rc-interpretation guidance is curated in test-plan §3.

### P35 — phase/distill/contract.binding-contradiction — 3 cases · weight 1 · 3 chunk(s)
**Pattern:** Three check-C contradictions between masters (halo hue driver design vs layouts; a11y CI binding; a scope-inferred claim restated).
**Evidence:** all 3 cases · summed impact: deferred=1

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-17-conductor-e2e-verification-closure | design and layouts disagree on what drives the P-025 halo hue: design-system.md says cumulative incident severity and explicitly names the error-rate formula superseded by its 2026-05-29 amendment, w… | — | .andromeda/runs/2026-08-17T19-22-45Z-phase/design.md + layouts.md |
| 2026-08-23-a11y-verification | Aggregate check C fired on the a11y-tests CI-gate binding: a11y-plan states npm run test:a11y reuses the tests 5-command driver in the same ci.yml pipeline, while test-plan section 9 CI stage table d… | deferred=1 | .andromeda/runs/2026-08-23T19-45-00Z-phase/a11y.md#contract-bindings |
| 2026-08-29-halo-state-pulse-signature-deferred | check C: the design extract restated scope inferred claim that the a11y MASTER carries SC 2.3.3 Halo wording while the a11y distiller measured the master + sidecar at zero Halo occurrences (wording l… | — | .andromeda/runs/2026-08-29T18-14-57Z-phase/fanout-results.md |

**Proposal:** None — the check discriminated each time; the halo case was resolved by the deferral chunk.

### Cross-step universal-type patterns
_These rollups absorb their per-step subgroups (the per-step split is shown in each table's rows); nothing below is double-counted in P1–P35._

### PU1 — cross-step · `contract.premise-falsified` — 52 cases · weight 10 · 26 chunk(s) · steps: implement/code, implement/fix-loop, implement/smoke, phase/plan, phase/research, phase/take-up, wrap-session/reconcile, wrap-session/report, wrap-session/route-resolve
**Pattern:** The epoch's largest class: verification falsified premises stated by authored artifacts at NINE steps across 26 chunks — route-entry EVIDENCE clauses stating causal mechanisms as fact (the cadence-runaway central premise; the gap-resume mechanism chain), stale coordinates, capability claims, in-repo comments. The pipeline absorbs each instance by design (research-corrects-intent, premise closure, report bullets); the VOLUME is the signal: 57+ extra reads at the research/take-up faces alone.
**Evidence:** all 52 cases · summed impact: iterations=2 dialogue_rounds=2 extra_reads=83 soft_exit=1

| chunk | what | impact | evidence |
|---|---|---|---|
| (session-scope) | the adaptation request instructed MOVE the deferred workspace-nextest PREREQ from P-075 onto the new first entry; the working-route contained zero PREREQ annotations (grep -c PREREQ = 0) — the deferr… | extra_reads=2 | — |
| 2026-08-14-fingerprint-feed-capture-repair | the chunk's central premise — a dead region between OTLP receipt and the fingerprint hook — did not reproduce at HEAD: the live capture recorded span_events_seen=936, fingerprints_computed=936, obser… | — | capture-datadir/logs/agent-latest.jsonl.2026-08-14 |
| 2026-08-14-fingerprint-feed-capture-repair | the plan's Expected-amendments list named obs-plan §3 Required tick fields per module as the site of the buffer.tick enumeration; it actually lives at §1 line 101, and §3's Heartbeat-ticks subsection… | extra_reads=1 | — |
| 2026-08-14-fingerprint-feed-capture-repair | a DOWNSTREAM tail entry's premise was falsified by this chunk's measurement — piece 1 assumed a fresh service's cue evaluation was unreachable behind the 3600s bootstrap gate, but source confirms tha… | dialogue_rounds=1 extra_reads=1 | — |
| 2026-08-14-workspace-key-alignment | two premises scope stated were falsified by research: (1) the normalization framing implied the derivation may normalize downstream, but detect() canonicalizes once and the P-079 test locks its verba… | extra_reads=2 | andromeda-pulse-0.3.0/chunks/2026-08-14-workspace-key-alignment/research.md |
| 2026-08-15-corpus-key-persistence | The working-route entry states the outcome as content 'recovered or retired'; operator directive 1 corrected it pre-work — recovery of pre-fix content is cryptographically closed (the mock store is p… | — | — |
| 2026-08-15-corpus-key-persistence | The operator's sanction for the inherited-fixture fix was premised on it being 'one file, test-only'. Repairing the 30-day time window made the test reach its real assertion for the first time, which… | dialogue_rounds=1 | — |
| 2026-08-16-fault-identity-semantics-decided | Two premises stated by the working entry were falsified by research: (a) the auto-resolve window is 120s (DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS, pinned to spec P-022 by a test) not the '~5min' th… | extra_reads=5 | .andromeda/runs/2026-08-16T19-21-36Z-phase/tree-query-2026-08-16-fault-identity… |
| 2026-08-16-fault-identity-semantics-decided | The obs extract stated obs-plan §8 enumerates NO allowlist leaf for the incident-outcome emit site, and built an acceptance criterion demanding the created/deduped fields be proven un-redacted from a… | extra_reads=1 | research.md §Obs readability of the decision evidence; plan.md §Implementation … |
| 2026-08-17-incident-fingerprint-producer-repaired | the prior chunk's decision.md still states the retrieval fingerprint_match arm 'is removed and the function is documented as scope-based' with 'three tests retargeted to the scope contract' — the shi… | extra_reads=1 | — |
| 2026-08-17-conductor-e2e-verification-closure | while drawing a boundary against the adjacent P-077 entry, its matrix observed_gap turned out stale: it states crates/ingest/examples/inject_demo.rs is currently untracked, but git ls-files resolves … | extra_reads=1 | — |
| 2026-08-17-conductor-e2e-verification-closure | the working-entry CARRY states the deterministic surface is ungradeable partly because degraded_mode is permanently true, attributing it to the canned L4 output; degraded_mode is not a field of that … | extra_reads=2 | andromeda-pulse-0.3.0/chunks/2026-08-17-conductor-e2e-verification-closure/rese… |
| 2026-08-17-conductor-e2e-verification-closure | the P-075 matrix acceptance requires asserting four delegated timing caps, but only one of them has an observable surface anywhere: metric.report.render_ms covers P-037 while P-025, P-027 and P-045 h… | extra_reads=1 | andromeda-pulse-0.3.0/chunks/2026-08-17-conductor-e2e-verification-closure/rese… |
| (session-scope) | The relay's intake #7 stated the buffer.tick trio renders <redacted> live; at Pulse HEAD the allowlist lookup resolves buffer.tick through its .tick-strip fallback to the "buffer" set, which contains… | extra_reads=6 | observability.rs:176 registration + for_target fallback chain at :2247-2260; he… |
| 2026-08-21-delegated-timing-observables | The cost claim "each target is a full TauRPC quadruple binding + an obs-allowlist leaf + an a11y-gated webview change" is overstated and had propagated through three authored artifacts (P-075 matrix … | extra_reads=3 | andromeda-pulse-0.3.0/chunks/2026-08-21-delegated-timing-observables/research.m… |
| 2026-08-21-delegated-timing-observables | Plan step 4 and research both name pulse-app/ui/src/halo/HaloCanvas.tsx:145-147 as the P-025 mark site, but that component has NO production render site — every reference is a test-file vi.mock facto… | extra_reads=3 | 2026-08-21T12:52:42Z-b |
| 2026-08-22-pii-scrubber-recall | two cited source coordinates were off by one line: the working entry named scrubber.rs:78 for the api_key regex (actually :77) and the directive named :83 for secret_kv (the paren opener; regex at :8… | — | 2026-08-22T15:25:19Z-c |
| 2026-08-22-pii-scrubber-recall | scope.md called pulse-app/tests/e2e_pii_scrubber_persistence_coverage.rs the natural home for the end-to-end leg because the legacy ledger names it for P-047; reading it showed 5 in-process tests ove… | extra_reads=1 | andromeda-pulse-0.3.0/chunks/2026-08-22-pii-scrubber-recall/research.md |
| 2026-08-22-pii-scrubber-recall | the plan's Test Commands section reasoned pulse-app/src/observability.rs counts for boot-smoke only 'as a boot concern' and omitted the gate, but the chunk edits that file's allowlist init and heartb… | — | 2026-08-22T17:54:34Z-d |
| 2026-08-23-ingestion-scrub-coverage | the five-column target set asserted by the working-route entry AND by the security-plan Logging amendment is four at HEAD: append_record_batch_to_table (the only DuckDB write path in crates/buffer) i… | extra_reads=2 | andromeda-pulse-0.3.0/chunks/2026-08-23-ingestion-scrub-coverage/scope.md §Veri… |
| 2026-08-23-ingestion-scrub-coverage | scope and the working-route entry both framed spans.service_name as a single write path (entry coordinates: extract_service_name appender.rs:45 to push :58 to StringArray :74 to column :83); the grap… | extra_reads=2 | .andromeda/runs/2026-08-23T06-05-00Z-phase/tree-query-2026-08-23-ingestion-scru… |
| 2026-08-23-ingestion-scrub-coverage | the phase directive argued metric_name is risky to scrub because it is the lookup key in corpus (contract.rs:474/:503); verification at synthesis showed that column is a product-internal namespace ho… | extra_reads=1 | andromeda-pulse-0.3.0/chunks/2026-08-23-ingestion-scrub-coverage/plan.md §Goal |
| 2026-08-23-metrics-points-identity | The sibling chunk scope.md listed "the report statement on whether the fix generalizes to metrics_points" as an in-scope deliverable, and its report.md does not contain it — the sole metric occurrenc… | extra_reads=2 | grep -i "metric/generaliz" over the sibling report returns only report.md:44 (q… |
| 2026-08-23-webview-self-verify | plan.md §Constraints and research.md cite the Conductor tauri-driver setup as a working precedent supporting the shape-(a) lean. Reading Conductor at HEAD, its own wdio.conf.ts header states DISPLAY-… | extra_reads=2 | probe-a run log: session established, 4 handles, aria-label Close to tray press… |
| 2026-08-23-integration-ux-e2e-test | intent.md F15 Housekeeping states crates/ingest/examples/inject_demo.rs is currently untracked; measured at HEAD it IS tracked by git, alongside three sibling injectors (inject_colliding_logs, inject… | extra_reads=1 | 2026-08-23T14:25:14Z-c |
| 2026-08-23-a11y-verification | CARRY-3 claims the Traces surface ships neither region[aria-label] nor a table role; measured at HEAD both halves are false — TraceTable.tsx:115 renders a native table (thead/tbody/th scope=col/aria-… | extra_reads=6 | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/scope.md#verified-pre… |
| 2026-08-23-a11y-verification | CARRY-1 names p13 as the slot for the deferred Investigate axe spec, but p13-empty-states.spec.ts already occupies it — the slot was free when the CARRY was authored and a later chunk took it, so an … | extra_reads=1 | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/scope.md#held-with-on… |
| 2026-08-23-headful-leg-extension | layout-templates §Component — Notifications trigger #4 states the close-to-tray signpost fires ONCE per session, but should_show_close_signpost (pulse-app/src/window.rs:135) returns notifications_ena… | — | andromeda-pulse-0.3.0/chunks/2026-08-23-headful-leg-extension/research.md |
| 2026-08-24-headful-mechanics-probe-race-disposition | The route CARRY assumed a drag stage could take an obs half from WindowEvent::Moved; research measured that the handler is guarded to the MAIN window only and emits no tracing record at all (record_m… | extra_reads=2 | .andromeda/runs/2026-08-24T19-33-56Z-phase/tree-query-2026-08-24-headful-mechan… |
| 2026-08-25-demo-injector-formalized-api-surface-retire | the chunk TITLE and its working entry both direct retiring `context/api-surface.md` once tree.db is built; at HEAD no such file and no context/ dir exists anywhere — both living-doc artifacts were de… | extra_reads=3 | git log --all -- **/api-surface.md → 5a83771; git show --stat 5a83771 (11708 de… |
| 2026-08-25-demo-injector-formalized-api-surface-retire | the load-bearing CARRY justifies sustained mode on the ground that a finite ~600-batch storm cannot hold a scenario across the L2-L3 20-60s window plus the L4 queue; measured constants are WARMUP_BAT… | extra_reads=1 | inject_demo.rs:102-103 constants + :214 sleep; :74-84 SERVICES payment-service … |
| 2026-08-25-demo-injector-formalized-api-surface-retire | the CARRY frames lint:a11y as escalating five jsx-a11y rules to error, implying plain `lint` does not enforce them. Measured: all five are ALREADY "error" in jsxA11y.flatConfigs.recommended.rules, wh… | extra_reads=1 | node -e over eslint-plugin-jsx-a11y flatConfigs.recommended.rules printed "erro… |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | The working entry names the predecessor feed-precondition as an ASSERTION that fails to catch the wedge; re-derivation at HEAD found no shipped mechanical check (grep rows_ingested returns nothing in… | extra_reads=2 | andromeda-pulse-0.3.0/chunks/2026-08-26-ingest-consumer-stall-under-sustained-l… |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | Plan step 4 directed moving the Q7 fallback off the shared connection, citing research. Verifying before editing showed there is nothing to fix: run_q7_with_timeout takes state.conn, and TriageSqlSta… | extra_reads=1 | crates/triage/src/baseline/sql.rs:300-330,448-455 + pulse-app/src/main.rs:543 |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | The RED leg falsified this chunk research PRIME causal hypothesis. Research named viz shared-appender-connection contention as the first-class candidate; the leg reproduced every stated precondition … | — | predecessor log at scratchpad/l4run-171923/logs/agent-latest.jsonl.2026-08-25 v… |
| 2026-08-26-cadence-runaway-blocking-pool | The chunk CENTRAL premise — that a cadence runaway starves the blocking pool and wedges the consumer — is contradicted by its own evidence log. Timeline: last duckdb.append 17:20:50.787, first servic… | extra_reads=4 | scratchpad/l4run-171923/logs/agent-latest.jsonl.2026-08-25 first/last-timestamp… |
| 2026-08-26-cadence-runaway-blocking-pool | plan step 7 stated triage.cue.tick currently resolves to nothing; measured at HEAD an exact leaf exists at observability.rs:1306 carrying 5 of the 7 fields the emit site emits, so the target was PART… | extra_reads=1 | mutation C: with the pre-fix 5-field leaf, cue_tick_resolves_to_an_exact_leaf P… |
| 2026-08-26-cadence-runaway-blocking-pool | plan step 6 named two existing tests as THE regression guard pinning the current 1:1 fan-out semantics; one of them pinned nothing — run_one_emit_cycle_tier_two_fans_to_cadence_triggers seeded 4 erro… | iterations=1 extra_reads=1 | first run after re-pointing: cues_emitted 0 with cues_evaluated 3; a throwaway … |
| 2026-08-26-cadence-runaway-blocking-pool | a SPEC-MASTER claim measured false while writing the Changes section: obs-plan §8:540 states five muted targets carry fields that resolve to NO allowlist entry, but triage.cue.tick resolved to an exa… | extra_reads=1 | mutation C at implement: with the pre-fix 5-field leaf, cue_tick_resolves_to_an… |
| 2026-08-27-report-window-copy-affordance | the /implement P4 report declared all acceptance criteria met, but the wrap report Outcome section re-checked them one by one and found the a11y criterion unsatisfied: aria-busy had been ADDED to the… | iterations=1 | 2026-08-27T21:55:30Z-b |
| 2026-08-27-incident-persist-vs-resolve-write-race | A CARRY on the Diagnostics-sweep entry cited the incidents.list_active.request allowlist leaf at pulse-app/src/observability.rs:2242; at HEAD that line is inside a different targets field list and th… | extra_reads=2 | corrected in andromeda-pulse-0.3.0/chunks/2026-08-27-incident-persist-vs-resolv… |
| 2026-08-27-incident-persist-vs-resolve-write-race | The working-route entry (authored from the predecessor wraps root-cause analysis) describes the defect as a race between TWO writers; the code-graph impact query returns SEVEN production call sites o… | extra_reads=3 | .andromeda/runs/2026-08-27T22-35-43Z-phase/tree-query-2026-08-27-incident-persi… |
| 2026-08-28-ingest-consumer-initiating-freeze | The working-route entrys EVIDENCE clause states three causal claims - the freeze happened under LIGHT load, the cue storm FOLLOWED it by 32 seconds, and the runaway is an amplifier and never the trig… | extra_reads=3 | <host-evidence>/pulse-l4run-171923/logs/agent-latest.jsonl.2026-08-25 |
| 2026-08-28-ingest-consumer-initiating-freeze | The chunks operator-selected shape (verify-and-pin, expecting the CueLatch bound to close the initiating freeze) measured FALSE. Arm A reproduced the wedge at HEAD with the runaway fully bounded - ca… | — | target/gap-resume/run-41760/logs/agent-latest.jsonl.2026-08-28 |
| 2026-08-28-ingest-consumer-block-under-gap-resume | Three durable artifacts state the wedge mechanism as dispatch_batch holding conn.lock() so BOTH shared-connection users die, naming the retention sweep as the second. Measured at HEAD: run_retention … | extra_reads=2 | andromeda-pulse-0.3.0/chunks/2026-08-28-ingest-consumer-block-under-gap-resume/… |
| 2026-08-28-duplicate-span-replay-fails-loudly | Four artifacts state the mechanism as first replayed batch fails LOUDLY then the NEXT hangs — the working-route entry, scope.md, research.md and obs-plan section 10 defect 4 plus rules/observability.… | soft_exit=1 | probe output: seed flush enter/returned ok=true then drop returned, replay flus… |
| (session-scope) | a CARRY annotation on the markerless entry being rewritten stated degraded_mode was driven by Resolved-only persistence so an Active incident always renders degraded; re-derivation at HEAD found two … | extra_reads=3 | registry.rs:338 and :355; falsified by 2026-08-26-interpretation-brief-complete… |
| 2026-08-30-npm-advisory-coverage | working entry CONTEXT states the npm channel is dev/harness-only with no runtime or bundle path; package.json carries 9 runtime dependencies (react, @tauri-apps/api, ...) Vite-bundled into the shippe… | extra_reads=1 | andromeda-pulse-0.3.0/chunks/2026-08-30-npm-advisory-coverage/scope.md §Premise… |
| 2026-08-30-diagnostics-un-muting-harness-truth-sweep | working-entry SCOPE item test-plan-s3-warm-boot-doc-fix-family does not reproduce at HEAD: grep -i warm over test-plan.md returns 0; the s3 body was re-synced by 3 later chunks; scope narrowed to wra… | extra_reads=2 | 2026-08-30T08:33:09Z-b |
| 2026-08-30-diagnostics-un-muting-harness-truth-sweep | working-entry CARRY stated harness:status constructs a tauri::test::mock_builder app; measured at HEAD it calls ui_bridge::health::current_health() in xtask own process with no tauri involvement — sa… | extra_reads=1 | 2026-08-30T08:33:09Z-c |
| 2026-08-30-diagnostics-un-muting-harness-truth-sweep | working-entry item B2 (agent-latest.jsonl bare-name family, 3 readers) is stale against HEAD: both code readers gained rotated-family resolution at 2026-08-25 and test-plan s3 documents it; the live … | extra_reads=1 | 2026-08-30T08:33:09Z-d |
| 2026-08-30-diagnostics-un-muting-harness-truth-sweep | the C4 fork example name (persistence_samples) rested on a 2-of-3-site inventory: research greps of persistence_seconds were head-truncated and hid evaluate.rs:187 where the SILENCE family assigns ge… | extra_reads=1 | 2026-08-30T09:11:12Z-b |

**Proposal:** The largest sub-source is route entries and specs stating mechanism claims as fact. Direction: entry/amendment authoring marks causal-mechanism claims as hypotheses-until-measured (the CARRY grammar already distinguishes annotation kinds), and the verify-at-HEAD discipline — already curated project-side — becomes a named line in the take-up/research playbooks. That converts churn into one cheap re-derivation per claim.

### PU2 — cross-step · `contract.narrow-basis-claim` — 38 cases · weight 16 · 22 chunk(s) · steps: implement/code, implement/fix-loop, implement/smoke, new-session/orientation, phase/research, phase/take-up, phase/validate, wrap-session/gates, wrap-session/reconcile, wrap-session/report, wrap-session/route-resolve
**Pattern:** 38 cases at 11 steps across 22 chunks: counts/absences/availability claimed from a basis narrower than the claim — tail-clipped pipes, single-manifest globs, host-target cargo tree, anchors composed from memory, a stated-authority trace disagreeing with working memory. Every case was caught by re-derivation before or shortly after landing.
**Evidence:** all 38 cases · summed impact: iterations=2 retries=8 reformulations=1 dialogue_rounds=2 extra_reads=33

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-workspace-key-alignment | the master-route append Edit failed because old_string was composed from the file as RENDERED in the CLAUDE.md @import rather than from a direct Read; the rendering differed from the bytes (a + separ… | retries=1 | — |
| 2026-08-14-workspace-key-alignment | an Edit to .claude/rules/observability.md failed because the old_string anchor was composed from memory of how similar rule files are worded rather than from reading the file; a grep for the real sec… | retries=1 | — |
| 2026-08-15-corpus-key-persistence | The cross-process key-persistence test nearly shipped hollow. The obvious in-process form — two OsKeychainBackend instances resolving the same key — would have PASSED under the defect, because keyrin… | — | — |
| 2026-08-15-corpus-key-persistence | Reported '605 incident events written' in the P3 smoke from a grep of '"target":"incident' — that pattern matches incidents.list_active.request, the webview's IPC poll, not incident creation. An inde… | — | — |
| 2026-08-15-corpus-key-persistence | The operator's directive framed the overlap signal's findings as '11 pre-existing advisory findings' needing ID-scoped entries with owners. Re-deriving from cargo deny's own output showed 10 distinct… | dialogue_rounds=1 | — |
| 2026-08-16-baseline-family-reachability | Research stated the reference counts for the two bootstrap constants (2 and 4, "both re-export sites") from a tail-CLIPPED view of the code-graph output. The trace rows field is the authority and say… | extra_reads=1 | .andromeda/runs/2026-08-16T16-30-00Z-phase/tree-query-2026-08-16-baseline-famil… |
| 2026-08-16-baseline-family-reachability | The light gate proved GREEN by exit code but its own test COUNT was never captured: the invocation piped each gate through tail -3, which cut the nextest Summary line. The reported 1790 therefore com… | — | — |
| 2026-08-16-fault-identity-semantics-decided | research.md stated the fingerprint value's cross-file blast radius as 'exactly 5 files' while its own inline enumeration listed 6 (the brace form e2e_{storm_detection,drain_template_assignment}.rs hi… | iterations=1 | .andromeda/runs/2026-08-16T19-21-36Z-phase/tree-query-2026-08-16-fault-identity… |
| (session-scope) | the first coverage extraction read a top-level status key on each matrix capability and yielded verified=0 / unclaimed=0; the contract nests status under verification, and the real counts are 19 veri… | extra_reads=1 | — |
| (session-scope) | Derived the set of registered obs-allowlist target keys by grepping quoted dotted strings, which matched test-literal occurrences of "buffer.tick" and led to a wrong intermediate conclusion that the … | extra_reads=2 reformulations=1 | 2026-08-21T11:53:25Z-c |
| 2026-08-22-pii-scrubber-recall | a file-count stated in the implement console disagreed with the list directly beneath it (6 vs 7); the report is the only detector input, so an internal disagreement could have fed a file-counting de… | — | 2026-08-22T18:42:30Z-b |
| 2026-08-23-ingestion-scrub-coverage | the chunk plan Expected-amendments list named three spec masters and read as complete, but a fourth amendment site existed that no plan step pointed at: architecture Occupied Resources enumerates res… | extra_reads=2 | andromeda-pulse-0.3.0/chunks/2026-08-23-ingestion-scrub-coverage/report.md §Spe… |
| 2026-08-23-ingestion-scrub-coverage | while authoring the arch amendment the orchestrator wrote that TWO of the seven reserved tables are producer-less and framed span_links as merely out-of-frame — an assertion about span_links that had… | iterations=1 extra_reads=1 | .andromeda/architecture-amendments.md |
| 2026-08-23-metrics-points-identity | The graph query result was first read through a tail-truncated shell pipe, which showed 8 of 11 rows and omitted the single most decisive one — reserve_log_seq_block having exactly one caller. Re-rea… | extra_reads=1 | trace rows: 11; the piped view showed rows 4-11 only |
| 2026-08-23-metrics-points-identity | The plan as first written asserted canary literals appear 0 times across the GREEN run agent-latest.jsonl — the BARE filename. The obs sink is tracing_appender::rolling::daily, which date-suffixes ev… | extra_reads=2 | verification-harness.md 2026-06-29: every agent-log READER must glob agent-late… |
| 2026-08-23-metrics-points-labels | the P4 dialogue card argued one option from a claim about the scrubber that was BACKWARDS, and the operator approved on it. The card said the secret_kv and api_key arms being key-name-anchored meant … | dialogue_rounds=1 | 2026-08-23T10:52:54Z-b |
| 2026-08-23-webview-self-verify | Verifying the directive claim that Conductor carries @crabnebula/tauri-driver + wdio + @axe-core/webdriverio in devDeps, the first probe globbed only ui/package.json and */package.json, found nothing… | retries=1 | the false conclusion was stated in reasoning only and corrected before any arti… |
| 2026-08-23-integration-ux-e2e-test | TWO narrow-basis events in one research pass, both caught by re-derivation, both FALSE-NEGATIVE in direction. (1) The first DOM-selector grep was scoped to components/ and routes/ relative to src, wh… | extra_reads=2 | 2026-08-23T14:43:48Z-b |
| 2026-08-23-integration-ux-e2e-test | The msedgedriver the leg REQUIRES to fire was nearly declared absent from a search narrower than the claim. A first probe over the user profile at maxdepth 4 plus a couple of tool directories returne… | extra_reads=2 | 2026-08-23T16:35:26Z-d |
| 2026-08-23-a11y-verification | The first probe for Traces a11y semantics grepped pulse-app/ui/src/components/TraceTable.tsx — a path that does not exist (the file lives at dashboard/routes/traces/) — with stderr suppressed by 2>/d… | retries=1 | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/scope.md#verified-pre… |
| 2026-08-23-a11y-verification | Two truncated probes in one step were read as complete, same mechanism, opposite outcomes. (1) tail -30 on the code-graph refs query backed an ABSENCE claim (zero HaloCanvas render sites); re-derived… | retries=1 | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/research.md#graph-imp… |
| 2026-08-23-a11y-verification | An EXCLUSION was claimed from a sample that cannot support one: n=5 per arm shows only that removing GPU/CPU contention failed to ELIMINATE the flake, not that load has no effect on its FREQUENCY (2/… | — | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/ |
| 2026-08-25-demo-injector-formalized-api-surface-retire | an operator-supplied pre-verified fact carried a correct CONCLUSION on a mis-aimed COORDINATE: the absence claim (model_identity never reaches the tracing wire) is exactly right, but the cited site d… | extra_reads=2 | diagnostics_router.rs:362 target is diagnostics.snapshot.request; contract.rs:8… |
| 2026-08-25-demo-injector-formalized-api-surface-retire | I nearly REJECTED an accurate detector proposal on a truncated view of my own making. Validating T4 (test-plan §1 logs one-liner) I grepped for ANDROMEDA_PULSE_LOG_DIR and piped through cut -c1-160; … | extra_reads=1 | grep with cut -c1-160 hid the clause; sed -n 66p showed `logs` (JSON-lines trac… |
| 2026-08-26-cadence-runaway-blocking-pool | the coverage-gate probe first reported version verified 0 of 22, which would have gone into the commit message as a false coverage collapse. The matrix has no top-level status key — status lives at v… | extra_reads=1 | re-query on verification.status returned 21 verified + 1 planned (P-075, chunk … |
| (session-scope) | an awk classifier tagging working-route entry lines frozen-vs-markerless tested for a leading bracket against a derived string whose colon-strip had left a leading space, so all 20 sampled entry line… | — | orientation trace, working-route entry-line listing |
| 2026-08-26-l4-runtime-security-residuals | an availability claim about a plan-deciding dependency was made from a basis narrower than the claim: probing only the conventional ~/.cargo/registry/src printed 'crate src: NOT FOUND' for strict-pat… | retries=1 extra_reads=1 | P3 probe output 'crate src: NOT FOUND' followed by the CARGO_HOME-aimed find re… |
| 2026-08-26-l4-runtime-security-residuals | an operator wrap directive asserted the dev-vs-bundled binary confusion 'has now cost two consecutive plans (P-077's deviation 1)'. Re-derived at HEAD: the predecessor's plan.md:117 names ./target/de… | extra_reads=2 | predecessor plan.md:117 vs the directive's stated deviation 1 |
| (session-scope) | Two health checks were first measured on a basis narrower than their stated criterion and re-derived before reporting: check 4 was probed as first-line-is-triple-dash (presence) where the criterion i… | extra_reads=4 | agent-run.ps1 L88-L130 switch arms boot/run/status/cleanup/logs |
| 2026-08-27-report-window-copy-affordance | strings on the built exe was used as an ACL-freshness probe (grep for the capability description text just added) and returned 0 both before AND after a clean rebuild — because Tauri strips the descr… | — | 2026-08-27T21:44:35Z-c |
| 2026-08-28-ingest-consumer-initiating-freeze | The relayed claim that the two dead spawn_blocking users used DIFFERENT mutexes (so no single lock explains both) is derived from HEAD, but the wedge corpus is dated 2026-08-25 and viz::query::read_c… | — | git log -S read_connection -- crates/viz/src/query.rs => 27c5ac7 2026-08-26 |
| 2026-08-28-ingest-consumer-block-under-gap-resume | The working entry EVIDENCE clause states a causal mechanism as fact — dispatch_batch holds conn.lock() across every append, SO both shared-connection users die — while the same entry SCOPE clause lis… | extra_reads=1 | andromeda-pulse-0.3.0/chunks/2026-08-28-ingest-consumer-block-under-gap-resume/… |
| 2026-08-28-duplicate-span-replay-fails-loudly | The caller count and first line number for append_record_batch_to_table were written into research.md from a tail-clipped view of the query output (6 callers, first at :673) when the trace rows field… | extra_reads=1 | the exact trap codebase-research.md Shell discipline names — never conclude fro… |
| 2026-08-28-duplicate-span-replay-fails-loudly | An in-repo source comment at crates/buffer/src/schema.rs lines 320-323 states a runtime PK check via the duplicate-INSERT path was observed to hang on libduckdb-sys 1.10502, and test-plan section 4 c… | extra_reads=2 | the first probe attempt was itself inconclusive — both inserts died on a NOT NU… |
| 2026-08-29-advisory-backlog | research.md stated 3 graph queries from working memory; the trace file (the stated authority) holds 2 - caught at the evolve ledger question, corrected before any consumer read it | extra_reads=1 | .andromeda/runs/2026-08-29T21-14-14Z-phase/tree-query-2026-08-29-advisory-backl… |
| 2026-08-29-advisory-backlog | research derived quick-xml 0.39.2 parents from a default-target cargo tree (plist sole parent); the graph-wide truth had a second Linux-only holder (wayland-scanner via the arboard clipboard chain) -… | extra_reads=2 | 2026-08-29T22:07:15Z-b |
| 2026-08-30-dead-lib-src-test-migration | the by-name collection probe parsed nextest list output with a guessed indented format and reported all 12 new targets ABSENT while the suite delta already proved +90; a known-present control (observ… | retries=2 | 2026-08-30T19:54:19Z-c |
| 2026-08-30-agent-harness-teardown-truth | research.md asserted ci.yml's harness step runs with NO exported ANDROMEDA_PULSE_DATA_DIR, derived from reading only the step block (sed 135-160); the workflow-level env at ci.yml:11-12 provides it —… | extra_reads=1 | 2026-08-30T21:31:24Z-b |

**Proposal:** The basis disciplines are curated project-side in several rule entries; the pipeline generalization is one line in the research/validate playbooks — 'a claim's basis must span the claim's full scope; re-derive before writing' — plus U3's shell recipes for the pipe-exit-code masking shape that manufactures narrow bases.

### PU3 — cross-step · `tooling.host-shell` — 36 cases · weight 33 · 15 chunk(s) · steps: implement/code, implement/fix-loop, implement/smoke, new-session/orientation, phase/distill, phase/plan, phase/research, phase/take-up, phase/validate, wrap-session/gates, wrap-session/reconcile
**Pattern:** 36 cases at 11 steps across 15 chunks — the Windows/MSYS semantics family: grep -P locale-gated, printf backslash collapse, MSYS path conversion (cmdkey /list, tasklist /FI, /tmp invisibility), msys≠Windows pid spaces, heredoc quoting failures, && chains dying on designed non-zero, Bash-tool cwd persistence. Every instance was worked around locally; the same knowledge is scattered across project rule files. See level candidate L1 — this is the band-aid theme's typed face.
**Evidence:** all 36 cases · summed impact: iterations=2 retries=28 reformulations=2 extra_reads=6

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-fingerprint-feed-capture-repair | piped a backgrounded cargo build through grep, so the pipeline's exit code became grep's and the completion notification reported exit 0 regardless of the build's real outcome — a repeat of the curat… | — | — |
| 2026-08-14-fingerprint-feed-capture-repair | a PowerShell port-check written inline in bash had its $p variable expanded away by bash before PowerShell parsed the command, producing a parser error on an empty if-condition; re-issued with single… | retries=1 | — |
| (session-scope) | grep -oP marker extraction emitted a locale error and produced EMPTY output, so the half-promote check printed working-route markers: 0 / master records: 0 and both comm diffs empty — a clean-looking… | retries=1 | — |
| 2026-08-15-corpus-key-persistence | cmdkey /list run from Git Bash never executed: MSYS path conversion rewrote the /list switch into a filesystem path, so the command printed its usage banner and exited. Piped through grep it yielded … | retries=1 | — |
| (session-scope) | a single and-chained bash call mixed a presence probe over two optional paths with unrelated reads; the probe's nonzero exit on the legitimately-absent second path aborted the chain and suppressed th… | retries=1 | — |
| 2026-08-17-incident-fingerprint-producer-repaired | a python heredoc embedding a literal Windows path failed to parse — the backslash sequence was read as a unicode escape (SyntaxError: truncated \uXXXX escape) before any code ran; reformulated by bui… | retries=1 | .andromeda/runs/2026-08-17T17-52-59Z-phase/ |
| 2026-08-17-incident-fingerprint-producer-repaired | the cargo audit probe's exit code was first read through a pipeline ending in tail, which reported 0 while cargo audit had actually exited 1; re-run writing to a file to capture the true code, since … | retries=1 | — |
| 2026-08-22-pii-scrubber-recall | a quoted-delimiter heredoc carrying markdown with backticks and apostrophes was rejected by Git Bash as an unmatched single quote; the same content wrote cleanly through the Write tool | retries=1 | 2026-08-22T15:25:19Z-b |
| 2026-08-22-log-records-identity | a single-quoted heredoc (<<QUOTED) carrying ~110 lines of UTF-8 markdown with em-dashes, middots, backticks and apostrophes aborted under Git Bash with an unmatched-quote EOF error despite the quoted… | retries=1 | phase P1 scope write, exit code 2 |
| 2026-08-23-ingestion-scrub-coverage | an evolve append failed at bash parse time because the record text contained an apostrophe (the word disciplines possessive form) inside a single-quoted printf argument, which terminated the string e… | retries=1 | scratchpad/evolve_code.json |
| 2026-08-23-metrics-points-identity | A liveness probe written as tasklist /FI "PID eq N" // echo 'not running' produced a FALSE-NEGATIVE-shaped result: Git Bash rewrote /FI into a Windows path, tasklist exited non-zero on the bad argume… | extra_reads=1 | tasklist error: Invalid argument/option - '<msys-root>/FI'; PowerShell… |
| (session-scope) | the host locale is neither unibyte nor UTF-8 for GNU grep, so -P aborts with a diagnostic instead of matching; the half-promote scan piped that aborted grep into a while-read loop, which then produce… | retries=1 | 2026-08-23T10:09:36Z-b |
| 2026-08-23-metrics-points-labels | a heredoc-quoted Python edit script twice failed to parse because the Rust line it targeted ends in a line-continuation backslash, and a trailing backslash inside a single-quoted Python literal escap… | retries=2 | 2026-08-23T11:15:14Z-c |
| (session-scope) | The half-promote scan's grep -oP aborted with 'grep: -P supports only unibyte and UTF-8 locales'; because the error went to stderr while the counting pipeline still completed, the scan reported 'froz… | retries=1 extra_reads=1 | 2026-08-23T11:49:04Z-b |
| (session-scope) | The host locale disables PCRE in grep, so grep -oP for the frozen-marker extraction returned a locale error instead of matches; the scan was re-expressed with sed substitution + sort/comm and returne… | retries=1 | 2026-08-23T23:03:37Z-b |
| (session-scope) | the Bash tool's cwd persists across calls, so a later `cd andromeda-pulse-0.3.0` failed from inside that dir and aborted its `&&` chain: the half-promote check never ran, while the trailing `; echo d… | retries=1 | orientation trace: cd error line preceding the (no output above = none) print |
| (session-scope) | grep -oP is unavailable on this host (locale-gated PCRE: "grep: -P supports only unibyte and UTF-8 locales"), so the [{marker}] extraction used for the half-promote scan had to be rewritten as an awk… | — | orientation trace: grep -P locale error |
| 2026-08-25-demo-injector-formalized-api-surface-retire | second occurrence this session of the Bash cwd-persistence failure: a `cd {version_dir} && ...` verification chain ran from inside that dir, so its relative paths resolved to nothing and the whole ch… | retries=1 | take-up trace: "cd: andromeda-pulse-0.3.0: No such file or directory" then "No … |
| 2026-08-25-demo-injector-formalized-api-surface-retire | third occurrence this session of the Bash cwd-persistence failure: after a `cd` into the phase run dir for a validation sweep, the next call's relative append path resolved outside the repo and error… | retries=1 | distill trace: ".andromeda/friction-log.ndjson: No such file or directory" imme… |
| 2026-08-25-demo-injector-formalized-api-surface-retire | fourth occurrence this session of the Bash cwd-persistence failure, this time silently splitting a two-record append: a mechanical-check command had cd-ed into the chunk dir, so both relative-path ap… | retries=1 | plan trace: two consecutive "No such file or directory" lines on the friction-l… |
| 2026-08-25-demo-injector-formalized-api-surface-retire | a DIFFERENT host-shell mechanism from this session's three cwd-persistence hits: $TMPDIR is unset in this Git Bash for Windows environment, so a heredoc written to "$TMPDIR/script.py" landed at the f… | retries=1 | validate trace: "/link_p077.py: Permission denied" plus python opening C:/Progr… |
| 2026-08-25-demo-injector-formalized-api-surface-retire | immediately after the 87.6 GiB reclaim, MSYS `df -h /d` reported 29G free while PowerShell Get-PSDrive reported 87.5 GB — ReFS reclaim was still settling and df had cached. Acting on df alone would h… | — | df 29G vs Get-PSDrive 87.5GB in the same minute; both 88G/87.5GB on re-read |
| 2026-08-25-demo-injector-formalized-api-surface-retire | my own chain watcher reported 0 for every marker while the chain had in fact completed — the counting pipeline was `grep -ch ... / paste -sd+ / bc` and `bc` is absent from this Git Bash, so the trail… | retries=1 | watcher printed model.load/incident.created/storm/digest all 0; the JSON target… |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | The output-masking family fired TWICE against my own probes in one step, past a handoff entry that names it as recurring. (1) The gate-discrimination harness read rc from a pipeline ending in tail, s… | iterations=1 extra_reads=2 | gate-proof.sh run 1 vs run 2 (stalled arm exit=0 -> exit=1 after removing the p… |
| 2026-08-26-ingest-consumer-stall-under-sustained-load | THIRD occurrence this session of the output-masking family, and the most consequential: the light gate was launched as a background command ending in / tail -3, so the task-notification reported exit… | iterations=1 | task-notification exit code 0 vs cargo exited with code 101 in the same output … |
| (session-scope) | Git Bash printf processed backslash escapes inside the %s ARGUMENT, collapsing a doubled backslash in the friction record's what-field to a single one and writing an invalid JSON escape; the append s… | retries=1 | json.decoder.JSONDecodeError Invalid escape at char 358 of the first-written li… |
| (session-scope) | grep -oP is unavailable on this host (locale not unibyte/UTF-8), so the PCRE marker-extraction for the half-promote anomaly check emitted only the locale error; a count-only fallback (43 frozen vs 43… | retries=1 | 2026-08-27T20:39:21Z-b |
| (session-scope) | printf on this host interpreted backslash escapes in its %s argument, collapsing the JSON \ escapes of an evolve record so the appended line was invalid JSON; the corruption was silent at write time … | reformulations=1 | two malformed lines truncated and re-appended via python json.dumps; the surviv… |
| 2026-08-28-duplicate-span-replay-fails-loudly | rev is absent from this host shell, so a tail-anchor extraction failed with command not found and was re-derived in python; this is the third fallback to python for host-tool behaviour in this sessio… | reformulations=1 | phase P1 take-up; the python slice returned the anchor on the next call and the… |
| (session-scope) | chained the CLAUDE.md presence probe with && into a state.yaml read in one call; the designed-absent .claude/CLAUDE.md made ls exit 2 and short-circuited the rest of the chain, so state.yaml was re-i… | retries=1 | 2026-08-29T16:44:09Z-b |
| 2026-08-29-advisory-backlog | a / tail pipe on the first nextest gate made the shell exit 0 while cargo exited 101 (the completion notification read success) AND truncated the diagnostic to the last lines twice; recovered by re-r… | retries=1 | scratchpad/nextest-run2.log |
| 2026-08-29-advisory-backlog | the persistent bash working directory had silently drifted into .andromeda (an earlier chained cd), so bare-relative master greps resolved correctly by accident while the sidecar-tail reads failed wi… | retries=1 | 2026-08-29T23:22:27Z-b |
| (session-scope) | git-bash grep rejects -P (PCRE) under this locale; the marker extraction silently yielded 0 lines, making the downstream half-promote check vacuous until redone with sed | retries=1 | 2026-08-30T00:12:46Z-b |
| 2026-08-30-npm-advisory-coverage | bash redirect to /tmp then Windows python read of /tmp/... failed (MSYS path mapping not visible to native python); one retry via the scratchpad path | retries=1 | 2026-08-30T01:03:55Z-b |
| 2026-08-30-npm-advisory-coverage | SAME-SESSION RECURRENCE of the /tmp mapping class: bash sed wrote /tmp/line117.txt, Windows python could not see it (FileNotFoundError); recovered via the scratchpad path — second hit after the imple… | retries=1 | 2026-08-30T01:40:27Z-b |
| 2026-08-30-diagnostics-un-muting-harness-truth-sweep | the smoke script wait-loop keyed phases on grep -q under set -o pipefail: grep -q exits on first match and closes the pipe, cat dies SIGPIPE, pipefail fails the pipeline ON THE MATCH — so the phase m… | extra_reads=2 | 2026-08-30T09:51:31Z-d |

**Proposal:** Consolidate a win32 host-recipe annex into the pipeline's shell-discipline references (python-by-path default, no -P, no printf-for-JSON, granular commands, PowerShell probes for pids/ports) so each new project stops rediscovering the family. Most content already exists as scattered curated rules here.

### PU4 — cross-step · `contract.structural-blind-spot` — 15 cases · weight 8 · 8 chunk(s) · steps: implement/code, implement/smoke, new-session/orientation, phase/plan, phase/research, phase/validate, wrap-session/gates, wrap-session/reconcile, wrap-session/route-resolve
**Pattern:** 15 cases at 9 steps across 8 chunks: documented mechanisms that could not reach a defect BY CONSTRUCTION — no detector owning falsified narrative mechanisms in spec bodies (3 reconcile cases), a deferred-gate age trigger that never fired across five re-pins, guards homed where [lib] test=false means they never run, per-procedure sweeps under-running fixed lists.
**Evidence:** all 15 cases · summed impact: iterations=1 dialogue_rounds=3 extra_reads=8 deferred=5

| chunk | what | impact | evidence |
|---|---|---|---|
| (session-scope) | the route-resolve deferred-gate AGE TRIGGER (halt once at the third consecutive re-pin) never fired across five consecutive workspace-nextest deferrals (origin 2026-07-06, re-deferred at 07-07/07-08/… | deferred=5 | — |
| 2026-08-14-workspace-key-alignment | no P5 check reaches a scope deliverable that is present as a plan STEP but absent from the acceptance criteria: mechanical check 4 only asks whether a criterion asserts an artifact with no producer (… | dialogue_rounds=1 iterations=1 | — |
| 2026-08-15-corpus-key-persistence | Research measured that the tracing target corpus.open.error has no resolvable entry in the pulse-app allowlist (no exact key, and no bare 'corpus' key — only the exact corpus.pipeline_metrics.purge),… | extra_reads=1 | — |
| 2026-08-15-corpus-key-persistence | The arm-zero classification's downstream narrowing was obstructed by default-deny redaction on three targets that carry exactly the counts needed: incidents.list_active.request/item_count, triage.inc… | extra_reads=2 | — |
| (session-scope) | The 0-pending no-op path never fires the Setup background code-graph refresh and never runs P4, yet its commit still MOVES HEAD - which silently invalidates .andromeda/cache/tree.db.commit and would … | — | commit 63316aa; cache listing showed no .refresh-done; tree.db.commit had equal… |
| 2026-08-22-log-records-identity | the plan instructed that the two out-of-crate fixture INSERTs be updated only 'if the fixture takes the column as NOT NULL' - a conditional whose predicate can never be false, because seq is a PRIMAR… | dialogue_rounds=1 | plan.md Files-to-modify note on the three INSERT sites, and the grep acceptance… |
| (session-scope) | The coverage surface derives unclaimed as chunk:null AND verification.status:planned, which cannot distinguish a cap pooled by explicit operator decision from one silently skipped; P-075 is pooled by… | extra_reads=2 | verification-matrix.json P-075 notes: "Cap stays pooled (chunk:null) by operato… |
| 2026-08-23-metrics-points-identity | security-plan §Security Anti-Patterns → Logging has required amendment at three consecutive chunks (2026-08-22-pii-scrubber-recall, 2026-08-23-ingestion-scrub-coverage, 2026-08-23-metrics-points-iden… | dialogue_rounds=1 | security-plan-amendments.md entries at 2026-08-22-pii-scrubber-recall and 2026-… |
| (session-scope) | A route entry's self-declared ordering claim is unenforced prose and had silently gone false. The Webview self-verify entry's own SCOPE reads 'this entry is their prerequisite, which is why it sits a… | extra_reads=1 | andromeda-pulse-0.3.0/verification-matrix.json P-076 verification.acceptance |
| 2026-08-25-demo-injector-formalized-api-surface-retire | a detector reasoned correctly and still missed real drift, by construction. D-security-input evaluated "no NEW path env var was introduced" — true, and faithful to the fan-out contract that Changes i… | — | security-plan detector returned proposals: [] with an explicit no-new-path-env-… |
| 2026-08-27-incident-persist-vs-resolve-write-race | My GREEN-leg wait condition was VACUOUS BY CONSTRUCTION. It polled the log for resolved_count matching [1-9] on triage.incident.auto_resolve.tick — a target with NO allowlist leaf, so the field rende… | — | scratchpad/green_leg.sh |
| 2026-08-28-duplicate-span-replay-fails-loudly | The obs extract acceptance requires the duckdb.append failure ERROR present in arm A while the scope item 2 removes the very collision that produces it — a direct contradiction between an extract and… | — | research.md premise 3 VERIFIED as a real gap; plan.md Steps 5-6 plus the order-… |
| 2026-08-28-duplicate-span-replay-fails-loudly | Eight new tests in an auto-discovered [[example]] target passed under cargo test --example while cargo nextest run --workspace collected zero of them, because Cargo defaults an example to test = fals… | extra_reads=2 | cargo nextest list --workspace grep inject_demo returned 0 before the manifest … |
| 2026-08-28-duplicate-span-replay-fails-loudly | obs-plan returned proposals empty while its own section 10 defect 4 — the defect this chunk closed — owed an amendment; its three detectors bind to new hot-path instrumentation, the logger stack and … | — | Validate check 5 raised it as routine; the predecessor chunk handoff recorded t… |
| 2026-08-28-duplicate-span-replay-fails-loudly | A measurement disproved a claim living in a SOURCE COMMENT (crates/buffer/src/schema.rs lines 320-323) rather than in a spec master; the spec that cited it was amended in P2 but wrap does not edit so… | — | report disproof 1; test-plan section 4 corrected at this wrap while the comment… |

**Proposal:** Each case minted its own fix mid-epoch (D-obs-defect-narrative and siblings landed). Direction: fold the recurring class — 'a falsified narrative/mechanism claim in a spec body' — into drift-base as a first-class detector, since three independent cases converged on exactly that gap.

### PU5 — cross-step · `contract.token-proxy-check` — 8 cases · weight 3 · 6 chunk(s) · steps: new-session/orientation, phase/distill, phase/plan, phase/research, phase/validate, wrap-session/gates
**Pattern:** 8 cases at 6 steps: checks written as literal-token tests where the property is semantic (the anchor check matching 'per ', placeholder-lookalike notation, epoch-diagnosed greps) — both failure directions occurred (spurious HALT and suppressed signal), and the truth came from reading the hits each time.
**Evidence:** all 8 cases · summed impact: retries=2 extra_reads=7

| chunk | what | impact | evidence |
|---|---|---|---|
| (session-scope) | the working-route entry count was cross-checked by counting separator lines with grep -c on the separator glyph, but grep -c counts lines CONTAINING the token rather than lines that ARE the separator… | extra_reads=1 | 2026-08-23T10:09:36Z-c |
| 2026-08-23-metrics-points-labels | the anchor check (check 3) was mechanized as a search for the literal shape per {plan}.md followed by a section sign, but the rule asks only that a bullet cite a plan section anchor. Six layouts Cons… | extra_reads=1 | 2026-08-23T10:27:49Z-c |
| 2026-08-23-metrics-points-labels | the directive specified the staged-bindings assertion as BOTH the mcp entries (5 today) AND labels: string. Measured before asserting: 5 is the count of LINES containing the substring mcp, and THREE … | extra_reads=2 | 2026-08-23T11:40:29Z-b |
| 2026-08-23-webview-self-verify | Re-verifying that every Expected-amendments entry names a spec master, the check grepped the block with the pattern backtick dot-andromeda-slash bracket a-z hyphen bracket plus dot-md — a character c… | retries=1 | first grep returned architecture.md + test-plan.md only; re-run with backtick-a… |
| 2026-08-23-a11y-verification | The pre-P5 leak check grepped scope.md for the literal token [inferred] to test the SEMANTIC property has the premise closure resolved every inferred bullet. It returned 1 hit and read as unresolved … | extra_reads=1 | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/scope.md#p3-premise-c… |
| 2026-08-27-incident-persist-vs-resolve-write-race | My own per-extract check-3 (anchor citation) tested for the literal token "per " where the intended property is semantic — does the bullet cite a plan section anchor. FALSE POSITIVE direction: 12 spu… | retries=1 | scratchpad/validate_extracts.py |
| 2026-08-27-incident-persist-vs-resolve-write-race | Second occurrence of the same class in one session. Verified the scope premise-closure by grep -c for the literal [inferred] token expecting 0; got 3. FALSE POSITIVE direction: all three hits were pr… | extra_reads=1 | 2026-08-27T22:56:59Z-c |
| 2026-08-28-duplicate-span-replay-fails-loudly | The per-extract anchor check (validation check 3) was implemented as a test for the literal token per, but the intended property is semantic — does the bullet cite a plan section anchor; it flagged 6… | extra_reads=1 | validation pass reported anchor-miss:Constraints:6; the bullet dump showed all … |

**Proposal:** Re-express the named checks (fan-out per-extract check 3; validate placeholder scan) in parse-based form; add the failure-direction note to their criteria so authors test both ways.

## Cross-step chains (starting heuristics)

### X1 — phase/research —[research]→ implement/code · implement/fix-loop · phase/plan — 13 joins, ≥10 chunks
The dominant lineage: research returns `outcome: ok` carrying the honest `unresolved-questions` signal, and downstream consumers grade the artifact `thin`/`wrong` — under-enumerated caller/boundary sets (fingerprint-feed, baseline-family, a11y ×3, interpretation-brief), uninspected live-leg forms (metrics-labels), one wrong-violator claim (ingest-stall). Both ends' records are in q-chains.json.
**Direction:** the producer-side checks in P23/P24 (graph caller-enumeration per signature change; live-leg invocation as a required research object). Consumers could also treat `unresolved-questions` as "boundary list provisional" rather than final.

### X2 — phase/plan —[plan]→ implement/fix-loop · implement/smoke · wrap-session/gates · phase/validate · implement/code — 13 joins, ≥9 chunks
Plan Test-Commands defects surface downstream: a combined deny invocation that can never be green, a wrong playwright config, a binary path that exists on no host, a SCENARIO= flag that does not exist, an omitted smoke on a boot-path touchpoint. Validate's producing-invocation rule catches some; the rest land at implement or the light gate.
**Direction:** a P5 mechanical executability pass over Test Commands (cheap --help/--list dry-runs; path existence), pairing with P3's upstream authoring checks.

### X3 — wrap-session/report —[report]→ wrap-session/reconcile — 2 chunks
Reports sufficient for six of seven detectors while the seventh finds a missed disproof; a wrong number attributed to a default. **Direction:** P26/P27 (report re-derivation pass + fact-slots).

Single-occurrence shapes (distill→research extracts; take-up self-joins; plan→report) are in q-chains.json; no aggregation claimed. 176 anchors total; most NO-PRODUCER joins reflect artifact-name granularity (file paths vs artifact names) — a known limit of the starting heuristics.

## Level candidates (systemic-masked-as-project)
_Facts and clusters in `q-level.json` (Pass-A pile: 172 workaround/prohibition · 39 removed-cause · 16 deferred · 13 overridden + 5 E3-lookback). No verdict that any level call was wrong — that judgment is the founder's._

### L1 — band-aid · environment · Windows host-shell semantics — 36 typed cases + ~25 workaround facts
**Facts:** the U3 table plus the workaround pile (grep -P locale ×6+, MSYS path conversion, printf collapse, heredoc EOFs, pid spaces, /tmp invisibility, && chain aborts, cwd persistence) — every fix a local workaround or a project rule entry.
**Level hypothesis:** the cause lives in the host+pipeline recipe layer (skills prescribe POSIX-shaped probes this host breaks); the fixes have accumulated in ONE project's rule files.
**Proposal:** the U3 win32 recipe annex at pipeline level.

### L2 — band-aid · environment · compound-command permission denials — ~11 facts
**Facts:** rm -rf + mkdir + GUI-launch compounds denied by the permission layer in ≥9 chunks; every chunk re-shaped to granular steps + fresh unique dirs (q-level.json pile).
**Level hypothesis:** the deny class is stable and the skills' smoke recipes keep producing the denied shape.
**Proposal:** smoke recipes prescribe granular steps + unique-dir-no-delete natively (no rm in the launch path).

### L3 — CLOSED band-aid · process/environment · harness verbs unusable for smokes — ~10 facts
**Facts:** direct-binary launches replacing agent-run boot/status across 9+ chunks (pile), plus the typed P9 group.
**Level hypothesis → outcome:** absorbed chunk-by-chunk until two dedicated project chunks removed the cause (harness:status; teardown-truth). The epoch's textbook band-aid→root-fix arc — recorded as closure, no proposal.

### L4 — override · process · trajectory-halt fires on pre-directed entries — 4+ facts (+ P6's 8 cases)
**Facts:** operator pre-directions repeatedly satisfying new-chunk-ahead halts (workspace-key wrap, ingest-stall, incident-persist, mechanics-probe re-home; E3 lookback ×2 same shape).
**Level hypothesis:** the RULE is miscalibrated for the already-directed case, not the project.
**Proposal:** pre-direction satisfies the gate (P6).

### L5 — rule-friction · process · inline-python vs file-by-path discipline — ~14 facts, both directions
**Facts:** ~9 deliberate inline uses noting "no mangling occurred" beside ~5 transport failures that vindicate the rule (heredoc EOFs, printf collapse, $TMPDIR-unset root write).
**Level hypothesis:** the rule's scope does not match the risk profile — small quote-free payloads are safe inline; document-sized or quote-bearing payloads are not.
**Proposal:** make the discipline size/content-conditional so compliance matches risk, instead of a blanket rule that is routinely and harmlessly violated until it isn't.

### L6 — chronic-degrade · resources · parallel-link memory + target/ disk accumulation — E3:1 → E4:17 typed + 6 removed-cause events
**Facts:** the P2 table; cargo-clean reclaims of 225G/290G/180G/87G; two new presentations of the same family (os 1455, 0xc000012d) folded into the testing.md family entry mid-epoch.
**Level hypothesis:** recurs across epochs, never halts, remedied per-instance — the halt policy structurally never surfaces it.
**Proposal:** P2's mechanical hygiene surface (health-check line + clean cadence).

### L7 — band-aid · process · raw-twin duty under entity escapes — ~8 facts
**Facts:** twins skipped/reconstructed-by-inverse-encode at ≥8 distills, each with an in-the-moment justification note (pile).
**Level hypothesis:** the reference's twin rule predates the entity-escape reality; reconstruction IS the de-facto sanctioned form.
**Proposal:** codify it (P1).

### L8 — deferred-forever scan · residue after closure-tracing
**Facts:** 16 deferred facts + 5 `deferral-open` signals. Closures TRACED for the major ones (muted diagnostics → 2026-08-30 sweep; strict-path → advisory-backlog DROP; injector counter → replay salt; cadence runaway → its chunk; flip-compaction regex → later flips ran).
**Residue:** the 2026-08-15 Suggested-storm drop asymmetry has no closure visible in-ledger; the msedgedriver-path unit-test gap is trigger-owned (open by design); TWO deferred facts carry EMPTY notes (idle-observer + diagnostics-sweep gates records) — a capture defect: a deferral fact with no note is untraceable. Worth one mechanism-health eye.

## Playbook-extension candidates (untyped patterns, F-4)
_75 untyped records in-epoch (E3 lookback: 6); clusters below meet F-4 (n≥3, or promoted-type evidence). Extending a playbook is an Andromeda change only the founder applies._

### U-A — implement/fix-loop — 8 cases → proposed `tooling.generated-artifact-clobber`
**Draft criteria line:** A generated artifact (TauRPC bindings.ts) is rewritten as a side effect of running a documented gate, reddening an unrelated gate downstream.
**Note:** 8 untyped cases (6 at fix-loop, 2 at gates beside the typed commit-mechanics group). Project-absorbed by the staged gate, but the TYPE is worth having: any codegen-emitting test run can clobber a committed artifact.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-14-workspace-key-alignment | cargo xtask capability-drift reported 3 missing mcp.* procedures purely because the default-features workspace nextest had overwritten pulse-app/ui/src/bindings/index.ts to the no-mcp shape; the docu… | retries=1 | — |
| 2026-08-22-log-records-identity | the PREVIOUS chunk's wrap commit (70344d5) shipped bindings.ts in the no-mcp shape, so cargo xtask capability-drift was RED at HEAD before this chunk began, while that chunk's handoff recorded capabi… | iterations=1 | git log -3 on the file; HEAD 70344d5 grep -c mcp = 0 |
| 2026-08-23-metrics-points-identity | The TauRPC bindings clobber recurred a SEVENTH time. As at the previous chunk, this one touched NO TauRPC at all - running the plan-mandated workspace nextest is itself what rewrote pulse-app/ui/src/… | extra_reads=1 retries=1 | grep -c '"mcp":' returned 0 after the workspace suite and 1 after regen; capabi… |
| 2026-08-26-l4-runtime-security-residuals | the bindings.ts clobber recurred: the default-features workspace nextest rewrote pulse-app/ui/src/bindings/index.ts to the no-mcp shape, so capability-drift reported 3 missing mcp procedures despite … | retries=1 | 2026-08-26T17:15:29Z-c |
| 2026-08-27-incident-persist-vs-resolve-write-race | The plans ## Test Commands block is not self-sufficient on this repo: cargo nextest run --workspace regenerates pulse-app/ui/src/bindings/index.ts to the no-mcp shape, so running the listed gates in … | — | 2026-08-27T23:28:44Z-b |
| 2026-08-29-app-registry-reconciliation | The light gate's own nextest --workspace clobbered pulse-app/ui/src/bindings/index.ts back to the no-mcp shape (staged check read 0 before regen, 1 after), exactly as the 2026-08-22 routine-ORDERING … | — | staged check: git show :pulse-app/ui/src/bindings/index.ts / grep -c mcp -> 1 |
| 2026-08-14-workspace-key-alignment | the operators directive 4 ordering proved load-bearing on the first wrap that applied it: the P7 light gate ran cargo nextest run --workspace (default features), which rewrote pulse-app/ui/src/bindin… | — | — |
| 2026-08-22-log-records-identity | the staged-bindings ordering rule codified earlier in THIS wrap fired on the very wrap that minted it, and caught a live clobber: after the light gate's nextest --workspace, the worktree bindings had… | — | worktree mcp count 1 -> 0 after the light gate, 1 after regen; staged assertion… |

### U-B — phase/take-up + wrap-session/route-resolve — 4 cases → proposed `tooling.long-line-edit`
**Draft criteria line:** A single-line multi-KB route/master entry defeats the anchored-Edit / Read path (anchor-match failures, consumed line breaks, forced duplicate reads).
**Note:** 4 cases; sibling of the orientation output-cap group (P12). The recovery recipes (python read-modify-write by path, structural extraction) recurred unprompted — a type would let them converge.

| chunk | what | impact | evidence |
|---|---|---|---|
| (session-scope) | Route entries have grown long enough that the Edit-tool write path is the binding constraint on reordering them: the first markerless entry is 9975 chars on ONE line (13 CARRYs + a PREREQ + a NOTE), … | extra_reads=1 | 2026-08-23T11:58:06Z-c |
| (session-scope) | An anchored Edit that removed a trailing annotation ending at a line terminus also consumed the following line break, merging the next separator line onto the edited line; the structural invariant ch… | retries=1 extra_reads=2 | wrap run dir 2026-08-25; frozen-set byte-identity + separator-count check post-… |
| 2026-08-26-l4-runtime-security-residuals | the Edit tool's read-before-edit precondition forced two Read-tool calls over content already read via bash sed in the same step, and both targets are single lines of roughly 2000 words (working-rout… | extra_reads=2 | 2026-08-26T13:45:02Z-b |
| 2026-08-29-app-registry-reconciliation | Anchoring an Edit on a long span copied from Read output failed against a ~4000-char master-route record; the promotion contract itself executed correctly (mint, folder, stamp, master-last), so the c… | retries=1 extra_reads=1 | phase P1 promotion, master-route.md line 68 append |

### U-C — wrap-session/route-resolve — 5 cases → proposed `contract.standing-pin-carriage`
**Draft criteria line:** A standing-deferral pin's migration across frozen entries hits an edge (verbatim-migration impossible after the satisfying wrap; compact form blocked by a changed owned set; between-point half-satisfied upstream).
**Note:** 5 cases on the cargo-audit pin alone, including one positive first-exercise. The machinery works; the type would make its recurring edges visible.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-16-baseline-family-reachability | FIRST live exercise of the ratified re-run-interval machinery codified at the 2026-08-16 0-pending wrap, and it held: this wrap is session 26, the probe fires at 28, so the chunk report recorded the … | — | — |
| 2026-08-17-conductor-e2e-verification-closure | the standing cargo-audit pin has now migrated across five consecutive working-route entries because the rule says it rides the CURRENT entry and every wrap freezes that entry — so each wrap must loca… | extra_reads=2 | — |
| (session-scope) | A route annotation carrying a forward-looking session-counted claim cannot migrate verbatim across the very wrap that satisfies it: the audit pin read 'the NEXT wrap is point 37 and fires the probe i… | extra_reads=1 | .andromeda/runs/2026-08-23T11-52-00Z-wrap/audit-probe.txt |
| 2026-08-23-a11y-verification | The cargo audit standing-deferral re-pin could NOT take the ratified compact form: the probe-auto-satisfy signature holds the basis AND the named overlap, and the overlap shifted from seven owned upg… | — | andromeda-pulse-0.3.0/chunks/2026-08-23-a11y-verification/report.md |
| 2026-08-28-ingest-consumer-block-under-gap-resume | The between-point pin requires re-verifying basis + overlap, and the chunk report had already ASSERTED the overlap result (8 distinct ids, identical to the prior enumeration) from the implement-phase… | extra_reads=1 | P5 re-verification: 8 distinct RUSTSEC ids 0189/0190/0194/0195/0204/0222/0253/0… |

### U-D — phase/validate + phase/take-up — 3 cases → proposed `input.out-of-pipeline-source`
**Draft criteria line:** The decisive fact lives outside every artifact the skill's input contract reads (a sibling repo, a gitignored side-note, a repo phase cannot read).
**Note:** 3 cases. Criteria line: record when a step's conclusion required (or was blocked by) a source outside the skill's declared inputs.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-15-tier-1-incident-path-investigation | The fact that decisively upgraded research's arithmetic hypothesis lived in a repo phase cannot read. Research derived the fit from Pulse's side only (6 measured fingerprints vs DEFAULT_AUTONOMOUS_TH… | dialogue_rounds=1 | — |
| 2026-08-23-metrics-points-labels | the no-external-witness premise could only be checked by reading a SECOND repository (../conductor at its own HEAD), because the claim is about what a sibling project does NOT emit. An absence claim … | extra_reads=2 reformulations=1 | conductor HEAD fb8a8a5: conductor-emit/src has no metrics module; grep across a… |
| 2026-08-25-demo-injector-formalized-api-surface-retire | the only record of a PRIOR paused attempt at this chunk's headline deliverable (a real-model chain proof) lives in AI-Model/RESUME-NOTE.md — a gitignored, non-Andromeda side-quest note outside every … | extra_reads=2 | build.rs TOKENIZER_MAX_BYTES = 32*1024*1024 + the refuse-truncation guard at :1… |

### U-E — cross-step — 4 cases → proposed `contract.skill-reference-drift`
**Draft criteria line:** An Andromeda skill body/reference states a path, enumeration, or rule that mismatches the deployed reality or its sibling reference (flat tree.db path vs per-plane; the nudge needing a read the evolve contract forbids; a 3-form fold list vs a 4th form in the wild; the version-dir glob aimed at the wrong root).
**Note:** 4 cases. These are pipeline defects invisible to project drift detection; a type routes them to the founder.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-15-corpus-key-persistence | promotion.md's fold instruction enumerates exactly three annotation forms (PREREQ:/CARRY:/BLOCKED-ON:), but the taken-up entry carried a 'FIRST STEP:' annotation — a fourth form wrap's route-resolve … | — | — |
| (session-scope) | two references disagree by construction: the new-session body derives the evolve nudge from whether friction-log.ndjson carries records for a frozen epoch, while evolve-system.md states the file is a… | extra_reads=1 | — |
| (session-scope) | The version-dir confirmation glob was aimed at .andromeda/andromeda-pulse-* though the skill body names the PROJECT ROOT; the probe returned empty (and exit 2), costing one re-probe at the correct lo… | extra_reads=1 | 2026-08-26T07:47:22Z-b |
| 2026-08-26-l4-runtime-security-residuals | phase SKILL.md Setup step 5 names the code-graph DB as .andromeda/cache/tree.db, but this project's real layout is per-plane at .andromeda/cache/{plane}/tree.db (rust + ts both built); the flat path … | retries=1 extra_reads=2 | _duckdb.CatalogException: Table with name symbols does not exist! Did you mean … |

### U-F — all steps — 1 cases → proposed `tooling.output-cap-overflow → promote to Universal`
**Draft criteria line:** The orientation playbook already types this; it fired untyped at phase/research (a -A6 grep) and appears as problem-facts at several other steps.
**Note:** Promote the existing type to the Universal list so any step can use it; the criteria line already exists in the orientation playbook.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-30-agent-harness-teardown-truth | first testing.md fingerprint-facet grep (-A6 context) overflowed the 34KB tool-result cap into a persisted file; recovered with one narrower -o pattern re-grep | retries=1 | 2026-08-30T21:00:52Z-b |

### U-G — phase/plan — 3 cases → proposed `input.extracts-conflict — broaden criteria`
**Draft criteria line:** 3 untyped conflicts were INTRA-extract or extract-vs-skill-contract — shapes the existing type's extract-vs-extract wording does not cover and P2 aggregate check C structurally cannot see.
**Note:** Broaden the type's criteria line to any authority-conflict surfacing at synthesis, and extend check C's classes to match (pairs with P22).

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-16-baseline-family-reachability | An INTRA-extract tension surfaced at synthesis that no aggregate check could catch (P2 checks A/B/C look across extracts, not within one): the arch extract bans in-process test-mode bypasses and back… | — | — |
| 2026-08-23-webview-self-verify | A conflict surfaced at synthesis that P2 aggregate check C structurally cannot catch, because it is not a binding contradiction between two extracts but a spec clause contradicting the chunk premise … | — | plan.md §Implementation notes → Expected amendments (wrap), second bullet |
| 2026-08-25-demo-injector-formalized-api-surface-retire | an extract and a skill contract disagreed on WHO applies a spec-master correction, a pair P2 aggregate check C cannot see (it compares extracts to extracts). The tests extract routed the two .androme… | iterations=1 | plan-template.md Discipline: Spec masters are NEVER touchpoints |

### U-H — wrap-session/route-resolve — 3 cases → proposed `contract.carry-no-owner — broaden to placement`
**Draft criteria line:** 3 untyped cases were placement/slot/owner ambiguity (a forced-early trajectory decision; unpinned slots after a partial operator direction; a non-obvious owner) — adjacent to the typed carry-no-owner group.
**Note:** Broaden the criteria to carry/entry PLACEMENT ambiguity, or add ambiguity.carry-placement beside it.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-15-tier-1-incident-path-investigation | The route TRAJECTORY decision (create a new markerless entry) was forced one phase EARLY, at P2 reconcile rather than at P5 route-resolve, because an obs-plan section 8 amendment had to NAME the rout… | dialogue_rounds=1 | — |
| (session-scope) | The operator adaptation request pinned the FIRST tail slot explicitly ("no other tail changes - Baseline-family reachability stays first") while the same request created a new entry that itself neede… | dialogue_rounds=1 | — |
| 2026-08-29-app-registry-reconciliation | One carry had a non-obvious owner and took an extra read to place: the pulse://stream/incidents producers-but-no-consumer finding is not a diagnostic MUTING issue (that entry's headline class) but a … | extra_reads=1 | working-route L120 SCOPE, read before pinning |

### U-I — implement/fix-loop + implement/smoke — 3 cases → proposed `contract.vacuous-check-found`
**Draft criteria line:** The mutation protocol keeps surfacing vacuous or insensitive checks (a pin iterating an empty vector; four pre-existing tests insensitive to the neutralized fixture; a verdict half true in both worlds).
**Note:** 3 cases, all positive yield. A type would let the diagnosis COUNT the mutation protocol's catch rate instead of finding it in prose.

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-08-17-conductor-e2e-verification-closure | the mutation check surfaced a vacuity in one of the new pins written this chunk: canned_evidence_refs_survive_the_scrubber_unredacted iterates the evidence_refs vector, so with the fixture neutralize… | — | — |
| 2026-08-17-conductor-e2e-verification-closure | the mutation check also answered the protocol's which-pre-existing-tests-are-insensitive question with a concrete finding: all four pre-existing MCP incident-tool tests stayed green under the neutral… | — | — |
| 2026-08-29-app-registry-reconciliation | The mutation check disproved one of the legs own verdict halves. Neutralizing reconciliation left item_count going 2 to 0 in BOTH arms — the finite storms auto-resolve fires 120s after last re-emissi… | iterations=1 | RED /tmp/leg-red.txt reconciled 0 declined 2; GREEN /tmp/leg-green5.txt reconci… |

## Below threshold — no action
_Visible for the founder's eye only; nothing here is proposed._

- [2×] phase/distill/retry.distiller-respawn — retries=2
- [2×] phase/research/retry.query-reformulation — reformulations=2
- [2×] new-session/orientation/input.handoff-git-mismatch — —
- [2×] phase/research/ambiguity.scope-boundary — extra_reads=3 deferred=1
- [2×] implement/code/input.conventions-gap — extra_reads=4
- [2×] implement/fix-loop/tooling.gate-deferral — deferred=3
- [2×] implement/smoke/contract.spec-reality-gap — extra_reads=1
- [1×] wrap-session/gates/contract.coverage-hold — dialogue_rounds=1 halted=1
- [1×] wrap-session/route-resolve/input.outcome-unclear — dialogue_rounds=1
- [1×] wrap-session/route-resolve/ambiguity.tier-routing — dialogue_rounds=1
- [1×] phase/research/input.cookbook-gap — reformulations=1
- [1×] implement/smoke/retry.smoke-reentry — retries=1
- [1×] wrap-session/curation/contract.marker-write — iterations=1
- [1×] wrap-session/route-resolve/contract.promotion-mechanics — iterations=1
- [1×] implement/fix-loop/ambiguity.scope-pressure — iterations=1
- [1×] new-session/orientation/contract.schema-assumed — retries=1
- [1×] phase/take-up/ambiguity.version-derivation — —
- [1×] implement/code/ambiguity.scope-pressure — extra_reads=1
- [1×] wrap-session/reconcile/contract.detector-fact-gap — extra_reads=1
- [1×] wrap-session/gates/contract.spec-reality-gap — deferred=1
- [1×] wrap-session/reconcile/input.spec-source-gap — extra_reads=2
- [1×] wrap-session/gates/contract.matrix-claim — —
- [1×] new-session/orientation/input.route-state-ambiguous — extra_reads=1

Emerging untyped singletons/pairs (watch next epoch): AskUserQuestion rejects preview:null options (1) · parse-mutate-json.dumps destroys authored JSON layout (1) · report-prose defects no gate reads (CJK stray char; bullet disposition) (2) · rotated-log bare-name trap re-hit (1) · same-chunk procedure+caller needs regen→dist→re-embed before any live leg (1, curated) · sidecar MCP double-gate applies to the sidecar binary itself (1) · code-graph calls view mis-attributed a caller (1) · smoke self-check catching harness-sourced false claims (accessor typo; total-0 misread) (2) · permission-policy denial has no covering type (1 — theme owned by L2).

_Chains twin: q-chains.json (176 anchors). Retractions twin: q-retractions.json. Generated by compose_proposals.py from the q-*.json evidence twins._