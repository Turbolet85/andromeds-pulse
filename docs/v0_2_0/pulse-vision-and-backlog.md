# pulse — vision and backlog (v3)

_Updated after a series of architectural refinements that formalized product contracts and pipeline design. Replaces v2._

_These are our notes, not a roadmap commitment. Used as anchor when in doubt, and as input for targeted regeneration of the pipeline when we reach Step 5 sequence._

**Implementation specifications (companion documents):**
- `pulse-capability-spec.md` v2 — formal product contract: 60 P-XXX capabilities across 11 categories
- `pulse-distillation-architecture.md` v3 — telemetry pipeline design: 6 layers + self-observability
- `pulse-v0_2_0-route.md` v2 — execution plan: 33 chunks + 2 blocking decisions

These three documents are the implementation reality. This vision is the **why** and the **soul**; the others are the **what** and the **how**.

---

## What changed vs v2

The v2 framing treated pulse as a coherent companion concept with rough sense of pillars, surfaces, and roadmap. Through subsequent architectural deep dives, several decisions tightened or shifted:

- **Three-surface communication architecture explicit.** v2 mentioned "incident card appears inside the widget." That was wrong — incident card embedded in widget violates ambient-companion model. Replaced by three distinct surfaces: widget (pure ambient, no card), findings counter + dropdown (passive aggregator under widget), Report (deliberate-access full-window diagnostic surface).

- **No interrupting notifications by default — strengthened invariant.** v2 said "notification only at autonomous severity." v3 says: no OS notifications, no modals, no toasts even at Autonomous. All severity escalation is visual through halo hue + counter increment. Aggressive notification modes are opt-in for users who explicitly prefer them.

- **Model size assumptions updated.** v2 mentioned "Generic open-weights small model (Nemotron / Llama 3.2 1B / SmolLM2 in Q4)." That underestimated what's needed for the diagnostic quality pulse claims. v3 reality: 7-13B class primary tier targeting 16GB VRAM, plus 3-4B fallback tier for hardware-constrained users. Primary tier produces ranked hypotheses, investigation steps, project-grounded interpretation. Fallback produces reduced quality with explicit annotation — never silent degradation.

- **MCP framing tightened from "MCP-first" to "MCP-optional."** Many developers are skeptical of MCP. Pulse should never require it. v3 positions MCP as one of three equal-tier output channels alongside in-app Report rendering and clipboard markdown export. Same content quality across all three. Pulse fully functional without any MCP setup.

- **v0.2.0 scope expanded.** v2 roadmap framed v0.2.0 as the skill onboarding milestone with v0.3.0 as "polish" (disk ring buffer, log digest pipeline). v3 reality: v0.2.0 absorbs much of what was v0.3.0 — full LLM interpretation, corpus persistence, log template mining via Drain, three-surface communication, self-observability. The skill onboarding milestone shifts to v0.3.0 or later. See updated Roadmap section below.

- **Pipeline self-observability formalized.** v2 didn't address how users diagnose "pulse feels off lately." v3 adds Settings → Diagnostics view exposing per-layer operational metrics. Transparency contract, not self-healing — users diagnose and decide what to do.

- **Hardware-honesty contract.** v2 assumed unspecified GPU availability. v3 acknowledges hardware diversity (gpu-primary, gpu-fallback, cpu-primary, cpu-fallback) with profile-dependent SLOs. CPU inference supported at degraded latency. Tier-2 acceleration disabled on cpu-primary because hardware cannot meet 20-second SLO honestly.

- **Corpus persistence as first-class subsystem.** v2 mentioned corpus contribution opt-in for community model training. v3 adds: full distilled history of every incident persists locally (encrypted SQLite). Pulse remembers across sessions, attributes to workspace, learns from project-specific patterns. Eventually opt-in export to community dataset, but local memory is the foundation.

Most of the v2 backlog remains relevant but is now formalized in capability spec as P-XXX entries. A few items dropped (see updated section below).

---

## Product soul

One paragraph that anchors everything else.

> Pulse is an ambient debug companion for the emerging generation of vibe-coders.
> Not a monitoring tool — a presence that notices what matters and quietly signals.
> The user doesn't configure observability — a Claude Code skill sets it up deep
> in the project on the agent's request. The user doesn't ask questions — pulse
> tells what's wrong on its own and suggests action. Privacy-first, local-first,
> AI-native. Open source Apache 2.0, free forever, optional cloud tier for
> convenience. First in the line of andromeda companions.

Every word matters. This is the **tone** pulse has:
- "Companion" (not tool, not platform, not assistant)
- "Quietly signals" (not alerts, not warnings, not notifications)
- "Notices what matters" (proactive understanding, not raw data display)
- "Free forever" (commitment, not trial)

Tagline for marketing: **"Telemetry without the hassle"**. Punchy in any language. Russian original was "Телеметрия без напрягов" — the casual register is intentional.

---

## The shift — why now

