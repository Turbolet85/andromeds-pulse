# Working Route — andromeda-pulse-0.3.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve then skips frozen lines); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation: AI-debug spine
[2026-06-28-deterministic-env-gated-l4-mode] Deterministic env-gated L4 mode — canned `L4Output` via `StubInferenceRunner`, env/flag-selected, so the incident path completes without GPU/3B (P-073 · intent F13a)
   ↓
[2026-06-28-tier1-incident-path-reliability] Tier1 incident-path reliability — coalesce identical hard-signals into one digest + elastic queue with heartbeat ticks; storm yields one incident (P-074 · intent F13b)
   ↓
[2026-06-28-investigate-actions-functional] Investigate actions functional — the four Investigate buttons run real LLM/MCP analysis with visible progress and a result; failures surface (P-072 · intent F12)

### Epoch 2 — Window & shell hygiene
[2026-06-29-window-geometry-movable-shell] Window geometry + movable shell — sane default size/position (centered or remembered) and a working custom-titlebar drag region (P-061 · intent F1 · layout-templates)
   ↓
[2026-06-29-predictable-close-self-verify] Predictable close + honest tray — close quits or minimizes-to-tray with a clear "still running" indication; the dashboard is closable (P-063 · intent F3) · /phase to fold in a new agent-headful-self-verify cap (~P-078) when planning
   ↓
[2026-06-29-window-size-constraints] Window size constraints — minimum size plus a sensible aspect-ratio constraint for the glance widget (P-062 · intent F2)
   ↓
[2026-06-30-browser-chrome-suppression] Browser-chrome suppression — default WebView2 context menu and canvas image-save suppressed app-wide in production (P-064/P-065 · intent F4/F5)
   ↓
[2026-06-30-widget-to-dashboard-navigation] Widget-to-dashboard navigation — an explicit in-app affordance expands the glance widget into the full dashboard (P-066 · intent F6)

### Epoch 3 — State honesty & legibility
[2026-07-01-live-only-service-truth] Live-only service truth — show only currently-live services; persisted/stale registry entries hidden or clearly marked historical (P-067 · intent F7)
   ↓
[2026-07-05-anomaly-surfacing] Anomaly surfacing — errors and anomalies sorted to the top of Traces, flagged with semantic error tokens, filterable (P-068 · intent F8 · design-system)
   ↓
[2026-07-05-legible-labeled-constellation] Legible labeled constellation — per-dot service names with health/severity encoded via design-system color + Halo (P-069 · intent F9)
   ↓
