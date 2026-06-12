# Pulse Capability Specification

**Status:** Draft v2, foundational baseline for v0.2.0 implementation kickoff
**Last revised:** 2026-05-14
**Reference documents:** `pulse-vision-and-backlog.md` (intent), `architecture.md` (implementation), `pulse-distillation-architecture.md` (data pipeline design v3), `pulse-v0_2_0-route.md` (delivery plan), `.andromeda/scope-validation/widget-state-validation-report-2026-05-14.md` (validation context)

---

## Purpose

This document is Pulse's product contract. It enumerates the capabilities Pulse claims to provide to its users — formal, testable, surface-mapped. Each capability has a stable identifier, a formal claim, an observable user-facing signal, a Conductor verification path, and an explicit scope boundary.

This document serves four downstream purposes:

1. **Product contract.** What users can expect from Pulse. Marketing copy, documentation, and README claims derive from here, not vice-versa.

2. **Regression target.** Future capability deletion or weakening is an explicit decision, never an accident. Conductor verifies each P-XXX continues to hold across releases.

3. **Conductor scope boundary.** Conductor (Pulse's test harness companion) implements exactly enough fault injection and scenario surface to verify each capability. No more (avoids feature creep), no less (avoids untested claims).

4. **Mini-route reorganization basis.** Implementation chunks are grouped by which capabilities they enable, rather than by architectural decomposition alone. A chunk earns its place by enabling specific P-XXX claims.

---

## Conventions

**Capability ID format:** P-NNN, three-digit, never reused even if a capability is deprecated.

**Claim verb:** "Pulse SHALL ..." for required behavior verified by Conductor; "Pulse SHOULD ..." for desired but not strictly verified behavior; "Pulse MAY ..." for optional behavior left to implementation taste.

**Observable signal:** The user-facing surface where this capability manifests. Empty when capability is internal infrastructure that downstream capabilities depend on.

**Conductor verification:** Concise description of how Pulse's test harness triggers and validates this capability. Becomes the implementation contract for Conductor's relevant scenario.

**Boundary:** Explicit out-of-scope statements that prevent claim drift.

---

## Model assumptions

Pulse's interpretation layer relies on a local LLM with the following invariants, not specific model identity:

- Runs locally on user hardware; no cloud dependency for core capabilities
- Quantized footprint ≤16GB VRAM for primary tier (covers approximately 7B-13B parameter range); ≤6GB VRAM for fallback tier (covers approximately 3B-4B parameter range)
- Context window ≥4K tokens; ≥8K preferred for richer corpus retrieval
- Supports JSON-structured output for deterministic interpretation parsing
- Instruction-following capable; specialized fine-tunes supported but not required
- CPU inference supported at degraded latency for users without GPU

Specific model selection is an implementation detail expected to evolve. Capabilities specify behavior invariants, not model identity.

---

## Categories

1. Connection & Health Awareness
2. Hard Signal Detection
3. Statistical Anomaly Detection
4. Pattern Recognition
5. Severity Calibration
6. Three-Surface Communication
7. Diagnostic Quality
8. Output Channels
9. Memory & Learning
10. Privacy & Trust
11. Pipeline Operations (NEW in v2)

---

# 1. Connection & Health Awareness

Pulse must distinguish between "the observed application is broken" and "Pulse is not seeing the application." These are different failures with different remediation paths, and conflating them produces user confusion and erodes trust.

### P-001 — Receiver Lifecycle State

Pulse SHALL maintain a connection state machine reflecting receiver lifecycle through five states: **Listening** (receivers up, zero spans ever), **Receiving** (spans within last 10s), **Idle** (10–60s quiet but otherwise healthy), **Stalled** (>60s quiet, suspect), **ReceiverFailed** (bind error or task panic). Initial state Listening; transition to Receiving within 1s of first valid span; ReceiverFailed overrides any prior state on receiver task failure.

**Observable:** Connection dot adjacent to widget title bar; color encodes state (dim / glow / warning / burgundy); tooltip shows last-span-ago.

**Conductor verification:** Start without emit (expect Listening), start workload (Receiving), stop (Idle then Stalled by timing), pre-bind port conflict (ReceiverFailed).

**Boundary:** Connection state describes receiver liveness, not app health. Receiving + 100% errors = healthy connection with broken app (covered separately by P-010).

### P-002 — Last-Span-Ago Tracking

Pulse SHALL maintain a monotonic tracker of time elapsed since the most recent valid span was ingested, with subsecond precision, accessible internally for state machine logic and externally for tooltips.

**Observable:** Connection dot tooltip displays "Last span Xs ago" when in Receiving, Idle, or Stalled states. Hidden in Listening or ReceiverFailed.

**Conductor verification:** Emit spans at known intervals; query the tracker; verify reported elapsed time matches reality within ±1 second.

**Boundary:** Tracker reflects only successfully ingested spans, not rejected or malformed payloads.

### P-003 — Receiver Failure Surface

Pulse SHALL detect and surface receiver task failures distinct from quiet-stream conditions. Failures include port bind errors at startup, runtime panics in receiver tasks, and explicit shutdown signals from the underlying transport. Failures surface in the UI within 2 seconds of occurrence.

**Observable:** Connection dot transitions to burgundy ReceiverFailed state. Tooltip displays the failure category in plain language ("Port 4317 in use", "Receiver task crashed", etc.).

**Conductor verification:** Start a sacrificial process holding port 4317, then start Pulse; observe ReceiverFailed within 2 seconds with tooltip identifying port conflict.

**Boundary:** Pulse does not attempt automatic recovery from receiver failures in v0.2.0. User must restart Pulse to clear the failure.

### P-004 — Orthogonal Health Domains

Pulse SHALL treat connection state and application health as independent, non-interfering signal domains. Connection state changes do not affect anomaly detection; conversely, anomalies in application telemetry do not affect connection state classification.

**Observable:** Connection dot and halo respond to orthogonal inputs. Halo can shift to alert burgundy (application errors) while connection dot remains in subtle glow (receiving normally). Conversely, ReceiverFailed dot does not silence halo if buffered telemetry contained pending incidents.

**Conductor verification:** Emit telemetry containing high error rate followed by clean shutdown of emitter. Verify halo correctly indicates error severity while connection dot transitions through Receiving → Idle without false alert state.

**Boundary:** No interaction between domains is permitted in v0.2.0.

---

# 2. Hard Signal Detection

Hard signals are deterministic, definitional failures present in OpenTelemetry semantic conventions. They require no baseline accumulation and provide reliable detection from the first emitted span. They are the floor of Pulse's detection capability.

### P-005 — Span Status Error Detection

Pulse SHALL identify and surface all spans with `Span.Status.Code = ERROR` regardless of context, baseline accumulation, or model availability. Detection latency from span ingestion to candidate emission SHALL be under 500 milliseconds at p99.

**Observable:** Error span aggregates contribute to per-service severity which propagates to halo hue and service constellation dot color. Root-span errors additionally drive incident creation per P-008.

**Conductor verification:** Emit spans with explicit ERROR status code; verify candidate emission to severity classifier within latency budget; verify halo and constellation reflect resulting severity.

**Boundary:** Pulse does not interpret error span content semantically at this layer. Semantic interpretation occurs in diagnostic Report generation (P-031).

### P-006 — Exception Event Capture

Pulse SHALL capture OpenTelemetry span events with `name = "exception"`, extracting `exception.type`, `exception.message` (after PII scrubbing per P-047), and `exception.stacktrace`. Captured exceptions feed both severity calculation and pattern fingerprinting (P-017).

**Observable:** Captured exceptions contribute to incident creation when patterns are detected (P-018) or when severity warrants surfacing.

**Conductor verification:** Emit spans containing exception events with synthetic stack traces; verify storage; verify exception content is accessible to pattern detector and Report generator.

**Boundary:** Pulse does not modify or normalize stack trace content beyond PII scrubbing.

### P-007 — High-Severity Log Capture

Pulse SHALL identify log records with `SeverityNumber ≥ 17` (ERROR and above per OpenTelemetry log spec) as candidate hard signals contributing to severity calculation.

**Observable:** High-severity logs contribute to per-service severity scores feeding halo hue and constellation dot color.

**Conductor verification:** Emit log records at varying severity numbers; verify ERROR and FATAL records contribute to severity; verify WARN and below do not contribute to hard-signal layer.

**Boundary:** Log content interpretation occurs in Report generation, not in this layer.

### P-008 — Root-Span Error Scope Distinction

Pulse SHALL distinguish error spans where the root span of the trace carries ERROR status from error spans deeper in the trace. Root-span errors indicate complete request failure; deeper-span errors may represent handled internal failures that did not propagate.

**Observable:** Root-vs-deep distinction is detected deterministically at the L1a aggregation layer (root identification via parent-span absence) and surfaced to the model as a fact in its interpretation context; the severity WEIGHTING of root vs deep errors is model-side per P-020 (no deterministic multiplier exists). Diagnostic Report explicitly distinguishes these cases when describing impact scope.

**Conductor verification:** Emit traces where (a) only an internal child span has ERROR status, root succeeds; (b) child span has ERROR and root has ERROR. Treat the severity comparison as a calibration-region check: (b) should TEND to produce equal-or-higher model-assessed severity than (a) for equivalent counts (model-interpretive tendency per P-020, not a hard deterministic assert).

**Boundary:** Trace assembly relies on standard W3C TraceContext propagation.

---

# 3. Statistical Anomaly Detection

Statistical signals require baseline accumulation. They detect deviations from a project's normal operating envelope rather than absolute thresholds. This is what makes Pulse meaningful for users who lack defined SLOs.

### P-009 — Per-Service Error Rate Baseline

Pulse SHALL maintain a continuously updated baseline of error rate per service identified by `service.name` resource attribute. Baseline uses exponentially weighted moving average with an effective window of approximately 5 minutes, calibrated to converge within 60 seconds of bootstrap. Baseline state SHALL be persisted to corpus across application restarts to eliminate cold-start blindness windows.

**Observable signal:** No direct user signal; baseline is internal infrastructure feeding P-010.

**Conductor verification:** Inject sustained error rate at known level; query baseline tracker after convergence window; verify reported baseline matches injection within ±10%. Restart Pulse mid-baseline and verify state restoration.

**Boundary:** Services with fewer than 10 spans per minute over the window may not produce statistically meaningful baselines and SHALL be excluded from spike detection until traffic exceeds this floor.

### P-010 — Error Rate Spike Detection

Pulse SHALL identify when the short-term (30-second window) error rate for a service exceeds its baseline by a factor of 3.0 or more, with persistence of at least 30 seconds before surfacing. Detection latency from threshold breach to candidate emission SHALL be under 2 seconds at p99.

**Observable signal:** Error rate spikes emit attention cues (P-021) which feed model-driven severity classification (P-020). When the model surfaces a resulting incident, halo shifts hue toward alert burgundy, service constellation dot for the affected service shifts color, findings counter increments.

**Conductor verification:** Establish baseline at known error rate over 90 seconds, then ramp injection to 3.5× baseline for 60 seconds. Verify candidate emission within 2 seconds of crossing 30-second persistence threshold.

**Boundary:** Detection is suppressed for 60 seconds following a Restart Event per P-016, except when dual-condition bypass per P-057 applies.

### P-011 — Per-Operation Latency Baseline

Pulse SHALL maintain streaming percentile distributions (p50, p95, p99) per operation identified by combination of `service.name` and operation identifier. Distributions use t-digest or equivalent streaming algorithm with 5-minute effective window.

**Observable signal:** No direct user signal; baseline feeds P-012.

**Conductor verification:** Inject operations at known latency distribution; query baseline percentiles after convergence; verify p99 matches injection within ±15%.

**Boundary:** Operations with fewer than 50 spans over the window may produce unreliable percentile estimates and SHALL be excluded from regression detection.

### P-012 — Latency Regression Detection

Pulse SHALL identify when current p99 latency for an operation exceeds baseline p99 by factor of 2.5 or more, with persistence of at least 60 seconds before surfacing. Errors are weighted independently from latency.

**Observable signal:** Latency regression emits attention cues (P-021) consumed by model-driven severity classification (P-020). When the model surfaces a resulting incident, affected service constellation dot reflects degraded state distinct from error state.

**Conductor verification:** Establish latency baseline at known p99 over 90 seconds, ramp to 3× p99 for 90 seconds. Verify candidate emission after persistence threshold.

**Boundary:** Pulse does not distinguish slowness caused by upstream dependency from slowness caused by local processing in this layer. Distinction emerges in diagnostic Report through model interpretation.

### P-013 — Service Activity Floor Learning

Pulse SHALL learn per-service activity histograms over rolling 24-hour windows and SHALL NOT emit ServiceWentSilent candidates when current quiet duration falls within learned normal quiet patterns for that service. Histogram state SHALL be persisted across application restarts.

**Observable signal:** No direct user signal; protective infrastructure for P-014.

**Conductor verification:** Train baseline with bursty pattern (active 5 minutes, quiet 10 minutes, repeating). After convergence, simulate developer-on-lunch (30 minutes quiet). Verify no ServiceWentSilent candidate emitted during expected quiet pattern. Restart Pulse mid-training and verify histogram restoration.

**Boundary:** Cold-start protection applies: during first 60 minutes of operation per service, ServiceWentSilent is universally suppressed while histogram bootstraps. State persistence eliminates per-restart cold-start blindness — histograms survive application restarts via corpus persistence.

### P-014 — Service Went Silent Detection

Pulse SHALL identify when a service that was previously active goes silent for a duration exceeding its 95th percentile historical quiet duration, with minimum threshold of 30 seconds. Detection emits an attention cue (P-021) with a default severity hint of Suggested; the model retains final discretion per P-020 and may dismiss the cue when context indicates the silence is expected.

**Observable signal:** When the model chooses to surface this, affected service constellation dot dims and shifts color to indicate silence. Service-level silence does not directly affect halo unless model-determined severity escalates.

**Conductor verification:** Establish active service baseline over 5 minutes of consistent activity. Stop emission abruptly. Verify attention cue emitted after 30 seconds (since no learned quiet pattern exists yet for this service). Verify the model's resulting incident, if any, reflects the silence context.

**Boundary:** Detection does not infer the cause of silence (crash, network partition, intentional stop). Cause interpretation is the diagnostic Report's domain.

---

# 4. Pattern Recognition

Pattern recognition operates on shape and sequence rather than threshold values. These signals are often the most user-meaningful — recurring exceptions and cascading failures are how engineers naturally think about problems.

### P-015 — Restart Event Detection

Pulse SHALL detect application restart events by observing per-service stream gaps exceeding 20 seconds followed by resume of span emission. Restart events are emitted to internal broadcast within 2 seconds of resume detection.

**Observable signal:** No direct user signal; protective infrastructure for P-016 and contextual signal for diagnostic Report generation.

**Conductor verification:** Emit consistent traffic, simulate restart (stop emission, wait 25 seconds, resume). Verify RestartEvent within 2 seconds of resume.

**Boundary:** Initial bootstrap (first appearance of a service) is not classified as a restart.

### P-016 — Restart-Window Suppression (Surgical)

Pulse SHALL suppress only short-persistence (<30 second) ErrorRateSpike candidates for 60 seconds following a RestartEvent. Suppression does NOT apply to: sustained error rate spikes exceeding 30-second persistence, hard signals (P-005 through P-008), latency regressions (P-012), or pattern detections (P-018). Suppression is overridden by dual-condition bypass per P-057 when relative magnitude or absolute threshold conditions hold. This surgical scope prevents false positives from startup error bursts while preserving detection of real problems during active development.

**Observable signal:** During suppression window, transient spikes do not affect halo or trigger findings. Sustained problems remain visible. Bypassed cues per P-057 surface with explicit annotation in Report.

**Conductor verification:** Trigger RestartEvent. Within suppression window inject (a) brief error burst lasting 15 seconds — verify suppressed; (b) sustained error rate lasting 60 seconds — verify surfaces after 30-second persistence threshold. Test P-057 bypass paths separately.

**Boundary:** Suppression is not user-configurable in v0.2.0 beyond bypass thresholds (P-057).

### P-017 — Exception Fingerprinting

Pulse SHALL compute stable fingerprints for exceptions consisting of cryptographic hash over `exception.type` plus first three stack frames after normalization (paths stripped to relative form, memory addresses removed, line and column numbers stripped). Fingerprints SHALL be stable across application restarts AND across trivial line-number shifts (code edits that move a frame without changing the logical frame sequence), but distinguish materially different exception sites.

**Observable signal:** No direct user signal; identifier infrastructure for P-018 and corpus learning (P-044).

**Conductor verification:** Inject exception sequences with (a) identical type and stack — verify identical fingerprint; (b) same type, different file paths but same logical location — verify identical fingerprint (path normalization); (c) same type, same logical frames, different line numbers — verify IDENTICAL fingerprint (line-number insensitivity).

**Boundary:** Fingerprinting is intentionally tolerant to path AND line-number differences but strict on logical frame identity (`exception.type` + normalized frame sequence) — stability across trivial shifts is what P-018 retry-storm counting requires.

### P-018 — Retry Storm Detection

Pulse SHALL identify when the same exception fingerprint (P-017) occurs 5 or more times within a 30-second rolling window. Detection emits an attention cue (P-021) with a default severity hint of Suggested. At 10 or more occurrences within the window, the cue severity hint escalates to Autonomous. Final severity is determined by model interpretation per P-020; the model retains discretion to dismiss or adjust based on context.

**Observable signal:** When the model surfaces a resulting incident, Report includes explicit fingerprint identification and storm characteristics. Affected service constellation dot reflects storm state. Halo escalates per cumulative model-determined severity.

**Conductor verification:** Inject 6 occurrences of identical fingerprint within 30 seconds — verify attention cue emitted with Suggested severity hint. Increase to 12 occurrences — verify cue hint escalates to Autonomous. Verify model surfaces resulting incident in default workspace context.

**Boundary:** Detection operates on fingerprint identity, not type identity alone.

---

# 5. Severity Calibration

Severity is what determines user-visible behavior — whether something surfaces, how prominently, and through which channels. In Pulse, the model owns user-facing severity decisions; algorithmic detectors feed the model attention cues but never bypass it to address users directly. Hard signals (P-005 through P-008) form a deterministic floor that operates regardless of model availability — Pulse's honest minimum when intelligence resources are absent.

### P-019 — Three-Tier Severity Model

Pulse SHALL classify every potential incident into one of three severity tiers: **Autonomous** (model is highly confident this represents a real problem warranting attention; surfaces with prominent halo shift and counter increment), **Suggested** (model believes this likely represents a problem but with reservations; surfaces quietly with counter increment but minimal halo impact), **Curious** (model considers this worth recording for pattern learning but does not warrant interruption; appears only in Findings dropdown collapsed section).

**Observable signal:** Severity tier determines halo response intensity, counter increment behavior, and Report prominence.

**Conductor verification:** Inject scenarios calibrated to produce each tier. Verify halo response (color shift magnitude), counter behavior (increment or not), and dropdown placement (top or collapsed).

**Boundary:** Severity escalation across tiers within a single incident lifecycle is permitted (Curious → Suggested when persistence increases). De-escalation is not; once surfaced as Autonomous an incident remains so until resolved.

### P-020 — Model-Driven Severity Decision

Pulse SHALL use local LLM interpretation as the sole mechanism for user-facing severity classification. The model receives a composite input consisting of: structured candidate data emerging from hard signal detection (P-005 to P-008), algorithmic attention cues from baseline and pattern detectors (P-021), project context (P-032), and recent corpus history (P-044). From this composite, the model produces severity tier with confidence score and human-readable justification. The model retains full discretion: an attention cue from the algorithmic layer is an invitation to investigate, not a mandate to surface.

**Observable signal:** Severity decisions drive user-visible behavior per P-019. Justification is captured in diagnostic Report (P-031).

**Conductor verification:** Verify identical input candidates produce consistent severity within model determinism bounds. Verify model decisions reference relevant context elements when present. Inject algorithmic attention cues with deliberately low severity context (transient noise) and verify model can choose to dismiss them.

**Boundary:** Pulse is not a deterministic classifier. Different model versions or quantization levels may produce different severity decisions for borderline cases. Conductor scenarios test calibration regions, not exact values.

**Graceful degradation when model unavailable:** If the local LLM is unavailable, has failed to initialize, or has been explicitly disabled by user, Pulse operates in reduced mode: hard signals (P-005 to P-008) continue surfacing as Suggested severity by default, while statistical and pattern detection remain silent to the user. Pulse never invents interpretations without model resources. Diagnostic Reports in reduced mode contain only the symptom description and raw evidence sections; hypothesis ranking and suggested investigation steps are omitted with explicit notice that intelligent interpretation is unavailable. Hardware-insufficient operation routes through fallback model tier (P-053) and adjusted SLOs (P-054, P-060). This is Pulse's honest minimum: errors are reported, intelligence is not faked.

### P-021 — Algorithmic Attention Cues

Pulse SHALL run threshold-based algorithmic detectors continuously over the outputs of baseline tracking (P-009 to P-014) and pattern recognition (P-017 to P-018). When a detector triggers, it emits an attention cue — a structured record naming the detected condition, the affected scope, and the magnitude of the deviation — into the model's input context for subsequent interpretation passes. Attention cues are internal infrastructure: they have no user-facing surface of their own and never become incidents by themselves. Their purpose is to ensure the model does not overlook statistically significant patterns due to attention-window limitations, prompt-engineering quirks, or context selection biases.

**Observable signal:** None directly. Attention cues appear in the diagnostic Report's evidence section as supporting context when the model chose to surface a related incident, but never as standalone user surfaces.

**Conductor verification:** Inject baseline-relative scenarios (error rate spike, latency regression, retry storm pattern, service silence within learned floor). Verify attention cues are emitted to model context at the expected detection point. Verify the model's resulting incident, if any, references the cue in its evidence section. Verify scenarios producing cues without model surfacing (model dismisses cue as transient) result in no user-visible incident — this is correct behavior, not a bug.

**Boundary:** Algorithmic detectors do not have authority over user-facing surface. If the model is unavailable (P-020 graceful degradation), attention cues continue to be generated for self-observability and corpus diagnostics, but they do not produce incidents. Detector thresholds are calibration starting points tunable via `[triage]` config.

### P-022 — Auto-Resolution and Lifecycle

Pulse SHALL automatically transition active incidents to Resolved status when the underlying detection signal has not been re-emitted for 120 seconds. Resolved incidents remain queryable in corpus and visible in recent-history surfaces for 10 minutes before collapsing into long-term storage. Upon transition to Resolved, Pulse generates a resolution summary interpretation per P-059, attached to the incident record without producing new surface notification.

**Observable signal:** Resolved incidents disappear from active counter. Resolution summary visible when Report is accessed historically, providing "what happened, how it ended" narrative.

**Conductor verification:** Trigger incident, verify Active status. Cease underlying signal, wait 120 seconds, verify auto-transition to Resolved. Verify resolution summary attached to incident record. Verify subsequent same-fingerprint detection creates new incident rather than reopening Resolved incident (subject to cool-down per P-023).

**Boundary:** Auto-resolution timing is calibration-tunable in `[triage]` config but defaults to 120 seconds.

### P-023 — Acknowledge Cool-Down

Pulse SHALL apply a 5-minute cool-down following user acknowledgment of an incident, during which re-detection of the same incident kind and scope does not create a new active incident but does record the occurrence in corpus.

**Observable signal:** Within cool-down, retriggering the same fingerprint does not increment counter or affect halo. Corpus records all occurrences.

**Conductor verification:** Trigger incident, acknowledge, retrigger within 1 minute — verify no counter increment, verify corpus contains both occurrences. Retrigger after 6 minutes — verify new incident created.

**Boundary:** Cool-down is per kind-and-scope, not global.

### P-060 — Tiered Triggering Priority

Pulse SHALL classify detection events into three priority tiers governing inference urgency. Tier-1 (hard signals per P-005 through P-008) triggers immediate digest assembly and LLM invocation with SLO < 5s end-to-end on gpu-primary profile. Tier-2 (medium-confidence cues, confidence 0.7-0.85) triggers accelerated cadence scheduling next inference within 20s rather than waiting full baseline cadence. Tier-3 (baseline cadence regardless of cue state) produces ambient interpretation rhythm with SLO < 90s. All SLOs are hardware-profile-dependent per P-054.

**Observable signal:** SLO-conformant latency between event occurrence and resulting incident surface (when severity warrants surfacing). Tier classification visible in incident metadata.

**Conductor verification:** For each tier, trigger appropriate event and measure end-to-end latency to surface. Verify SLO compliance per hardware profile. On cpu-primary profile, verify Tier-2 acceleration is disabled and medium-confidence cues fall through to Tier-3 baseline behavior.

**Boundary:** SLOs are best-effort under bounded load. Sustained extreme conditions (50k+ spans/sec or LLM unavailability) may degrade Tier-1 to Tier-3-like timing or to hard-signal-only mode per P-020 graceful degradation. Conductor scenario tests verify SLO under typical and high load profiles, not at saturation extremes.

---

# 6. Three-Surface Communication

Pulse communicates through three distinct surfaces, each with different attention demand and information density. Mixing roles between surfaces produces user confusion and erodes the ambient-companion model.

### P-024 — Widget Ambient Surface

Pulse SHALL provide a compact, always-visible widget that communicates current state through visual properties alone, without numerical readouts or scrolling content. The widget contains: halo (central living signature element), service constellation (per-service dots), and connection state dot in header.

**Observable:** Widget is the entirety of this surface.

**Conductor verification:** Verify widget renders in compact and expanded modes. Verify no numerical readouts (ingest rate, error percentage, retention duration) appear in widget chrome.

**Boundary:** Widget never displays incident card content, never displays counters or rates, never displays toasts or popups. All interrupting or content-rich surfaces are separate (P-028, P-031).

### P-025 — Halo Hue Encoding

Pulse SHALL encode current cumulative severity (maximum severity among active incidents) into halo hue via interpolation between Earth Blue (no active concerning incidents) and Alert Burgundy (Autonomous severity active). Suggested severity produces partial shift; Curious severity produces no shift.

**Observable signal:** Halo hue is the primary ambient severity signal, readable in peripheral vision within 100 milliseconds.

**Conductor verification:** Inject scenarios producing each severity tier; verify halo hue interpolation reaches expected positions within 2 seconds of severity change.

**Boundary:** Halo hue reflects current active state only.

### P-026 — Halo Breathing Encoding

Pulse SHALL animate halo through breathing motion with rate proportional to overall activity level. Quiet activity produces slow breathing (4-5 second cycle), active flow produces faster breathing (down to 2 seconds), within bounded range to prevent frantic appearance. Breathing implementation uses opacity and blur modulation only, never scale modulation.

**Observable signal:** Halo breathing communicates "system is alive and proportional to activity" without explicit rate display.

**Conductor verification:** Inject traffic at varying rates; verify breathing rate adjusts smoothly within bounded range; verify never exhibits scale animation.

**Boundary:** Breathing rate calibrates per workspace based on observed traffic norms.

### P-027 — Service Constellation Auto-Discovery

Pulse SHALL automatically discover services from `service.name` resource attributes in incoming spans and render each as a constellation dot. No manual service registration is required or supported. New services appear in constellation within 5 seconds of first valid span.

**Observable signal:** Constellation grows organically as project services emit telemetry. Dot positions are stable across sessions (deterministic by service name hash).

**Conductor verification:** Emit spans from new service identifier; verify constellation dot appears within 5 seconds. Restart Pulse; verify dot positions match prior session.

**Boundary:** Services with empty `service.name` are dropped per P-048. Up to 20 services rendered; additional services contribute to severity calculation but constellation display is capped at this size for legibility.

### P-028 — Findings Counter

Pulse SHALL display a compact unread-incident counter below the widget. Counter shows count of unread active incidents for the current workspace. Counter is hidden when count is zero. Counter persists across application restarts.

**Observable signal:** Counter is the dedicated surface for "something happened that you haven't reviewed yet."

**Conductor verification:** Generate incidents; verify counter increments. Restart Pulse; verify counter persists across restart. Open Reports for all incidents; verify counter clears to zero and hides.

**Boundary:** Counter never auto-clears based on time. Only explicit Report-opening or "Mark all as read" action clears unread state.

### P-029 — Findings Dropdown

Pulse SHALL render a dropdown listing all unread active incidents when counter is clicked. Rows sort by severity descending then chronologically. Each row shows: severity indicator, incident title (truncated if needed with hover-tooltip for full), relative timestamp ("3m ago"). Row click opens diagnostic Report (P-031). Dropdown footer includes "Mark all as read" action.

**Observable signal:** Dropdown is the navigation surface from "I see there are problems" to "let me look at this specific one."

**Conductor verification:** Generate 5 incidents across severity tiers; verify dropdown order is Autonomous-then-Suggested-then-Curious, recency-sorted within tier. Click incident; verify Report opens for correct incident.

**Boundary:** Dropdown displays only unread incidents. Read incidents drop from the dropdown but remain in corpus.

### P-030 — No Interrupting Notifications by Default

Pulse SHALL NOT display modal alerts, system toasts, OS notifications, sound alerts, or other interrupting surfaces for incidents in default configuration. All severity escalation is purely visual through halo hue shift and counter increment. Aggressive notification modes are opt-in via configuration for users who explicitly prefer them.

**Observable signal:** Absence of interruption is itself the contract. Users are never pulled out of flow by Pulse against their will.

**Conductor verification:** Generate Autonomous-severity incident in default config; verify no OS notification, no modal, no sound. Enable aggressive notification config; repeat; verify expected interruption surfaces.

**Boundary:** This invariant is foundational to product positioning and SHALL NOT be weakened in future versions. Adding interrupting surfaces — even with good intent — violates the calm-companion contract.

---

# 7. Diagnostic Quality

The Report is where Pulse's AI-native positioning becomes tangible. When users explicitly choose to open a Report, they want depth, grounding, and actionable framing.

### P-031 — Report Structure

Pulse SHALL produce diagnostic Reports for each active incident containing the following sections in this order: (1) symptom description in plain language, (2) timeline of contributing events with relative timestamps, (3) ranked root-cause hypotheses with confidence indicators, (4) suggested investigation steps not auto-fix actions, (5) relevant telemetry excerpts after anonymization per P-047, (6) project context section grounding the incident in workspace state.

**Observable signal:** Report renders in full-window panel or new window, navigable via section anchors, with telemetry excerpts collapsible.

**Conductor verification:** Generate incidents of varying types; verify all six sections present; verify section order matches specification; verify hypothesis ranking is consistent.

**Boundary:** Reports do not include raw OTLP payloads. Excerpts are paraphrased or sanitized per privacy contract (P-048). Reports do not auto-execute fixes or propose specific code edits without explicit user-driven agent workflow (P-039). In graceful degradation mode (P-020, model unavailable), sections 3 (hypotheses) and 4 (investigation steps) are omitted; the Report contains only symptom, timeline, evidence excerpts, and project context, with an explicit notice that intelligent interpretation is unavailable. Fallback model tier (P-053) produces sections 3 and 4 at reduced fidelity with explicit annotation.

### P-032 — Project Context Grounding

Pulse SHALL ground Report content in observable project context including: workspace path, primary programming language detected via file extensions, framework signals detected via package files, and recent git activity (last 5 commits with messages and timestamps).

**Observable signal:** Report's "Project context" section explicitly displays grounding elements. Hypothesis section references context where relevant.

**Conductor verification:** Run Conductor inside a real git workspace with known recent commits; trigger incident; verify Report context section accurately reflects workspace state; verify hypotheses reference recent commits when temporally relevant.

**Boundary:** Pulse reads git history via local filesystem inspection. It does not connect to git remotes, does not authenticate, and does not access content beyond what is locally accessible.

### P-033 — Ranked Hypothesis Generation

Pulse SHALL produce up to three ranked root-cause hypotheses per incident, each with: hypothesis statement in plain language, confidence level (high / medium / low), and brief justification referencing specific evidence. Hypotheses SHALL be ranked by model-assessed likelihood.

**Observable signal:** Hypothesis section in Report is the most actionable content for users.

**Conductor verification:** Inject scenarios with known root cause; verify highest-ranked hypothesis correctly identifies cause within reasonable interpretation of model output.

**Boundary:** Pulse's hypotheses are diagnostic suggestions, not authoritative diagnoses. Reports SHALL clearly mark hypotheses as model interpretation, not factual claims. Fallback tier (P-053) produces single hypothesis instead of ranked list.

### P-034 — Suggested Investigation Steps

Pulse SHALL produce up to five suggested investigation steps per incident, each a concrete action a user or connected agent can take. Steps are ordered by expected information yield. Steps reference specific files, configuration keys, log queries, or commands where possible.

**Observable signal:** Investigation steps section in Report provides the bridge from "what might be wrong" to "what should I do next."

**Conductor verification:** Generate Report; verify steps section contains 1-5 entries; verify entries reference concrete actions; verify ordering reflects likely information yield.

**Boundary:** Pulse does not execute suggested steps automatically. Fallback tier (P-053) produces up to 2 steps instead of 5.

### P-035 — Anonymized Telemetry Excerpts

Pulse SHALL include relevant telemetry excerpts in Reports after passing through anonymization layer (P-047). Excerpts focus on span timing and structure rather than payload content.

**Observable signal:** Excerpts section in Report shows the evidence underlying hypotheses without exposing sensitive content.

**Conductor verification:** Generate incident from spans containing test PII patterns; verify Report excerpts have these scrubbed; verify span structure is preserved.

**Boundary:** Excerpt selection is interpretive, not exhaustive.

### P-036 — Cross-Incident Pattern Reference

Pulse SHALL surface in Reports when current incident matches patterns from corpus history of the same workspace. Match criteria include exception fingerprint similarity (P-017), service-and-operation scope, and temporal context. Matched references appear in Report as a "Previously seen" subsection with timestamps and resolution outcomes where known.

**Observable signal:** "Previously seen" subsection appears when historical matches exist.

**Conductor verification:** Generate recurring incident pattern across two sessions; verify second session's Report references first session's occurrence; verify resolution outcome of first session is reflected if it was resolved.

**Boundary:** Cross-incident matching is per-workspace only.

---

# 8. Output Channels

Reports must reach users wherever they prefer to work. The three channels are equal-tier: no channel is privileged over another in content quality. Users skeptical of MCP get full diagnostic value through clipboard.

### P-037 — In-App Report Surface

Pulse SHALL render Reports in a dedicated in-app surface. Surface supports keyboard navigation, copy of full Report content via standard OS shortcut, and section collapse for managing visual density.

**Observable signal:** In-app Report is the default and always-available consumption surface.

**Conductor verification:** Open Report from dropdown; verify rendering completes within 2 seconds; verify keyboard navigation works.

**Boundary:** In-app surface is read-only for v0.2.0.

### P-038 — Copy to Clipboard

Pulse SHALL provide explicit "Copy Report" action that places the complete Report as markdown into the system clipboard. The markdown content SHALL be identical character-for-character to what MCP delivery (P-039) would transmit.

**Observable signal:** "Copy Report" button in Report toolbar. Toast confirmation after copy.

**Conductor verification:** Open Report; click Copy; verify clipboard content matches Report rendering; verify markdown validates against CommonMark parser.

**Boundary:** Clipboard format is markdown only.

### P-039 — MCP Delivery When Configured

Pulse SHALL provide MCP server interface exposing tools for connected agents to: query current incident list, retrieve full Report for specific incident, retrieve recent telemetry slice for incident context, and mark incident as resolved through agent action. MCP delivery is available only when MCP server is configured and an agent is connected. "Send to agent" button visible only in this state.

**Observable signal:** "Send to agent" button visible only when MCP active.

**Conductor verification:** Configure MCP with mock agent client; open Report; verify Send to agent button visible. Disconnect mock agent; verify button hides. Click Send to agent; verify mock agent receives Report content identical to clipboard format.

**Boundary:** MCP delivery transmits identical content to clipboard format.

### P-040 — MCP Independence

Pulse SHALL function fully without MCP configuration. All capabilities except MCP delivery itself (P-039) are available in MCP-absent state.

**Observable signal:** Absence of MCP-related UI when not configured. No degradation messaging, no nag prompts.

**Conductor verification:** Run Pulse in fresh state without MCP configuration; verify all P-001 through P-038 capabilities pass; verify no MCP-related elements in UI; verify settings include MCP enablement as opt-in option.

**Boundary:** This invariant is product-positional and SHALL NOT weaken in future versions.

---

# 9. Memory & Learning

Distilled history is what makes Pulse smarter over time and what produces the corpus suitable for community model training.

### P-041 — Persistent Incident Corpus

Pulse SHALL persistently store every detected incident in local SQLite-based corpus including: trigger context (anonymized telemetry slice that drove detection), model interpretation (Report content), user response timeline, and resolution outcome. No incident data is silently discarded or overwritten. Beyond incident records, Pulse persistently stores pipeline operational metrics per P-058 with 30-day default retention.

**Observable signal:** No direct user-facing signal. Corpus is foundational infrastructure for P-044 and P-046.

**Conductor verification:** Generate incidents; query corpus via direct SQLite inspection; verify all incidents present with complete fields. Restart Pulse; verify corpus survives.

**Boundary:** Corpus growth is bounded by data shape rather than retention policy. Automatic purging is not performed in v0.2.0; users can manually clear corpus through Settings.

### P-042 — Cross-Session Continuity

Pulse SHALL preserve incident state, including read/unread status, acknowledgment status, and active/resolved status, across application restarts.

**Observable signal:** Counter reflects accumulated state across sessions.

**Conductor verification:** Generate incidents, close Pulse, restart Pulse; verify counter and dropdown reflect prior state. Acknowledge incidents, close, restart; verify acknowledgments persisted.

**Boundary:** State persistence is local-only. State does not synchronize across multiple Pulse installations or devices.

### P-043 — Project-Scoped Memory

Pulse SHALL attribute all incidents to the workspace they originated from. Counter and dropdown displays SHALL be filtered to current workspace by default. Cross-workspace incidents are not conflated in interpretations or pattern matching.

**Observable signal:** When user switches workspaces, counter and dropdown reflect new workspace state.

**Conductor verification:** Generate incidents while Pulse observes workspace A; switch to workspace B; verify counter reflects workspace B state only.

**Boundary:** Workspace identification depends on workspace-detector accuracy.

### P-044 — Retrieval-Augmented Interpretation

Pulse SHALL provide recent corpus context to the local LLM during interpretation, including: prior incidents from same workspace within last 30 days, particularly those matching current fingerprint or scope; resolution outcomes of prior incidents; user feedback patterns.

**Observable signal:** Reports include "Previously seen" subsection per P-036 when applicable.

**Conductor verification:** Generate recurring incident pattern; on second occurrence verify Report references first occurrence in interpretation.

**Boundary:** Context retrieval is bounded by model context window. Pulse retrieves up to N most relevant entries (default 5).

### P-045 — Counter Derivation from Corpus

Pulse SHALL derive the findings counter from corpus query on demand, not from a separate state file. Counter shows count of incidents in current workspace where `read_at IS NULL AND status = 'active'`.

**Observable signal:** Counter consistency with reality.

**Conductor verification:** Query counter, modify corpus directly to mark incident read, query counter again; verify counter updates within 1 second of next refresh.

**Boundary:** Counter refresh occurs on incident events and on widget focus.

### P-046 — Export for Community Training

Pulse SHALL provide explicit export action producing shareable training dataset from corpus. Export format is JSONL containing: anonymized trigger context, model interpretation, user feedback, resolution outcome. Export is opt-in only — there is no automatic submission. Users SHALL be able to inspect exported content before submitting.

**Observable signal:** Export action in Settings; preview dialog before submission.

**Conductor verification:** Generate incidents with mock feedback signals; perform export; verify JSONL content matches incidents; verify preview dialog displays summary before any transmission; verify no transmission occurs without explicit user confirmation.

**Boundary:** Export targets are not pre-configured in v0.2.0.

---

# 10. Privacy & Trust

Privacy is not a feature in Pulse — it is a structural constraint enforced at every layer where data crosses persistence or process boundaries.

### P-047 — PII Scrubbing at Ingestion

Pulse SHALL apply pattern-based PII scrubbing to all content before persistence to corpus or inclusion in Reports. Scrubbing targets include: email address patterns, JWT tokens, bearer tokens, API keys, credit card patterns, social security number patterns, and key-value pairs where key matches secret-like patterns.

**Observable signal:** No direct user-facing signal. Scrubbing is invisible when working correctly.

**Conductor verification:** Inject spans with each PII pattern category; verify corpus storage and Report rendering shows scrubbed values.

**Boundary:** Scrubbing is pattern-based and may miss novel formats. Users with sensitive workloads SHOULD apply additional sanitization at source.

### P-048 — No Raw OTLP Attribute Values Stored

Pulse SHALL NOT store raw OTLP attribute values in corpus beyond what is necessary for hypothesis generation. Anonymized fingerprints, normalized identifiers, and structural metadata are permitted.

**Observable signal:** No direct user-facing signal. Corpus content is structurally lean.

**Conductor verification:** Inject spans containing rich attribute payloads; inspect corpus storage directly; verify storage contains only normalized fingerprints and metadata.

**Boundary:** Some OTLP attributes are themselves identifiers (request_id, trace_id) and are retained as opaque identifiers.

### P-049 — Encryption at Rest

Pulse SHALL encrypt corpus SQLite database at rest using key from OS keychain (when available) or user-provided passphrase. Encryption is transparent to corpus operations during runtime.

**Observable signal:** First-run setup prompts for keychain access or passphrase.

**Conductor verification:** Run Pulse, generate incidents, terminate Pulse, inspect corpus file directly; verify file is encrypted and not directly readable.

**Boundary:** Encryption protects against casual disk inspection, not sophisticated forensic recovery.

### P-050 — Cross-Project Sharing Opt-In

Pulse SHALL NOT share corpus content across workspaces, across devices, or with any external endpoint without explicit per-action user opt-in. Default state is fully local.

**Observable signal:** No automatic synchronization, no telemetry beacons to external services.

**Conductor verification:** Monitor network traffic during fresh Pulse operation; verify no outbound connections occur beyond user-configured destinations.

**Boundary:** Future community-training submission paths (P-046) remain explicit per-action choices.

### P-051 — Transparent Storage

Pulse SHALL provide users with the ability to inspect, export, and delete corpus content through Settings interface.

**Observable signal:** Settings includes "Storage" section with inspection, export, and deletion controls.

**Conductor verification:** Access Storage settings; verify inspection displays corpus entries; verify export produces complete JSONL; verify deletion clears corpus and is preceded by explicit confirmation dialog.

**Boundary:** Inspection is read-only.

---

# 11. Pipeline Operations (NEW in v2)

Pipeline Operations capabilities formalize Pulse's contract about its own software behavior — how the distillation pipeline operates, adapts to hardware, accepts configuration, and surfaces its own health. These are user-facing contracts even though many are infrastructure-level: users may not look at them daily, but their existence as formal commitments protects users from silent operational degradation.

### P-052 — Cadence Configuration

Pulse SHALL provide configurable cadence intervals for distillation pipeline operation: baseline cadence (default 60s) governs how frequently Pulse evaluates current state when nothing is fired; accelerated cadence (default 20s) governs Tier-2 medium-priority cue response; reflection cadence (default 1800s) governs long-window cumulative pattern analysis. All three are tunable via `[triage.cadence]` config without application restart.

**Observable signal:** None directly user-facing. Configured intervals visible in Settings → Diagnostics → Cadence. Behavioral effects observable as interpretation frequency.

**Conductor verification:** Configure cadence intervals to known values (10s baseline, 5s accelerated, 60s reflection); inject deterministic workload; verify L4 invocation rate matches configuration.

**Boundary:** Cadence intervals cannot be reduced below safety floor (5s baseline minimum) to prevent runaway LLM invocation. Reflection cadence cannot be below 5 minutes. Out-of-range values rejected with diagnostic warning, previous valid configuration retained.

### P-053 — Fallback Model Tier

Pulse SHALL support a fallback model tier (3-4B parameter class, 4-6GB VRAM footprint, CPU-inference-capable) as an alternative to primary tier for users with hardware below primary tier envelope. Fallback tier produces reduced-quality interpretation: single hypothesis instead of ranked list, up to 2 investigation steps instead of 5, less specific project context grounding. Reports explicitly annotated with `model_tier: "fallback"` in metadata and visible "fallback tier" indicator in Report UI.

**Observable signal:** Fallback-tier indicator on every Report generated by fallback model. Settings → Diagnostics → Model shows currently active tier. No silent shipping of reduced quality.

**Conductor verification:** Configure Pulse with fallback-tier model; trigger interpretation scenario; verify Report contains fallback indicator; verify Report structure matches reduced-quality contract.

**Boundary:** Fallback tier provides reduced quality but full functionality. Hard signals continue to surface; capabilities P-001 through P-018 unchanged. Interpretation-dependent capabilities (P-033, P-034) operate at reduced fidelity but remain functional. Capability spec amendments may further constrain fallback fidelity in future versions but never weaken below hard-signal floor.

### P-054 — Hardware Profile Awareness

Pulse SHALL detect available hardware capability at startup and classify into one of four profiles: gpu-primary (≥16GB VRAM with primary model), gpu-fallback (4-8GB VRAM with fallback model), cpu-primary (no GPU with primary model), cpu-fallback (no GPU with fallback model). Detected profile governs operational SLOs per P-060 and model tier selection. Profile is visible in Settings → Diagnostics → Hardware Profile. When CPU inference is detected with primary-tier model configured, Pulse presents a one-time notice recommending fallback-tier switch; user retains choice.

**Observable signal:** Profile name and details displayed in Diagnostics. One-time notice on cpu-primary detection with "Switch to fallback" and "Keep current" options. No automatic tier switching without user consent.

**Conductor verification:** Run Pulse on each of four hardware configurations; verify profile detection matches expected; verify cpu-primary triggers the recommendation notice exactly once per fresh installation.

**Boundary:** Profile classification is best-effort based on detected hardware capabilities at startup. Mid-session hardware changes (eGPU disconnected, thermal throttling) are not detected; user restart required to re-detect.

### P-055 — Configuration Hot Reload

Pulse SHALL apply configuration changes for operational parameters (thresholds, cadences, severity rules, suppression behavior, lifecycle boundaries) within 2 seconds of config file save, without application restart. Foundationally-fixed parameters (Drain settings, model path, network ports, storage paths) require restart with explicit notice. The split between hot-reloadable and restart-required parameters is documented in Settings → Configuration and follows operational mutability rather than aesthetic categorization.

**Observable signal:** Config changes take effect within 2 seconds. Restart-required changes raise diagnostic notice indicating which keys require restart and why.

**Conductor verification:** Modify hot-reloadable config key (e.g., error_rate_multiplier); verify new threshold applies on next L1a tick. Modify restart-required key; verify notice raised, no behavior change until restart.

**Boundary:** Malformed config (parse error, type mismatch, value out of allowed range) rejected with notice; previous valid config remains active. Hot reload does not reset L1b baselines or other accumulated state. Threshold changes apply prospectively per P-056.

### P-056 — Prospective Threshold Application

Pulse SHALL apply threshold changes to future evaluation only. Hot-reloaded threshold changes (e.g., reducing error_rate_multiplier from 3.0 to 1.5) do not retroactively re-evaluate accumulated baselines or rolling windows. Cues that would have fired under new thresholds for already-observed data are not retroactively triggered. Users wanting retrospective application can opt-in via Settings → Diagnostics → "Re-evaluate recent window with current thresholds" action.

**Observable signal:** No notification flood when thresholds are lowered. Threshold changes feel like adjustments to future sensitivity, not retroactive revisions of past observations.

**Conductor verification:** Accumulate baseline over 90s at known rate. Reduce threshold multiplier such that current rate would now exceed it. Verify no immediate cue emission. Verify cues emit only when new data arrives that exceeds new threshold against fresh evaluation. Trigger opt-in backfill action; verify retrospective re-evaluation produces expected cues.

**Boundary:** Prospective application is a non-negotiable invariant for hot reload. The opt-in backfill action is explicit user choice; never automatic.

### P-057 — Dual-Condition Suppression Bypass

Pulse SHALL override restart-window suppression (P-016) when either of two conditions holds: relative magnitude exceeds 10× baseline (configurable via `[triage.suppression.magnitude_bypass_multiplier]`), OR absolute error rate exceeds 5% (configurable via `[triage.suppression.absolute_bypass_threshold]`). Either condition triggers surfacing despite suppression window. Catches catastrophic regressions on projects with naturally low baselines where relative magnitude alone misses meaningful absolute breakouts.

**Observable signal:** Suppressed-yet-bypassed cues surface during restart window with explicit annotation in Report ("Bypassed restart suppression due to: relative_magnitude=12.4× / absolute_error_rate=8.2%").

**Conductor verification:** Trigger restart event. Within 60s suppression window, inject (a) 8× relative spike with 3% absolute — verify suppressed; (b) 12× relative spike with 4% absolute — verify bypassed via relative condition; (c) 6× relative spike with 7% absolute — verify bypassed via absolute condition.

**Boundary:** Bypass applies only to ErrorRateSpike under restart suppression. Other suppression rules (acknowledge cool-down per P-023) are not affected. Setting both thresholds to infinity disables bypass entirely, recovering blanket-suppression behavior.

### P-058 — Pipeline Self-Observability

Pulse SHALL expose per-layer operational metrics for the distillation pipeline through Settings → Diagnostics view. Metrics include inference success rate, JSON parse failure counts, Q7 query timeout rates, magnitude bypass trigger counts, baseline cold-start vs restored bootstrap counts, model tier in use, hardware profile, and approximately 30 other operational metrics. Users SHALL be able to verify pipeline health independently of product surface — distinguishing "Pulse feels off because something operationally degraded" from "Pulse feels off perceived but unactual."

**Observable signal:** Diagnostics view shows current pipeline state and 30-day historical trends. Read-only by default per P-051.

**Conductor verification:** Generate scenario triggering known pipeline behaviors (Drain template explosion, LLM parse failure, Q7 timeout); verify Diagnostics view reflects the events with correct counters within next refresh cycle.

**Boundary:** Self-observability is a transparency contract, not a self-healing contract. Pulse does not auto-correct pipeline issues based on its own metrics. Users diagnose and either retry, reconfigure, or report issues. Pipeline metrics retention is 30 days default (configurable).

### P-059 — Active-Incident Interpretation Continuity

Pulse SHALL interpret evolution of active incidents through their lifecycle, not just final state. While an incident is active (severity ≥ Suggested, not yet Resolved), cadence-mode digests for that incident's scope are exempt from last-write-wins inference queue replacement. Each digest produces its own interpretation. Upon incident resolution (auto-resolution per P-022 or user action), Pulse generates a resolution summary interpretation attached to the incident record without producing new surface notification.

**Observable signal:** Active incidents accumulate interpretation updates visible in Report when accessed during their lifecycle. Resolved incidents include "How this resolved" section synthesizing the lifecycle.

**Conductor verification:** Trigger sustained incident (e.g., persistent error rate spike for 10 minutes). Verify multiple interpretation updates occur during incident lifecycle, not single final-state interpretation. Verify resolution summary attaches when incident transitions to Resolved.

**Boundary:** Active-incident exception is bounded at 5 concurrent active incidents to prevent runaway LLM invocation. Beyond depth 5, oldest queued is dropped with diagnostic warning. Resolution summary generation is best-effort; failure does not block resolution itself, but failure is logged for visibility per P-058.

---

## Capability Summary

| Category | Count | Coverage |
|---|---|---|
| Connection & Health Awareness | 4 | P-001 to P-004 |
| Hard Signal Detection | 4 | P-005 to P-008 |
| Statistical Anomaly Detection | 6 | P-009 to P-014 |
| Pattern Recognition | 4 | P-015 to P-018 |
| Severity Calibration | 6 | P-019 to P-023, P-060 |
| Three-Surface Communication | 7 | P-024 to P-030 |
| Diagnostic Quality | 6 | P-031 to P-036 |
| Output Channels | 4 | P-037 to P-040 |
| Memory & Learning | 6 | P-041 to P-046 |
| Privacy & Trust | 5 | P-047 to P-051 |
| Pipeline Operations | 8 | P-052 to P-059 |
| **Total** | **60** | **P-001 to P-060** |

---

## Open questions for review

The following are calibration questions, not blocking architectural decisions. They resolve through Conductor scenario testing during implementation:

1. **Severity thresholds (3.0× error rate, 2.5× latency, 5/10 retry storm counts).** Multipliers are calibration starting points; real-world tuning during Conductor scenario development may revise.

2. **Cool-down duration in P-023:** 5 minutes default. Solo-dev rebuild cycles can be faster.

3. **Auto-resolution in P-022:** 120 seconds default. May be too aggressive for slow-recovery incidents.

4. **Cross-workspace boundaries in P-043:** Strict isolation chosen. Cross-workspace pattern visibility may surface as opt-in capability in future versions.

5. **Encryption fallback in P-049:** Passphrase fallback when OS keychain unavailable. Risk of user-forgotten passphrase causing permanent corpus loss.

6. **MCP tool surface in P-039:** Four tools listed. May expand based on agent workflow patterns observed during Conductor development.

7. **Cadence defaults in P-052:** Calibration through real workload feel.

8. **Magnitude bypass thresholds in P-057:** 10× relative, 5% absolute. Calibration through Conductor scenarios.

9. **Pipeline metric retention in P-058:** 30 days. Trades storage for historical visibility.

10. **Active incident queue depth in P-059:** 5 concurrent. Balances continuity against backlog risk.

---

## Next steps

This document is the foundational baseline for:

1. **Conductor scope definition** — each P-XXX produces one or more Conductor scenarios that trigger and verify the capability. Conductor implementation chunks are organized by which capabilities they enable.

2. **Pulse-v0_2_0-route capability mapping** — `pulse-v0_2_0-route.md` §Capability-to-chunk mapping (line 815) lists which P-XXX claims each chunk enables, surfacing drift if a capability is claimed but no chunk supports it. (Initial mapping landed v3 plan; refreshed during consolidation chunks #70-#77.)

3. **README and marketing copy** — language describing Pulse to potential users derives from claims here. Marketing claims that do not correspond to a P-XXX entry are not allowed.

4. **Conductor verification suite** — each capability's "Conductor verification" section becomes a test scenario implemented in Conductor. The test suite is the regression contract.

5. **Future capability decisions** — proposals to add capabilities create new P-XXX entries through evolve mechanism. Proposals to remove or weaken existing capabilities require explicit decision recorded in arch §Established Decisions.

---

## Changelog

### v2.1 — 2026-06-12

Divergence-sync revision per the post-#99 capability audit
(`docs/v0_2_0/pulse-v0_2_0-capability-audit-2026-06-12.md` F2, remediation
defaults 2-3) — two spec-side amendments where implementation reality is the
better engineering; applied at chunk #100 via /andromeda-implement with
amendment markers (Trigger 4 ceremony):

- **P-008 — Root-Span Error Scope Distinction.** Observable reworded: root-vs-deep is detected deterministically (L1a Q7 `parent_span_id IS NULL`) and surfaced to the model as a *fact*; severity WEIGHTING is model-side per P-020 — no deterministic multiplier. Conductor clause becomes a calibration-region check (tendency, not hard assert). Amendment record: `.andromeda/runs/2026-06-12T18-52-00-spec-amendment-p008-root-weighting-model-side/amendment.md`.
- **P-017 — Exception Fingerprinting.** "Line numbers preserved" → line/column numbers STRIPPED during normalization; Conductor clause (c) inverted (same logical frames + different line numbers → IDENTICAL fingerprint); Boundary reworded — stability across trivial line shifts is what P-018 storm counting requires. Amendment record: `.andromeda/runs/2026-06-12T18-51-27-spec-amendment-p017-line-insensitive-fingerprints/amendment.md`.

**Total capability count:** unchanged (60 across 11 categories).

### v2 — 2026-05-14

Revision aligning capability spec with distillation architecture v3 decisions. Formalizes architectural concepts that were previously embedded in distillation document as actual product contracts with P-XXX numbers.

**New capabilities (9 added):**

- **P-052 — Cadence Configuration.** Formalizes baseline/accelerated/reflection cadence as user-facing configurable contract with hot-reload via `[triage.cadence]` config and safety floor enforcement.
- **P-053 — Fallback Model Tier.** Formalizes 3-4B class fallback model as full capability with reduced quality, explicit Report annotation, never silent.
- **P-054 — Hardware Profile Awareness.** Formalizes four-profile detection (gpu-primary, gpu-fallback, cpu-primary, cpu-fallback) with profile-dependent SLOs and one-time user notice on cpu-primary detection.
- **P-055 — Configuration Hot Reload.** Formalizes the split between hot-reloadable operational parameters and restart-required foundational parameters as user-facing stability contract.
- **P-056 — Prospective Threshold Application.** Formalizes "threshold changes apply to future evaluation only, never retroactively" as no-surprise contract preventing notification floods.
- **P-057 — Dual-Condition Suppression Bypass.** Formalizes relative-OR-absolute bypass conditions for restart suppression as catastrophic-regression escape valve.
- **P-058 — Pipeline Self-Observability.** Formalizes operational metric exposure through Diagnostics as transparency contract, 30-day retention default.
- **P-059 — Active-Incident Interpretation Continuity.** Formalizes LWW exception during active incidents and resolution summary generation as interpretation-evolution contract.
- **P-060 — Tiered Triggering Priority.** Formalizes three-tier priority system (Tier-1 <5s, Tier-2 <20s, Tier-3 <90s) with hardware-profile-dependent SLOs as urgency-routing contract.

**Existing capabilities updated:**

- **P-009 — Per-Service Error Rate Baseline.** Added explicit requirement for baseline state persistence across restarts.
- **P-013 — Service Activity Floor Learning.** Added persistence requirement and clarified that state persistence eliminates per-restart cold-start blindness.
- **P-016 — Restart-Window Suppression.** Added reference to P-057 dual-condition bypass as suppression override mechanism.
- **P-020 — Model-Driven Severity Decision.** Extended graceful degradation section to cover hardware-insufficient cases via fallback tier (P-053) and adjusted SLOs (P-054, P-060).
- **P-022 — Auto-Resolution and Lifecycle.** Added reference to P-059 resolution summary generation at incident close.
- **P-031 — Report Structure.** Updated boundary to cover fallback tier (P-053) reduced fidelity and confirm graceful degradation behavior.
- **P-033 — Ranked Hypothesis Generation.** Added boundary note about fallback tier producing single hypothesis.
- **P-034 — Suggested Investigation Steps.** Added boundary note about fallback tier producing up to 2 steps instead of 5.
- **P-041 — Persistent Incident Corpus.** Added reference to pipeline metrics retention per P-058.

**Structural changes:**

- **New category 11 — Pipeline Operations.** Eight capabilities (P-052 through P-059) grouped together because they share operational-behavior theme. Improves discoverability for "what does Pulse promise about its own operations."
- **P-060 added to category 5 — Severity Calibration.** Placed with severity-related capabilities because tier priority is about severity routing, not operational config.
- **Model assumptions section updated** to acknowledge fallback tier (≤6GB VRAM) and CPU inference support alongside primary tier (≤16GB VRAM).

**Total capability count:** 51 → 60 across 10 → 11 categories.

### v1 — 2026-05-14 (earlier the same day)

Initial draft. 51 capabilities across 10 categories: Connection & Health Awareness, Hard Signal Detection, Statistical Anomaly Detection, Pattern Recognition, Severity Calibration, Three-Surface Communication, Diagnostic Quality, Output Channels, Memory & Learning, Privacy & Trust. Established product contract, conventions, model assumptions, and conductor verification methodology.