Thesis: AI-native era developer tools require a different philosophy than pre-AI era ones.

Pre-AI observability tools assumed:
- Developer **knows** what to look for in telemetry
- Developer **opens** dashboards regularly
- Developer **learns** observability as a skill
- Developer **configures** SDK / collectors / exporters by hand

None of this is true for the vibe-coder of 2026. The vibe-coder:
- **Doesn't know** observability conventions, learns as needed
- **Doesn't want** dashboard discipline, wants focus on code
- **Doesn't learn** observability separately — learns through using it
- **Doesn't configure** infrastructure — asks the agent to

Pulse adopts these assumptions explicitly. Every design decision is validated against them.

---

## Three pillars

Pulse is not one thing — it's three tightly coupled artifacts that work together.

### Pillar 1 — Pulse the application

Always-visible compact widget on the desktop. Halo Pulse as a living signature element — slow breathing when quiet, accelerating when the stream is active. Service constellation as the glance pattern.

**Three communication surfaces, never mixed:**

1. **Widget — ambient surface.** Pure visual state. Halo encodes cumulative severity via hue (Earth Blue → Alert Burgundy). Halo breathes via opacity and blur modulation, never scale. Service constellation shows per-service state (brightness=activity, hue=severity). Connection state dot in header. **No numerical readouts, no incident cards, no toasts inside the widget.** This is the ambient peripheral-vision contract.

2. **Findings counter + dropdown — passive aggregator.** Small counter under the widget showing unread active incident count. Hidden when zero. Click expands dropdown listing unread incidents sorted by severity then chronologically. Row click opens Report. "Mark all as read" action. This is where "something happened, you haven't reviewed it yet" lives — without ever interrupting the user.

3. **Diagnostic Report — deliberate-access surface.** Full-window or separate-window panel opened on explicit click. Rich content: symptom description, timeline of contributing events, ranked root-cause hypotheses with confidence indicators, suggested investigation steps, anonymized telemetry excerpts, project context grounding (workspace path, language, framework, recent git activity). This is where AI-native interpretation becomes tangible. Three equal-tier delivery paths: read in-app, copy markdown to clipboard, send via MCP to connected agent. Same content quality across all three.

**No interrupting notifications by default.** Halo hue and counter increment are the only severity signals. No OS notifications, no modals, no sound alerts. Users in flow stay in flow. Aggressive notification modes opt-in for users who want them.

Tray icon — secondary minimal status indicator. Connection state visible at a glance.

### Pillar 2 — Pulse the skill

Claude Code skill (extensible to Cursor / other agent platforms later) that ships together with the pulse application.

Skill runs several phases when the user asks the agent to "set up pulse for this project":

1. **Detection** — runtime / language / framework / package manager / entry points / build system / existing observability artifacts
2. **Decision** — which OTel SDK to install, which auto-instrumentations, where to inject init code, how to configure exporter
3. **Application** — modifies dependencies (package.json / pyproject.toml / Cargo.toml), generates configs, modifies entry points for init, creates README sections
4. **Verification** — build/install check, sends test trace, confirms it arrived in pulse, generates fix suggestions if something is wrong
5. **Continuous** — after initial setup the skill stays available. User adds a new route → "want me to add instrumentation?"

**"Make the agent work hard"** philosophy — the skill goes **deep** vs minimal-invasive. Beyond auto-instrumentation it suggests custom semantic spans for domain-specific events (checkout completed, payment failed, user signup). Through collaborative deepening — agent analyzes and suggests, user approves one at a time. User learns observability through agent's diffs.

The skill also **integrates with pulse's MCP interface when configured**. When something breaks in code — the agent through the skill can query pulse via MCP for current incidents, retrieve Reports, and act on diagnostic context. Closed-loop workflow for users who want it. Users who don't configure MCP still get full skill value through standard agent workflows.

### Pillar 3 — Pulse the model

**Long-term aspiration with v0.2.0 foundation.**

**v0.2.0 reality** — Local LLM integration with two tiers accommodating hardware diversity:

- **Primary tier** — 7-13B parameter class quantized to fit 16GB VRAM. Produces ranked hypotheses with confidence, investigation steps, project-grounded interpretation. Target latency 1-3 seconds on GPU.
- **Fallback tier** — 3-4B parameter class fitting 4-6GB VRAM, also CPU-inference capable. Reduced quality with explicit annotation: single hypothesis, simplified investigation steps. Never silent degradation — Reports clearly mark fallback-tier provenance.

Specific model selection is implementation detail expected to evolve. Pulse specifies invariants (≤16GB primary / ≤6GB fallback, JSON-structured output, instruction-following, ≥4K context) not model identity. Any compatible local model works.

When hardware insufficient even for fallback tier, pulse operates in graceful degradation mode: hard signals (errors, exceptions, fatal logs) continue surfacing with explicit notice that intelligent interpretation is unavailable. **Errors are reported, intelligence is not faked.**