[2026-07-05-constellation-severity-live-wiring] Constellation severity live-wiring — reconcile the incident workspace key so the per-service severity join lights up: the resolver's `incident_workspace_key` (`main.rs` = `data_dir`) ≠ the incident producer's `digest.workspace` (detected project root, `\\?\`-canonicalized), so the workspace-filtered `list_active()` finds ZERO active incidents → constellation dots all read "healthy" AND the incidents panel stays empty even under a live storm. Fix both sides to one detected-workspace source. HIGH VALUE — unblocks P-069's live severity differentiation + the incidents surface (both runtime-inert today; the chunk-#91 join landed forward-inert pending exactly this). Operator-surfaced at the 2026-07-05-legible-labeled-constellation (P-069) leave-running verify (docs/session-learnings.md 2026-07-05).
   ↓
[2026-07-06-incidents-panel-dropdown-layout-bug] Incidents-panel dropdown layout bug — the incidents dropdown (under the widget's unread badge) renders broken: it STRETCHES the window and overflows with a white / mis-clipped background instead of sitting as a bounded popover. PRE-EXISTING in the incidents-panel UI (~chunk #91); never visible because the workspace-key bug made `list_active()` always return 0 so the dropdown never populated — P-079 lit up the surface and EXPOSED the layout bug (NOT a P-079 regression; P-079 is 100% backend, zero `pulse-app/ui/**`). Focused frontend chunk: bounded-popover positioning/overflow (no window stretch; correct design-token `--color-inset` background). Operator-surfaced at the 2026-07-05-constellation-severity-live-wiring (P-079) live-verify.
   ↓
[2026-07-07-traces-table-auto-refresh] Traces table auto-refresh — the Traces table's `viz.query.traces` runs once at mount (row_count 0 before data lands) and never re-polls, so it reads "No traces yet" / stale even as spans flow (the constellation polls every 1s; the table does not). Add a periodic re-poll / refresh. PRIORITY-BUMPED at the 2026-07-05-constellation-severity-live-wiring (P-079) wrap — operator hit it live (Traces stays empty while the constellation + incidents show live data; reads as broken for an observability app), same visible-brokenness class as the incidents-dropdown bug; originally filed at the P-069 leave-running verify.
   ↓
[2026-07-07-plain-language-connection-status] Plain-language connection status — a human-readable services-connected, spans-per-second, and buffer-state line using design-system typography (P-070 · intent F10)
   ↓
[2026-07-08-self-explaining-empty-states] Self-explaining empty states — Metrics and Logs empty surfaces explain themselves with an actionable hint and design-system iconography (P-071 · intent F11)
   ↓
[2026-07-09-traces-table-layout-polish] Traces table layout polish — the trace table gets its own internal scroll (max-height + overflow-y:auto on the table container) so the constellation hero + filter toolbar stay fixed (today the whole dashboard scrolls); + refresh the desktop-webview Traces wireframe to show the "Errors only" filter toolbar above the table. Operator-surfaced on the 2026-07-05-anomaly-surfacing (P-068) live boot — the internal-scroll is PRE-EXISTING layout NOT introduced by P-068 (it does not block P-068's anomaly-first intent); the wireframe-toolbar doc is the D-layout-surface within-surface-refinement routine-REJECTED at the P-068 wrap, homed here.
   ↓
[2026-07-10-incidents-floating-window-disclosure] Incidents floating-window disclosure — surface unread incidents in a SEPARATE borderless always-on-top window docked directly BELOW the compact widget (drops "under the widget"), replacing the in-widget upward popover (2026-07-06-incidents-panel-dropdown-layout-bug landed the bounded upward popover as the interim bug-fix; its bounded/scrollable/opaque FindingsDropdown content is reused in the new window). Needs: a window-label render branch (like the widget/dashboard split), position-below-the-widget from the widget's live position+size incl. off-screen/multi-monitor clamping, cross-window focus + the chunk #87 disclosure a11y contract (Esc returns focus to the badge across windows), dismiss lifecycle (blur/Esc/row-select/mark-all-read → hide), and the core:window capability grants (create/position/show/hide — negative-default per the 2026-06-29 core:window family). Operator-directed at the 2026-07-06-incidents-panel-dropdown-layout-bug wrap (chose "separate floating panel below the widget" over the upward-popover / grow-the-window options; /andromeda-phase to de-risk the positioning + cross-window-focus edge cases).

### Epoch 4 — Polish & ship: verification
[2026-08-14-fingerprint-feed-capture-repair] Fingerprint-feed capture and repair — the storm detector's feed observed live, then the dead region between ingest receipt and the fingerprint hook repaired (operator-directed 2026-08-14; evidence: Conductor fingerprint-feed-verdict.md)
   ↓
[2026-08-15-corpus-key-persistence] Corpus key persistence — the incident corpus stays readable across process boundaries, and content written under ephemeral keys is recovered or retired (operator-directed 2026-08-15; measured at 2026-08-14-workspace-key-alignment: `keyring 3.6.3` resolves with `[log, zeroize]` only — no platform credential-store backend linked — so the AES-256-GCM cell key comes from keyring's non-persisting mock store and is per-process ephemeral; second app process emitted 13 `decryption_failed` warnings against rows its own predecessor wrote, and `cmdkey /list` lists zero entries. Every corpus payload written under an ephemeral key is unreadable by any later process, so historical encrypted incident/digest content is orphaned until a migration exists. Carries cargo-deny / dependency-graph consequences. Blocks preflight-green: the e2e read-back path now REACHES decryption and fails there, so P-075 and P-076 sit behind this)
   ↓
[2026-08-16-baseline-family-reachability] Baseline-family reachability — the silence and activity-floor families become reachable for a fresh service within a bounded warm-up, so Conductor's Epoch-3 family proofs are not gated on an hour of wall-clock (operator-directed 2026-08-14; evidence: Conductor two-launch-verdict.md §Re-run) · PREMISE CORRECTED 2026-08-14-fingerprint-feed-capture-repair: the storm path never touches the Ready gate — it runs buffer fingerprint hook → StormObserverAdapter → detector → cue broadcast, never reading BaselineState — so storm→cue→incident flows in seconds on a fresh dir (measured); the 3600s gate binds ONLY the baseline-derived families, which is what this entry now targets
   ↓
[2026-08-16-fault-identity-semantics-decided] Fault-identity semantics decided — fingerprint normalization and incident dedupe agree on when two faults are one (operator-directed 2026-08-16 at the 0-pending adaptation wrap; evidence: `conductor/conductor-0.2.0/chunks/2026-08-15-*/` + `2026-08-16-*/`, all measured against this repo's HEAD d090314; that repo is READ-ONLY from here — cite, never copy) · TWO LAYERS, ONE QUESTION — what makes two faults the same fault: (a) FINGERPRINT identity — `is_absolute_path_start` fires on ANY `/`, so in effect only the LEADING path segment stays identity-significant, while the module doc and an inline comment both claim absolute-paths-only; distinct faults can therefore over-coalesce into a single storm. Decide the intended semantic FIRST, then align impl or docs to it — never infer the intent from current behaviour. (b) INCIDENT identity — dedupe keys on an OPEN incident, NOT on the fingerprint: while one is open a storm carrying a DIFFERENT fingerprint is absorbed into it (`created:false` / `deduped:true` — a deliberately unique-typed canary deduped anyway), with the ~5min auto-resolve window the only same-data-dir cure. The design question: should a distinct-fingerprint storm open a second concurrent incident? · WHY IT PRECEDES P-075/P-076: both assert on incident formation and read-back, so their acceptances encode whatever fault-identity means; settling it first writes them once against decided semantics instead of encoding today's measured-but-undecided behaviour into two suites
   ↓
[2026-08-14-workspace-key-alignment] Workspace-key alignment — the MCP sidecar reads incidents by the same workspace key derivation the app writes them under (operator-directed 2026-08-14; evidence: Conductor two-launch-verdict.md §Re-run)
   ↓
[2026-08-15-tier-1-incident-path-investigation] Tier-1 incident-path investigation — a detected storm produces an incident again (operator-directed 2026-08-15; evidence: this version's `2026-08-15-corpus-key-persistence/evidence/arm-zero-classification.md`)
   ↓
[2026-08-17-incident-fingerprint-producer-repaired] Incident-fingerprint producer repaired — the field carries the hash its contract and consumers already expect (operator-directed 2026-08-17 at the fault-identity-semantics-decided wrap; evidence: that chunk's report + `decision.md`) · MEASURED, not inferred: `crates/triage/src/contract.rs` documents `Incident.fingerprint` as an "anonymized fingerprint hash for cross-incident grouping", and the digest assembler's own fixtures populate it with 32-char lowercase hex fed by `hex_lower` of Q3 blake3 bytes — but the producer writes the model-authored `L4Output.fingerprint` string into it. So the corpus-retrieval `fingerprint_match` arm is correct-per-contract and STARVED, never dead; removing it was attempted at the fault-identity chunk and FULLY REVERTED when the assembler's tests rejected it, leaving an in-code guard against re-simplification
   ↓
[2026-08-17-conductor-e2e-verification-closure] Conductor e2e verification closure — a deterministic incident drives MCP read-back proving end-to-end fidelity and delegated timing caps P-025/P-027/P-037/P-045 (P-075 · intent F14)
   ↓
[2026-08-21-delegated-timing-observables] Delegated timing observables — halo hue, constellation discovery and findings-counter refresh observable at the wire with real timing values (P-025 · P-027 · P-045)
   ↓
[2026-08-22-pii-scrubber-recall] PII scrubber recall — a bare provider key in telemetry is redacted before it reaches stored fields, not only its keyed form (P-047)
   ↓
[2026-08-22-log-records-identity] log_records identity — two log records arriving in the same tick at the same severity both survive ingestion
   ↓
[2026-08-23-ingestion-scrub-coverage] Ingestion scrub coverage — the five client-controlled columns that never reach `scrub_attribute`

   ↓

[2026-08-23-metrics-points-identity] metrics_points identity — two data points of one metric differing only by label set both survive ingestion, with their labels
   ↓
[2026-08-23-metrics-points-labels] metrics_points labels — a metric point's label set survives ingestion and reads back, so dimensioned metrics stop arriving indistinguishable
   ↓
[2026-08-23-webview-self-verify] Webview self-verify on the Windows host — an agent presses a real control in the live Tauri window and the app's own ui.layout.transition record is the verdict (tauri-driver + WebdriverIO)

   ↓

[2026-08-23-integration-ux-e2e-test] Integration UX e2e test — real assembled path under deterministic-L4 (launch, telemetry, real-time push, Traces, storm, incident, Investigate) guards regressions (P-076 · intent F15)
   ↓
[2026-08-23-a11y-verification] A11y verification — v0.3.0 interactive surfaces (window, widget-to-dashboard nav, anomaly controls, constellation, status, empty states): focus/keyboard/contrast/SR + SC 2.3.3 (per a11y-plan §3/§6/§7)
   ↓
[2026-08-23-headful-leg-extension] Headful leg extension — the window-mechanics affordances a live drive can prove but the assembled path never touches
   ↓
[2026-08-24-headful-mechanics-probe-race-disposition] Headful mechanics probe + navigation-race disposition — the deferred window-mechanic stages land or are declined on a measured driver probe, and the race's production exposure is measured or guarded
   ↓
[2026-08-25-demo-injector-formalized-api-surface-retire] Demo injector formalized + api-surface retire — `inject_demo.rs` as a supported dev/test tool; retire `context/api-surface.md` once `tree.db` is built (P-077 · intent §5)
   ↓
[2026-08-26-ingest-consumer-stall-under-sustained-load] Ingest consumer stall under sustained load — the buffer keeps draining, and a wedged consumer is visible instead of silent
   ↓
[2026-08-26-cadence-runaway-blocking-pool] Cadence runaway starves the blocking pool — the cue→cadence→digest loop stays bounded under sustained load, and the buffer keeps draining
   ↓
[2026-08-26-l4-runtime-security-residuals] L4 runtime security residuals — the product's model/binary path inputs and its inference prompt argument carry a stated guard, not a convention
   ↓
[2026-08-26-interpretation-brief-completeness] Interpretation brief completeness — an incident's report carries the model's hypotheses and real evidence ids, not "interpretation pending" and invented ones
   ↓
[2026-08-27-idle-observer-generation-damper] Idle-observer generation damper — a quiet system stops burning standing GPU: unchanged conditions re-analyze rarely, and hanging-incident churn stops defeating auto-resolve
   ↓
[2026-08-27-report-window-copy-affordance] Report-window Copy affordance — the report's Copy control copies instead of failing, and a gate presses it
   ↓
[2026-08-27-incident-persist-vs-resolve-write-race] Incident persist-vs-resolve write race — a corpus resolution stays resolved: the stale-snapshot persist loop stops silently reverting direct writers
   ↓
[2026-08-28-ingest-consumer-initiating-freeze] Ingest consumer initiating freeze — what stops the consumer under light load is identified, so the wedge has a named cause and not only a survivable aftermath
   ↓
[2026-08-28-ingest-consumer-block-under-gap-resume] Ingest consumer block under gap→resume — the consumer keeps draining after a producer restart, so accepted spans stop being silently dropped
   ↓
[2026-08-28-duplicate-span-replay-fails-loudly] Duplicate-span replay fails loudly — a constraint-violating flush never wedges ingest, and the injector stops colliding with itself across restarts
   ↓
[2026-08-29-app-registry-reconciliation] App-registry reconciliation with externally-resolved rows — an incident resolved outside the app stops showing active until restart
   ↓
[2026-08-29-halo-state-pulse-signature-deferred] Halo State Pulse signature deferred — the specs stop claiming a surface that does not render, and the design half moves to the next version
   ↓
[2026-08-29-advisory-backlog] Advisory backlog — the RustSec findings that have a stated safe upgrade are upgraded rather than accepted (operator-directed 2026-08-15 at the corpus-key-persistence wrap)   ↓
[2026-08-30-npm-advisory-coverage] npm advisory coverage — the webview devDependency channel is scanned for advisories, not only version-bumped
   ↓
[2026-08-30-diagnostics-un-muting-harness-truth-sweep] Diagnostics un-muting + harness-truth sweep — the deferred targeted-cleanup cluster gets an owner, so a diagnostic's silence stops being unreadable (operator-directed 2026-08-15 at the tier-1-incident-path-investigation wrap)

   ↓

[2026-08-30-staged-bindings-assertion] Staged-bindings assertion — the generated TauRPC bindings a commit actually carries match the procedure pin, checked mechanically rather than remembered
   ↓
[2026-08-30-acl-rejection-logging] ACL-rejection logging — a capability-rejected webview IPC leaves a record instead of vanishing   ↓
[2026-08-30-dead-lib-src-test-migration] Dead lib-src test migration — the ~101 never-run pulse-app src tests execute in the gate, and the ratchet's legacy baseline empties
   ↓
[2026-08-30-agent-harness-teardown-truth] Agent-harness teardown truth — `cleanup` terminates the APP and `boot`'s ceiling fits the env-triggered relink, so harness exit codes stop lying
   ↓
[2026-09-29-p-025-hue-shift-observable-made-gradable] P-025 hue-shift observable made gradable — the emitted hue-shift timing measures the interval the ≤2 s budget bounds, per Conductor's P-025 measurement contract
   ↓
[2026-09-29-ci-wall-time-and-round-trips] CI wall time and round-trips — a CI round stops being the bottleneck: parallel jobs, a kept cache, no duplicate rebuilds, a local Linux pre-push check
   ↓
[2026-09-29-scrubber-path-false-positive] Scrubber path false positive — a digit run in a path or workspace key is not redacted as a card number, so the model's PROJECT line stops reporting a credit-card leak that does not exist
   ↓
[2026-09-30-dual-license] Dual license — the project ships under MIT OR Apache-2.0 with both license texts, and every manifest says so
   ↓
[2026-09-30-p-027-discovery-bound] P-027 discovery bound — a new service's first constellation dot appears within the 5 s bound, not at the registry's first lifecycle tick
   ↓
[2026-09-30-perf-budget-gate-reads-real-samples] Perf-budget gate reads real samples — the CI perf-budget gate can fail: it reads a log carrying frame, memory and snapshot samples
   ↓
[2026-09-30-perf-instruments-measure-their-budgets] Perf instruments measure what their budgets name — the snapshot timer spans what its 500 ms budget bounds and the gate reads it; a frame-less run names its cause
   ↓
[2026-09-30-span-level-redaction] Span-level redaction — a secret inside a larger value redacts only its matched span, so one true positive no longer blanks the whole digest the model reads
   ↓
Real-model incident surfacing — the real model turns a storm digest into an incident reliably, not in some storms only · CONTEXT: founder 2026-09-30 (asked, answered) «Чинить в Pulse 0.3.0» (same relay §2); measured by Conductor's series on clean input (Conductor `:56`, Pulse `fcc31b2`; that repo is read-only from here — re-verify at /phase): Llama 3.2 3B surfaced the canary in 4 of 9 storms and dismissed the scenario's storm digest both times it emitted (parse `ok`, no incident); measure WHY first — the prompt's surface/dismiss guidance, what the digest carries, the thresholds, the sampling (`build_llama_cli_args` passes no `--seed` / `--temp`, so llama.cpp's own sampling defaults apply — measured at HEAD `pulse-app/src/llamacli_inference.rs:405`) — then fix; ordered after span-level redaction so it is measured on an honest input; Conductor's third v3-09 series waits on this entry · PREREQ: the CI source-lint pair from 2026-09-30-span-level-redaction (operator-directed at its wrap, 2026-10-01, as a PREREQ here, not a new entry) — (1) the ci.yml "no Cyrillic in source" step (an inline python step over `.rs`/`.ts`/`.tsx` under crates · pulse-app/src · pulse-app/tests · pulse-app/ui/src · xtask/src) crashes with `UnicodeEncodeError` under the Windows runner's cp1252 console whenever it has hits to print, so on windows-latest it fails as a bare `exit code 1` with no readable finding (measured on `ci#36842111417`, sha `9d14166`); (2) `cargo xtask pre-push:linux` does not run that step, so a Cyrillic source literal passed the local pre-push green and failed CI on all three `lint / test` jobs (same run)
   ↓
Conductor return — the external P-075 assert round runs and the version's last unclaimed capability verifies
