# Scope — The L4 probe reproduces the canary-history miss

**Marker:** `2026-10-07-l4-probe-reproduces-the-canary-history-miss` · version `andromeda-pulse-0.3.0` · Epoch 4 — Polish & ship: verification
**Working entry (`working-route.md:180`):** "The L4 probe reproduces the canary-history miss — the baseline prompt fails
the service bar on a probe shape built toward the third drive's digest; then a remedy is measured against that shape"

## Intent
The sentence shipped at `f70be92` (prompt `v2.6`) obliges the first hypothesis to name the triggering cue's `scope_id`.
Conductor's fifth series followed it in two drives and not in the third, the same drive that missed in the fourth
series. The dev probe cannot say why: its baseline arm passes every sibling shape it has, so it holds no known-positive
for the miss. This chunk first makes the probe reproduce the miss (a shape on which the prompt as shipped fails the
service bar), reads which property of the corpus block carries it, and only then measures a remedy against that shape.

**Revised 2026-10-07, at the plan revision (inputs#I13, inputs#I14).** The first reading ran once and read NOT
REPRODUCED (`evidence/reading-reproduction.md`): no synthetic shape is a known-positive. The founder then ruled that
the builder must be able to read the digest itself (inputs#I14 §1). The operator's wrapper at the model-binary handle
captures the real prompts, with no key read (the vehicle the founder picked at 13:49 local, inputs#I15), and the
known-positive becomes the miss drive's captured prompt replayed through the probe. The order is unchanged: a
reproduction first, a remedy measured against it second.

## What the chunk builds
- **A known-positive first.** [premise-corrected: the eight synthetic shapes did not reproduce. The shipped arm read
  `both` 159 of 160 over S9-S16, one `signal_only` on S16, verdict NOT REPRODUCED (`evidence/reading-reproduction.md`;
  re-derived at the revision from `runs.json`, sha256 `02921811…ff67`: 200 rows, S16 19 `both` and 1 `signal_only`,
  every other shape 20 `both`).] The known-positive is the miss drive's real creating prompt, captured by the operator's
  wrapper and replayed verbatim (the revision's bullets below). The shapes S9-S16 stay in the probe as the measured negative. They
  were built toward what the third drive's digest plausibly held: up to five corpus lines for the sibling scope, mixed
  kinds (one retry storm, several error-rate spikes), in recency order, the corpus block varied ALONE against a
  sibling-shape control.
  - The control is the existing sibling shapes (S7 without a corpus block, S8 with two retry-storm lines); the new
    shapes differ from them in the corpus block only. Verified at P3: a shape is `{id, services, cue, corpus_matches}`
    (`pulse-app/examples/l4_decision_probe.rs:237-242`) and `prepare` renders it through the product's
    `render_payload` with that list (`:592-601`), so a shape reusing `sibling_services()` and S7's cue differs in the
    corpus block and nowhere else.
  - The baseline arm is `shipped` (`v2.6`, the product as it stands at HEAD), not `ns`. Verified at P3: `shipped`
    applies no transform (`l4_decision_probe.rs:107`, `:643-645`), HEAD reads `PROMPT_VERSION_PRIMARY = "v2.6"`
    (`crates/interpretation/src/schema.rs:35`), and the fifth series ran `v2.6` (inputs#I2 `:576`).
  - Added at P3, from the third drive's own capture (inputs#I2 `:407-413`, `:422`, `:426`, `:450-451`) and the code:
    two more properties the block can differ in, beside the three the entry names. Scope mix: the selection keeps a
    line for ANY service row in the window, so the digest plausibly held the triggering service's own earlier storms
    beside the sibling's lines. Title form: the two prior titles the capture shows read like hypothesis statements, and
    the missed first hypothesis repeats that form with the sibling's name. The shape set covers both.
- **The reproduction is pre-registered on its own, before any run, apart from the remedy** (inputs#I8).
  - Its pass condition is the baseline arm FAILING the service bar on at least one new shape.
  - A baseline that passes everywhere is a FINDING, reported as such and never a pass: the chunk stops for the founder
    and ships no remedy against nothing (the entry's CONTEXT 1; inputs#I1 §2 item 1; inputs#I8).
  - The grader is the predecessor's, unchanged. Verified at P3: `identifies` at `l4_decision_probe.rs:966`, its
    whole-word rule at `:940` and its retry token at `:956`.
  - [premise-corrected: the predecessor's bar machinery cannot express this reading — `bar_request`
    (`l4_decision_probe.rs:1470-1497`) refuses a run without both the `shipped` and `ns` arms and without an ordinary
    shape, it folds the sibling half over the fixed pair S7 + S8 (`:165`), and its verdict is PASS iff the selection is
    `shipped` (`:1153-1177`).] The reproduction needs a per-shape reading of its own: the predecessor's threshold
    (19 of 20) applied to each new shape alone, with a verdict that reads the baseline's failure as the expected
    reading and a baseline that passes everywhere as a non-pass.
  - Amended at P5 (validation-1, intent-incomplete): the per-shape count is service misses, a parsed first hypothesis
    that does not name the cue's `scope_id`, 2 or more of 20. It is the service half of the bar alone, because that is
    the miss being reproduced; an unparsed generation is counted apart and is never a miss, so a parse failure cannot
    read as a reproduction.
- **Then a remedy, measured against the reproducing shape**, with the regression guard on the ordinary shapes. The
  candidates are the phase's: cap or reorder the corpus block, restate the scope obligation after it, exclude other
  scopes' history. [premise-corrected: no shape reproduces (`evidence/reading-reproduction.md`), so the remedy is
  measured against the replayed miss prompt, the candidates applied as edits of its corpus block (inputs#I14 §3). The
  rules below stand as approved.]
  - The remedy has its own pre-registration, written after the first reading and before its own runs (inputs#I8): it
    names the reproducing shape, the bar on it, and the guard on the ordinary and existing sibling shapes.
  - Decided by the operator at the P5 review (inputs#I10):
    - The shapes are built and the reproduction reading is launched BEFORE the candidate arms are built. The remedy
      run measures `shipped` again as its known-positive, so its validity does not rest on one binary, and a NOT
      REPRODUCED verdict costs no arm work and reaches the founder sooner, in GPU daylight.
    - The remedy pre-registration sizes n per reproducing shape from the first reading's measured miss rate, so the
      baseline is expected to miss at least 5 times there, and says so. A candidate at 19 of 20 beside a baseline at
      18 of 20 is noise.
    - Standing as planned: the shapes, the threshold of 2 misses of 20, the candidate order, no combinations.
    - At the revision (inputs#I14 §3): the first reading ran under the build-nothing-first rule and cost no arm work.
      For the second reading the arms are built before the capture lands, as dry-runnable transformations, because
      that build needs no GPU and no capture.
  - Decided by the operator at the review's second round (inputs#I11): in the remedy run the known-positive holds on a
    shape only when `shipped` reads at least the allowance plus 2 misses there (3 or more at n = 20, 25 and 34; 4 or
    more at n = 50). One miss over a passing candidate is a margin the last reading could not tell from noise. A
    remedy reading that does not fit the daylight waits for the next daytime go.
  - [premise-corrected: where each candidate lands, read at P3 — none of the four needs prompt-template text.]
    - Restate after the block: the prompt's own `# Corpus Retrieval` section is never rendered in production (both
      builder calls pass an empty string, `pulse-app/src/inference_runtime.rs:423`, `:429`); the corpus block reaches
      the model inside the digest (`render_payload`, `crates/triage/src/digest/assembler.rs:707-713`). A restatement
      "after it" is therefore a static digest line. A prompt-side restatement would collide with the pin that holds
      the scope obligation exactly once per tier (`crates/interpretation/src/prompt.rs:639-663`).
    - Cap: `DIGEST_CORPUS_RETRIEVAL_LIMIT` (`crates/triage/src/digest/mod.rs:67`) is also the limit of the report's
      "Previously Seen" list (`pulse-app/src/incidents_router.rs:467`) and of the MCP sidecar
      (`crates/mcp-server/src/tools.rs:424`). A cap is a digest-own limit; that constant does not move.
    - Exclude other scopes: `select_corpus_matches` (`crates/triage/src/digest/retrieval.rs:88`) has one production
      caller (`assemble`, `assembler.rs:299`), which passes every service row as a scope (`:287-288`). Narrowing is
      passing the triggering cue's `scope_id` alone when a cue is present; the fingerprint arm is untouched.
    - Reorder: a change of the sort key inside the same function (`retrieval.rs:103`).
    The prompt lineage (`v2.6` / `v1.5-fallback` / `v1.5-reflection`) moves only if the selected remedy changes
    prompt-template text; a digest-side remedy leaves the three labels and their pins where they are.
- **The real-model readings are legs that ask the operator for the go before each launch** (inputs#I8): the GPU is
  open today on the founder's 07:42 word and closed tonight, so the reproduction reading is planned to finish in
  daylight. Conductor is mid-wrap and binds nothing now (inputs#I8).
  - Sized at P3 from the predecessor's reading: 240 generations took 1492 s, 6.2 s each
    (`chunks/2026-10-06-l4-first-hypothesis-names-the-triggering-service/evidence/reading.md:126`). The legs source
    the env file of inputs#I9. Measured by the first reading: 200 generations in 1305 s, 6.5 s each
    (`evidence/reading-reproduction.md`).
  - At the revision (inputs#I14 §4): the GPU is shared with Conductor's capture run. The overseer is asked for the go
    before each launch, and a reading that does not fit the daylight waits for the next daytime go.

### Added by the revision (inputs#I13, inputs#I14), each closed at the revision's P3
- **The replay.** The probe gains an input that reads one captured model invocation from a path given at run time and
  replays its prompt bytes verbatim through the product's argv and the committed grammar. The file is never in the
  tree. Verified at the revision:
  - The prompt is the last argv operand, after `-p` (`pulse-app/src/llamacli_inference.rs:491-492`), as the relay
    cites.
  - The product's one spawn site passes `DEFAULT_MAX_TOKENS` (`llamacli_inference.rs:850-853`), and the probe's
    `compose_argv` builds through the same `build_llama_cli_args` with the same value (`l4_decision_probe.rs:1652-1659`).
    So a replayed prompt goes through the argv the product used, apart from the model, grammar-file and prompt operands.
  - No product file differs between `f70be92`, the sha the capture runs against, and HEAD `48714f0`
    (`git diff --stat f70be92 HEAD -- crates pulse-app xtask Cargo.toml Cargo.lock`: no output). The committed GBNF and
    the prompt template are therefore the ones the captured run used.
- **The second reproduction reading**, pre-registered on its own before it runs: the miss drive's creating prompt at
  n = 20, with a passing drive's creating prompt as its control; the threshold of 2 misses of 20 stands (inputs#I14 §3).
  - REPRODUCED: the difference between the miss prompt and the passing drive's, and between it and S14 and S16, is
    read section by section, and a remedy is measured.
  - NOT REPRODUCED while the live drive missed: a second finding (the miss is not determined by the prompt bytes).
    The chunk stops and reports; no remedy is guessed.
  - The capture run's third drive does not miss: the overseer decides the next run and the chunk waits.
- **Remedies as transformations of the replayed prompt**, under the rules already approved (inputs#I10, inputs#I11):
  n sized for an expected 5 baseline misses, the known-positive holding only at the allowance plus 2, the guards on S7,
  S8 and S1-S4. The four candidates apply as edits of its corpus block; a further one may appear once the prompt is
  read.
  - Verified at the revision: a rendered corpus line holds the fingerprint, the title, the age and the status, and
    not the incident's scope (`crates/triage/src/digest/retrieval.rs:136-148`). Which lines belong to the triggering
    scope cannot be read off a captured block, so the reorder and exclude edits take that set from outside the text.
- **A durable known-positive, derived** (inputs#I16, the operator's addition at the review's second round: it is what
  the entry's title promises, since the captures are deleted when the chunk closes). After the replay reproduces and
  the section reader says what differs, ONE synthetic shape is derived from that difference, written by the builder
  with no capture text, as S13's titles were. It is read with `shipped` at n = 20 in the same GPU slot as the remedy
  reading, its outcome pre-registered beside the remedy's.
  - It reproduces at 2 of 20 or more: it is committed as the probe's standing known-positive.
  - [premise-corrected: the shape does not join the selection bar. The operator withdrew that part at the review's
    third round, on the plan's own reading that a candidate fixing the real prompt could then fail on the synthetic
    one (inputs#I17).] Selection stays on the real prompt, the known-positive rule and the guards, as approved. The
    shape is read under `shipped` and every candidate, and recorded.
    - The selected candidate reads within 1 not-`both` of 20 on it: it becomes the standing guard for the remedy,
      with the pre-remedy baseline arm as its failing side.
    - The selected candidate does not clear it: it is still committed, as a known-positive the remedy does not cover,
      and the report names that as an open finding.
    - The ground truth for shipping is the real prompt; a synthetic shape that fails for another reason does not
      veto a real fix.
  - It reads clean: the report says the probe keeps no durable positive, and why.
  - It never blocks the remedy.
  - Verified at the review against the probe's own convention: a fix that ships changes what `shipped` composes, and
    the pre-fix composition then lives on as a baseline arm (`ns`, `l4_decision_probe.rs:137-138`, `:986-1001`). So a
    known-positive that is to stay one after a remedy ships needs the same: an arm that composes the corpus block as
    it was before the remedy.
- **Built before the capture lands, CPU only**: the replay input and its loader, the section-difference reader, the
  four block edits as dry-runnable transformations, their pins and mutation checks (inputs#I14 §3).
- **The capture's text never leaves the host.** [premise-corrected: no longer provisional. The founder picked the
  vehicle, "the operator's wrapper", by dialog at 13:49 local, in a question that stated the captures never enter a
  repository; the pass-through at the model-binary handle is the pc overseer's, not Conductor's (inputs#I15).] A
  capture is never committed, pasted into a report or quoted in a card. Committed evidence carries derived facts only:
  a sha256, byte and line counts per section, which sections differ. A shape derived from reading a capture and
  written synthetically is committable. A commit that would carry capture text is a boundary widening: stop and ask
  (inputs#I14 §2).
- **The capture's form** (inputs#I15, measured by the operator on a self-test). Each model invocation is one directory
  named by its start instant, a nanosecond count and the pid, holding `prompt.txt` (the bare `-p` operand, exact
  bytes), `argv.nul` (the argv NUL-terminated, `argv[0]` first, then the operands) and `started-utc`. Verified at the
  review on the self-test capture, as counts only: one directory, the three files, `argv.nul` 4 fields ending in a
  NUL with `llama-cli` first, and the operand after `-p` equal to `prompt.txt` byte for byte (69 bytes). The capture
  run's own directory did not exist yet at that read.
- **The captures are deleted when this chunk closes** (inputs#I15). Nothing may depend on them after the wrap: every
  read of a capture happens at /implement, and what stays is the derived record in `evidence/`.

## Mechanism claims carried by the entry (closed at P3)
- [premise-corrected: measured by the first reading and not carried by the corpus block alone, in the eight
  variations read: count, kind mix, position, fingerprint, title form and scope mix each left the shipped arm at 19 or
  20 of 20 (`evidence/reading-reproduction.md`). At n = 20 this shows 20 generations did not miss, not that a property
  has no influence. S8 had already ruled out one property as sufficient: its newest corpus line is an active sibling
  retry storm, three minutes old, and both arms read 20 of 20 on it
  (`…first-hypothesis-names-the-triggering-service/report.md:207-208`).]
  The entry's text: "hypothesis: the corpus block's count, kind mix or position carries the miss".
- "A covariate, not a cause: the row count rises with the drive ordinal, the app's uptime and the canary's history, and
  no drive varied one without the others." Verified at P3 against the six captures: the count reads 1 / 3 / 6 in both
  series and each drive is one ordinal, one uptime and one history.
- "That number is `candidate_count`, logged BEFORE selection (`crates/triage/src/digest/assembler.rs:298-311`);
  `select_corpus_matches` (`crates/triage/src/digest/retrieval.rs:88`) keeps scope or fingerprint matches, newest
  first, capped at `DIGEST_CORPUS_RETRIEVAL_LIMIT` = 5 (`crates/triage/src/digest/mod.rs:67`), so d3's digest rendered
  at most five corpus lines; how many, and for which scopes, is NOT measured (the digest exists only cell-encrypted)."
  - Verified at HEAD `48714f0`: `candidate_count` at `assembler.rs:298`, logged as `row_count_returned` at `:309`,
    before nothing else reads the selection; the retain, the newest-first sort and the truncate at
    `retrieval.rs:94-104`; the const at `mod.rs:67`; a second `take` of the same limit at render (`assembler.rs:710`).
  - What "scope match" means, read at P3: a candidate is kept when its `scope_id` equals ANY service row of the window
    (`assembler.rs:287-288`, `retrieval.rs:97-100`), by exact string equality. The triggering cue's scope plays no
    part. So a `conductor-canary` incident reaches a `conductor` digest whenever the canary is a service row, which it
    was in all three drives (each narrative cites both services).
- "Conductor's fifth pre-registered `v3-09` series, against `f70be92` (prompt `v2.6`), read NOT MET 2 of 3 on the same
  drive as its fourth (`5f77859`, `v2.5`): rank 1 names `conductor` in d1 and d2 and reads "Active Retry Storm Detected
  on conductor-canary." in d3".
  - Verified at take-up: fifth series rank 1 at inputs#I3 `:421`, inputs#I4 `:424`, inputs#I2 `:426`; fourth series d3
    rank 1 at inputs#I5 `:426`.
- "Each capture's `creating digest corpus retrieval rows` line reads 1 / 3 / 6 for d1 / d2 / d3 in BOTH series, and
  only d3's lists other cue-bearing canary digests, four, all `error_rate_spike`".
  - Verified at take-up: fifth 1 / 3 / 6 at inputs#I3 `:529`, inputs#I4 `:546`, inputs#I2 `:577`; fourth 1 / 3 / 6 at
    inputs#I6 `:531`, inputs#I7 `:553`, inputs#I5 `:573`. The "other cue-bearing digests" line reads 0 in d1 and d2 and
    `4 (error_rate_spike ×4)` in d3, in both series (inputs#I2 `:580`, inputs#I5 `:576`).
- What the d3 capture itself bounds, read at take-up and at P3, not stated by the entry (inputs#I2):
  - It lists seven incidents; six opened before the creating digest, and two of those six carry the scenario
    fingerprint (`:407-413`). Those two are the earlier drives' own `conductor` storms, and the report's "Previously
    Seen" shows their titles, both prefixed `Retry storm:` (`:450-451`).
  - If each of the six carried a `scope_id` naming a service row of that window, the cap applied and the digest
    rendered the newest five. The capture does not show the four non-scenario incidents' scopes or kinds, so this
    stays a bound, not a reading.
  - The newest of the six opened 180 s before the scenario's emission (`:412` beside `:401`), the instant the canary's
    own retry storm surfaced (`:578`). The report's timeline reads "A retry storm was detected on conductor-canary
    3 minutes ago, which is currently active" and names a resolved incident 18 minutes back (`:422`), which is the
    age of the oldest scenario-fingerprint incident (`:408`).
  - The missed first hypothesis, "Active Retry Storm Detected on conductor-canary." (`:426`), repeats the wording of a
    prior incident's title, "Active Retry Storm Detected on Conductor Service" (`:450`), with the sibling's name.
- "the probe today — `S8` (`pulse-app/examples/l4_decision_probe.rs:406`), its one sibling shape with a corpus block,
  carries TWO corpus lines, both retry storms scoped to `conductor-canary` under a `conductor` cue
  (`sibling_corpus_incidents`); at 2026-10-06-l4-first-hypothesis-names-the-triggering-service the then-baseline arm
  (`ns`, `v2.5`) and `shipped` (`v2.6`) both read 20/20 on the sibling shapes S7 + S8".
  - Verified at HEAD `48714f0`: S8 at `:406`, `sibling_corpus_incidents` at `:431-465` (one active line three minutes
    old, one resolved line nine minutes old); the predecessor's `report.md:207-208` reads `shipped` 20/20 and `ns`
    20/20 on the sibling half.
  - Read at P3: both S8 lines carry the cue's own fingerprint (`:438`), and descriptive titles ("calls fail and loop
    back"). In the third drive the sibling's incidents carried a different fingerprint from the cue's (inputs#I2
    `:407-413`). The probe's comment that a sibling-scoped incident reaches a digest "through the fingerprint arm"
    (`:429-430`) is narrower than the code: it reaches it through the scope arm as well.

## Boundaries
- **The service on the `TRIGGER:` line is the founder's boundary call** (CARRY a). The 2026-10-04 kind-label-only
  ruling stands on a security ground (arch §Established Decisions [Fault Identity]). A phase that reaches for it stops
  and asks; it is never answered on a delegated word (inputs#I1 §2 item 1). The probe's `L` and `LI` arms render that
  line and are not remedy candidates here. If the real prompt shows that only the service on that line would fix the
  miss, that is a finding for the founder, not a candidate arm (inputs#I14 §3).
- **The d3 replay from the real digest stays unmeasured** (CARRY b). The creating digest is archive row 45 of the
  fourth series' corpus, cell-encrypted; the credential read is not routed around, and allowing it is the founder's
  alone (`chunks/2026-10-06-l4-first-hypothesis-names-the-triggering-service/evidence/post-hoc-d3-replay.md`). The
  revision does not touch this: what the operator's wrapper captures is a new run's prompts, recorded at the
  model-binary handle as the product hands them over. That archive row stays unread and no key is read, by a tool or otherwise
  (inputs#I14 §2).
- No captured digest content enters the tree: the new shapes are synthetic, built from what the captures and the code
  show, never from a recovered payload. A title seen in a capture is not copied; a synthetic title may take its form.
  The captured prompts are read from a path outside the tree at run time. The probe prints and stores closed labels,
  counts and whether sections differ, and no prompt or model text (inputs#I14 §2).
- Conductor's rule, its sixth series and its version close are Conductor's; this chunk changes nothing in that
  repository. They wait on the sha that ships this chunk.
- No model, sampling, GBNF or llama.cpp change. Fault identity is untouched, and the corpus retrieval's
  `fingerprint_match` arm stays fed: a remedy that excludes other scopes' history narrows the scope arm only. Verified
  at P3 as standing rules (arch §Established Decisions [Fault Identity], [LLM Inference Runtime]).
- Prompt and digest text the chunk adds stays ASCII (argv transport). Verified at P3: the pin over the three composed
  tiers (`crates/interpretation/src/prompt.rs:911-942`) and the digest constants' own note
  (`crates/triage/src/digest/assembler.rs:604-614`).
- No real-model run without the operator's go, and none tonight (inputs#I8). Ports 4317/4318 are shared with
  conductor-builder. Verified at P3: the probe binds neither, opens no corpus and starts no app (0 hits for a listener,
  either port, `Corpus::open`, `observability::init` or `keyring` in its two source files).
- "The Linux boot smoke is deterministic" and the rest of the tail are the next entries, not this one.

## Folded freight (the entry's five blocks)
1. **CONTEXT — founder ruling 2026-10-07 11:49 local** (inputs#I1): reproduce, then fix, then a sixth series → the
   three build bullets and the finding rule above. Head placement is the overseer's, founder-delegated; the founder can
   move it. The real-model readings need the GPU.
2. **CONTEXT — the measured basis** → the mechanism bullets above.
3. **CONTEXT — the probe today** → the last mechanism bullet. The predecessor wrap's handoff item "the probe's missing
   known-positive", left without an owner entry on purpose, is this entry's first half.
4. **CARRY (a)** → the first boundary.
5. **CARRY (b)** → the second boundary.

## Directive (inputs#I8) — folded
- Promote the head entry: done by this promotion.
- The GPU is open today and closed tonight; the reproduction reading finishes in daylight → the fourth build bullet.
- A go is asked before each real-model launch → the fourth build bullet and the last-but-one boundary.
- The reproduction is pre-registered apart from the remedy; the baseline failing is its pass condition; a baseline that
  passes everywhere stops the chunk for the founder → the second build bullet.

## Directive of the revision (inputs#I13) — folded
- Revise the pending chunk's plan; the stop was NOT REPRODUCED → the first build bullet's correction.
- The probe edit in the tree is uncommitted and is kept → the plan names the steps already done and re-enters after
  them; the scope guard still reads against the chunk base `48714f0`.
- The founder's ruling of 13:13 local and the capture by a model-binary shim, no key read → the revision's bullets
  and the second boundary.
- The known-positive becomes the real prompt replayed, and what is built before the capture lands → the revision's
  bullets (inputs#I14 §3).

## The P5 review of the revision (inputs#I15) — folded
- No rule change; the review's points 2 to 6 stand as planned, the guard mechanics included → the plan's
  Pre-registrations as written.
- The vehicle is the operator's wrapper, picked by the founder, and the captures never enter a repository → the
  corrected capture bullet.
- The capture's directory form → the capture's-form bullet and the replay loader.
- The captures are deleted when the chunk closes → the deletion bullet: every capture-reading gate entry is an
  operator leg fired at /implement.
- Second round (inputs#I16): the durable known-positive belongs in this chunk → the derived-shape bullet. Everything
  else stands.
- Third round (inputs#I17): the derived shape does not join the selection bar; it is recorded, and becomes the
  remedy's standing guard or an open finding → the bullet's correction. The baseline arm stands.

## Second fold source — the CI verdict read at Setup 5a
- `48714f09c95e` (HEAD, the 0-pending wrap): **verdict not yet available** — `ci#37604531939` queued and
  `secret-scan#37604531980` in progress at take-up (13/13 checks registered). Not read as green; nothing to
  disposition yet.
- `f70be92c2ca1` (the last wrap's flip): green, 13/13 checks, wall 2428 s (`ci#37585281667`, with its secret-scan run).

## Gate
- none — the entry carries no `BLOCKED-ON`.