**Long-term flywheel aspiration:**

After traction — opt-in telemetry contribution flow. Users who agree send sanitized telemetry (PII stripped locally) to a community dataset. Periodic re-training of a Nemotron-class model on this dataset. Better-tuned weights ship back to pulse via update.

This is the **flywheel**: usage → data → better model → better experience → more users → more data.

No one is doing this currently. Existing observability companies own domain data privately. Open community-trained model on real-world OTel data — uncovered ground.

**Honest about requirements** for the flywheel:
- Aggregation infrastructure (cloud cost)
- Storage (terabytes scale)
- Training compute (H100s or equivalent)
- ML expertise for curating dataset
- Trust (privacy-strict sanitization, opt-in mandatory, transparent process)

All of this **requires funding** or partnership (NVIDIA Inception, Hugging Face, Mozilla Foundation, NLnet — all realistic targets but conditional on traction proof).

**Corpus as foundation:** every incident pulse handles locally is preserved in encrypted persistent corpus — trigger context, model interpretation, user feedback, resolution outcome. This local corpus is what makes pulse smarter over time per-project. It's also what makes opt-in community export meaningful when that flywheel activates — the export format is the same shape as what pulse already records locally, just sanitized and consented for upload.

---

## How they reinforce each other

```
                    ┌──────────────────────┐
                    │   Pulse the model    │
                    │ (Local primary +     │
                    │  fallback tiers;     │
                    │  community-trained   │
                    │  Nemotron long-term) │
                    └──────────┬───────────┘
                               │ ships weights
                               ▼
                    ┌──────────────────────┐
                    │ Pulse the application│
                    │  (ambient companion  │
                    │  + corpus memory)    │
                    └──┬────────────────┬──┘
                       │ MCP server     │ three surfaces
                       │ (optional)     │ (widget/counter/report)
                       ▼                ▼
                    ┌──────────────────────┐
                    │  Pulse the skill     │
                    │ (deep instrumentation│
                    │  + agent integration)│
                    └──────────┬───────────┘
                               │ instrumented code
                               ▼
                    ┌──────────────────────┐
                    │   User's project     │
                    │  (instrumented OTel) │
                    └──────────┬───────────┘
                               │ telemetry stream
                               ▼ (back to application)
                       ┌───────────────┐
                       │ opt-in only:  │
                       │ corpus export │
                       │ to community  │
                       │ training data │
                       └───────────────┘
```

Each part is **redundant without the others**:
- Application without skill = friction onboarding, audience can't set up
- Skill without application = nowhere for telemetry to go
- Application without model = generic statistical floor, not AI-native (still useful via hard signals, but missing the diagnostic depth)
- Model without application = just weights without a deployment surface

All three together = the first cohesive AI-native local-first observability system.

---

## Family context — pulse as first of andromeda

Pulse is the **first product** in the line of andromeda companions. The pipeline (`andromeda` skills set) generates these tools. Pulse demonstrates what the pipeline can do.

**Family DNA** shared across products:
- Ambient companion philosophy (not active tools)
- Local-first execution
- Skill-based onboarding and agent integration
- AI-native intelligence (local model + optional cloud)
- Open source Apache 2.0 + optional cloud tier
- Visual / voice identity (calm-confident, terse-glanceable, space-age palette, signature pulse element)
- **Three-surface communication pattern** — ambient state, passive aggregation, deliberate-access detail (new in v3 — established by pulse, expected to carry to future family members)

**Planned next member**: `andromeda-security` — security companion for shift-left vulnerability awareness. Same philosophy, different domain. Same brand DNA, different signature element (likely).

**Timing**: security does NOT begin until pulse v1.0 proves the model. If pulse doesn't resonate — fix pulse before expansion. Family approach is worthwhile only if first member is proven.

**Long-term**: the pipeline (`andromeda` itself) may eventually become a product. Other developers can use the pipeline to generate their own AI-native tools. Andromeda becomes a platform. This is **far future** speculation but architecture choices now should support it (modularity, reusability, not tightly coupled to specific products).

**External messaging**: at public launch of pulse, **don't** advertise family aspirations heavily. Looks scope-creepy for solo dev. Vision document contains family context (internal anchor) but external messaging stays pulse-focused. Family announcement comes at v1.0 milestone when pulse traction supports the claim.

---

## License & sustainability

**Decided framework**:

- **Apache 2.0** for pulse application + skill — maximum adoption, patent grant, standard for Rust ecosystem, vibe-coder friendly
- **Trademark registration** on "andromeda-pulse" name — protects branding even though code is freely forkable. ~$300-700 cost, real defense against branding hijack
- **Optional cloud tier**, pay-per-use only, no subscriptions. Users can use their own API key (Claude / OpenAI / Gemini) or pay-per-use through pulse-hosted endpoint
- **No trial periods**. No licensing flips planned. Stable commitment to OSS philosophy
- **Funding strategy**: donations (GitHub Sponsors / Open Collective) → cloud tier revenue (when launched) → grants (NLnet / Sovereign Tech / Mozilla once traction is proven) → corporate sponsorship (NVIDIA / observability companies for data flywheel infrastructure)

