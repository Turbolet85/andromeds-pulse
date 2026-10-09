# Fan-out results — 2026-08-28-ingest-consumer-initiating-freeze

7 doc-agents, one per spec source. All returns were clean YAML needing no stripping and carrying no
proposals, so no `.raw-fanout-{doc}.md` twins were warranted — this file is their sanctioned audit record.

| doc | detectors | verdict | basis (summarized from the return) |
|---|---|---|---|
| arch | D-arch-resources · D-arch-decisions | `proposals: []` | No new IPC method / endpoint / event / port / env var / workspace crate. The leg CONSUMES no env var and SETS three already-registered ones (`ANDROMEDA_PULSE_DATA_DIR`, `_BASELINE_BOOTSTRAP_SECONDS`, `_L4_DETERMINISTIC`), none of which carries a sole-setter claim this leg would retire. `gap_resume` is an xtask MODULE, not a workspace crate, and arch registers no xtask-subcommand registry — test-plan §3 owns it. No dependency delta; no locked decision contradicted. |
| security-plan | D-security-input · D-security-auth · D-security-deps · D-security-logging | `proposals: []` | No new product external-input surface (harness-only clap args; validation `n/a` per the report's Coverage row). No identity/session/token/key path touched. "Dependencies: none added, none bumped" + `deny` exit 0. No scrubbed column, redaction counter, or store/log boundary touched — the five-cell coverage set, the `exception_type` exclusion, and the unencrypted-ring-buffer residual all remain accurate, restatements at §Threat Model Summary and §Data Protection checked too. |
| design-system | D-design-tokens | `proposals: []` | Both new surfaces carry `tokens n/a (no UI)`; no Coverage row carries `hardcoded✗`. Nothing touches color / spacing / typography. |
| layout-templates | D-layout-surface | `proposals: []` | No UI/webview file in Files; `bindings/index.ts` byte-identical to HEAD; both Coverage rows `a11y n/a (no UI)`. The §Wireframe surface inventory remains complete. |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness | *(see note below)* | — |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-pii | `proposals: []` | — |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | `proposals: []` | No interactive UI element added. "Schema / config: none … no violation schema" — the obs §10 item named in Expected amendments touches SLO/detector prose, not the obs §6 structured-log schema the violation JSON binds to. |

## Orchestrator-raised (Validate check 5 — the plan's Expected-amendments list is the coverage floor)

The detector set returned zero proposals, but the chunk's `plan.md` carries an `Expected amendments (wrap)`
list naming **two**. Both are raised by the orchestrator per amendment-flow §Validate step 5; neither is a
detector failure in the ordinary sense — they fall in the known blind classes (a doc gaining a *treatment*
for an already-documented mechanism; a section whose framing, not its facts, went stale).

See `validation.md` in this run dir for the disposition of each, and for the Validate check 6 pass over the
report's four `Spec claims disproved by measurement` entries.
