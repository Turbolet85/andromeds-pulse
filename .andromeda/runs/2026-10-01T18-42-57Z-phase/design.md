# design extract

## Relevance
partial — the chunk renders no new UI and adds no tokens; design binds only through the P-025 clause of the P-075 acceptance, whose graded quantity is the design-owned severity signature (the exit-cause PREREQ has no design surface).

## Constraints
- The 0.3.0 severity signature is the constellation DOT hue (cumulative-incident-severity LCH shift Earth Blue ↔ Alert Burgundy via `severityToHueFraction`) on the full dashboard constellation map and the compact widget constellation canvas. Any P-025 "hue update" wording in the concretized P-075 acceptance must name that dot-hue surface (per design-system §Self-Validation Protocol → 3. Signature Test; §Brand Identity Signature element).
- The Halo State Pulse glow layer (breathing / blur / the tray unified halo) is DEFERRED to the next version and renders on no surface in 0.3.0. No acceptance, matrix `notes` line or evidence citation may claim that the glow renders or that it is what P-025 grades (per design-system §Brand Identity Signature element, which owns the status; §Self-Validation Protocol → 3. Signature Test "no output CLAIMS the glow layer renders").
- Hue (severity) and connection state are orthogonal axes: connection is a grayout/desaturation axis independent of the severity hue. A P-025 hue-shift assertion must not use a connection-state change as its trigger or its readout (per design-system §Motion → High-impact moments 1, P-004 health-vs-severity orthogonality; §Brand Identity Signature element).
- Under `prefers-reduced-motion: reduce` the severity hue still updates. Only rhythm and transitions degrade, so a P-025 hue-update bound is not waived for a reduced-motion run (per design-system §Motion → Accessibility).
- Canvas motion (the constellation, the throughput counter) is a separate dimension from the chrome expression budget (150–200 ms). The P-025 ≤2 s and P-045 ≤1 s budgets are data-latency budgets and are not to be read against the chrome duration scale (per design-system §Motion, opening paragraph and the Duration scale table).

## Patterns to follow
- The Signature Test is the design-side verification shape: point to the dot hue on both rendering surfaces (dashboard map and compact widget). It is the natural form for the P-025 clause's fidelity half, alongside Conductor's timing half (per design-system §Self-Validation Protocol → 3. Signature Test).
- Severity semantics for an incident driven by Conductor's deterministic scenario: the hue moves toward Alert Burgundy (#C7556A, the anomaly/error accent) as cumulative incident severity rises, away from Earth Blue (#4A90E2) (per design-system §Color Palette → Core Colors; §Brand Identity Domain anchors, Spectroscopy).
- Whether the shipped dot-hue path already emits the re-shaped interval in the form Conductor's `pulse-p025-measurement-contract.md` grades is research's question (P3). Design only fixes WHICH visual quantity is legitimate.

## Anti-patterns to avoid
- Claiming the deferred Halo State Pulse glow, or its breathing cadence, as a live 0.3.0 surface in any durable text the chunk writes (matrix `ref` / `notes`, report, evidence citation) (per design-system §Brand Identity Signature element; §Self-Validation Protocol → 3. Signature Test).
- Collapsing the severity-hue axis with the connection grayout axis when describing what P-025 measures (per design-system §Motion → High-impact moments 1).

## Contract bindings
- design ↔ obs/tests: the P-025 hue-update quantity (the emitted interval re-shaped at `2026-09-29-p-025-hue-shift-observable-made-gradable`) must correspond to the design-owned dot-hue severity signature, not to the deferred glow (design-system §Self-Validation Protocol → 3. Signature Test ↔ obs-plan §10 perf budgets ↔ Conductor's P-025 measurement contract, which is cited and never copied).
- design ↔ a11y: the reduced-motion degrade rule (the hue keeps updating; transitions go to 0 ms) binds to a11y-plan §6 / SC 2.3.3. It is relevant only if the P-075 round runs a reduced-motion configuration (per design-system §Motion → Accessibility).
- design ↔ a11y: the hue is a color-only severity encoding on the dot. Whether the dot pairs it with a non-color cue is a11y's SC 1.4.1 question and is not asserted by this chunk (per design-system §Color Palette → Semantic Colors).

## Acceptance criteria contributions
- (design) The concretized P-075 acceptance names the P-025 graded quantity as the constellation dot-hue severity shift (dashboard map and/or compact widget), never the Halo State Pulse glow (per design-system §Self-Validation Protocol → 3. Signature Test).
- (design) No durable text the chunk writes (matrix `ref` / `notes`, chunk report, evidence citation) claims the Halo glow layer renders in 0.3.0. Check: grep the chunk's written artifacts for glow, breathing or halo-render claims, which should match 0 (per design-system §Brand Identity Signature element).
- (design) The P-025 assertion's trigger is an incident-severity change, not a connection-state change, so the hue axis is measured orthogonally to the grayout axis (per design-system §Motion → High-impact moments 1).