**Cloud tier scope** (when / if built):
- Hosted endpoint for cloud LLM escalation without requiring user's API key
- Maybe: aggregation infrastructure for opt-in telemetry contributions
- Maybe: priority support / official builds / signed plugins

**What cloud tier WILL NOT be**:
- Multi-tenant SaaS observability hosting
- Replacement for local-first nature
- Required for basic functionality
- Subscription-based (only pay-per-use)

**Trademark policy** philosophy: anyone forks code, anyone modifies, anyone distributes. Anyone calls a fork "andromeda-pulse" — trademark violation. Brand strength keeps users on official version even if technical forks exist.

---

## Roadmap shape

Reshaped from v2 to reflect actual v0.2.0 scope expansion. The original v0.2.0 "skill onboarding milestone" framing was optimistic — v0.2.0 turned out to be a more foundational redesign milestone that absorbs much of original v0.3.0 scope.

**v0.1.0 — minimal viable companion (SHIPPED, baseline at commit `de35e82`)**
- Application works locally with standard OTel ingestion
- Compact widget always-visible (renders synthetic data until v0.2.0)
- Full view focused incident response surface
- MCP server functional with basic tools
- Manual OTel setup in user applications through docs
- Open source Apache 2.0
- Three-platform packages

**v0.2.0 — foundational redesign and AI-native completion (current target)**

Defined formally in `pulse-capability-spec.md` v2 (60 P-XXX capabilities) and implemented per `pulse-v0_2_0-route.md` (33 chunks). Headline deliverables:

