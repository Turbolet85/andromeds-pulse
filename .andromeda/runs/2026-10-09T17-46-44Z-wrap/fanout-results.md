# Fan-out results — 2026-10-09-supply-chain-job-same-on-push-and-pull-request

Seven Explore doc-agents, one parallel batch, each with the report, its master, its detectors (19 detector ids over
7 docs: arch 2 · security-plan 4 · design-system 2 · layout-templates 2 · test-plan 3 · obs-plan 4 · a11y-plan 2) and,
for the four migrated masters, its keyed-contracts render. The entity probe after stripping: 0 entities in every
return. Proposal fields below are the returns' own; `rationale` is given by its substance (the returns stand in the
session's agent transcripts, which the tree does not carry).

Totals: 17 proposals · applied 14 · rejected 1 (Validate check 1) · escalated 2 (to the operator, at the route card) ·
rejected for a source the report does not carry: 0.

## Verdict lines

- **architecture** — 3 proposals. Stripped: two comment lines naming D-arch-decisions clean and the two sweeps it ran
  over its master and key files (the boot-job claim: one site, `architecture.md:180`; the audit action: 0 sites).
- **security-plan** — 5 proposals. Stripped: four comment lines — D-security-input and D-security-auth clean;
  D-security-deps's literal invariant holds and its proposals are stale §Dependency Security text, severity set to
  `warning` by the agent "for the orchestrator to confirm"; D-security-logging one stale behaviour claim.
- **design-system** — `proposals: []`. Stripped: a basis block — no new UI, no status claim of the doc touched by the
  five disproved claims or the rulings; one sweep hit, unrelated. Raw twin: `.raw-fanout-design-system.md`.
- **layout-templates** — `proposals: []`. Stripped: a basis block of the same shape; 0 sweep hits. Raw twin:
  `.raw-fanout-layout-templates.md`.
- **test-plan** — 5 proposals, two of them at severity `escalate` set by the agent. Nothing stripped but the YAML's
  own framing.
- **obs-plan** — 4 proposals. Stripped: a comment block naming the three clean detectors and what
  D-obs-defect-narrative checked and did not propose (the DuckDB tally; the boot-smoke watch, which §7's contract
  already covers; disproved items 2–5).
- **a11y-plan** — `proposals: []`. Stripped: four comment lines — both detectors clean, the `a11y` job claims
  standing, 0 sweep hits in the master and its ten key files. Raw twin: `.raw-fanout-a11y-plan.md`.

## architecture

1. D-arch-resources · warning · §Infrastructure Patterns → CI/CD approach · change: the `supply-chain` clause names
   `cargo audit` as a plain `run:` step (no action, no token, no `continue-on-error`, no job-level `permissions:`; the
   image's `cargo-audit` 0.22.2; exits 0/1/2; no `--deny warnings`; its pin), and the trigger block's pin · basis: the
   key file; `ci.yml:426-427`.
   → **apply** — check 1: the plan's P5-approved expected amendment names the change (the audit step named as a plain
   `cargo audit`); playbook "Accurate this-chunk addition". Applied re-derived, shorter than the proposal's line.
2. D-arch-resources · warning · same key · change: append the fourth dated cache reading (17:49:51Z; 10 entries, the
   same bytes; the four `main` orphans re-accessed 17:15Z; the two `pull/39` entries alone least recently used).
   → **apply** — check 5: the plan's expected amendment and the entry's first `CARRY:`.
3. D-arch-resources · warning · §Occupied Resources → `telemetry.frontend.record_webgpu_adapter` · change: "its only
   live witness … (the CI boot job stops the app before the webview issues any IPC)" → a witness on some runs only ·
   basis: `architecture.md:180`.
   → **apply** — check 6 (disproved claim 1); playbook "doc claim a pre-existing reality falsifies, impl already
   correct": no impl half, every site of the claim amended in this pass.

## security-plan

1. D-security-deps · warning · §Dependency Security → CI integration · change: the plain `run: cargo audit` step, no
   action, no token, the image's own tool, three exits, its pin · basis: `security-plan.md:216`; `ci.yml:426-427`.
   → **apply** — check 1: the expected amendment names the change. D-security-deps carries `escalate`; the playbook's
   escalate-detector-outside-its-class rule governs (the report's Dependencies bullet: none added or bumped).
2. D-security-deps · warning · §Bootstrap phases → `dep-security-ci-gate` · change: `actions-rust-lang/audit` leaves
   the SHA-pinned action list · basis: `:259` · dependent-of D-security-deps.
   → **apply** with its primary.
3. D-security-deps · warning · §Dependency Security → Audit tool · change: the line says the CI gate runs the image's
   0.22.2 and no longer presents 0.22.1 as the gate's version · dependent-of D-security-deps.
   → **reject** — check 1, playbook "Not this chunk's drift": the report's Dev-tool versions bullet reads "none
   changed", and which binary the gate runs did not change either (research.md: the replaced action called the image's
   own `cargo audit` with no install). The 0.22.2 reading is written at the primary site with its host and evidence;
   the section's 0.22.1 stays (the sidecar entry's Kept).
