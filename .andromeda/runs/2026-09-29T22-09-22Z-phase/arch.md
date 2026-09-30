# arch extract

## Relevance
partial — the catalog's content (arm shape, Luhn, P-047 wording) is the security domain's; arch contributes where the code lives, the DAG, the public-surface discipline, and the identity knock-on of changing a scrub output.

## Constraints
- The fix lives in the reserved `security` crate (per architecture §Occupied Resources → Cargo workspace crate names; one crate per module per §Inherited Defaults → Module boundaries). Fixing the primitive instead of each consumer fits the crate-per-module template (§Project Intent → Template patterns). Every consumer then inherits the change and none needs an edit.
- The dependency direction stays a DAG rooted at `pulse-app` (per architecture §Cross-cutting Patterns → Module dependency direction). A context-aware card check must not give `security` a new sibling edge, for example on `workspace-detector` to recognise a workspace key or path. Any context detection is self-contained string logic inside the arm.
- Validation takes no validation library by default (per architecture §Inherited Defaults → Validation; §Stack and Technologies → Validation row). A Luhn/length check is small enough to hand-roll. A third-party checksum crate would be a Stack delta that the wrap records in the Stack table.
- The public contract surface stays unchanged: `scrub_attribute` and `ScrubbedValue` remain the only `pub` items, and new helpers (Luhn, separator grammar, boundary test) are `pub(crate)` (per architecture §Conventions → Module visibility discipline). The category set carried in `ScrubbedValue::Redacted` crosses into resolver and MCP payloads, so it must stay serde-stable (per architecture §Cross-cutting Patterns → Cross-bridge data shape).
- A changed scrub output is a changed stored string, and some stored strings feed identity. Per architecture §Established Decisions [Fault Identity], identity is DECIDED, not inferable. Changing an output from `[REDACTED:credit_card]` to verbatim can change the value of any identity key built from a scrubbed field (service name → DuckDB and the baseline registry; incident `scope_id`; the L1 fingerprint, if it is computed over scrubbed text). Research must answer which identity inputs are scrubbed values, and whether the before/after change is inert for them or must be recorded.
- The chunk lands no new resources (per architecture §Occupied Resources): no env var, TauRPC procedure, table, crate or capability. `ANDROMEDA_PULSE_DATA_DIR` (§Occupied Resources → data dir) is how a harness TempDir basename like `rm-20260923-093840` becomes the data root. It is read here as the input shape, not modified.
- Curated LLM context is a first-class output (per architecture §Design Philosophy → Token-efficient curation as a first-class output). A false-positive redaction that reaches the L4 PROJECT line corrupts that output, which is why the fix must restore precision without dropping recall.

## Patterns to follow
- Boundary-conditioned matching precedent: L1 fingerprint normalization acts only when the marker STARTS a token (position 0 or a preceding non-path byte), via `crates/buffer/src/fingerprint.rs::is_token_boundary` (per architecture §Established Decisions [Fault Identity] L1). It is the in-repo model for a card arm that looks at surrounding-character context such as path or key delimiters. Whether the regex crate's `\b` gives the same boundary semantics is research's question.
- Crate template (per architecture §Project Intent → Template patterns): one public contract module, `pub(crate)` internals, and the crate's `thiserror` enum. The arm change fits inside it without growing the surface.
- End-to-end pin through the real ingress (per architecture §Cross-cutting Patterns → Test-time telemetry injection). If the workspace-key or PROJECT-line form is pinned beyond unit level, synthetic telemetry goes in over OTLP `:4317`/`:4318` and is read back through the query routers, never through an in-process bypass.

## Anti-patterns to avoid
- A new sibling dependency edge from `security` to another library crate (for example to learn what a workspace key or path looks like), which bends the DAG (per architecture §Cross-cutting Patterns → Module dependency direction).
- Per-consumer or per-field exemptions (skipping the scrub for workspace keys or paths at one call site) in place of fixing the primitive. This splits one choke point into divergent behaviours across the crates that consume it (per architecture §Project Intent → Template patterns; §Inherited Defaults → Module boundaries).
- An in-process test-mode back door to inject the false-positive value for an end-to-end pin (per architecture §Cross-cutting Patterns → Test-time telemetry injection).

## Contract bindings
- arch ↔ security: the `security` crate placement and surface are arch's. The P-047 arm shape and its documented "13-19 digits, no Luhn" description are security-plan's, and any wording amendment is owed there at wrap, not in architecture.md.
- arch ↔ tests: an end-to-end pin of the workspace-key form follows §Cross-cutting Patterns → Test-time telemetry injection (OTLP ingress, read back through the query routers).
- arch §Established Decisions [Fault Identity] ↔ triage/corpus: any identity key derived from a scrubbed field sees a value change for affected inputs. Whether that is inert or needs a recorded note is research's question.

## Acceptance criteria contributions
- (arch) Production code changes stay inside `crates/security` (plus its tests), and no library crate's `Cargo.toml` gains a new sibling-crate dependency (per architecture §Cross-cutting Patterns → Module dependency direction).
- (arch) The public surface of `security` is unchanged: same `pub` items, same `ScrubbedValue` variants, same category-label set (per architecture §Conventions → Module visibility discipline; §Cross-cutting Patterns → Cross-bridge data shape).
- (arch) No new Occupied Resources entry (env var, procedure, table, crate or capability) is introduced (per architecture §Occupied Resources).
- (arch) Any new third-party dependency (for example a Luhn crate) is either absent or recorded as a Stack delta at wrap (per architecture §Stack and Technologies; §Inherited Defaults → Validation).