- Widget renders real data (no more synthetic — original v0.1.0 widget rendered fake values)
- Three-surface communication architecture: ambient widget + findings counter+dropdown + diagnostic Report
- Full distillation pipeline: 6 layers from OTLP ingestion to LLM interpretation
- Local LLM integration with primary (7-13B / 16GB VRAM) and fallback (3-4B / 6GB VRAM) tiers
- Hardware profile awareness with profile-dependent SLOs
- Algorithmic attention cues as model input (rules don't bypass model)
- Three-tier triggering priority (hard signal <5s, medium cue <20s, baseline <90s)
- Encrypted SQLite corpus persistence — incidents, baselines, pipeline metrics, service registry
- Diagnostic Report with ranked hypotheses, investigation steps, project context grounding
- Three equal-tier output channels: in-app render, copy markdown, MCP delivery (optional)
- Service identity lifecycle (seven states with configurable boundaries)
- Configuration hot reload with prospective threshold application
- Pipeline self-observability through Settings → Diagnostics view
- No interrupting notifications by default

**v0.3.0 — skill onboarding and expanded reach**
- Claude Code skill v1 — detection / decision / application / verification phases
- Top 5 runtimes supported (Node.js, Python, Go, Rust, Java)
- Skill ships bundled with pulse application install
- Continuous mode (skill stays available after initial setup)
- README quality, plugin authoring guide, MCP integration guide
- First-run UX through skill (project detection + auto setup offering)
- Cursor agents support (skill portability)

**v0.4.0 — extended retention and history**
- Disk ring buffer (hour+ retention) for analyzing past anomalies
- Comparison view / time-travel through corpus history
- Saved queries / saved views per workspace
- Background reflection cadence producing trend-level insights (if not already in v0.2.0)

**v1.0.0 — public launch ready**
- All v0.x stable
- Plugin signing minimum (SHA verification)
- Theme / customization (palette presets within space-age constraint)
- Multi-platform package quality (AUR, Flatpak, .rpm)
- Honest comparison docs vs alternatives
- Public launch coordination (targeted communities, not HN front-page push)

**v1.5.0 — flywheel preparation**
- Opt-in telemetry contribution flow (corpus export already exists as v0.2.0 capability P-046, but this milestone adds upload infrastructure)
- Local sanitization layer audit and hardening
- Aggregation infrastructure (requires funding)
- Per-attribute consent UX

**v2.0.0 — first community model**
- Community-trained Nemotron-class model shipped via update
- Periodic retraining cycle established
- Open dataset published on Hugging Face
- "First public AI-native observability dataset & model"

**Beyond v2.0**: scope additions per traction signals. Possible directions: streaming/reactive dashboards via differential dataflow, billion-row interactive for massive log files, GPU-calc for aggregations, plugin marketplace, internationalization. All conditional on community signals — not commitments.

---

## What pulse is and what pulse is NOT

### Pulse is

- Ambient debug companion for vibe-coders building side projects
- Local-first, privacy-strict, no data leaves the machine without explicit consent
- AI-native — proactive triage, structured digests for LLMs, MCP-optional integration
- Skill-based onboarding — agent sets up observability, user doesn't configure by hand
- First in andromeda family of developer companions
- Open source Apache 2.0, free forever, optional cloud tier for convenience

### Pulse is NOT

- Production-grade APM (Datadog / New Relic / Honeycomb territory)
- Multi-tenant SaaS observability hosting
- Team collaboration / shared dashboards platform
- SLO management / error budget / incident management workflow tool
- Infrastructure monitoring (CPU / memory / network / disk metrics)
- Distributed cluster trace aggregation (multi-machine scale)
- Code profiler / flame graph deep-dive tool
- Log aggregation for compliance / audit purposes
- Alert delivery / PagerDuty replacement / on-call management
- Cost-monitoring / cloud spend tool
- Subscription-based commercial product
- Cloud-first product with local fallback
- MCP-coupled tool that breaks when MCP isn't configured

### Pulse takes pride in

- Saying nothing when nothing is wrong (silence is a feature)
- Never interrupting flow (no OS notifications by default — visual signals only)
- One-line incident summaries, not paragraphs of metrics
- Honest LLM uncertainty ("I'm not sure which caused this") not fabricated certainty
- Honest about hardware (fallback tier annotated, CPU inference acknowledged as slower, no pretense)
- Honest about pipeline health (Diagnostics view exposes operational state, no hidden degradation)
- Calm even on errors — quiet drama, not frantic alarms
- Working perfectly invisible in peripheral vision
- Remembering across sessions per-workspace, getting smarter project-specific over time

### Pulse explicitly rejects

- Dashboard mindset — no general-purpose dashboards
- Chat-as-primary-interface — vibe-coders don't know what to ask
- Live throughput counters in widget chrome — metric noise against ambient confidence
- Incident cards embedded in widget — violates ambient surface (use counter+dropdown instead)
- OS notifications / modals / toasts by default — calm-by-default invariant
- Glassmorphism aesthetic for vibe — translucency only when functional
- Dark patterns in onboarding — no progress bars to hold attention, no nag prompts
- Notification fatigue — strict thresholds for autonomous notifications, snooze without drama
- Vendor lock-in — open weights, open API, open dataset when models will be trained
- Faked intelligence when hardware insufficient — reduced tier annotated, degraded mode surfaced honestly

---

## Visual & voice identity

(Most of this is well expressed in the design plan — here just a product-level summary)

**Voice**: calm-confident, terse-glanceable. "Quiet drama" in creative directives. Speaks like trusted colleague, not assistant. Says less rather than more. Acknowledges uncertainty when it exists.

**Visual signature**: Halo Pulse as central living element. Service constellation as glance pattern. Findings counter as compact circular marker below widget when there's unread state. Opaque dark surface (Stellar Black + Cosmic Indigo edges), space-age palette (Earth Blue / Alert Burgundy for severity shift). IBM Plex Sans for status, Plex Mono for metric chips.

**Anti-aesthetics** (already in design plan but worth repeating):
- No gradient bombast
- No glassmorphism for vibes
- No retro CRT green
- No gym-bro neon (Datadog territory)
- No enterprise blue-grey (Honeycomb territory)

**Brand consistency for family**: andromeda-security and future members maintain same voice / palette / dark surface / signature pulse-style element (different specifics per domain).

---

## Success criteria

Per phase. Honest signals, not vanity metrics.

**v0.1.0 success** (already achieved at commit `de35e82`):
- Author uses pulse daily for debugging own projects (including the pipeline itself)
- Three-platform install works on clean machines
- Manual OTel setup process completes successfully for at least 3 different runtimes
- Zero crashes / OOMs in daily use over 30 days

**v0.2.0 success** (current target):
- Widget renders real data, no synthetic fallback in shipping build
- Three-surface architecture (widget + counter+dropdown + Report) ships and feels coherent
- Local LLM integration works on author's primary development machine (M-series Mac and / or GPU desktop)
- Fallback tier validated on at least one hardware-constrained configuration
- All 60 P-XXX capabilities verified via Conductor scenarios
- Author finds incident interpretation genuinely useful (not lipstick on rule-based detection)
- Performance budgets hold under typical workload (<5% CPU steady state, <500MB memory, end-to-end SLOs per profile)

**v0.3.0 success** (skill milestone):
- Skill correctly sets up pulse in 80%+ of projects on supported runtimes
- "Set up pulse" works in Claude Code single-shot for majority of cases
- 50+ active community installations (via download metrics or opt-in heartbeat)
- 5+ external contributors with merged PRs

**v1.0.0 success**:
- 500+ active installations
- Sustained donations or grants covering operational costs (infrastructure for cloud tier when launched, domain, trademark renewals)
- Pulse cited in vibe-coder community spaces (Twitter / Discord / Reddit) as a recommended tool
- Author still personally uses pulse without friction

**v2.0.0 success** (flywheel milestone):
- 5000+ installations
- 500+ opt-in telemetry contributors
- First community-trained model shipped successfully
- Hugging Face dataset published with positive community reception
- Funding secured (grant / corporate sponsorship) for ongoing infrastructure

**Family success** (andromeda level):
- Andromeda-security launched and gaining traction independent of pulse
- Brand recognition: vibe-coders associate "andromeda" with specific quality / philosophy
- Community speaks of "andromeda tools" as a category, not individual products

**Failure modes** to watch:
- Notification fatigue from poor severity calibration → silent uninstall (mitigated by no-interrupt-by-default invariant + LLM-driven calibration)
- Skill breaking projects → trust collapse
- Performance issues in always-visible widget → uninstall as annoyance
- Vibe-coder audience doesn't resonate → wrong product for wrong audience
- Local model quality plateau → AI-native value prop dilutes (mitigated by fallback tier explicit, hardware-honesty contract)
- Pipeline self-degradation invisible to user → "pulse feels off lately" without explanation (mitigated by Diagnostics view)

---

## Honest open challenges

Things not resolved which need attention as the project progresses.

**1. LLM runtime ecosystem maturity (v0.2.0 blocking decision)**
mistralrs vs candle vs other options. Choice affects API patterns, error handling, dependency tree. Decision must resolve before L4 implementation chunks start. See `pulse-v0_2_0-route.md` Pre-D1.

**2. Drain implementation scope (v0.2.0 blocking decision)**
Rust port of Drain3 estimated at 1000-1500 LOC. Validate via spike before committing implementation order. See `pulse-v0_2_0-route.md` Pre-D2.

**3. Skill maintenance burden (v0.3.0+)**
OTel SDK ecosystem evolves. Each supported runtime requires updating skill knowledge as SDK API changes. Sustainable maintenance approach is unclear — automated tests against SDK versions? Community contributions per runtime? TBD.

**4. Privacy for opt-in telemetry contribution (v1.5.0+)**
Most sensitive part of the future flywheel. Sanitization rigor required is extreme. Local sanitization mandatory (sanitize-then-maybe-upload, not upload-then-sanitize). Differential privacy techniques are possible. Community audit of the process for trust. Per-attribute consent UX. All of this is unresolved. Foundation already laid in v0.2.0 capability P-046 (export for community training) but upload-side infrastructure deferred.

**5. Legal structure**
Currently a personal project. If cloud tier launches with revenue → entity structure (LLC) needed. If eventually company aspirations → migrate copyright to entity in advance. Decision is not critical day 1 but needed **before** public release.

**6. Backend cost for flywheel infrastructure**
Aggregation server + training compute = real money. Funding strategy depends on traction. Don't precommit to free infrastructure until traction is proven.

**7. "Skill makes the agent work hard" calibration (v0.3.0+)**
How deep should the skill go into user code without breaking things. v0.3.0 — collaborative depth (suggest, user approves). v0.4.0+ — possibly more autonomous. Calibration learned from real usage.

**8. Community management bandwidth**
Solo maintainer + active community require time investment. Response speed matters for community building. Plan: prioritize first 1-3 months after launch for intensive issue response. Beyond — sustainable cadence to be calibrated.

**9. Multi-platform skill portability**
v0.3.0 = Claude Code. v0.4.0 = Cursor agents. Beyond = ?? Industry hasn't standardized agent skill format. Pulse skill may need rewrites or adapters per platform. Cost of multi-platform support is unclear.

**10. Family timing**
Pulse v1.0 must succeed before andromeda-security starts. Definition of "succeed enough to expand" is unclear. Probably: 6 months sustained traction, positive community signal, author's own use frictionless. Calibrate when closer.

**11. Model evolution and corpus continuity (v1.0+)**
As local models improve year-over-year, pulse should benefit. But interpretation prompt + JSON schema may evolve. Corpus records carry `prompt_version` and `schema_version` for provenance; old records remain queryable but may differ stylistically. Acceptable trade-off currently; revisit if it becomes confusing to users.

**Resolved since v2** (moved from open to decided):

- ~~Model size assumption (1B-class)~~ — resolved: 7-13B primary + 3-4B fallback with 16GB / 6GB VRAM envelopes per capability P-053, P-054
- ~~Notification calibration approach~~ — resolved: no OS notifications by default, all severity visual through halo + counter (P-030 invariant)
- ~~MCP coupling concern~~ — resolved: MCP is one of three equal-tier output channels, never required (P-039, P-040)
- ~~Surface architecture ambiguity~~ — resolved: three distinct surfaces with no overlapping responsibilities (P-024 through P-031)

---

## Non-negotiable principles

Decision guides. When in doubt, return here.

1. **Local-first**. Data stays on the user's machine unless explicit consent.
2. **Calm-by-default**. Silence when nothing is wrong is a feature. No interrupting notifications by default — visual signals only. Aggressive notification modes opt-in for users who want them.
3. **Honest about uncertainty**. Hedge wording when model is unsure. No fabricated certainty.
4. **Honest about hardware**. Fallback tier annotated, CPU inference acknowledged as slower, degraded mode surfaced — no pretense of capability the hardware doesn't have.
5. **Honest about pipeline health**. Diagnostics view exposes operational state. Users can see "pulse feels off because LLM in degraded mode" rather than guessing.
6. **Vibe-coder fit**. Every decision passes "would my target user want / understand this?". If unclear — probably no.
7. **Skill-first onboarding**. Don't require user to read docs to set up. Agent does it.
8. **No dark patterns**. No nag prompts, no progress bars to hold attention, no friction to disable.
9. **Trademark protected, code free**. Apache 2.0 forever, brand defended.
10. **No subscription**. Pay-per-use cloud tier only.
11. **Family DNA stays consistent**. New members maintain shared philosophy, voice, visual language.
12. **Author uses it**. If author doesn't use pulse daily — something is not right, fix it before pushing further.
13. **Three surfaces, never mixed**. Widget stays ambient. Counter aggregates passively. Report is deliberate-access. Mixing roles between surfaces breaks the companion model.
14. **MCP is convenience, never coupling**. Pulse fully functional without MCP. Same content quality across in-app render, clipboard, MCP delivery.

---

## Notes for future self

- **Don't confuse pulse with pipeline**. Pulse — product. Pipeline — meta-product that generates pulse plus future products. Different aspirations, different timing, different decisions.
- **Re-licensing escape hatch closed**. Apache 2.0 forever. If you ever entertain thoughts about license flip — review trademark + cloud tier + family approach as better long-term protection.
- **Don't over-promise family**. Pulse first. Security after pulse v1.0 proven. Andromeda as platform after both proven. One step at a time.
- **Notifications calibration is product**. Spend disproportionate effort on it. Bad notifications = uninstall = death. Default of "no OS notifications" simplifies this dramatically — calibration mostly about halo response timing and severity classification, not "should we notify."
- **Skill development is critically important** but deferred to v0.3.0. Skill quality determines onboarding success determines adoption. Not an afterthought — central pillar. v0.2.0 builds the substrate the skill will install onto.
- **Resist feature drift**. Vibe-coders hobby market doesn't fund enterprise feature creep. If someone asks for SLO management / on-call rotations / compliance reports — wrong audience pulling. Politely decline.
- **Public launch timing**. Pulse v0.1.0 doesn't announce to the world. v0.2.0 may not either if not stable yet. Quiet ramp through v0.3.0 / v0.4.0, 10-50 active users initially, learn, refine. Loud announcement at v1.0 milestone when traction supports the claim.
- **The three foundational documents are reality**. When in doubt about specifics, check `pulse-capability-spec.md`, `pulse-distillation-architecture.md`, `pulse-v0_2_0-route.md`. This vision is the why; those are the what and how.

---

## What dropped from earlier versions

Documented so we don't return to them by mistake.

**Dropped in v2 (kept dropped):**
- **Compact widget with live throughput counter** — metric noise against ambient confidence. Removed.
- **Tabs for browsing data in full view** — dashboard mindset, conflict with focused incident response. Removed.
- **Plugin marketplace ranking system** — premature, requires community scale that probably won't be reached in the foreseeable future. Pushed to "maybe far future" if organic demand appears.
- **Subscription model / commercial trial** — wrong audience fit, kills community possibility. Resolved via Apache 2.0 + cloud tier pay-per-use.
- **Aggressive comparison with Datadog / Honeycomb in marketing** — wrong audience. Pulse doesn't compete with production observability tools. Different category. Comparison docs should be **honest about boundaries**, not aggressive positioning.
- **Multi-machine cluster aggregation** — conflict with local-first principle. Out of scope permanently.
- **Replay mode / session recording** — interesting but scope creep for core mission. Maybe far future if organic demand.

**Newly dropped in v3:**
- **Incident card slot embedded in widget** — violates ambient surface invariant. Three-surface architecture replaces it: widget stays pure ambient, counter+dropdown handles aggregation, Report handles diagnostic detail.
- **OS notifications at Autonomous severity by default** — original v2 framing said "notification only at autonomous severity." Strengthened to no interrupting notifications by default, all severity visual through halo + counter. Aggressive notification modes opt-in.
- **Rule-based severity classifier as primary mechanism** — original implementation plan had rule-based table deciding severity directly. Architectural refinement determined that LLM should own severity decisions with algorithmic rules feeding attention cues. Rule-based survives only as deterministic floor when LLM unavailable (graceful degradation), not as parallel classifier.
- **MCP-first integration framing** — softened to MCP-optional. MCP is one delivery channel among three equal-tier options. Pulse never requires MCP.
- **Small (1B-class) model assumption** — replaced with 7-13B primary + 3-4B fallback tier architecture. 1B-class models proved insufficient for the diagnostic quality pulse claims (ranked hypotheses, investigation steps, project grounding).

---

## Reference documents

Three implementation documents form the foundational baseline against which all v0.2.0 work proceeds:

1. **`pulse-capability-spec.md`** v2 — Formal product contract. 60 P-XXX capabilities across 11 categories. Each capability has stable ID, formal claim, observable signal, Conductor verification path, scope boundary. Marketing copy and documentation derive from here. Conductor scenarios verify each P-XXX. Mini-route chunks enable specific P-XXX claims.

2. **`pulse-distillation-architecture.md`** v3 — Telemetry pipeline design. Six layers (L0 ring buffer, L1 streaming distillation, L2 attention cue detectors, L3 digest assembler, L4 LLM interpretation, L5 surface and persist) plus L6 self-observability cross-cutting. SQL templates, performance budgets, schema additions, blocking decisions, testability strategy.

3. **`pulse-v0_2_0-route.md`** v2 — Execution plan. 33 implementation chunks (#57-#89) plus 2 blocking decisions (Pre-D1 LLM runtime choice, Pre-D2 Drain spike). Each chunk has dependencies, capability mapping, distillation layer mapping, specialist plan touches. Phase structure mirrors capability spec category structure.

When this vision document and the three implementation documents disagree, the implementation documents are correct — they reflect later, more carefully scrutinized decisions. This vision document is updated to align when divergence is noticed.

---

_Living document. Update when product framing evolves. Reference at decision points. Use as input for targeted regeneration of the pipeline when we reach Step 5._

---

## Changelog

### v3 — 2026-05-14

Revision aligning vision with capability spec v2, distillation architecture v3, and route v2. Surgical updates preserving voice and intentional choices from v2, factual content updated where architectural refinement diverged from earlier assumptions.

**Updated sections:**

- **"What changed vs v2"** — new section opening document, replacing v2's "What changed vs the first version" since that retrospect is no longer the primary lens
- **Three pillars / Pillar 1** — three-surface architecture explicit (widget ambient, counter+dropdown passive aggregator, Report deliberate-access). Removed "incident card appears inside" framing. Added no-interrupting-notifications invariant.
- **Three pillars / Pillar 2** — minor clarification that MCP integration is optional accelerator, not requirement
- **Three pillars / Pillar 3** — model size assumption updated from 1B-class to 7-13B primary + 3-4B fallback. Hardware-honesty contract added. Corpus persistence positioned as foundation for eventual flywheel.
- **How they reinforce each other diagram** — updated to show three surfaces, corpus memory, optional MCP positioning
- **Family context** — three-surface communication pattern added to shared family DNA
- **Roadmap shape** — significantly restructured. v0.2.0 scope expanded to absorb much of original v0.3.0 work (LLM interpretation, corpus, log digest pipeline via Drain). Original v0.2.0 skill onboarding milestone shifted to v0.3.0.
- **What pulse is and what pulse is NOT** — MCP framing softened from "MCP-first" to "MCP-optional." Added explicit rejections for incident cards in widget, OS notifications by default, faked intelligence on insufficient hardware. Added "MCP-coupled" to "is NOT" list.
- **"Pulse takes pride in"** — added items about honesty (uncertainty, hardware, pipeline health) and corpus memory persistence
- **Visual & voice identity** — added findings counter as compact circular marker to visual signature
- **Success criteria / v0.2.0** — rewritten to reflect actual v0.2.0 scope (60 P-XXX capability verification, three-surface architecture, LLM integration tiers, etc.)
- **Honest open challenges** — added LLM runtime choice and Drain spike as v0.2.0 blocking decisions. Added model evolution and corpus continuity concern. Added "Resolved since v2" subsection documenting what's no longer open.
- **Non-negotiable principles** — strengthened calm-by-default (#2). Added honest-about-hardware (#4) and honest-about-pipeline-health (#5). Added three-surfaces-never-mixed (#13). Added MCP-convenience-never-coupling (#14). Total principles: 12 → 14.
- **Notes for future self** — added note about three foundational documents as implementation reality
- **What dropped** — added v3 newly-dropped items: incident card in widget, OS notifications by default, rule-based severity classifier as primary, MCP-first framing, 1B-class model assumption

**New sections:**

- **Reference documents** — explicit cross-reference to capability spec v2, distillation architecture v3, route v2 as canonical implementation specifications

**Preserved unchanged from v2:**

- Product soul paragraph (foundational framing)
- The shift — why now (still accurate)
- License & sustainability (unchanged decisions)
- Family context core philosophy
- Visual / voice identity core (just additions for new surface element)
- Failure modes
- Family timing and external messaging philosophy

### v2 — earlier 2026

Major restructuring from initial backlog. Product framing shifted from "OTel viewer with good features" to "ambient companion." Three pillars introduced (application + skill + model). Family product strategy established. Apache 2.0 + trademark + pay-per-use cloud tier locked in. Most current vision document structure originates here.

### v1 — earlier 2026

Initial backlog. Treated pulse as monitoring tool with feature roadmap. Largely superseded by v2 reframing.