4. D-security-deps · warning · §Dependency Security → Critical CVE response SLA · change: one dated departure from
   "reaches `main` with the version" · basis: `:226`.
   → **apply** — check 1: the expected amendment names the change; the founder's ruling relayed verbatim (inputs#I3).
   Not a boundary widening: no token, trigger or permission moves.
5. D-security-logging · warning · §Security Anti-Patterns → Logging, `ui.webgpu.adapter` · change: the boot job
   witnesses the record on some runs only · basis: `:447`.
   → **apply** — check 6 (disproved claim 1). "Both fields rendered" is the wrap's own read of the four logs.

## obs-plan

1. D-obs-defect-narrative · warning · §10 Performance budgets → WebGPU canvas frame row · change: the boot-job
   mechanism scoped to `ci#36765040464`; seven logs, four with the record, three without · basis: `obs-plan.md:545`.
   → **apply** — check 6. The `ci-gates` frame line on the two green runs with the record was read by the wrap after
   the fan-out and is written in.
2. same detector · §1 Telemetry triggers → `multi-platform-exporter-compat` row · basis: `:132` · dependent.
   → **apply**.
3. same detector · §8 whitelist → the `ui.webgpu.adapter` leaf ("its only live witness"; found by the agent's sweep of
   its own master, a site the report's grep did not match) · dependent.
   → **apply** — check 4: the site was read at `obs-plan.md:445` before the edit.
4. same detector · §10 CI gates → the perf-budget bullet (`ci#36765040464`'s `no adapter record in this log`) ·
   dependent; the agent's lowest-confidence.
   → **apply** — the wrap read the line on five runs and it varies with the log, so the bullet's single text was a
   rule in effect.

## test-plan

1. D-tests-framework · warning · §9 Pipeline structure, Supply chain row · change: the row gains the audit step, its
   exits and pins · basis: `test-plan.md:494`.
   → **apply** — check 1: the expected amendment names the change. Written as a clause of its own: the report does not
   give the step's place among the job's steps.
2. D-tests-coverage · warning · §1 Pending coverage triggers → `ipc-rejection-wire-coverage` · change: "no CI job can
   witness it" → the boot job is a run-dependent witness; the owed assertion stands · basis: `:142`.
   → **apply** — check 6.
3. D-tests-coverage · warning · §1 Coverage triggers → WebGPU canvas throughput row · basis: `:100` · dependent.
   → **apply** — with the frame line the wrap read (the proposal asked that it not be stated unread).
4. D-tests-obs-harness · **escalate** (the agent's) · §3 → 5-command implementation, `boot` → Readiness signal ·
   change: operator decision — (a) correct the label to what the verb does (ready on the `harness:status` verdict
   alone; it does not wait for the OTLP receivers to bind; drop "confirm TCP handshake succeeds on bound ports (log
   parse)"), or (b) keep the label as the contract and record that the shipped verb does not meet it, with a route
   owner · basis: the key file `registries/contracts/test-plan/5-command-implementation.md`.
   → **escalate** — check 1: the playbook's "measured OPEN defect, impl half owned by a NAMED route entry" rule would
   make it routine, but its own clause escalates when no owner exists in the route, and none does until the route
   card. Verified by a read of `scripts/agent-run.sh:101-112` (the readiness loop prints ready on the status verdict;
   no handshake) and the key file's `:5`. Carried to the route-resolve card with the watch's owner, one attended stop.
5. D-tests-obs-harness · escalate · §1 Test harness requirements ("confirms TCP handshake on `:4317`/`:4318`") ·
   dependent-of D-tests-obs-harness.
   → **escalate** with its primary.

   **Resolved with the operator at the route-resolve card** (inputs#I7; the operator — the pc overseer,
   founder-delegated, 2026-10-09): "Keep the readiness clause as the intended contract, record the measured gap beside
   it, the new entry owns it - ready before the receivers bind is the engine own matter too." Both applied in that
   form (option (b)); the owner is the route entry "Boot smoke's early exit found and closed" (P-129), minted at the
   same card. Final totals: 17 proposals · applied 16 · rejected 1 · escalations resolved 2 · open 0.

## Orchestrator's own checks

- Check 2: no two proposals edit one section in opposing directions.
- Check 3: the report diverges from the plan's acceptance in one clause (the build branch's run red in boot smoke) —
  justified by the operator-ratified reading (inputs#I5 item 2), owner P7.3's `refine`; the corrected log read stands
  in for entries 40–41 — justified in the report's Deviations. Scope record: none.
- Check 5: the plan's five expected amendments are each matched (architecture 1 and 2 · security-plan 1, 2 and 4 ·
  test-plan 1). No citation row was dispositioned `claim false`.
- Check 6: disproved claim 1 → architecture 3, security-plan 5, obs-plan 1–4, test-plan 2–3 · claim 2 → security-plan
  1–2 · claim 3 → security-plan 4 · claim 4 → P7.3 (a ledger) · claim 5 → the report's Deviations and curation (a
  plan, not a master).
