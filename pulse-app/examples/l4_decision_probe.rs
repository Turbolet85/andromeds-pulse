//! Dev-only L4 decision probe — measures what the REAL model decides over
//! synthetic Tier1 storm digests, one factor varied per arm. Neither a test
//! nor a gate: a real-model generation cannot give a deterministic verdict.
//!
//! ```text
//! cargo build -p pulse-app --example l4_decision_probe
//! ./target/debug/examples/l4_decision_probe[.exe] --arms A0,A1,A2,A3,A4,A5 [--shapes S1,S2,S3,S4] --n 10 [--min 27] [--min-rank1 36] [--bar-sibling 19 --bar-ordinary 36] [--reproduce-misses 2] [--out DIR] [--footprint] [--sampling '…'] [--dry-run]
//! ./target/debug/examples/l4_decision_probe[.exe] --arms shipped --shapes A1,...,C3 [--renders today,enriched] --n 10 --out target/DIR [--footprint] [--sampling '…'] [--dry-run]
//! ./target/debug/examples/l4_decision_probe[.exe] --audit-draw ROOT --audit-seed N | --audit-grade ROOT | --table ROOT
//! ./target/debug/examples/l4_decision_probe[.exe] --arms shipped[,CR,CO,CC,CX] --replay LABEL=DIR[,LABEL=DIR] --replay-scope ID [--own-lines 2,5|none|unknown] [--reproduce-misses 2 | --remedy-from target/DIR [--remedy-read-as LABEL] --bar-sibling 38 --bar-ordinary 72] [--count-naming ID] --n 20 [--out DIR] [--dry-run [--product-path]]
//! ./target/debug/examples/l4_decision_probe[.exe] --sections SRC,SRC[,SRC]
//! ```
//!
//! Inputs come from the product's own guarded resolution: the hardware
//! profile picks `ANDROMEDA_PULSE_LLAMA_{CUDA,CPU}_BIN_PATH`, and
//! `ANDROMEDA_PULSE_MODEL_PATH` names the GGUF; `ANDROMEDA_PULSE_L4_ALLOW_ROOT`
//! confines both when set. Only basenames are printed. An unset or rejected
//! path exits 2 (INCONCLUSIVE) before any generation.
//!
//! Digests are rendered through the real `render_payload` + `cue_summary`
//! and composed with the real `build_primary_tier_prompt`; a shape's every
//! string is synthetic ASCII, never captured telemetry. A replay
//! (`replay.rs`) is the one input that is not: a captured prompt read from a
//! run-time path and passed unchanged, never printed and never stored. Each
//! generation is one
//! llama-cli spawn with production's argv (`build_llama_cli_args`), output
//! cap, wall-clock timeout and `kill_on_drop`; its `--grammar-file` is a file
//! holding `L4_OUTPUT_GBNF`, written once per run into the out dir.
//! `--sampling` REPLACES a sampling pair the production argv already carries
//! and appends any other, so no flag is ever passed twice.
//!
//! Per generation it keeps only bounded labels — the decision, the severity,
//! the model's `is_resolution_summary`, whether the producer would create an
//! incident, the first three JSON keys (the grammar's field order) and a
//! hash of the output (the determinism reading). The S shapes write no
//! title, symptom, hypothesis, justification or raw output anywhere. Rows go
//! to `{out}/runs.json`; stdout carries one summary line per arm, and with
//! `--min K` a final verdict line (exit 0 PASS / 1 FAIL).
//!
//! The pattern shapes (`patterns.rs`: A1-A7, B1-B3, C1-C3) score each
//! generation with the closed labels `valid`, `detect` and `cause` against a
//! ground truth fixed in code. For the blind audit only, a pattern run also
//! writes each generation's synthetic digest and raw stdout to
//! `{out}/texts/{shape}-{render}-{run}.{digest,stdout}.txt`, and only when
//! `{out}` resolves under the current directory's gitignored `target/`;
//! otherwise it is INCONCLUSIVE before any spawn. `--renders` runs the C
//! shapes in today's render, the probe-only enriched one (baselines and a
//! short trend), or both. Over a series root holding one dir per model,
//! `--audit-draw` writes a seeded blind sample and its key, `--audit-grade`
//! reads the overseer's verdicts against it, and `--table` re-grades the whole
//! series from its stored outputs and applies the pre-registered rule; none of
//! them spawns anything.
//!
//! Each row also carries `names_trigger`, a closed label computed in-process:
//! `rank1` (the first hypothesis names the triggering cue), `elsewhere` (the
//! title, the symptom or a later hypothesis does), `none`, or `unparsed`.
//! `--min-rank1 K` adds a names-trigger verdict line over all generations;
//! with both flags set, the exit is 1 if either verdict fails. Rows and
//! summary lines also carry `names_trigger_stem`, the same reading over the
//! stem forms (`retries`, `retried`); it is recorded only and never feeds a
//! verdict.
//!
//! Each row carries `identifies` too, a closed label read over the FIRST
//! hypothesis statement only, ASCII-lowercased: `both` (it names the cue's
//! `scope_id` as a whole word, neither neighbour in `[a-z0-9_-]`, and a retry
//! token, a maximal run of ASCII letters equal to `retry`, `retries` or
//! `retrying`), `service_only`, `signal_only`, `neither`, or `unparsed` (no
//! parse, or no first hypothesis). The rule reads no negation, passes a
//! space-separated sibling, and takes `retried` for no token.
//! `--bar-sibling K --bar-ordinary K` (both or neither; they need the
//! `shipped` and `ns` arms and at least one sibling and one ordinary shape,
//! else INCONCLUSIVE) grade each arm's `both` count over the sibling shapes
//! (S7, S8) and the ordinary ones (S1-S4), then print the selection (the first
//! of `shipped`, `L`, `LI` whose bar is met, or `none`), the service verdict
//! (PASS iff the selection is `shipped`; exit 0 / 1) and the regression guard
//! (TRIPPED iff `shipped` reads below `ns` on either half; it moves no exit).
//!
//! Footprint readings, per row and as one `footprint` summary line per arm:
//! `thinking` (`present` when the raw stdout carries b9305's
//! `[Start thinking]` marker, `absent` when not, `unread` when no stdout was
//! captured), `elapsed_ms` (spawn to reap, failed rows included),
//! `peak_rss_kib` (the child's `VmHWM` polled from `/proc/{pid}/status`,
//! Linux only) and, with `--footprint`, `peak_vram_mib` (the child's largest
//! `used_memory` polled from `nvidia-smi`). An unread value is `null` in the
//! rows and `-` in the summary.
//!
//! Shapes S1-S3 are retry storms on one service each; S4 is S1 plus one
//! corpus match (an older, active error-rate-spike incident on another
//! service), rendered through the real `format_corpus_match_line`. S5 is a
//! storm whose only abnormal metric is latency; S6 carries both an elevated
//! error rate and an elevated latency. S7 and S8 are the sibling shapes: a
//! retry storm scoped to `conductor` beside a `conductor-canary` services
//! row, both at a 100 % error rate; S8 adds two corpus matches for earlier
//! retry-storm incidents scoped to the canary. S9-S16 are S7 with a corpus
//! block that differs from S8's in one named property each: S9 in count (five
//! lines), S10 in kind mix (a retry storm and an error-rate spike), S11 in
//! position against S10, S12 in fingerprint (not the cue's), S13 in title form
//! (titles that read as a hypothesis statement naming the canary); S14 holds
//! five such lines of mixed kind, S15 the same lines with the retry storm
//! oldest, S16 five lines of mixed scope. S17 is the one shape derived from a
//! replayed prompt that reproduced, written with no text of it: the sibling's
//! services row stands first and both rows read the same low rate, 100 % error
//! rate and zero p99; `OVERALL:` counts one active incident; and the corpus
//! block holds three retry-storm lines whose titles state that a named
//! service is caught in one, the sibling's (3 m, active, another fingerprint), the
//! triggering service's own (9 m, resolved, the cue's fingerprint) and the
//! sibling's again (12 m, resolved, a third fingerprint). Every shape keeps its synthetic
//! incidents, and its corpus block is what the product's
//! `select_corpus_matches` keeps of them under the shape's cue. Since the
//! exclude remedy shipped that is the lines scoped to the cue's service or
//! carrying the cue's fingerprint: S4, S12, S14 and S15 compose with no
//! block, S16 with its two conductor lines and S17 with its one, and no other
//! shape's block changes. The blocks the readings measured under `shipped`
//! (every active scope's lines, newest first) are the `nb` arm's. S17 read
//! CLEAN there in the remedy reading (1 service miss of 20), so it is no
//! known-positive: the one the readings found was a captured prompt, and the
//! probe holds none of its own. `--shapes` selects among the shapes (default
//! S1-S4).
//!
//! `--reproduce-misses K` is the reproduction reading: the `shipped` arm
//! alone over at least one of S9-S16, no other verdict flag. A service miss
//! is a generation labelled `signal_only` or `neither`; `unparsed` is counted
//! apart. Per shape it prints REPRODUCES (K or more misses), CLEAN (misses
//! plus `unparsed` below K) or UNREAD; then REPRODUCED (exit 0) when a shape
//! of S9-S16 reproduces, NOT REPRODUCED (exit 1) when all eight read CLEAN,
//! INCONCLUSIVE (exit 2) otherwise. S7 and S8 are read and printed as
//! controls and move no verdict.
//!
//! `--replay LABEL=DIR[,…] --replay-scope ID` adds one captured model
//! invocation per label as the shape `replay:LABEL`: `DIR/prompt.txt` is the
//! `-p` operand and `DIR/argv.nul` the recorded argv, which must equal the
//! probe's own apart from the model and grammar paths. A directory inside
//! this work tree and outside `target/`, a prompt the production bound
//! rejects, or a first cue line whose `scope_id` is not `ID` is INCONCLUSIVE
//! before any spawn. A replay runs beside S shapes under `shipped` and the
//! candidate arms only; with it and no `--shapes`, no S shape runs. With
//! `--replay`, `--reproduce-misses K` reads every replay but the one labelled
//! `control` by the per-shape rule above: REPRODUCED (exit 0) when one of them
//! reproduces, NOT REPRODUCED (exit 1) when all read CLEAN, INCONCLUSIVE
//! (exit 2) otherwise. It needs a replay labelled `miss`; the label names a
//! role in the reading, whatever the captured drive itself did. `control` is
//! printed and moves no verdict.
//!
//! `--product-path`, beside `--dry-run` and `--own-lines`, also composes the
//! `miss` replay's corpus block through the product. Each line is read back
//! into an incident, scoped to the cue's service at the own positions and to
//! another of the digest's services elsewhere, with the prompt's citable
//! fingerprints as the window's; `select_corpus_matches`, the line format and
//! `render_payload` then write the block. It prints whether that reads as the
//! capture with no triggering scope and as the `CX` edit under the cue's, and
//! exits 0 when both do, 1 when one differs. A capture holds neither an
//! incident's scope nor the window's other fingerprints, so both are inputs
//! of the comparison and not read by it.
//!
//! `--count-naming ID` records a second count beside any reading and moves no
//! verdict, bar or exit: per generation the closed label `names_other`, read
//! over the FIRST hypothesis statement only, ASCII-lowercased. It is `named`
//! when the statement holds `ID` as a whole word, with a `-`, `_` or space
//! inside `ID` read as any of the three, `not_named` when it does not, and
//! `unparsed` as for `identifies`. It does not read where the statement
//! places the signal: any mention counts. `ID` itself is never printed.
//!
//! `--sections SRC,SRC[,…]` spawns nothing: each source, `LABEL=DIR` or an S
//! shape id, is cut into the prompt builder's sections and its digest into the
//! render's, and every source after the first is compared with the first, one
//! row per closed label (same or differs, bytes and lines of each side).
//!
//! `--remedy-from DIR` is the remedy reading. It takes m, the service misses
//! in `DIR/runs.json` of `replay:miss`, or of `replay:LABEL` with
//! `--remedy-read-as LABEL` when the replay reading read the prompt now in the
//! `miss` role under another label, runs the `miss` replay at
//! n = max(20, ceil(100 / m)) and the `control` replay, S1-S4, S7, S8 and the
//! derived shape, when the probe defines one, at `--n`. It needs `shipped`
//! and a candidate arm, `--own-lines` and both bar flags. A candidate that
//! composes as `shipped` on `replay:miss` is skipped; one that composes as
//! `shipped` on a guard shape takes `shipped`'s counts there. The
//! known-positive is HELD when `shipped` misses at least floor(n / 20) + 2
//! times on `replay:miss` (else INCONCLUSIVE, exit 2). A candidate's bar is
//! MET when its generations that are not `both` stay within floor(n / 20)
//! there and its `both` counts reach the two bar flags; its guard is TRIPPED
//! when a `both` count reads below `shipped`'s. The selection is the first of
//! `CR`, `CO`, `CC`, `CX` with its bar met and its guard holding (SELECTED,
//! exit 0), or `none` (NONE, exit 1). The derived shape is recorded and moves
//! neither.
//!
//! Arms (each differs from A0 in ONE factor):
//! - `A0` — the rendered digest, prompt and argv as the tree has them
//! - `A1` — A0 plus `--temp 0`
//! - `A2` — A0 with the `OVERALL:` line made truthful (`anomalous` when the
//!   digest carries a cue)
//! - `A3` — A0 with the cue line carrying the cue's magnitude, absolute
//!   value and persistence
//! - `A4` — A0 plus one sentence in the conventions stating when a signal
//!   warrants `surface`
//! - `A5` — A0 with `decision` / `severity` moved after `hypotheses` in the
//!   prompt's embedded schema copy (the shipped order since prompt v2.3, so A5
//!   now composes as A0; the argv's grammar fixes that order either way)
//! - `nf` — the shipped composition with the three trigger-framing lines
//!   removed: the digest's TRIGGER line, its corpus framing note and the
//!   prompt's framing instruction (the no-framing counterfactual)
//! - `shipped` — the tree as it is, no transform (the post-fix re-measure)
//! - `nr` — `shipped` with the `-rea off` pair removed from the argv (does
//!   the model think when the product does not pass the switch?)
//! - `R1` — the framing instruction reworded to oblige the first hypothesis
//!   statement to name the TRIGGER line's signal in its own words
//! - `R3` — one conventions sentence carrying the same obligation
//! - `R2` — one sentence in the schema's `hypotheses` description carrying
//!   it; the grammar carries no description, so R2 varies the prompt's
//!   embedded copy only
//!
//! The former `gb` arm (the schema file swapped for a grammar file) is the
//! shipped argv now, so it is retired and `--arms gb` is an unknown arm.
//! - `R1R3` / `R1R2` / `R3R2` — the two named candidates applied together
//! - `ns` — `shipped` with the scope sentence removed from the framing
//!   instruction (the prompt v2.5 composition, the baseline)
//! - `L` — `ns` with the digest's one TRIGGER line carrying the cue's
//!   `scope_id` (`TRIGGER: {cause label} on {scope_id}`); a harness render
//! - `LI` — `shipped` with the same TRIGGER line rewrite; a harness render
//! - `nb` — `shipped` with the corpus block as it was before the exclude
//!   remedy: the product's selection with no triggering scope, every active
//!   scope's lines newest first (the baseline, and what the readings measured
//!   as `shipped`). It does not apply to a replay, whose captured block is
//!   that one already
//! - `CR` / `CO` / `CC` / `CX` — `nb`, or a replay's prompt, with one edit of
//!   the corpus block, the lines from `CORPUS MATCHES:` to the end of the
//!   digest: one static line restating the cue after the last corpus line
//!   (`CR`), the triggering scope's own lines moved first (`CO`), the first
//!   two lines kept (`CC`), or the own lines kept and the others removed
//!   (`CX`). The own lines are those of incidents scoped to the cue's
//!   `scope_id` or carrying the cue's fingerprint; a shape derives them from
//!   its incidents, and the replay labelled `miss` takes them from
//!   `--own-lines` (any other replay's are unknown, so `CO` and `CX` are
//!   skipped on it). The remedy reading's order selected `CO`; the founder
//!   chose `CX` on the third reading's counts, and `CX` is the product's
//!   selection now, so on every shape it composes as `shipped`. On a replay
//!   it still edits, a captured block being the unremedied one
//!
//! Each candidate composes as A0 once its text is already in the tree, and
//! `ns`, `L` and `LI` compose on a tree with or without the scope sentence.

// The example file is a crate root, so a bare `mod patterns;` would resolve
// beside it, where Cargo auto-discovers every `examples/*.rs` as an example.
#[path = "l4_decision_probe/patterns.rs"]
mod patterns;
#[path = "l4_decision_probe/replay.rs"]
mod replay;

use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::{ExitCode, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use interpretation::hardware::HardwareProfileDetector;
use interpretation::prompt::{TRIGGER_FRAMING_INSTRUCTION, build_primary_tier_prompt};
use interpretation::schema::{Decision, L4_OUTPUT_JSON_SCHEMA, L4Output, Severity, parse_bounded};
use patterns::{Outcome, PatternShape, Render};
use pulse_app::llamacli_inference::{
    DEFAULT_MAX_TOKENS, ENV_MODEL_PATH, L4_OUTPUT_GBNF, LLAMA_CLI_MAX_OUTPUT_BYTES,
    LLAMA_CLI_TIMEOUT, MAX_PROMPT_BYTES, binary_target_for_profile, build_llama_cli_args,
    extract_json_object_bounded, resolve_allow_root, validate_path_input, validate_prompt_bounded,
};
use replay::{Composed, OwnLines, Replay, SectionSource};
use serde_json::{Value, json};
use tokio::io::AsyncReadExt;
use triage::contract::{
    AttentionCue, CORPUS_MATCHES_FRAMING_NOTE, CueKind, CueScope, DIGEST_CORPUS_RETRIEVAL_LIMIT,
    DigestCueRef, DigestProjectContext, DigestServiceRow, EvidenceRefs, HardwareProfileSource,
    Incident, IncidentStatus, PriorityTier, Severity as IncidentSeverity, TRIGGER_LINE_PREFIX,
    cue_cause_label, cue_summary, format_corpus_match_line, render_payload, select_corpus_matches,
};

const ARMS: [&str; 23] = [
    "A0", "A1", "A2", "A3", "A4", "A5", "nf", "shipped", "nr", "R1", "R3", "R2", "R1R3", "R1R2",
    "R3R2", "ns", "L", "LI", "nb", "CR", "CO", "CC", "CX",
];
// The two halves of the service bar. S5 and S6 belong to neither.
const SIBLING_SHAPES: [&str; 2] = ["S7", "S8"];
const ORDINARY_SHAPES: [&str; 4] = ["S1", "S2", "S3", "S4"];
// The shapes the reproduction reading grades; S7 and S8 are its controls.
const REPRODUCTION_SHAPES: [&str; 8] = ["S9", "S10", "S11", "S12", "S13", "S14", "S15", "S16"];
// The selection rule's fixed order; `ns` is the baseline and never a candidate.
const SELECTION_ORDER: [&str; 3] = ["shipped", "L", "LI"];
// The replay the reproduction and remedy readings grade, and its control.
const MISS_REPLAY: &str = "miss";
const CONTROL_REPLAY: &str = "control";
const MISS_SHAPE: &str = "replay:miss";
const CONTROL_SHAPE: &str = "replay:control";
// The replay reading's fixed size and per-prompt minimum: the remedy reading
// sizes itself from a `replay:miss` count of 20, reproduced at 2 or more.
const REPLAY_READING_N: u32 = 20;
const REPLAY_REPRODUCE_MISSES: u32 = 2;
// The one synthetic shape derived from a reproducing replay, read in the
// remedy reading when `shapes()` defines it. It reproduces at 2 misses of 20
// and is covered by a selected candidate within 1 generation that is not
// `both`.
const DERIVED_SHAPE: &str = "S17";
const BASELINE_ARM: &str = "nb";
const DERIVED_COVERED_MAX: u32 = 1;
// The grammar file each run writes into its out dir and passes as
// `--grammar-file`.
const GRAMMAR_FILE_NAME: &str = "l4-output.gbnf";
// b9305 `tools/cli/cli.cpp` prints a reasoning pass to stdout between
// `[Start thinking]` and `[End thinking]`, ahead of the content.
const THINKING_MARKER: &str = "[Start thinking]";
// The only llama-cli flags `--sampling` may carry: a model's sampling values
// and its chat-template kwargs, never a path, a grammar or a model.
const SAMPLING_FLAGS: [&str; 6] = [
    "--temp",
    "--top-p",
    "--top-k",
    "--min-p",
    "--presence-penalty",
    "--chat-template-kwargs",
];
const RSS_POLL_INTERVAL: Duration = Duration::from_millis(100);
const VRAM_POLL_INTERVAL: Duration = Duration::from_millis(250);
const NVIDIA_SMI_ARGS: [&str; 2] = [
    "--query-compute-apps=pid,used_memory",
    "--format=csv,noheader,nounits",
];
const DEFAULT_SHAPES: [&str; 4] = ["S1", "S2", "S3", "S4"];
// `crate::cadence::mode_label(CadenceMode::Tier1)` — a storm cue always takes
// the Tier1 cycle, whose window is 60 s.
const TIER1_MODE_LABEL: &str = "tier1";
const TIER1_WINDOW: Duration = Duration::from_secs(60);
const WORKSPACE: &str = "/synthetic/demo-shop";
const STORM_FINGERPRINT: &str = "5e1f0a9c3b7d42e68a0c1f3e5b7d9a2c";
const CORPUS_FINGERPRINT: &str = "9c2e7b41d05a3f86e1b4c7d02a59f3e8";
const THIRD_FINGERPRINT: &str = "3a7f1c9e5b2d4806c4e8a1f7d3b59e20";
// A fixed render instant keeps the corpus line's age, and so the prompt bytes,
// identical across runs.
const RENDER_NOW_UNIX_NANO: i64 = 1_700_000_000_000_000_000;
const CORPUS_MATCH_AGE_NANOS: i64 = 180_000_000_000;
const SIBLING_SERVICE: &str = "conductor";
const SIBLING_CANARY: &str = "conductor-canary";
const NANOS_PER_MINUTE: i64 = 60_000_000_000;
// A corpus block's lines by position, newest first: age in minutes and status.
// The titles move between positions; the slots do not.
const TWO_LINE_SLOTS: [(i64, IncidentStatus); 2] =
    [(3, IncidentStatus::Active), (9, IncidentStatus::Resolved)];
const THREE_LINE_SLOTS: [(i64, IncidentStatus); 3] = [
    (3, IncidentStatus::Active),
    (9, IncidentStatus::Resolved),
    (12, IncidentStatus::Resolved),
];
const FIVE_LINE_SLOTS_ONE_ACTIVE: [(i64, IncidentStatus); 5] = [
    (3, IncidentStatus::Active),
    (9, IncidentStatus::Resolved),
    (12, IncidentStatus::Resolved),
    (15, IncidentStatus::Resolved),
    (18, IncidentStatus::Resolved),
];
const FIVE_LINE_SLOTS_TWO_ACTIVE: [(i64, IncidentStatus); 5] = [
    (3, IncidentStatus::Active),
    (9, IncidentStatus::Active),
    (12, IncidentStatus::Resolved),
    (15, IncidentStatus::Resolved),
    (18, IncidentStatus::Resolved),
];
const FIVE_LINE_SLOTS_MIXED_SCOPE: [(i64, IncidentStatus); 5] = [
    (3, IncidentStatus::Active),
    (9, IncidentStatus::Resolved),
    (12, IncidentStatus::Active),
    (15, IncidentStatus::Resolved),
    (18, IncidentStatus::Resolved),
];
// Descriptive titles name the sibling as a display name and never as its
// scope_id; statement titles read as a first hypothesis and end on a scope_id.
const DESCRIPTIVE_STORM_TITLES: [&str; 5] = [
    "Conductor-Canary calls fail and loop back",
    "Conductor-Canary saturated by repeated calls",
    "Conductor-Canary requests re-sent after timeouts",
    "Conductor-Canary flooded by repeated attempts",
    "Conductor-Canary callers looping on failed calls",
];
const DESCRIPTIVE_SPIKE_TITLE: &str = "Conductor-Canary failing a rising share of calls";
const CANARY_STORM_STATEMENTS: [&str; 2] = [
    "Retry Storm Observed on conductor-canary",
    "Repeated Call Retries Seen on conductor-canary",
];
const CANARY_SPIKE_STATEMENTS: [&str; 4] = [
    "Error Rate Spike Observed on conductor-canary",
    "Elevated Error Rate Reported on conductor-canary",
    "Failing Requests Rising on conductor-canary",
    "Error Share Climbing on conductor-canary",
];
const SERVICE_STORM_STATEMENTS: [&str; 2] = [
    "Retry Storm Observed on conductor",
    "Repeated Call Retries Seen on conductor",
];
// S17's titles, the probe's own sentence: each states that a named service
// is caught in the signal kind. The sibling's scope_id in a lower-case
// sentence, the triggering service as a display name in title case, the
// sibling's scope_id inside title case.
const DERIVED_TITLES: [&str; 3] = [
    "conductor-canary service is caught in a retry storm",
    "Conductor Service Is Caught in a Retry Storm",
    "conductor-canary Service Is Caught in a Retry Storm",
];
const A4_ANCHOR: &str = "\"watch\" (record but do not surface). ";
const A4_SENTENCE: &str = "A signal warrants \"surface\" when a service's error rate or \
latency is far above its baseline or an attention cue reports a storm. ";
// The candidate texts are kind-generic and never name a cue kind, so a
// remedy cannot be a word plant the grader rewards.
const R1_FRAMING_INSTRUCTION: &str = "\
When the digest carries a TRIGGER line, that line names the signal this \
output describes: the title, the symptom and the first hypothesis must be \
about that signal, and the first hypothesis statement must name that signal \
in the TRIGGER line's own words. Treat any other abnormal metric on the same \
service as a cause or an effect of that signal, never as a separate first \
hypothesis. CORPUS MATCHES lines are OTHER incidents, past or still open on \
another signal, given for context only; never describe one of them as the \
current signal.";
// The framing instruction as prompt v2.5 carried it: the `ns` baseline, held
// here so it does not drift when the product text is edited again.
const V25_FRAMING_INSTRUCTION: &str = R1_FRAMING_INSTRUCTION;
// The scope sentence prompt v2.6 appends to that instruction, one space after
// its last sentence.
const SCOPE_SENTENCE: &str = "When the cue line under ATTENTION CUES carries a \
scope_id, the first hypothesis statement must name that scope_id value exactly \
as written there, and must not attribute the signal to anything else, including \
a service whose name merely contains it.";
const R3_ANCHOR: &str = "Investigation steps point to concrete checks";
const R3_CONVENTIONS_SENTENCE: &str = "When the digest carries a TRIGGER \
line, the first hypothesis names that signal in the TRIGGER line's own words. ";
const R2_ANCHOR: &str =
    "Ranked hypothesis list per P-033 (primary tier emits up to 5; fallback tier emits 1).";
const R2_SCHEMA_SENTENCE: &str = " When the digest carries a TRIGGER line, the \
first hypothesis statement names that signal in the TRIGGER line's own words.";

struct Shape {
    id: &'static str,
    services: Vec<DigestServiceRow>,
    cue: AttentionCue,
    /// The synthetic incidents the corpus block is selected and rendered from.
    corpus_incidents: Vec<Incident>,
    /// The active incidents the `OVERALL:` line counts; the render shows the
    /// count alone.
    active_incidents: usize,
}

/// One synthetic corpus incident before it takes a position in a block.
#[derive(Clone, Copy)]
struct CorpusSpec {
    kind: CueKind,
    scope_id: &'static str,
    fingerprint: &'static str,
    title: &'static str,
}

struct Args {
    arms: Vec<String>,
    shapes: Vec<String>,
    n: u32,
    min: Option<u32>,
    min_rank1: Option<u32>,
    /// `--bar-sibling` / `--bar-ordinary`, set together or not at all.
    bar: Option<Bar>,
    /// `--reproduce-misses K`: the reproduction reading's per-shape minimum.
    reproduce: Option<u32>,
    out: PathBuf,
    dry_run: bool,
    footprint: bool,
    /// Validated `--sampling` flag/value tokens, applied to every arm's argv.
    sampling: Vec<String>,
    /// The C shapes' renders; A and B always run today's.
    renders: Vec<Render>,
    /// `--replay`: each capture directory under its label.
    replays: Vec<(String, PathBuf)>,
    /// `--replay-scope`: the `scope_id` every replay's first cue line carries.
    replay_scope: Option<String>,
    /// `--own-lines`: a replay's own corpus lines; unknown when not given.
    own_lines: OwnLines,
    /// `--remedy-from`: the replay reading's out dir the remedy reading sizes
    /// itself from.
    remedy_from: Option<PathBuf>,
    /// `--remedy-read-as`: the label the replay reading read the remedy
    /// prompt under; `miss` when not given.
    remedy_read_as: String,
    /// `--count-naming`: the service id the second count looks for.
    count_naming: Option<String>,
    /// `--product-path`: a dry run also composes the `miss` replay through
    /// the product's selection and render.
    product_path: bool,
    mode: Mode,
}

/// What a call does: run generations, or read its sources and spawn nothing.
#[derive(Debug, PartialEq)]
enum Mode {
    Run,
    AuditDraw(PathBuf, u64),
    AuditGrade(PathBuf),
    Table(PathBuf),
    Sections(Vec<SectionSource>),
}

/// The service bar's two minimums: `both` over the sibling shapes and over
/// the ordinary ones.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Bar {
    sibling_min: u32,
    ordinary_min: u32,
}

/// One arm's `both` counts and generation counts over each half of the bar.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct BarCounts {
    sibling_both: u32,
    sibling_n: u32,
    ordinary_both: u32,
    ordinary_n: u32,
}

/// One shape's counts in the reproduction reading. A generation is a service
/// miss, `unparsed`, or neither; never both.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct MissCounts {
    misses: u32,
    unparsed: u32,
    n: u32,
}

/// One shape's second count under one arm: the generations whose first
/// hypothesis names the `--count-naming` id, and those with none to read.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct NamedCounts {
    named: u32,
    unparsed: u32,
    n: u32,
}

/// One arm's generations over one source in the remedy reading. A generation
/// that is `unparsed` or a service miss is not `both`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Tally {
    both: u32,
    misses: u32,
    unparsed: u32,
    n: u32,
}

impl Tally {
    fn not_both(self) -> u32 {
        self.n - self.both
    }

    fn plus(self, other: Tally) -> Tally {
        Tally {
            both: self.both + other.both,
            misses: self.misses + other.misses,
            unparsed: self.unparsed + other.unparsed,
            n: self.n + other.n,
        }
    }
}

/// One arm's counts in the remedy reading: on the remedy prompt, over the
/// sibling guard shapes, over the ordinary ones, and on the derived shape
/// when the probe defines one.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct RemedyCounts {
    remedy: Tally,
    sibling: Tally,
    ordinary: Tally,
    derived: Option<Tally>,
}

/// A candidate arm in the remedy reading: skipped with its reason, or read.
#[derive(Clone, Copy, Debug, PartialEq)]
enum RemedyArm {
    Skipped(&'static str),
    Read(RemedyCounts),
}

/// What a candidate arm generates in the remedy reading: nothing, or every
/// source but the listed shapes, whose counts it takes from `shipped`.
#[derive(Debug, PartialEq)]
enum ArmPlan {
    Skipped(&'static str),
    Run(Vec<&'static str>),
}

/// The replay reading a remedy reading sizes itself from.
struct RemedySource {
    /// The service misses of `replay:miss`, of `REPLAY_READING_N`.
    misses: u32,
    /// The generations the remedy prompt runs per arm.
    n: u32,
    dir_name: String,
    /// The label the replay reading read this prompt under.
    read_as: String,
}

struct Prepared {
    arm: String,
    shape: &'static str,
    trigger: CueKind,
    /// The triggering cue's `scope_id`, read by the `identifies` grader.
    scope_id: Option<String>,
    prompt: String,
    schema: String,
    extra_args: Vec<String>,
    /// Flags removed from the production argv together with their value.
    drop_flags: Vec<&'static str>,
}

fn row(service: &str, rate: f64, error_rate: f64, p99: f64) -> DigestServiceRow {
    DigestServiceRow {
        service: service.to_string(),
        rate_per_sec: rate,
        rate_baseline_per_sec: rate,
        error_rate,
        error_rate_baseline: error_rate,
        p99_latency_ms: p99,
        p99_baseline_ms: p99,
    }
}

fn storm_cue(service: &str, magnitude: f64, absolute_value: f64) -> AttentionCue {
    AttentionCue {
        kind: CueKind::RetryStorm,
        scope: CueScope::Service,
        scope_id: Some(service.to_string()),
        magnitude,
        absolute_value,
        persistence: 30,
        confidence: 0.95,
        priority_tier: PriorityTier::Autonomous,
        suppression_bypassed: false,
        fingerprint: Some(STORM_FINGERPRINT.to_string()),
    }
}

fn shapes() -> Vec<Shape> {
    vec![
        Shape {
            id: "S1",
            services: vec![
                row("checkout-api", 12.0, 1.0, 180.0),
                row("payment-service", 9.0, 0.0, 95.0),
                row("inventory-service", 7.0, 0.0, 60.0),
                row("auth-service", 15.0, 0.0, 40.0),
            ],
            cue: storm_cue("checkout-api", 20.0, 1.0),
            corpus_incidents: Vec::new(),
            active_incidents: 0,
        },
        Shape {
            id: "S2",
            services: vec![
                row("checkout-api", 11.0, 0.0, 170.0),
                row("payment-service", 9.0, 0.35, 110.0),
                row("inventory-service", 7.0, 0.0, 60.0),
                row("auth-service", 15.0, 0.0, 40.0),
            ],
            cue: storm_cue("payment-service", 12.0, 0.35),
            corpus_incidents: Vec::new(),
            active_incidents: 0,
        },
        Shape {
            id: "S3",
            services: vec![
                row("checkout-api", 11.0, 0.0, 120.0),
                row("payment-service", 9.0, 0.0, 110.0),
                row("inventory-service", 7.0, 0.12, 480.0),
                row("auth-service", 15.0, 0.0, 115.0),
            ],
            cue: storm_cue("inventory-service", 6.0, 0.12),
            corpus_incidents: Vec::new(),
            active_incidents: 0,
        },
        Shape {
            id: "S4",
            services: vec![
                row("checkout-api", 12.0, 1.0, 180.0),
                row("payment-service", 9.0, 0.0, 95.0),
                row("inventory-service", 7.0, 0.0, 60.0),
                row("auth-service", 15.0, 0.0, 40.0),
            ],
            cue: storm_cue("checkout-api", 20.0, 1.0),
            corpus_incidents: vec![corpus_match_incident()],
            active_incidents: 0,
        },
        Shape {
            id: "S5",
            services: vec![
                row("checkout-api", 12.0, 0.0, 120.0),
                row("payment-service", 9.0, 0.0, 95.0),
                row("inventory-service", 7.0, 0.0, 60.0),
                row("auth-service", 15.0, 0.0, 620.0),
            ],
            cue: storm_cue("auth-service", 9.0, 0.0),
            corpus_incidents: Vec::new(),
            active_incidents: 0,
        },
        Shape {
            id: "S6",
            services: vec![
                row("checkout-api", 11.0, 0.20, 390.0),
                row("payment-service", 9.0, 0.0, 95.0),
                row("inventory-service", 7.0, 0.0, 60.0),
                row("auth-service", 15.0, 0.0, 40.0),
            ],
            cue: storm_cue("checkout-api", 10.0, 0.20),
            corpus_incidents: Vec::new(),
            active_incidents: 0,
        },
        sibling_shape("S7", Vec::new()),
        sibling_shape("S8", sibling_corpus_incidents()),
        sibling_shape(
            "S9",
            corpus_incidents(
                &DESCRIPTIVE_STORM_TITLES
                    .map(|title| storm_spec(SIBLING_CANARY, STORM_FINGERPRINT, title)),
                &FIVE_LINE_SLOTS_ONE_ACTIVE,
            ),
        ),
        sibling_shape("S10", corpus_incidents(&kind_mix_specs(), &TWO_LINE_SLOTS)),
        sibling_shape("S11", {
            let [storm, spike] = kind_mix_specs();
            corpus_incidents(&[spike, storm], &TWO_LINE_SLOTS)
        }),
        sibling_shape(
            "S12",
            corpus_incidents(
                &[
                    storm_spec(
                        SIBLING_CANARY,
                        CORPUS_FINGERPRINT,
                        DESCRIPTIVE_STORM_TITLES[0],
                    ),
                    storm_spec(
                        SIBLING_CANARY,
                        CORPUS_FINGERPRINT,
                        DESCRIPTIVE_STORM_TITLES[1],
                    ),
                ],
                &TWO_LINE_SLOTS,
            ),
        ),
        sibling_shape(
            "S13",
            corpus_incidents(
                &CANARY_STORM_STATEMENTS
                    .map(|title| storm_spec(SIBLING_CANARY, STORM_FINGERPRINT, title)),
                &TWO_LINE_SLOTS,
            ),
        ),
        sibling_shape(
            "S14",
            corpus_incidents(&founder_specs(), &FIVE_LINE_SLOTS_TWO_ACTIVE),
        ),
        sibling_shape("S15", {
            let mut specs = founder_specs();
            specs.rotate_left(1);
            corpus_incidents(&specs, &FIVE_LINE_SLOTS_TWO_ACTIVE)
        }),
        sibling_shape(
            "S16",
            corpus_incidents(
                &[
                    storm_spec(
                        SIBLING_CANARY,
                        CORPUS_FINGERPRINT,
                        CANARY_STORM_STATEMENTS[0],
                    ),
                    storm_spec(
                        SIBLING_SERVICE,
                        STORM_FINGERPRINT,
                        SERVICE_STORM_STATEMENTS[0],
                    ),
                    spike_spec(
                        SIBLING_CANARY,
                        CORPUS_FINGERPRINT,
                        CANARY_SPIKE_STATEMENTS[0],
                    ),
                    spike_spec(
                        SIBLING_CANARY,
                        CORPUS_FINGERPRINT,
                        CANARY_SPIKE_STATEMENTS[1],
                    ),
                    storm_spec(
                        SIBLING_SERVICE,
                        STORM_FINGERPRINT,
                        SERVICE_STORM_STATEMENTS[1],
                    ),
                ],
                &FIVE_LINE_SLOTS_MIXED_SCOPE,
            ),
        ),
        Shape {
            id: DERIVED_SHAPE,
            services: vec![
                row(SIBLING_CANARY, 0.3, 1.0, 0.0),
                row(SIBLING_SERVICE, 0.3, 1.0, 0.0),
            ],
            cue: storm_cue(SIBLING_SERVICE, 20.0, 1.0),
            corpus_incidents: corpus_incidents(&derived_specs(), &THREE_LINE_SLOTS),
            active_incidents: 1,
        },
    ]
}

/// A sibling shape: S7's two services rows and S7's cue, a retry storm scoped
/// to the triggering service, over the given corpus incidents. The sibling
/// shapes differ in the corpus block alone.
fn sibling_shape(id: &'static str, corpus_incidents: Vec<Incident>) -> Shape {
    Shape {
        id,
        services: sibling_services(),
        cue: storm_cue(SIBLING_SERVICE, 20.0, 1.0),
        corpus_incidents,
        active_incidents: 0,
    }
}

/// The sibling shapes' services rows: the triggering service and its
/// hyphenated sibling, both at a 100 % error rate.
fn sibling_services() -> Vec<DigestServiceRow> {
    vec![
        row(SIBLING_SERVICE, 12.0, 1.0, 180.0),
        row(SIBLING_CANARY, 4.0, 1.0, 160.0),
    ]
}

fn storm_spec(
    scope_id: &'static str,
    fingerprint: &'static str,
    title: &'static str,
) -> CorpusSpec {
    CorpusSpec {
        kind: CueKind::RetryStorm,
        scope_id,
        fingerprint,
        title,
    }
}

fn spike_spec(
    scope_id: &'static str,
    fingerprint: &'static str,
    title: &'static str,
) -> CorpusSpec {
    CorpusSpec {
        kind: CueKind::ErrorRateSpike,
        scope_id,
        fingerprint,
        title,
    }
}

/// S10's and S11's two sibling lines, the retry storm first: descriptive
/// titles, the cue's fingerprint.
fn kind_mix_specs() -> [CorpusSpec; 2] {
    [
        storm_spec(
            SIBLING_CANARY,
            STORM_FINGERPRINT,
            DESCRIPTIVE_STORM_TITLES[0],
        ),
        spike_spec(SIBLING_CANARY, STORM_FINGERPRINT, DESCRIPTIVE_SPIKE_TITLE),
    ]
}

/// S17's three retry-storm lines, newest first: the sibling's with a
/// fingerprint other than the cue's, the triggering service's own with the
/// cue's, the sibling's again with a third fingerprint.
fn derived_specs() -> [CorpusSpec; 3] {
    [
        storm_spec(SIBLING_CANARY, CORPUS_FINGERPRINT, DERIVED_TITLES[0]),
        storm_spec(SIBLING_SERVICE, STORM_FINGERPRINT, DERIVED_TITLES[1]),
        storm_spec(SIBLING_CANARY, THIRD_FINGERPRINT, DERIVED_TITLES[2]),
    ]
}

/// S14's and S15's five sibling lines, the retry storm first: one retry storm
/// and four error-rate spikes, statement titles, a fingerprint other than the
/// cue's.
fn founder_specs() -> [CorpusSpec; 5] {
    let spike = |title| spike_spec(SIBLING_CANARY, CORPUS_FINGERPRINT, title);
    [
        storm_spec(
            SIBLING_CANARY,
            CORPUS_FINGERPRINT,
            CANARY_STORM_STATEMENTS[0],
        ),
        spike(CANARY_SPIKE_STATEMENTS[0]),
        spike(CANARY_SPIKE_STATEMENTS[1]),
        spike(CANARY_SPIKE_STATEMENTS[2]),
        spike(CANARY_SPIKE_STATEMENTS[3]),
    ]
}

/// Synthetic corpus incidents, newest first: the i-th spec takes the i-th
/// slot's age and status, and its title the producer's
/// `{cause label}: {model title}` form.
fn corpus_incidents(specs: &[CorpusSpec], slots: &[(i64, IncidentStatus)]) -> Vec<Incident> {
    specs
        .iter()
        .zip(slots)
        .zip(1_i64..)
        .map(|((spec, (age_minutes, status)), id)| {
            let opened = RENDER_NOW_UNIX_NANO - age_minutes * NANOS_PER_MINUTE;
            Incident {
                id,
                fingerprint: spec.fingerprint.to_string(),
                title: format!("{}: {}", cue_cause_label(spec.kind), spec.title),
                kind: spec.kind,
                scope_id: Some(spec.scope_id.to_string()),
                status: *status,
                severity: IncidentSeverity::Error,
                priority_tier: PriorityTier::Autonomous,
                opened_at_unix_nano: opened,
                updated_at_unix_nano: opened,
                resolved_at_unix_nano: (*status == IncidentStatus::Resolved).then_some(opened),
                ..corpus_match_incident()
            }
        })
        .collect()
}

/// S8's two earlier retry-storm incidents scoped to the sibling, newest
/// first: one still active, one resolved. Their titles name the sibling. A
/// sibling-scoped incident reaches a digest through the scope arm of retrieval
/// whenever the sibling is a services row, and through the fingerprint arm
/// too when, as here, it carries the storm's fingerprint.
fn sibling_corpus_incidents() -> Vec<Incident> {
    corpus_incidents(
        &[
            storm_spec(
                SIBLING_CANARY,
                STORM_FINGERPRINT,
                DESCRIPTIVE_STORM_TITLES[0],
            ),
            storm_spec(
                SIBLING_CANARY,
                STORM_FINGERPRINT,
                DESCRIPTIVE_STORM_TITLES[1],
            ),
        ],
        &TWO_LINE_SLOTS,
    )
}

/// The shape's corpus incidents as the product selects them for a digest
/// under the given triggering scope: the cue's fingerprint as the window's,
/// every services row as a scope, the product limit; newest first.
fn corpus_incidents_under(shape: &Shape, triggering_scope: Option<&str>) -> Vec<Incident> {
    let fingerprints: Vec<String> = shape.cue.fingerprint.iter().cloned().collect();
    let scopes: Vec<String> = shape.services.iter().map(|r| r.service.clone()).collect();
    select_corpus_matches(
        shape.corpus_incidents.clone(),
        &fingerprints,
        &scopes,
        triggering_scope,
        DIGEST_CORPUS_RETRIEVAL_LIMIT,
    )
}

/// The shape's corpus incidents as the product selects them for its cue.
fn selected_corpus_incidents(shape: &Shape) -> Vec<Incident> {
    corpus_incidents_under(shape, shape.cue.scope_id.as_deref())
}

/// The shape's corpus incidents as the product selected them before the
/// remedy, and still does for a digest without a cue scope: every active
/// scope's matches.
fn baseline_corpus_incidents(shape: &Shape) -> Vec<Incident> {
    corpus_incidents_under(shape, None)
}

/// The corpus block the product's render writes for `lines`, the payload's
/// last section: empty without a line.
fn rendered_corpus_block(lines: &[String]) -> Result<String, String> {
    let render = |lines: &[String]| {
        render_payload(
            TIER1_WINDOW,
            TIER1_MODE_LABEL,
            &project(),
            &[],
            &[],
            lines,
            &[],
            false,
        )
    };
    render(lines)
        .strip_prefix(&render(&[]))
        .map(str::to_string)
        .ok_or_else(|| "the corpus block is not the payload's last section".to_string())
}

/// The corpus block's lines, each through the real `format_corpus_match_line`
/// at the fixed render instant.
fn corpus_match_lines(incidents: &[Incident]) -> Vec<String> {
    incidents
        .iter()
        .map(|incident| format_corpus_match_line(incident, RENDER_NOW_UNIX_NANO))
        .collect()
}

/// The listed shapes, in `shapes()` order whatever the list order.
fn select_shapes(ids: &[String]) -> Vec<Shape> {
    shapes()
        .into_iter()
        .filter(|s| ids.iter().any(|id| id == s.id))
        .collect()
}

/// An older, still-active incident of another kind on another service: the
/// d3 shape, where a corpus line sat in front of a retry storm.
fn corpus_match_incident() -> Incident {
    let opened = RENDER_NOW_UNIX_NANO - CORPUS_MATCH_AGE_NANOS;
    Incident {
        id: 7,
        workspace: WORKSPACE.to_string(),
        fingerprint: CORPUS_FINGERPRINT.to_string(),
        title: "Error-rate spike: Error Rate Spike in demo-shop".to_string(),
        detail: String::new(),
        kind: CueKind::ErrorRateSpike,
        scope: CueScope::Service,
        scope_id: Some("payment-service".to_string()),
        status: IncidentStatus::Active,
        severity: IncidentSeverity::Warn,
        priority_tier: PriorityTier::Suggested,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: Vec::new(),
            fingerprint_hashes: Vec::new(),
            timestamps_unix_nano: Vec::new(),
        },
        opened_at_unix_nano: opened,
        updated_at_unix_nano: opened,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: None,
        read_at_unix_nano: None,
        resolution_summary_text: None,
    }
}

fn project() -> DigestProjectContext {
    DigestProjectContext {
        workspace_canonical_path: WORKSPACE.to_string(),
        project_name: Some("demo-shop".to_string()),
        vcs_type: Some("git"),
        recent_commits: Vec::new(),
        framework_signals: Vec::new(),
    }
}

fn quantified_cue_summary(cue: &AttentionCue) -> String {
    format!(
        "{} magnitude={:.1}x_baseline absolute_value={:.3} persistence={}",
        cue_summary(cue),
        cue.magnitude,
        cue.absolute_value,
        cue.persistence
    )
}

fn truthful_overall(payload: &str, cue_count: usize, incident_count: usize) -> String {
    payload
        .lines()
        .map(|line| {
            if line.starts_with("OVERALL: ") {
                let word = if cue_count > 0 {
                    "anomalous"
                } else {
                    "nominal"
                };
                format!("OVERALL: {word} ({incident_count} active incident(s); {cue_count} cue(s))")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

/// `decision` / `severity` moved after `hypotheses` in the schema's
/// `properties`, every byte of each block kept.
fn reordered_schema() -> Result<String, String> {
    let s = L4_OUTPUT_JSON_SCHEMA;
    let cut_start = s
        .find("\n    \"decision\": {")
        .ok_or("schema: decision block not found")?;
    let cut_end = s
        .find("\n    \"title\": {")
        .ok_or("schema: title block not found")?;
    if cut_start > cut_end {
        // The embedded schema already lists decision / severity after the
        // analysis (prompt v2.3+), so A5's order is the shipped one.
        return Ok(s.to_string());
    }
    let block = &s[cut_start..cut_end];
    let rest = format!("{}{}", &s[..cut_start], &s[cut_end..]);
    let insert_at = rest
        .find("\n    \"investigation_steps\": {")
        .ok_or("schema: investigation_steps block not found")?;
    let out = format!("{}{}{}", &rest[..insert_at], block, &rest[insert_at..]);
    let parsed: Value =
        serde_json::from_str(&out).map_err(|_| "schema: reordered text is not JSON")?;
    let original: Value =
        serde_json::from_str(s).map_err(|_| "schema: embedded text is not JSON")?;
    if parsed != original {
        return Err("schema: reorder changed content".to_string());
    }
    Ok(out)
}

/// The triggering scope's own lines among a shape's selected incidents, by
/// 1-based position in the block: an incident scoped to the cue's `scope_id`,
/// or one the fingerprint arm keeps on its own (the cue's fingerprint is the
/// window's set here). It is the set the product keeps when its scope arm is
/// narrowed to the triggering scope.
fn own_lines(shape: &Shape, incidents: &[Incident]) -> OwnLines {
    let own = |incident: &Incident| {
        (shape.cue.scope_id.is_some() && incident.scope_id == shape.cue.scope_id)
            || (!incident.fingerprint.is_empty()
                && shape.cue.fingerprint.as_ref() == Some(&incident.fingerprint))
    };
    OwnLines::Positions(
        incidents
            .iter()
            .zip(1..)
            .filter(|(incident, _)| own(incident))
            .map(|(_, position)| position)
            .collect(),
    )
}

/// One arm over one shape. `nb` is `shipped` with the corpus block as it was
/// before the remedy, and a candidate arm is `nb` with its one edit made to
/// the composed prompt's corpus block.
fn prepare(arm: &str, shape: &Shape) -> Result<Prepared, String> {
    let candidate = replay::candidate(arm);
    if candidate.is_none() && arm != BASELINE_ARM {
        let lines = corpus_match_lines(&selected_corpus_incidents(shape));
        return prepare_rendered(arm, shape, &lines);
    }
    let incidents = baseline_corpus_incidents(shape);
    let mut prepared = prepare_rendered("shipped", shape, &corpus_match_lines(&incidents))?;
    prepared.arm = arm.to_string();
    let Some(candidate) = candidate else {
        return Ok(prepared);
    };
    let edited = replay::edit_block(&prepared.prompt, candidate, &own_lines(shape, &incidents))?
        .ok_or_else(|| format!("{arm}: {}", replay::OWN_LINES_UNKNOWN))?;
    validate_prompt_bounded(&edited)
        .map_err(|r| format!("{arm}: prompt rejected: {}", r.label()))?;
    prepared.prompt = edited;
    Ok(prepared)
}

/// A non-candidate arm over one shape, its corpus block rendered from the
/// given lines.
fn prepare_rendered(
    arm: &str,
    shape: &Shape,
    corpus_matches: &[String],
) -> Result<Prepared, String> {
    let summary = if arm == "A3" {
        quantified_cue_summary(&shape.cue)
    } else {
        cue_summary(&shape.cue)
    };
    let cue_ref = DigestCueRef {
        kind: shape.cue.kind,
        priority_tier: shape.cue.priority_tier,
        summary,
        scope: shape.cue.scope,
        fingerprint: shape.cue.fingerprint.clone(),
        scope_id: shape.cue.scope_id.clone(),
    };
    let cues = [cue_ref];
    let mut payload = render_payload(
        TIER1_WINDOW,
        TIER1_MODE_LABEL,
        &project(),
        &shape.services,
        &cues,
        corpus_matches,
        &vec![String::new(); shape.active_incidents],
        false,
    );
    if arm == "A2" {
        payload = truthful_overall(&payload, cues.len(), shape.active_incidents);
    }
    if arm == "nf" {
        payload = remove_lines(&payload, is_trigger_line, 1, "TRIGGER line")?;
        let notes = usize::from(!corpus_matches.is_empty());
        payload = remove_lines(&payload, is_framing_note_line, notes, "corpus framing note")?;
    }
    if matches!(arm, "L" | "LI") {
        payload = scope_trigger_line(&payload, shape.cue.scope_id.as_deref())?;
    }
    let citable = vec![STORM_FINGERPRINT.to_string()];
    let mut prompt =
        build_primary_tier_prompt(&payload, &format!("workspace={WORKSPACE}"), "", &citable);
    if arm == "nf" {
        prompt = remove_lines(
            &prompt,
            is_framing_instruction_line,
            1,
            "framing instruction",
        )?;
    }
    let mut schema = L4_OUTPUT_JSON_SCHEMA.to_string();
    let mut extra_args = Vec::new();
    let mut drop_flags = Vec::new();
    match arm {
        "A1" => extra_args.extend(["--temp".to_string(), "0".to_string()]),
        "nr" => drop_flags.push("-rea"),
        "A4" => {
            if prompt.matches(A4_ANCHOR).count() != 1 {
                return Err("A4: conventions anchor not found exactly once".to_string());
            }
            prompt = prompt.replacen(A4_ANCHOR, &format!("{A4_ANCHOR}{A4_SENTENCE}"), 1);
        }
        "A5" => {
            if prompt.matches(L4_OUTPUT_JSON_SCHEMA).count() != 1 {
                return Err("A5: embedded schema not found exactly once".to_string());
            }
            schema = reordered_schema()?;
            prompt = prompt.replacen(L4_OUTPUT_JSON_SCHEMA, &schema, 1);
        }
        "ns" | "L" => prompt = set_scope_sentence(&prompt, false)?,
        "LI" => prompt = set_scope_sentence(&prompt, true)?,
        _ => {}
    }
    for part in candidate_parts(arm) {
        match *part {
            "R1" => prompt = apply_r1(&prompt)?,
            "R3" => prompt = apply_r3(&prompt)?,
            "R2" => (prompt, schema) = apply_r2(&prompt, &schema)?,
            _ => {}
        }
    }
    validate_prompt_bounded(&prompt)
        .map_err(|r| format!("{arm}: prompt rejected: {}", r.label()))?;
    Ok(Prepared {
        arm: arm.to_string(),
        shape: shape.id,
        trigger: shape.cue.kind,
        scope_id: shape.cue.scope_id.clone(),
        prompt,
        schema,
        extra_args,
        drop_flags,
    })
}

fn candidate_parts(arm: &str) -> &'static [&'static str] {
    match arm {
        "R1" => &["R1"],
        "R3" => &["R3"],
        "R2" => &["R2"],
        "R1R3" => &["R1", "R3"],
        "R1R2" => &["R1", "R2"],
        "R3R2" => &["R3", "R2"],
        _ => &[],
    }
}

/// Whether a line is the v2.5 framing instruction, alone or followed by the
/// scope sentence: the two forms that carry R1's text.
fn is_known_framing_line(line: &str) -> bool {
    line.strip_prefix(V25_FRAMING_INSTRUCTION)
        .is_some_and(|rest| rest.is_empty() || rest.strip_prefix(' ') == Some(SCOPE_SENTENCE))
}

/// Every whole line matching `target` replaced by `replacement(line)`, each
/// line's own terminator kept.
fn replace_lines(
    text: &str,
    target: fn(&str) -> bool,
    replacement: impl Fn(&str) -> String,
) -> String {
    text.split_inclusive('\n')
        .map(|l| {
            let body = l.trim_end_matches('\n');
            if target(body) {
                format!("{}{}", replacement(body), &l[body.len()..])
            } else {
                l.to_string()
            }
        })
        .collect()
}

/// R1: the framing instruction line replaced by the R1 text. A line that
/// already carries it, alone or ahead of the scope sentence, is left as is.
fn apply_r1(prompt: &str) -> Result<String, String> {
    match count_matching_lines(prompt, is_known_framing_line) {
        1 => return Ok(prompt.to_string()),
        0 => {}
        hits => return Err(format!("R1: text found {hits} times")),
    }
    let hits = count_matching_lines(prompt, is_framing_instruction_line);
    if hits != 1 {
        return Err(format!(
            "R1: framing instruction found {hits} times, expected 1"
        ));
    }
    Ok(replace_lines(prompt, is_framing_instruction_line, |_| {
        R1_FRAMING_INSTRUCTION.to_string()
    }))
}

/// ns / L / LI: the prompt's one framing instruction line set to the v2.5
/// text with (`carry`) or without the scope sentence, whichever of the two
/// the tree's instruction is.
fn set_scope_sentence(prompt: &str, carry: bool) -> Result<String, String> {
    let hits = count_matching_lines(prompt, is_known_framing_line);
    if hits != 1 {
        return Err(format!(
            "scope sentence: known framing instruction found {hits} times, expected 1"
        ));
    }
    Ok(replace_lines(prompt, is_known_framing_line, |_| {
        if carry {
            format!("{V25_FRAMING_INSTRUCTION} {SCOPE_SENTENCE}")
        } else {
            V25_FRAMING_INSTRUCTION.to_string()
        }
    }))
}

/// L / LI: the digest's one TRIGGER line gains ` on {scope_id}`.
fn scope_trigger_line(payload: &str, scope_id: Option<&str>) -> Result<String, String> {
    let scope_id = scope_id.ok_or("L: the cue carries no scope_id")?;
    let hits = count_matching_lines(payload, is_trigger_line);
    if hits != 1 {
        return Err(format!("L: TRIGGER line found {hits} times, expected 1"));
    }
    Ok(replace_lines(payload, is_trigger_line, |line| {
        format!("{line} on {scope_id}")
    }))
}

/// R3: the conventions sentence inserted before the investigation-steps one.
fn apply_r3(prompt: &str) -> Result<String, String> {
    match prompt.matches(R3_CONVENTIONS_SENTENCE).count() {
        1 => return Ok(prompt.to_string()),
        0 => {}
        hits => return Err(format!("R3: text found {hits} times")),
    }
    if prompt.matches(R3_ANCHOR).count() != 1 {
        return Err("R3: conventions anchor not found exactly once".to_string());
    }
    Ok(prompt.replacen(
        R3_ANCHOR,
        &format!("{R3_CONVENTIONS_SENTENCE}{R3_ANCHOR}"),
        1,
    ))
}

/// The schema with R2's sentence appended to the `hypotheses` description,
/// checked to differ from `schema` in that one string only.
fn r2_schema(schema: &str) -> Result<String, String> {
    let extended = format!("{R2_ANCHOR}{R2_SCHEMA_SENTENCE}");
    match schema.matches(&extended).count() {
        1 => return Ok(schema.to_string()),
        0 => {}
        hits => return Err(format!("R2: text found {hits} times")),
    }
    if schema.matches(R2_ANCHOR).count() != 1 {
        return Err("R2: hypotheses description not found exactly once".to_string());
    }
    let out = schema.replacen(R2_ANCHOR, &extended, 1);
    let mut parsed: Value =
        serde_json::from_str(&out).map_err(|_| "R2: edited schema is not JSON")?;
    let original: Value = serde_json::from_str(schema).map_err(|_| "R2: schema is not JSON")?;
    let description = parsed
        .pointer_mut("/properties/hypotheses/description")
        .ok_or("R2: hypotheses description not at its path")?;
    if *description != Value::String(extended) {
        return Err("R2: the sentence landed outside the hypotheses description".to_string());
    }
    *description = Value::String(R2_ANCHOR.to_string());
    if parsed != original {
        return Err("R2: edit changed more than the hypotheses description".to_string());
    }
    Ok(out)
}

/// R2: both schema copies carry the sentence — the arm's own copy and the one
/// embedded in the prompt. Only the prompt copy reaches the model: the argv's
/// grammar carries no description.
fn apply_r2(prompt: &str, schema: &str) -> Result<(String, String), String> {
    let edited = r2_schema(schema)?;
    if edited == schema {
        return Ok((prompt.to_string(), edited));
    }
    if prompt.matches(schema).count() != 1 {
        return Err("R2: embedded schema not found exactly once".to_string());
    }
    Ok((prompt.replacen(schema, &edited, 1), edited))
}

fn count_matching_lines(text: &str, target: fn(&str) -> bool) -> usize {
    text.lines().filter(|l| target(l)).count()
}

fn is_trigger_line(line: &str) -> bool {
    line.starts_with(TRIGGER_LINE_PREFIX)
}

fn is_framing_note_line(line: &str) -> bool {
    line.strip_prefix("  ") == Some(CORPUS_MATCHES_FRAMING_NOTE)
}

fn is_framing_instruction_line(line: &str) -> bool {
    line == TRIGGER_FRAMING_INSTRUCTION
}

/// Drops every whole line matching `target`, after checking it occurs exactly
/// `expected` times (the A4/A5 exactly-once transform discipline).
fn remove_lines(
    text: &str,
    target: fn(&str) -> bool,
    expected: usize,
    what: &str,
) -> Result<String, String> {
    let hits = text.lines().filter(|l| target(l)).count();
    if hits != expected {
        return Err(format!(
            "nf: {what} found {hits} times, expected {expected}"
        ));
    }
    Ok(text
        .split_inclusive('\n')
        .filter(|l| !target(l.trim_end_matches('\n')))
        .collect())
}

/// Case-insensitive ASCII terms that name each cue kind in model text.
fn trigger_terms(kind: CueKind) -> &'static [&'static str] {
    match kind {
        CueKind::RetryStorm => &["retry"],
        CueKind::ErrorRateSpike => &["error rate", "error-rate"],
        CueKind::LatencyRegression => &["latency"],
        CueKind::RestartEvent => &["restart"],
        CueKind::ServiceWentSilent => &["silent", "silence"],
        CueKind::ReflectionTrend => &["trend"],
    }
}

/// Whether a parsed interpretation names its triggering cue: in the rank-1
/// hypothesis (`rank1`), only in the title, the symptom or a later hypothesis
/// (`elsewhere`), or nowhere (`none`). Reads model text in-process only.
fn names_trigger(output: &L4Output, kind: CueKind) -> &'static str {
    let terms = trigger_terms(kind);
    let names = |text: &str| {
        let lower = text.to_ascii_lowercase();
        terms.iter().any(|t| lower.contains(t))
    };
    if output
        .hypotheses
        .first()
        .is_some_and(|h| names(&h.statement))
    {
        return "rank1";
    }
    let later = output
        .hypotheses
        .iter()
        .skip(1)
        .any(|h| names(&h.statement));
    if names(&output.title) || names(&output.symptom) || later {
        "elsewhere"
    } else {
        "none"
    }
}

fn names_trigger_label(parsed: Option<&L4Output>, kind: CueKind) -> &'static str {
    parsed.map_or("unparsed", |out| names_trigger(out, kind))
}

/// `trigger_terms` widened to the stem forms a substring `retry` misses
/// (`retrying` and `retry_storm` already contain it).
fn stem_terms(kind: CueKind) -> &'static [&'static str] {
    match kind {
        CueKind::RetryStorm => &["retry", "retries", "retried"],
        other => trigger_terms(other),
    }
}

/// `names_trigger` over `stem_terms`: a record-only reading that never
/// feeds a verdict.
fn names_trigger_stem(output: &L4Output, kind: CueKind) -> &'static str {
    let terms = stem_terms(kind);
    let names = |text: &str| {
        let lower = text.to_ascii_lowercase();
        terms.iter().any(|t| lower.contains(t))
    };
    if output
        .hypotheses
        .first()
        .is_some_and(|h| names(&h.statement))
    {
        return "rank1";
    }
    let later = output
        .hypotheses
        .iter()
        .skip(1)
        .any(|h| names(&h.statement));
    if names(&output.title) || names(&output.symptom) || later {
        "elsewhere"
    } else {
        "none"
    }
}

fn names_trigger_stem_label(parsed: Option<&L4Output>, kind: CueKind) -> &'static str {
    parsed.map_or("unparsed", |out| names_trigger_stem(out, kind))
}

/// `word` in ASCII-lowercased text with neither neighbour in `[a-z0-9_-]`, so
/// a hyphen-joined or underscore-joined sibling does not name it. Its own
/// function: `patterns::names_service` is a substring match.
fn names_whole_word(lower: &str, word: &str) -> bool {
    if word.is_empty() {
        return false;
    }
    let bytes = lower.as_bytes();
    let joins = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-';
    lower.match_indices(word).any(|(at, hit)| {
        let before = at.checked_sub(1).map(|i| bytes[i]);
        let after = bytes.get(at + hit.len()).copied();
        !before.is_some_and(joins) && !after.is_some_and(joins)
    })
}

/// A maximal run of ASCII letters equal to `retry`, `retries` or `retrying`:
/// `retry_storm` splits into `retry` and `storm`; `retried` and
/// `non-retryable` yield none.
fn names_retry_token(lower: &str) -> bool {
    lower
        .split(|c: char| !c.is_ascii_alphabetic())
        .any(|run| matches!(run, "retry" | "retries" | "retrying"))
}

/// The pre-registered label of one generation, read over the FIRST hypothesis
/// statement only, ASCII-lowercased: whether it names the triggering cue's
/// `scope_id` as a whole word and a retry token. Stated limits: negation is
/// not read, a space-separated sibling passes, `retried` is not a token.
fn identifies(parsed: Option<&L4Output>, scope_id: Option<&str>) -> &'static str {
    let Some(first) = parsed.and_then(|out| out.hypotheses.first()) else {
        return "unparsed";
    };
    let lower = first.statement.to_ascii_lowercase();
    let service = scope_id.is_some_and(|id| names_whole_word(&lower, &id.to_ascii_lowercase()));
    match (service, names_retry_token(&lower)) {
        (true, true) => "both",
        (true, false) => "service_only",
        (false, true) => "signal_only",
        (false, false) => "neither",
    }
}

/// The second count's label for one generation, read over the FIRST
/// hypothesis statement only, ASCII-lowercased: whether it holds `other` as a
/// whole word, a `-`, `_` or space inside `other` read as any of the three.
/// Stated limit: it reads no placement, so any mention counts.
fn names_other(parsed: Option<&L4Output>, other: &str) -> &'static str {
    let Some(first) = parsed.and_then(|out| out.hypotheses.first()) else {
        return "unparsed";
    };
    let lower = first.statement.to_ascii_lowercase();
    let name = other.to_ascii_lowercase();
    let named = ['-', '_', ' '].into_iter().any(|joint| {
        let spelling = name.replace(['-', '_', ' '], &joint.to_string());
        names_whole_word(&lower, &spelling)
    });
    if named { "named" } else { "not_named" }
}

/// One generation's parsed output, by the steps `read_labels` takes.
fn parsed_output(outcome: &Spawned) -> Option<L4Output> {
    let Spawned::Output(text, _) = outcome else {
        return None;
    };
    let obj = extract_json_object_bounded(text).ok()?;
    parse_bounded(obj.as_bytes()).ok()
}

fn fold_named(counts: &mut NamedCounts, names_other: &str) {
    counts.n += 1;
    match names_other {
        "named" => counts.named += 1,
        "unparsed" => counts.unparsed += 1,
        _ => {}
    }
}

fn second_count_line(arm: &str, shape: &str, counts: NamedCounts) -> String {
    format!(
        "  arm {arm}: second count {shape} · other scope named {}/{} · unparsed {}",
        counts.named, counts.n, counts.unparsed,
    )
}

/// A `runs.json` row with the second count's label beside the others.
fn with_names_other(mut row: Value, names_other: &'static str) -> Value {
    row["names_other"] = json!(names_other);
    row
}

/// The bounded labels kept for one generation. No field holds model text.
struct RunLabels {
    decision: &'static str,
    severity: &'static str,
    is_resolution_summary: bool,
    first_keys: Vec<String>,
    output_hash: Option<u64>,
    names_trigger: &'static str,
    names_trigger_stem: &'static str,
    identifies: &'static str,
}

impl RunLabels {
    fn would_create(&self) -> bool {
        matches!(self.decision, "surface" | "watch")
            && self.severity != "none"
            && !self.is_resolution_summary
    }
}

/// Reads one generation's outcome down to its labels, in-process.
fn read_labels(outcome: &Spawned, p: &Prepared) -> RunLabels {
    let scope_id = p.scope_id.as_deref();
    let mut labels = RunLabels {
        decision: "parse_failed",
        severity: "",
        is_resolution_summary: false,
        first_keys: Vec::new(),
        output_hash: None,
        names_trigger: names_trigger_label(None, p.trigger),
        names_trigger_stem: names_trigger_stem_label(None, p.trigger),
        identifies: identifies(None, scope_id),
    };
    let text = match outcome {
        Spawned::Output(text, _) => text,
        Spawned::Failed(why, _) => {
            labels.decision = *why;
            return labels;
        }
    };
    let Ok(obj) = extract_json_object_bounded(text) else {
        return labels;
    };
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    labels.output_hash = Some(hasher.finish());
    labels.first_keys = first_keys(obj);
    if let Ok(out) = parse_bounded(obj.as_bytes()) {
        labels.decision = decision_label(out.decision);
        labels.severity = severity_label(out.severity);
        labels.is_resolution_summary = out.is_resolution_summary;
        labels.names_trigger = names_trigger_label(Some(&out), p.trigger);
        labels.names_trigger_stem = names_trigger_stem_label(Some(&out), p.trigger);
        labels.identifies = identifies(Some(&out), scope_id);
    }
    labels
}

/// One generation's `runs.json` row.
fn row_json(p: &Prepared, run: u32, labels: &RunLabels, thinking: &str, metrics: Metrics) -> Value {
    json!({
        "arm": p.arm,
        "shape": p.shape,
        "run": run,
        "decision": labels.decision,
        "severity": labels.severity,
        "is_resolution_summary": labels.is_resolution_summary,
        "would_create": labels.would_create(),
        "first_keys": labels.first_keys,
        "output_hash": labels.output_hash.map(|d| format!("{d:016x}")),
        "names_trigger": labels.names_trigger,
        "names_trigger_stem": labels.names_trigger_stem,
        "identifies": labels.identifies,
        "thinking": thinking,
        "elapsed_ms": metrics.elapsed_ms,
        "peak_rss_kib": metrics.peak_rss_kib,
        "peak_vram_mib": metrics.peak_vram_mib,
    })
}

/// One generation's stderr line.
fn run_line(p: &Prepared, run: u32, labels: &RunLabels, thinking: &str, elapsed_ms: u64) -> String {
    format!(
        "l4-decision-probe: {} {} run {run}: decision {} severity {} would_create {} names_trigger {} names_trigger_stem {} identifies {} thinking {thinking} elapsed_ms {elapsed_ms}",
        p.arm,
        p.shape,
        labels.decision,
        if labels.severity.is_empty() {
            "-"
        } else {
            labels.severity
        },
        labels.would_create(),
        labels.names_trigger,
        labels.names_trigger_stem,
        labels.identifies,
    )
}

/// An arm's `both` and generation counts per shape, folded onto the two
/// halves of the bar. A shape in neither half counts in neither.
fn bar_counts(both: &BTreeMap<&'static str, u32>, runs: &BTreeMap<&'static str, u32>) -> BarCounts {
    let sum = |counts: &BTreeMap<&'static str, u32>, shapes: &[&str]| -> u32 {
        shapes.iter().filter_map(|id| counts.get(id)).sum()
    };
    BarCounts {
        sibling_both: sum(both, &SIBLING_SHAPES),
        sibling_n: sum(runs, &SIBLING_SHAPES),
        ordinary_both: sum(both, &ORDINARY_SHAPES),
        ordinary_n: sum(runs, &ORDINARY_SHAPES),
    }
}

/// MET iff `both` reaches the minimum on each half. The denominators are the
/// generations run, so an `unparsed` generation is not `both`.
fn bar_met(counts: BarCounts, bar: Bar) -> bool {
    counts.sibling_both >= bar.sibling_min && counts.ordinary_both >= bar.ordinary_min
}

/// The first arm in `SELECTION_ORDER` whose bar is met, or `none`.
fn select_arm(arms: &BTreeMap<String, BarCounts>, bar: Bar) -> &'static str {
    SELECTION_ORDER
        .into_iter()
        .find(|arm| arms.get(*arm).is_some_and(|counts| bar_met(*counts, bar)))
        .unwrap_or("none")
}

/// TRIPPED iff the shipped arm's `both` count is lower than the baseline's on
/// either half of the bar; an equal count holds.
fn regression_guard(shipped: BarCounts, ns: BarCounts) -> &'static str {
    if shipped.sibling_both < ns.sibling_both || shipped.ordinary_both < ns.ordinary_both {
        "TRIPPED"
    } else {
        "HOLDS"
    }
}

fn identifies_line(
    arm: &str,
    counts: &BTreeMap<&'static str, u32>,
    n_arm: u32,
    per_shape_both: &str,
) -> String {
    let count = |k: &str| counts.get(k).copied().unwrap_or(0);
    format!(
        "  arm {arm}: identifies both {}/{n_arm} · service_only {} · signal_only {} · neither {} · unparsed {} · per shape both {per_shape_both}",
        count("both"),
        count("service_only"),
        count("signal_only"),
        count("neither"),
        count("unparsed"),
    )
}

fn bar_line(arm: &str, counts: BarCounts, bar: Bar) -> String {
    format!(
        "  arm {arm}: bar {} · sibling both {}/{} (min {}) · ordinary both {}/{} (min {})",
        if bar_met(counts, bar) {
            "MET"
        } else {
            "NOT MET"
        },
        counts.sibling_both,
        counts.sibling_n,
        bar.sibling_min,
        counts.ordinary_both,
        counts.ordinary_n,
        bar.ordinary_min,
    )
}

/// The selection, service verdict and regression guard lines, in print
/// order, and whether the service verdict passed.
fn service_verdict_lines(arms: &BTreeMap<String, BarCounts>, bar: Bar) -> (Vec<String>, bool) {
    let selection = select_arm(arms, bar);
    let pass = selection == "shipped";
    let of = |arm: &str| arms.get(arm).copied().unwrap_or_default();
    let (shipped, ns) = (of("shipped"), of("ns"));
    let lines = vec![
        format!(
            "l4-decision-probe: selection: {selection} · order {}",
            SELECTION_ORDER.join(",")
        ),
        format!(
            "l4-decision-probe: service verdict: {} · arm shipped",
            if pass { "PASS" } else { "FAIL" }
        ),
        format!(
            "l4-decision-probe: regression guard: {} · sibling shipped {} vs ns {} · ordinary shipped {} vs ns {}",
            regression_guard(shipped, ns),
            shipped.sibling_both,
            ns.sibling_both,
            shipped.ordinary_both,
            ns.ordinary_both,
        ),
    ];
    (lines, pass)
}

/// One generation's `identifies` label folded into its shape's counts. A
/// service miss parsed and did not name the cue's `scope_id` (`signal_only`
/// or `neither`); `unparsed` is counted apart and is never a miss.
fn fold_miss(counts: &mut MissCounts, identifies: &str) {
    counts.n += 1;
    match identifies {
        "signal_only" | "neither" => counts.misses += 1,
        "unparsed" => counts.unparsed += 1,
        _ => {}
    }
}

/// REPRODUCES at `k` or more service misses; CLEAN when the misses and the
/// `unparsed` rows together stay below `k`, so no unparsed row could have made
/// it reproduce; UNREAD otherwise, and for a shape with no generation.
fn reproduce_shape(counts: MissCounts, k: u32) -> &'static str {
    if counts.n == 0 {
        "UNREAD"
    } else if counts.misses >= k {
        "REPRODUCES"
    } else if counts.misses + counts.unparsed < k {
        "CLEAN"
    } else {
        "UNREAD"
    }
}

fn reproduce_line(shape: &str, counts: MissCounts, k: u32) -> String {
    format!(
        "  arm shipped: reproduce {shape} {} · misses {}/{} · unparsed {}",
        reproduce_shape(counts, k),
        counts.misses,
        counts.n,
        counts.unparsed,
    )
}

/// The reproduction reading's last line and exit code, over the shapes of
/// `REPRODUCTION_SHAPES` alone: REPRODUCED (0) when one reproduces, NOT
/// REPRODUCED (1) when all eight read CLEAN, INCONCLUSIVE (2) otherwise. A
/// shape that was not run is unread. The controls are not read here.
fn reproduction_verdict(shapes: &BTreeMap<&'static str, MissCounts>, k: u32) -> (String, u8) {
    let read = |id: &str| reproduce_shape(shapes.get(id).copied().unwrap_or_default(), k);
    let with = |word: &str| -> Vec<&'static str> {
        REPRODUCTION_SHAPES
            .into_iter()
            .filter(|id| read(id) == word)
            .collect()
    };
    let reproducing = with("REPRODUCES");
    if !reproducing.is_empty() {
        return (
            format!(
                "l4-decision-probe: reproduction verdict: REPRODUCED · shapes {}",
                reproducing.join(",")
            ),
            0,
        );
    }
    let unread = with("UNREAD");
    if unread.is_empty() {
        return (
            "l4-decision-probe: reproduction verdict: NOT REPRODUCED · shapes none".to_string(),
            1,
        );
    }
    (
        inconclusive_line(&format!(
            "no shape reproduces and {} unread",
            unread.join(",")
        )),
        2,
    )
}

/// The replay reading's last line and exit code: the per-shape rule over
/// every replay but `control`, the one labelled `miss` first. REPRODUCED (0)
/// when one reproduces, NOT REPRODUCED (1) when all read CLEAN, INCONCLUSIVE
/// (2) otherwise; `miss` is unread when it was not run. The control and the S
/// shapes are not read here.
fn replay_verdict(shapes: &BTreeMap<&'static str, MissCounts>, k: u32) -> (String, u8) {
    let others = shapes
        .keys()
        .copied()
        .filter(|id| replay::is_replay_shape(id) && *id != MISS_SHAPE && *id != CONTROL_SHAPE);
    let read: Vec<(&'static str, &'static str)> = std::iter::once(MISS_SHAPE)
        .chain(others)
        .map(|id| {
            let counts = shapes.get(id).copied().unwrap_or_default();
            (id, reproduce_shape(counts, k))
        })
        .collect();
    let with = |word: &str| -> Vec<&'static str> {
        read.iter()
            .filter(|(_, reading)| *reading == word)
            .map(|(id, _)| *id)
            .collect()
    };
    let reproducing = with("REPRODUCES");
    if !reproducing.is_empty() {
        return (
            format!(
                "l4-decision-probe: reproduction verdict: REPRODUCED · {}",
                reproducing.join(",")
            ),
            0,
        );
    }
    let unread = with("UNREAD");
    if unread.is_empty() {
        return (
            format!(
                "l4-decision-probe: reproduction verdict: NOT REPRODUCED · {}",
                with("CLEAN").join(",")
            ),
            1,
        );
    }
    (
        inconclusive_line(&format!("{} unread", unread.join(","))),
        2,
    )
}

/// One generation's `identifies` label folded into its arm's tally for one
/// source.
fn fold_tally(tally: &mut Tally, identifies: &str) {
    tally.n += 1;
    match identifies {
        "both" => tally.both += 1,
        "signal_only" | "neither" => tally.misses += 1,
        "unparsed" => tally.unparsed += 1,
        _ => {}
    }
}

/// The generations the remedy prompt runs per arm, from m service misses of
/// 20 in the replay reading: max(20, ceil(100 / m)), so the baseline is
/// expected to miss at least 5 times. `None` below the reproduction minimum.
fn remedy_n(misses: u32) -> Option<u32> {
    (misses >= REPLAY_REPRODUCE_MISSES).then(|| 100u32.div_ceil(misses).max(REPLAY_READING_N))
}

/// The generations that may be other than `both` on the remedy prompt.
fn remedy_allowance(n: u32) -> u32 {
    n / 20
}

/// HELD iff `shipped` misses the service at least the allowance plus 2 times
/// on the remedy prompt in this run: one miss over a passing candidate is a
/// margin a reading cannot tell from noise.
fn known_positive_held(shipped_misses: u32, n: u32) -> bool {
    shipped_misses >= remedy_allowance(n) + 2
}

/// MET iff the generations that are not `both` stay within the allowance on
/// the remedy prompt and `both` reaches the minimum on each guard half.
fn remedy_bar_met(counts: RemedyCounts, allowance: u32, bar: Bar) -> bool {
    counts.remedy.not_both() <= allowance
        && counts.sibling.both >= bar.sibling_min
        && counts.ordinary.both >= bar.ordinary_min
}

/// TRIPPED iff the candidate's `both` count is lower than `shipped`'s in this
/// run on either guard half; an equal count holds.
fn remedy_guard(counts: RemedyCounts, shipped: RemedyCounts) -> &'static str {
    if counts.sibling.both < shipped.sibling.both || counts.ordinary.both < shipped.ordinary.both {
        "TRIPPED"
    } else {
        "HOLDS"
    }
}

/// The first candidate, in `CANDIDATE_ARMS` order, whose bar is met and whose
/// guard holds. A skipped arm is never selectable.
fn remedy_selection(
    candidates: &[(&'static str, RemedyArm)],
    shipped: RemedyCounts,
    allowance: u32,
    bar: Bar,
) -> Option<&'static str> {
    replay::CANDIDATE_ARMS.into_iter().find(|arm| {
        candidates.iter().any(|(name, state)| {
            name == arm
                && matches!(state, RemedyArm::Read(counts)
                    if remedy_bar_met(*counts, allowance, bar)
                        && remedy_guard(*counts, shipped) == "HOLDS")
        })
    })
}

/// What one candidate arm generates in the remedy reading, read off the
/// compositions: skipped when it has none on the remedy prompt (its own lines
/// are unknown) or composes there byte for byte as `shipped`; otherwise run,
/// with `shipped`'s counts taken on each S shape it composes identically.
fn remedy_arm_plan(prepared: &[Prepared], arm: &str) -> ArmPlan {
    let prompt = |arm: &str, shape: &str| {
        prepared
            .iter()
            .find(|p| p.arm == arm && p.shape == shape)
            .map(|p| p.prompt.as_str())
    };
    let Some(on_miss) = prompt(arm, MISS_SHAPE) else {
        return ArmPlan::Skipped(replay::OWN_LINES_UNKNOWN);
    };
    if prompt("shipped", MISS_SHAPE) == Some(on_miss) {
        return ArmPlan::Skipped("composes as shipped on replay:miss");
    }
    ArmPlan::Run(
        prepared
            .iter()
            .filter(|p| p.arm == arm && !replay::is_replay_shape(p.shape))
            .filter(|p| prompt("shipped", p.shape) == Some(p.prompt.as_str()))
            .map(|p| p.shape)
            .collect(),
    )
}

/// One arm's counts in the remedy reading, each `reused` shape read from
/// `shipped`'s generations.
fn remedy_counts(
    tallies: &BTreeMap<(String, &'static str), Tally>,
    arm: &str,
    reused: &[&'static str],
    derived: bool,
) -> RemedyCounts {
    let of = |shape: &'static str| {
        let from = if reused.contains(&shape) {
            "shipped"
        } else {
            arm
        };
        tallies
            .get(&(from.to_string(), shape))
            .copied()
            .unwrap_or_default()
    };
    let over = |shapes: &[&'static str]| {
        shapes
            .iter()
            .fold(Tally::default(), |sum, shape| sum.plus(of(shape)))
    };
    RemedyCounts {
        remedy: of(MISS_SHAPE),
        sibling: over(&SIBLING_SHAPES),
        ordinary: over(&ORDINARY_SHAPES),
        derived: derived.then(|| of(DERIVED_SHAPE)),
    }
}

/// The derived shape's two words: how `shipped` reads on it by the per-shape
/// rule, and whether the selected candidate covers it. Without a selection,
/// or on a shape that does not reproduce, nothing is read against one.
fn derived_reading(shipped: Tally, selected: Option<Tally>) -> (&'static str, &'static str) {
    let counts = MissCounts {
        misses: shipped.misses,
        unparsed: shipped.unparsed,
        n: shipped.n,
    };
    let reads = reproduce_shape(counts, REPLAY_REPRODUCE_MISSES);
    let cover = match selected {
        Some(tally) if reads == "REPRODUCES" => {
            if tally.not_both() <= DERIVED_COVERED_MAX {
                "COVERED"
            } else {
                "NOT COVERED"
            }
        }
        _ => "not read against a selection",
    };
    (reads, cover)
}

fn remedy_arm_line(
    arm: &str,
    state: RemedyArm,
    shipped: RemedyCounts,
    allowance: u32,
    bar: Bar,
) -> String {
    let counts = match state {
        RemedyArm::Skipped(reason) => return format!("  arm {arm}: skipped · {reason}"),
        RemedyArm::Read(counts) => counts,
    };
    format!(
        "  arm {arm}: remedy bar {} · guard {} · not both {}/{} (allowance {allowance}) · sibling both {}/{} (min {}, shipped {}) · ordinary both {}/{} (min {}, shipped {})",
        if remedy_bar_met(counts, allowance, bar) {
            "MET"
        } else {
            "NOT MET"
        },
        remedy_guard(counts, shipped),
        counts.remedy.not_both(),
        counts.remedy.n,
        counts.sibling.both,
        counts.sibling.n,
        bar.sibling_min,
        shipped.sibling.both,
        counts.ordinary.both,
        counts.ordinary.n,
        bar.ordinary_min,
        shipped.ordinary.both,
    )
}

fn derived_line(
    shipped: Option<Tally>,
    candidates: &[(&'static str, RemedyArm)],
    selection: Option<&str>,
) -> String {
    let Some(shipped) = shipped else {
        return "l4-decision-probe: derived shape: absent".to_string();
    };
    let on_derived = |state: &RemedyArm| match state {
        RemedyArm::Read(counts) => counts.derived,
        RemedyArm::Skipped(_) => None,
    };
    let selected = selection
        .and_then(|arm| candidates.iter().find(|(name, _)| *name == arm))
        .and_then(|(_, state)| on_derived(state));
    let (reads, cover) = derived_reading(shipped, selected);
    let read: Vec<String> = candidates
        .iter()
        .filter_map(|(arm, state)| {
            on_derived(state).map(|t| format!("{arm} {}/{}", t.not_both(), t.n))
        })
        .collect();
    format!(
        "l4-decision-probe: derived shape: {DERIVED_SHAPE} {reads} · {cover} · shipped misses {}/{} · unparsed {} · not both {}",
        shipped.misses,
        shipped.n,
        shipped.unparsed,
        listed(read),
    )
}

/// The remedy reading's lines after its generations, in print order, and its
/// exit code: SELECTED (0), NONE (1), or INCONCLUSIVE (2) when the
/// known-positive is lost. The derived shape is recorded and moves neither the
/// selection nor the exit.
fn remedy_reading(
    shipped: RemedyCounts,
    candidates: &[(&'static str, RemedyArm)],
    n: u32,
    bar: Bar,
) -> (Vec<String>, u8) {
    let allowance = remedy_allowance(n);
    let mut lines: Vec<String> = candidates
        .iter()
        .map(|(arm, state)| remedy_arm_line(arm, *state, shipped, allowance, bar))
        .collect();
    let held = known_positive_held(shipped.remedy.misses, n);
    lines.push(format!(
        "l4-decision-probe: known-positive: {} · shipped misses {}/{} on {MISS_SHAPE} (min {})",
        if held { "HELD" } else { "LOST" },
        shipped.remedy.misses,
        shipped.remedy.n,
        allowance + 2,
    ));
    if !held {
        lines.push(derived_line(shipped.derived, candidates, None));
        lines.push(inconclusive_line(&format!(
            "the known-positive is lost on {MISS_SHAPE}"
        )));
        return (lines, 2);
    }
    let selection = remedy_selection(candidates, shipped, allowance, bar);
    lines.push(format!(
        "l4-decision-probe: remedy selection: {} · order {}",
        selection.unwrap_or("none"),
        replay::CANDIDATE_ARMS.join(",")
    ));
    lines.push(derived_line(shipped.derived, candidates, selection));
    match selection {
        Some(arm) => {
            lines.push(format!(
                "l4-decision-probe: remedy verdict: SELECTED · arm {arm}"
            ));
            (lines, 0)
        }
        None => {
            lines.push("l4-decision-probe: remedy verdict: NONE".to_string());
            (lines, 1)
        }
    }
}

/// m for the remedy reading: the service misses of `replay:miss` under
/// `shipped` in a replay reading's rows, refused unless the reading holds
/// exactly `REPLAY_READING_N` such rows.
fn replay_reading_misses(rows: &[Value], read_as: &str) -> Result<u32, String> {
    let shape = format!("{}{read_as}", replay::REPLAY_SHAPE_PREFIX);
    let of_miss: Vec<&Value> = rows
        .iter()
        .filter(|row| row["arm"] == "shipped" && row["shape"] == shape.as_str())
        .collect();
    if of_miss.len() != REPLAY_READING_N as usize {
        return Err(format!(
            "the replay reading holds {} rows for {shape}, expected {REPLAY_READING_N}",
            of_miss.len()
        ));
    }
    let mut counts = MissCounts::default();
    for row in of_miss {
        fold_miss(
            &mut counts,
            row["identifies"].as_str().unwrap_or("unparsed"),
        );
    }
    Ok(counts.misses)
}

/// The first three top-level keys of a JSON object, read off its text.
fn first_keys(json_text: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut current = String::new();
    let mut last_string: Option<String> = None;
    for c in json_text.chars() {
        if in_string {
            if escaped {
                escaped = false;
                current.push(c);
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
                last_string = Some(std::mem::take(&mut current));
            } else {
                current.push(c);
            }
            continue;
        }
        match c {
            '"' => in_string = true,
            '{' | '[' => {
                depth += 1;
                last_string = None;
            }
            '}' | ']' => depth = depth.saturating_sub(1),
            ':' if depth == 1 => {
                if let Some(key) = last_string.take() {
                    keys.push(key);
                    if keys.len() == 3 {
                        break;
                    }
                }
            }
            c if !c.is_whitespace() => last_string = None,
            _ => {}
        }
    }
    keys
}

fn decision_label(d: Decision) -> &'static str {
    match d {
        Decision::Surface => "surface",
        Decision::Dismiss => "dismiss",
        Decision::Watch => "watch",
    }
}

fn severity_label(s: Severity) -> &'static str {
    match s {
        Severity::Autonomous => "autonomous",
        Severity::Suggested => "suggested",
        Severity::Curious => "curious",
        Severity::None => "none",
    }
}

/// Footprint readings of one generation; `None` is an unread value.
#[derive(Clone, Copy)]
struct Metrics {
    elapsed_ms: u64,
    peak_rss_kib: Option<u64>,
    peak_vram_mib: Option<u64>,
}

enum Spawned {
    Output(String, Metrics),
    Failed(&'static str, Metrics),
}

/// `present` when the raw stdout carries the b9305 thinking marker, `absent`
/// when it does not, `unread` when no stdout was captured.
fn thinking_label(stdout: Option<&str>) -> &'static str {
    match stdout {
        Some(text) if text.contains(THINKING_MARKER) => "present",
        Some(_) => "absent",
        None => "unread",
    }
}

/// The `VmHWM` (peak resident set) value of a `/proc/{pid}/status` text, in KiB.
fn parse_vm_hwm_kib(status: &str) -> Option<u64> {
    status
        .lines()
        .find_map(|l| l.strip_prefix("VmHWM:"))
        .and_then(|v| v.trim().strip_suffix("kB"))
        .and_then(|v| v.trim().parse().ok())
}

/// The largest `used_memory` (MiB) listed for `pid` in
/// `nvidia-smi --query-compute-apps=pid,used_memory --format=csv,noheader,nounits`.
fn parse_vram_mib(csv: &str, pid: u32) -> Option<u64> {
    csv.lines()
        .filter_map(|l| l.split_once(','))
        .filter(|(p, _)| p.trim().parse::<u32>().ok() == Some(pid))
        .filter_map(|(_, used)| used.trim().parse::<u64>().ok())
        .max()
}

fn read_vm_hwm_kib(pid: u32) -> Option<u64> {
    if cfg!(target_os = "linux") {
        std::fs::read_to_string(format!("/proc/{pid}/status"))
            .ok()
            .as_deref()
            .and_then(parse_vm_hwm_kib)
    } else {
        None
    }
}

async fn read_vram_mib(pid: u32) -> Option<u64> {
    let out = tokio::process::Command::new("nvidia-smi")
        .args(NVIDIA_SMI_ARGS)
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .await
        .ok()?;
    parse_vram_mib(std::str::from_utf8(&out.stdout).ok()?, pid)
}

fn record_peak(peak: &Mutex<Option<u64>>, value: Option<u64>) {
    if let (Some(v), Ok(mut p)) = (value, peak.lock()) {
        *p = Some(p.map_or(v, |old| old.max(v)));
    }
}

fn read_peak(peak: &Mutex<Option<u64>>) -> Option<u64> {
    peak.lock().ok().and_then(|p| *p)
}

/// The production argv for one generation, with the arm's dropped flags
/// (each with its value) removed and its extra flag/value pairs applied: a
/// pair whose flag the argv already carries replaces that value, any other
/// is appended, so no flag is passed twice.
fn compose_argv(prepared: &Prepared, model: &Path, ngl: u32, grammar_path: &Path) -> Vec<String> {
    let mut args = build_llama_cli_args(
        model,
        ngl,
        DEFAULT_MAX_TOKENS,
        grammar_path,
        &prepared.prompt,
    );
    for flag in &prepared.drop_flags {
        if let Some(at) = args.iter().position(|a| a == flag) {
            let end = (at + 2).min(args.len());
            args.drain(at..end);
        }
    }
    for pair in prepared.extra_args.chunks(2) {
        let [flag, value] = pair else {
            args.extend(pair.iter().cloned());
            continue;
        };
        match args.iter().position(|a| a == flag) {
            Some(at) if at + 1 < args.len() => args[at + 1] = value.clone(),
            _ => args.extend([flag.clone(), value.clone()]),
        }
    }
    args
}

async fn generate(
    binary: &Path,
    model: &Path,
    ngl: u32,
    prepared: &Prepared,
    grammar_path: &Path,
    footprint: bool,
) -> Spawned {
    let args = compose_argv(prepared, model, ngl, grammar_path);
    let mut cmd = tokio::process::Command::new(binary);
    cmd.args(&args)
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let started = Instant::now();
    let unread = |started: Instant| Metrics {
        elapsed_ms: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        peak_rss_kib: None,
        peak_vram_mib: None,
    };
    let Ok(mut child) = cmd.spawn() else {
        return Spawned::Failed("spawn_failed", unread(started));
    };
    let rss_peak = Arc::new(Mutex::new(None));
    let vram_peak = Arc::new(Mutex::new(None));
    let mut pollers = Vec::new();
    if let Some(pid) = child.id() {
        let peak = Arc::clone(&rss_peak);
        pollers.push(tokio::spawn(async move {
            loop {
                record_peak(&peak, read_vm_hwm_kib(pid));
                tokio::time::sleep(RSS_POLL_INTERVAL).await;
            }
        }));
        if footprint {
            let peak = Arc::clone(&vram_peak);
            pollers.push(tokio::spawn(async move {
                loop {
                    record_peak(&peak, read_vram_mib(pid).await);
                    tokio::time::sleep(VRAM_POLL_INTERVAL).await;
                }
            }));
        }
    }
    let stdout = child.stdout.take();
    let waited = tokio::time::timeout(LLAMA_CLI_TIMEOUT, async move {
        let mut buf = Vec::new();
        if let Some(mut h) = stdout {
            let _ = h.read_to_end(&mut buf).await;
        }
        (child.wait().await, buf)
    })
    .await;
    for poller in &pollers {
        poller.abort();
    }
    let metrics = Metrics {
        peak_rss_kib: read_peak(&rss_peak),
        peak_vram_mib: read_peak(&vram_peak),
        ..unread(started)
    };
    let Ok((status, bytes)) = waited else {
        return Spawned::Failed("timeout", metrics);
    };
    if !status.map(|s| s.success()).unwrap_or(false) {
        return Spawned::Failed("exit_failure", metrics);
    }
    if bytes.len() > LLAMA_CLI_MAX_OUTPUT_BYTES {
        return Spawned::Failed("output_too_large", metrics);
    }
    match String::from_utf8(bytes) {
        Ok(text) => Spawned::Output(text, metrics),
        Err(_) => Spawned::Failed("stdout_utf8_invalid", metrics),
    }
}

/// Nearest-rank median.
fn p50(values: &[u64]) -> Option<u64> {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let rank = sorted.len().div_ceil(2);
    rank.checked_sub(1).and_then(|i| sorted.get(i).copied())
}

fn reading(value: Option<u64>) -> String {
    value.map_or_else(|| "-".to_string(), |v| v.to_string())
}

/// `--sampling '--temp 0.7 --top-p 0.8 …'`: whitespace-separated flag/value
/// pairs, each flag in `SAMPLING_FLAGS`, each value a finite number except
/// `--chat-template-kwargs`, whose value is a JSON object.
fn parse_sampling(value: &str) -> Result<Vec<String>, String> {
    let tokens: Vec<&str> = value.split_whitespace().collect();
    if !tokens.len().is_multiple_of(2) {
        return Err("--sampling takes flag value pairs".to_string());
    }
    let mut out = Vec::new();
    for pair in tokens.chunks(2) {
        let (flag, v) = (pair[0], pair[1]);
        if !SAMPLING_FLAGS.contains(&flag) {
            return Err(format!("--sampling: unsupported flag {flag}"));
        }
        let ok = if flag == "--chat-template-kwargs" {
            serde_json::from_str::<Value>(v).is_ok_and(|j| j.is_object())
        } else {
            v.parse::<f64>().is_ok_and(f64::is_finite)
        };
        if !ok {
            return Err(format!("--sampling: bad value for {flag}"));
        }
        out.extend([flag.to_string(), v.to_string()]);
    }
    Ok(out)
}

fn parse_args() -> Result<Args, String> {
    parse_args_from(std::env::args().skip(1))
}

/// `--bar-sibling K --bar-ordinary K`: both or neither. A bar is graded
/// against the `shipped` arm and guarded against `ns`, over at least one
/// sibling and one ordinary shape, so it needs all four. In the remedy
/// reading the bar grades each candidate against `shipped` over the guard
/// shapes that reading selects itself, and needs no `ns` arm.
fn bar_request(
    sibling_min: Option<u32>,
    ordinary_min: Option<u32>,
    arms: &[String],
    shape_ids: &[String],
    remedy: bool,
) -> Result<Option<Bar>, String> {
    let (sibling_min, ordinary_min) = match (sibling_min, ordinary_min) {
        (None, None) => return Ok(None),
        (Some(sibling), Some(ordinary)) => (sibling, ordinary),
        _ => return Err("--bar-sibling and --bar-ordinary go together".to_string()),
    };
    if remedy {
        return Ok(Some(Bar {
            sibling_min,
            ordinary_min,
        }));
    }
    for arm in ["shipped", "ns"] {
        if !arms.iter().any(|a| a == arm) {
            return Err(format!("the bar needs arm {arm}"));
        }
    }
    let selects = |half: &[&str]| shape_ids.iter().any(|id| half.contains(&id.as_str()));
    if !selects(&SIBLING_SHAPES) {
        return Err("the bar needs a sibling shape (S7, S8)".to_string());
    }
    if !selects(&ORDINARY_SHAPES) {
        return Err("the bar needs an ordinary shape (S1-S4)".to_string());
    }
    Ok(Some(Bar {
        sibling_min,
        ordinary_min,
    }))
}

/// `--reproduce-misses K`: the reproduction reading grades the `shipped` arm
/// alone over at least one of S9-S16, by its own rule and no other verdict
/// flag, so one exit code never stands for two readings. Beside `--replay` it
/// reads the replay labelled `miss` instead and needs no shape.
fn reproduce_request(
    k: Option<u32>,
    arms: &[String],
    shape_ids: &[String],
    other_verdict_flag: bool,
    replays: &[(String, PathBuf)],
) -> Result<Option<u32>, String> {
    let Some(k) = k else {
        return Ok(None);
    };
    if k == 0 {
        return Err("--reproduce-misses takes a number of at least 1".to_string());
    }
    if arms != ["shipped"] {
        return Err("the reproduction reading takes arm shipped alone".to_string());
    }
    if !replays.is_empty() {
        if !replays.iter().any(|(label, _)| label == MISS_REPLAY) {
            return Err("the replay reading needs a replay labelled miss".to_string());
        }
    } else if !shape_ids
        .iter()
        .any(|id| REPRODUCTION_SHAPES.contains(&id.as_str()))
    {
        return Err("the reproduction reading needs a shape of S9-S16".to_string());
    }
    if other_verdict_flag {
        return Err("--reproduce-misses takes no other verdict flag".to_string());
    }
    Ok(Some(k))
}

/// `--remedy-from DIR`: the remedy reading runs `shipped` beside at least one
/// candidate arm over the `miss` and `control` replays, with `--own-lines`
/// and both bar flags, and no other reading's flag. It selects its own S
/// shapes, so `--shapes` is refused.
fn remedy_request(
    arms: &[String],
    replays: &[(String, PathBuf)],
    own_lines_given: bool,
    bar_given: bool,
    shapes_given: bool,
    other_reading_flag: bool,
) -> Result<(), String> {
    for label in [MISS_REPLAY, CONTROL_REPLAY] {
        if !replays.iter().any(|(known, _)| known == label) {
            return Err(format!(
                "the remedy reading needs a replay labelled {label}"
            ));
        }
    }
    if !arms.iter().any(|a| a == "shipped") || !arms.iter().any(|a| replay::candidate(a).is_some())
    {
        return Err("the remedy reading needs arm shipped and a candidate arm".to_string());
    }
    if !own_lines_given {
        return Err("the remedy reading needs --own-lines".to_string());
    }
    if !bar_given {
        return Err("the remedy reading needs --bar-sibling and --bar-ordinary".to_string());
    }
    if shapes_given {
        return Err("the remedy reading selects its own shapes".to_string());
    }
    if other_reading_flag {
        return Err("--remedy-from takes no other reading's flag".to_string());
    }
    Ok(())
}

/// The S shapes the remedy reading runs: the ordinary and sibling guards, and
/// the derived shape when the probe defines one.
fn remedy_shape_ids() -> Vec<String> {
    let derived = shapes()
        .iter()
        .any(|s| s.id == DERIVED_SHAPE)
        .then_some(DERIVED_SHAPE);
    ORDINARY_SHAPES
        .into_iter()
        .chain(SIBLING_SHAPES)
        .chain(derived)
        .map(str::to_string)
        .collect()
}

fn parse_args_from(argv: impl IntoIterator<Item = String>) -> Result<Args, String> {
    let mut arms = vec!["A0".to_string()];
    let mut shape_ids: Vec<String> = DEFAULT_SHAPES.iter().map(|s| s.to_string()).collect();
    let mut n = 10;
    let mut min = None;
    let mut min_rank1 = None;
    let (mut bar_sibling, mut bar_ordinary) = (None, None);
    let mut reproduce = None;
    let mut out = None;
    let mut dry_run = false;
    let mut footprint = false;
    let mut sampling = Vec::new();
    let mut renders = vec![Render::Today];
    let (mut audit_draw, mut audit_seed, mut audit_grade, mut table) = (None, None, None, None);
    let mut shapes_given = false;
    let mut replays: Vec<(String, PathBuf)> = Vec::new();
    let (mut replay_scope, mut own_lines, mut remedy_from) = (None, None, None);
    let mut remedy_read_as = None;
    let mut count_naming = None;
    let mut product_path = false;
    let mut sections = None;
    let mut it = argv.into_iter();
    while let Some(flag) = it.next() {
        if flag == "--dry-run" {
            dry_run = true;
            continue;
        }
        if flag == "--product-path" {
            product_path = true;
            continue;
        }
        if flag == "--footprint" {
            footprint = true;
            continue;
        }
        let value = it.next().ok_or_else(|| format!("{flag} needs a value"))?;
        match flag.as_str() {
            "--arms" => {
                arms = value.split(',').map(|a| a.trim().to_string()).collect();
                if let Some(bad) = arms.iter().find(|a| !ARMS.contains(&a.as_str())) {
                    return Err(format!("unknown arm {bad}"));
                }
            }
            "--shapes" => {
                shapes_given = true;
                shape_ids = value.split(',').map(|s| s.trim().to_string()).collect();
                let known = shapes();
                if let Some(bad) = shape_ids.iter().find(|id| {
                    !known.iter().any(|s| s.id == id.as_str()) && !patterns::is_pattern_id(id)
                }) {
                    return Err(format!("unknown shape {bad}"));
                }
                let pattern = shape_ids
                    .iter()
                    .filter(|id| patterns::is_pattern_id(id))
                    .count();
                if pattern > 0 && pattern < shape_ids.len() {
                    return Err("--shapes mixes S and pattern shapes".to_string());
                }
            }
            "--renders" => renders = patterns::parse_renders(&value)?,
            "--audit-draw" => audit_draw = Some(PathBuf::from(value)),
            "--audit-seed" => {
                audit_seed = Some(
                    value
                        .parse::<u64>()
                        .map_err(|_| "--audit-seed takes a number")?,
                )
            }
            "--audit-grade" => audit_grade = Some(PathBuf::from(value)),
            "--table" => table = Some(PathBuf::from(value)),
            "--n" => n = value.parse().map_err(|_| "--n takes a number")?,
            "--min" => min = Some(value.parse().map_err(|_| "--min takes a number")?),
            "--min-rank1" => {
                min_rank1 = Some(value.parse().map_err(|_| "--min-rank1 takes a number")?)
            }
            "--bar-sibling" => {
                bar_sibling = Some(value.parse().map_err(|_| "--bar-sibling takes a number")?)
            }
            "--bar-ordinary" => {
                bar_ordinary = Some(value.parse().map_err(|_| "--bar-ordinary takes a number")?)
            }
            "--reproduce-misses" => {
                reproduce = Some(
                    value
                        .parse()
                        .map_err(|_| "--reproduce-misses takes a number")?,
                )
            }
            "--out" => out = Some(PathBuf::from(value)),
            "--sampling" => sampling = parse_sampling(&value)?,
            "--replay" => replays = replay::parse_sources(&value)?,
            "--replay-scope" => replay_scope = Some(value),
            "--own-lines" => own_lines = Some(replay::parse_own_lines(&value)?),
            "--remedy-from" => remedy_from = Some(PathBuf::from(value)),
            "--remedy-read-as" => {
                if !replay::is_label(&value) {
                    return Err("--remedy-read-as takes a replay label".to_string());
                }
                remedy_read_as = Some(value);
            }
            "--count-naming" => {
                let id = |c: char| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.');
                if value.is_empty() || !value.chars().all(id) {
                    return Err("--count-naming takes a service id".to_string());
                }
                count_naming = Some(value);
            }
            "--sections" => sections = Some(replay::parse_section_sources(&value)?),
            other => return Err(format!("unknown flag {other}")),
        }
    }
    let out = out.unwrap_or_else(|| {
        PathBuf::from("target/l4-decision-probe")
            .join(chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string())
    });
    let mode = match (audit_draw, audit_seed, audit_grade, table, sections) {
        (None, None, None, None, None) => Mode::Run,
        (Some(root), Some(seed), None, None, None) => Mode::AuditDraw(root, seed),
        (Some(_), None, None, None, None) => {
            return Err("--audit-draw needs --audit-seed".to_string());
        }
        (None, Some(_), _, _, _) => return Err("--audit-seed needs --audit-draw".to_string()),
        (None, None, Some(root), None, None) => Mode::AuditGrade(root),
        (None, None, None, Some(root), None) => Mode::Table(root),
        (None, None, None, None, Some(sources)) => {
            let known = shapes();
            if let Some(bad) = sources.iter().find(|source| {
                matches!(source, SectionSource::Shape(id) if !known.iter().any(|s| s.id == id.as_str()))
            }) {
                return Err(format!("unknown shape {}", bad.label()));
            }
            Mode::Sections(sources)
        }
        (_, _, _, _, Some(_)) => return Err("--sections takes no other mode flag".to_string()),
        _ => return Err("--audit-draw, --audit-grade and --table are exclusive".to_string()),
    };
    if replays.is_empty() {
        if replay_scope.is_some() || own_lines.is_some() || remedy_from.is_some() {
            return Err("--replay-scope, --own-lines and --remedy-from need --replay".to_string());
        }
    } else {
        if replay_scope.is_none() {
            return Err("--replay needs --replay-scope".to_string());
        }
        // A replay brings its own prompt: no S shape runs unless one is named.
        if !shapes_given {
            shape_ids.clear();
        }
        if shape_ids.iter().any(|id| patterns::is_pattern_id(id)) {
            return Err("--replay never runs beside a pattern shape".to_string());
        }
        if !sampling.is_empty() {
            return Err("--sampling does not apply beside --replay".to_string());
        }
        if own_lines.is_some() && !replays.iter().any(|(label, _)| label == MISS_REPLAY) {
            return Err("--own-lines describes the replay labelled miss".to_string());
        }
        if let Some(bad) = arms
            .iter()
            .find(|a| *a != "shipped" && replay::candidate(a).is_none())
        {
            return Err(format!("arm {bad} does not apply to a replay"));
        }
    }
    if remedy_read_as.is_some() && remedy_from.is_none() {
        return Err("--remedy-read-as needs --remedy-from".to_string());
    }
    if product_path && !(dry_run && matches!(own_lines, Some(OwnLines::Positions(_)))) {
        return Err("--product-path needs --dry-run and the own lines of a replay".to_string());
    }
    if remedy_from.is_some() {
        remedy_request(
            &arms,
            &replays,
            own_lines.is_some(),
            bar_sibling.is_some() || bar_ordinary.is_some(),
            shapes_given,
            reproduce.is_some() || min.is_some() || min_rank1.is_some(),
        )?;
        shape_ids = remedy_shape_ids();
    }
    let other_verdict_flag =
        min.is_some() || min_rank1.is_some() || bar_sibling.is_some() || bar_ordinary.is_some();
    let reproduce = reproduce_request(reproduce, &arms, &shape_ids, other_verdict_flag, &replays)?;
    let bar = bar_request(
        bar_sibling,
        bar_ordinary,
        &arms,
        &shape_ids,
        remedy_from.is_some(),
    )?;
    Ok(Args {
        arms,
        shapes: shape_ids,
        n,
        min,
        min_rank1,
        bar,
        reproduce,
        out,
        dry_run,
        footprint,
        sampling,
        renders,
        replays,
        replay_scope,
        own_lines: own_lines.unwrap_or(OwnLines::Unknown),
        remedy_from,
        remedy_read_as: remedy_read_as.unwrap_or_else(|| MISS_REPLAY.to_string()),
        count_naming,
        product_path,
        mode,
    })
}

/// `S1 9 S2 6 …` over the run's shape ids: the selected shapes in `shapes()`
/// order, then the replays.
fn per_shape_counts(ids: &[&'static str], count: impl Fn(&str) -> usize) -> String {
    ids.iter()
        .map(|id| format!("{id} {}", count(id)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn basename(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown".to_string())
}

fn inconclusive_line(why: &str) -> String {
    format!("l4-decision-probe: INCONCLUSIVE - {why}")
}

fn inconclusive(why: &str) -> ExitCode {
    println!("{}", inconclusive_line(why));
    ExitCode::from(2)
}

/// The llama-cli binary and the model through the product's own guarded
/// resolution: `(binary, model, ngl, binary kind)`.
fn resolve_runtime() -> Result<(PathBuf, PathBuf, u32, &'static str), String> {
    let profile = HardwareProfileDetector::new().current_profile();
    let (bin_env, ngl, binary_kind) = binary_target_for_profile(profile);
    let allow_root = resolve_allow_root();
    let mut resolved = Vec::new();
    for env_name in [bin_env, ENV_MODEL_PATH] {
        let raw = std::env::var(env_name).unwrap_or_default();
        let raw = raw.trim();
        if raw.is_empty() {
            return Err(format!("{env_name} is unset"));
        }
        let path = validate_path_input(Path::new(raw), &allow_root)
            .map_err(|rejection| format!("{env_name} rejected: {}", rejection.label()))?;
        resolved.push(path);
    }
    Ok((resolved[0].clone(), resolved[1].clone(), ngl, binary_kind))
}

/// Writes `L4_OUTPUT_GBNF` into `dir` once per run; the path every
/// generation passes as `--grammar-file`.
fn write_grammar_file(dir: &Path) -> Result<PathBuf, String> {
    let path = dir.join(GRAMMAR_FILE_NAME);
    std::fs::write(&path, L4_OUTPUT_GBNF).map_err(|_| "grammar file not writable".to_string())?;
    Ok(path)
}

fn listed(items: Vec<String>) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items.join(" ")
    }
}

/// A pattern shape-render under an arm that varies the argv only; the
/// prompt-transform arms are S-shape instruments.
fn prepare_pattern(
    arm: &str,
    shape: &PatternShape,
    render: Render,
) -> Result<(Prepared, String), String> {
    let (extra_args, drop_flags): (Vec<String>, Vec<&'static str>) = match arm {
        "A0" | "shipped" => (Vec::new(), Vec::new()),
        "A1" => (vec!["--temp".to_string(), "0".to_string()], Vec::new()),
        "nr" => (Vec::new(), vec!["-rea"]),
        other => return Err(format!("arm {other} does not apply to pattern shapes")),
    };
    let composed = patterns::compose(shape, render)?;
    let prepared = Prepared {
        arm: arm.to_string(),
        shape: shape.id,
        // Read only by the S-shape names_trigger grader, never on this path.
        trigger: CueKind::RetryStorm,
        scope_id: None,
        prompt: composed.prompt,
        schema: L4_OUTPUT_JSON_SCHEMA.to_string(),
        extra_args,
        drop_flags,
    };
    Ok((prepared, composed.payload))
}

struct PatternJob {
    prepared: Prepared,
    shape: usize,
    render: Render,
    payload: String,
}

fn series_root(root: &Path) -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|_| "current directory unreadable".to_string())?;
    patterns::series_root(root, &cwd)
}

fn audit_draw_mode(root: &Path, seed: u64) -> ExitCode {
    match series_root(root).and_then(|root| patterns::audit_draw(&root, seed)) {
        Ok((drawn, rows)) => {
            println!(
                "audit: drew {drawn} of {rows} rows · seed {seed} · sample {} · key {}",
                patterns::AUDIT_SAMPLE,
                patterns::AUDIT_KEY
            );
            ExitCode::SUCCESS
        }
        Err(why) => inconclusive(&why),
    }
}

fn audit_grade_mode(root: &Path) -> ExitCode {
    match series_root(root).and_then(|root| patterns::audit_grade(&root)) {
        Ok((disagree, sample)) => {
            println!(
                "audit: disagreement {disagree}/{sample} · bar 10% · {}",
                patterns::audit_disposition(disagree, sample)
            );
            ExitCode::SUCCESS
        }
        Err(why) => inconclusive(&why),
    }
}

fn table_mode(root: &Path) -> ExitCode {
    match series_root(root).and_then(|root| patterns::table(&root)) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
            ExitCode::SUCCESS
        }
        Err(why) => inconclusive(&why),
    }
}

/// The own corpus lines of one replay: `--own-lines` describes the replay
/// labelled `miss`, whose block its positions were read against; any other
/// replay's are unknown.
fn replay_own_lines(args: &Args, shape: &str) -> OwnLines {
    if shape == MISS_SHAPE {
        args.own_lines.clone()
    } else {
        OwnLines::Unknown
    }
}

/// `--product-path`: the `miss` replay's corpus block through the product's
/// selection and render, beside the capture and the `CX` edit of it. Exit 0
/// when both read the same, 1 when one differs.
fn product_path_mode(args: &Args, replays: &[Replay]) -> ExitCode {
    let Some(replay) = replays.iter().find(|r| r.shape == MISS_SHAPE) else {
        return inconclusive("--product-path reads the replay labelled miss");
    };
    let scope = args.replay_scope.as_deref().unwrap_or_default();
    match replay::product_path(&replay.prompt, &args.own_lines, scope) {
        Ok(Some(path)) => {
            println!("{}", replay::product_path_line(replay.shape, &path));
            if path.unremedied_same && path.remedied_same {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Ok(None) => inconclusive(replay::OWN_LINES_UNKNOWN),
        Err(why) => inconclusive(&format!("{}: {why}", replay.shape)),
    }
}

/// One capture directory loaded as a replay. Its recorded argv is compared
/// with the probe's own for this host's hardware profile, the profile a real
/// run resolves its binary from.
fn load_replay(label: &str, dir: &Path, scope: Option<&str>) -> Result<Replay, String> {
    let cwd = std::env::current_dir().map_err(|_| "current directory unreadable".to_string())?;
    let (_, ngl, _) = binary_target_for_profile(HardwareProfileDetector::new().current_profile());
    replay::load(label, dir, &replay::work_tree(&cwd), scope, ngl)
        .map_err(|why| format!("replay {label}: {why}"))
}

/// The section reader's rows: every source after the first compared with the
/// first.
fn section_rows(sources: &[SectionSource]) -> Result<Vec<String>, String> {
    let mut cut = Vec::new();
    for source in sources {
        let prompt = match source {
            SectionSource::Shape(id) => {
                let shape = shapes()
                    .into_iter()
                    .find(|s| s.id == id.as_str())
                    .ok_or_else(|| format!("unknown shape {id}"))?;
                prepare("shipped", &shape)?.prompt
            }
            SectionSource::Capture(label, dir) => load_replay(label, dir, None)?.prompt,
        };
        let label = source.label();
        cut.push(replay::sectioned(label, &prompt).map_err(|why| format!("{label}: {why}"))?);
    }
    let Some((first, rest)) = cut.split_first() else {
        return Ok(Vec::new());
    };
    Ok(rest
        .iter()
        .flat_map(|other| replay::compare(first, other))
        .collect())
}

fn sections_mode(sources: &[SectionSource]) -> ExitCode {
    match section_rows(sources) {
        Ok(rows) => {
            for row in rows {
                println!("{row}");
            }
            ExitCode::SUCCESS
        }
        Err(why) => inconclusive(&why),
    }
}

/// The replay reading a remedy reading sizes itself from: its out dir, under
/// `target/` like every series root.
fn remedy_source(dir: &Path, read_as: &str) -> Result<RemedySource, String> {
    let root = series_root(dir)?;
    let text = std::fs::read_to_string(root.join("runs.json"))
        .map_err(|_| "the replay reading's runs.json is not readable".to_string())?;
    let rows: Vec<Value> = serde_json::from_str(&text)
        .map_err(|_| "the replay reading's runs.json is not a JSON array".to_string())?;
    let misses = replay_reading_misses(&rows, read_as)?;
    let n = remedy_n(misses).ok_or_else(|| {
        format!("the replay reading did not reproduce: misses {misses}/{REPLAY_READING_N}")
    })?;
    Ok(RemedySource {
        misses,
        n,
        dir_name: basename(&root),
        read_as: read_as.to_string(),
    })
}

fn remedy_prompt_line(source: &RemedySource) -> String {
    let read_as = if source.read_as == MISS_REPLAY {
        String::new()
    } else {
        format!(" as {}{}", replay::REPLAY_SHAPE_PREFIX, source.read_as)
    };
    format!(
        "l4-decision-probe: remedy prompt: {MISS_SHAPE} n {} (replay reading {}/{REPLAY_READING_N}{read_as}) · from {}",
        source.n, source.misses, source.dir_name
    )
}

/// The pattern series for one model: every arm over every shape-render, `n`
/// generations each, scored in-process; rows to `runs.json`, texts to
/// `texts/` under the target-only out dir.
async fn run_patterns(args: &Args) -> ExitCode {
    let shapes = patterns::select_pattern_shapes(&args.shapes);
    let mut jobs = Vec::new();
    for arm in &args.arms {
        for (shape, render) in patterns::shape_renders(&shapes, &args.renders) {
            match prepare_pattern(arm, &shapes[shape], render) {
                Ok((mut prepared, payload)) => {
                    prepared.extra_args.extend(args.sampling.iter().cloned());
                    jobs.push(PatternJob {
                        prepared,
                        shape,
                        render,
                        payload,
                    });
                }
                Err(why) => return inconclusive(&why),
            }
        }
    }

    if args.dry_run {
        for job in &jobs {
            let p = &job.prepared;
            println!(
                "dry-run: arm {} {} {}: prompt {} bytes (max {}) · schema {} bytes · extra args {} · dropped args {}",
                p.arm,
                p.shape,
                job.render.label(),
                p.prompt.len(),
                MAX_PROMPT_BYTES,
                p.schema.len(),
                listed(p.extra_args.clone()),
                listed(p.drop_flags.iter().map(|f| f.to_string()).collect()),
            );
        }
        if let Some(job) = jobs.iter().max_by_key(|j| j.prepared.prompt.len()) {
            println!(
                "dry-run: largest pattern prompt {} bytes ({} {})",
                job.prepared.prompt.len(),
                job.prepared.shape,
                job.render.label()
            );
        }
        return ExitCode::SUCCESS;
    }

    let out = match std::env::current_dir()
        .map_err(|_| "current directory unreadable".to_string())
        .and_then(|cwd| patterns::texts_root(&args.out, &cwd))
    {
        Ok(out) => out,
        Err(why) => return inconclusive(&why),
    };
    let (binary, model, ngl, binary_kind) = match resolve_runtime() {
        Ok(runtime) => runtime,
        Err(why) => return inconclusive(&why),
    };
    let grammar_path = match write_grammar_file(&out) {
        Ok(path) => path,
        Err(why) => return inconclusive(&why),
    };
    println!(
        "l4-decision-probe: sampling {}",
        if args.sampling.is_empty() {
            "default".to_string()
        } else {
            args.sampling.join(" ")
        }
    );
    println!(
        "l4-decision-probe: binary {} ({binary_kind}, -ngl {ngl}) · model {} · arms {} · shapes {} · renders {} · n {} per shape-render",
        basename(&binary),
        basename(&model),
        args.arms.join(","),
        shapes.iter().map(|s| s.id).collect::<Vec<_>>().join(","),
        args.renders
            .iter()
            .map(|r| r.label())
            .collect::<Vec<_>>()
            .join(","),
        args.n
    );

    let mut rows: Vec<Value> = Vec::new();
    for arm in &args.arms {
        let started = Instant::now();
        let mut scored = Vec::new();
        let mut decisions: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut thinking_present = 0u32;
        for job in jobs.iter().filter(|j| &j.prepared.arm == arm) {
            let (p, shape) = (&job.prepared, &shapes[job.shape]);
            for run in 1..=args.n {
                let text = |kind| patterns::text_path(&out, shape.id, job.render, run, kind);
                if std::fs::write(text("digest"), &job.payload).is_err() {
                    return inconclusive("texts not writable");
                }
                let outcome =
                    generate(&binary, &model, ngl, p, &grammar_path, args.footprint).await;
                let (graded, thinking, metrics) = match &outcome {
                    Spawned::Failed("spawn_failed", _) => {
                        return inconclusive("llama-cli did not spawn");
                    }
                    Spawned::Output(stdout, m) => {
                        if std::fs::write(text("stdout"), stdout).is_err() {
                            return inconclusive("texts not writable");
                        }
                        let graded = patterns::grade(shape, Outcome::Stdout(stdout));
                        (graded, thinking_label(Some(stdout)), *m)
                    }
                    Spawned::Failed(why, m) => (
                        patterns::grade(shape, Outcome::Failed(why)),
                        thinking_label(None),
                        *m,
                    ),
                };
                if thinking == "present" {
                    thinking_present += 1;
                }
                *decisions
                    .entry(graded.decision.unwrap_or("none"))
                    .or_default() += 1;
                eprintln!(
                    "l4-decision-probe: {arm} {} {} run {run}: valid {} detect {} cause {} decision {} thinking {thinking} elapsed_ms {}",
                    shape.id,
                    job.render.label(),
                    graded.valid,
                    graded.detect,
                    graded.cause,
                    graded.decision.unwrap_or("-"),
                    metrics.elapsed_ms
                );
                rows.push(patterns::row_json(
                    arm, shape, job.render, run, &graded, thinking, metrics,
                ));
                scored.push(patterns::StoredRow {
                    shape: shape.id.to_string(),
                    family: shape.family,
                    render: job.render,
                    run,
                    stored: graded.clone(),
                    graded,
                    elapsed_ms: Some(metrics.elapsed_ms),
                    peak_rss_kib: metrics.peak_rss_kib,
                    peak_vram_mib: metrics.peak_vram_mib,
                });
            }
        }
        let summary = patterns::summarize(arm, &scored);
        let count = |k: &str| decisions.get(k).copied().unwrap_or(0);
        println!(
            "arm {arm}: decision {}/{}/{} · no decision {} · wall {}s",
            count("surface"),
            count("dismiss"),
            count("watch"),
            count("none"),
            started.elapsed().as_secs()
        );
        println!("  arm {arm}: {}", patterns::patterns_line(&summary));
        println!(
            "  arm {arm}: footprint thinking present {thinking_present}/{} · elapsed_ms p50 {} max {} · peak_rss_kib max {} · peak_vram_mib max {}",
            summary.rows,
            reading(p50(&summary.elapsed)),
            reading(summary.elapsed.iter().copied().max()),
            reading(summary.peak_rss_kib),
            reading(summary.peak_vram_mib),
        );
    }

    match serde_json::to_string_pretty(&rows) {
        Ok(text) if std::fs::write(out.join("runs.json"), &text).is_ok() => ExitCode::SUCCESS,
        _ => inconclusive("runs.json not writable"),
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(args) => args,
        Err(why) => return inconclusive(&why),
    };

    match &args.mode {
        Mode::AuditDraw(root, seed) => return audit_draw_mode(root, *seed),
        Mode::AuditGrade(root) => return audit_grade_mode(root),
        Mode::Table(root) => return table_mode(root),
        Mode::Sections(sources) => return sections_mode(sources),
        Mode::Run => {}
    }
    if args.shapes.iter().any(|id| patterns::is_pattern_id(id)) {
        return run_patterns(&args).await;
    }

    let selected = select_shapes(&args.shapes);
    let mut replays = Vec::new();
    for (label, dir) in &args.replays {
        match load_replay(label, dir, args.replay_scope.as_deref()) {
            Ok(replay) => replays.push(replay),
            Err(why) => return inconclusive(&why),
        }
    }
    let remedy = match args
        .remedy_from
        .as_deref()
        .map(|dir| remedy_source(dir, &args.remedy_read_as))
    {
        Some(Ok(source)) => Some(source),
        Some(Err(why)) => return inconclusive(&why),
        None => None,
    };
    let shape_ids: Vec<&'static str> = selected
        .iter()
        .map(|s| s.id)
        .chain(replays.iter().map(|r| r.shape))
        .collect();

    let mut prepared = Vec::new();
    // What is not generated, each a printed line's tail: an arm skipped whole
    // or on one replay, and the shapes a candidate takes `shipped`'s counts on.
    let mut left_out: Vec<(String, Option<&'static str>, String)> = Vec::new();
    for arm in &args.arms {
        for shape in &selected {
            match prepare(arm, shape) {
                Ok(p) => prepared.push(p),
                Err(why) => return inconclusive(&why),
            }
        }
        for replay in &replays {
            match replay.compose(arm, &replay_own_lines(&args, replay.shape)) {
                Ok(Composed::Run(p)) => prepared.push(p),
                Ok(Composed::Skipped(why)) => {
                    left_out.push((arm.clone(), Some(replay.shape), format!("skipped · {why}")))
                }
                Err(why) => return inconclusive(&format!("{}: {why}", replay.shape)),
            }
        }
    }

    // The remedy reading generates no arm that composes as `shipped` on the
    // remedy prompt, and no candidate again on a shape it composes as
    // `shipped`: a second sample of one distribution could trip the guard
    // only by chance.
    let mut reused: BTreeMap<String, Vec<&'static str>> = BTreeMap::new();
    let mut skipped_arms: BTreeMap<String, &'static str> = BTreeMap::new();
    if remedy.is_some() {
        for arm in args.arms.iter().filter(|a| *a != "shipped") {
            match remedy_arm_plan(&prepared, arm) {
                ArmPlan::Skipped(why) => {
                    left_out.retain(|(skipped, _, _)| skipped != arm);
                    left_out.push((arm.clone(), None, format!("skipped · {why}")));
                    skipped_arms.insert(arm.clone(), why);
                }
                ArmPlan::Run(shapes) => {
                    if !shapes.is_empty() {
                        let tail = format!(
                            "composes as shipped on {} · takes its counts there",
                            shapes.join(",")
                        );
                        left_out.push((arm.clone(), None, tail));
                    }
                    reused.insert(arm.clone(), shapes);
                }
            }
        }
        prepared.retain(|p| {
            !skipped_arms.contains_key(&p.arm)
                && !reused
                    .get(&p.arm)
                    .is_some_and(|shapes| shapes.contains(&p.shape))
        });
    }

    for p in &mut prepared {
        p.extra_args.extend(args.sampling.iter().cloned());
    }
    let left_out_line = |(arm, shape, tail): &(String, Option<&'static str>, String)| match shape {
        Some(shape) => format!("arm {arm} {shape}: {tail}"),
        None => format!("arm {arm}: {tail}"),
    };

    if args.dry_run {
        // Composes every arm's prompt and spawns nothing: proves the transforms
        // apply and every prompt passes the production bound before a slot.
        if let Some(source) = &remedy {
            println!("{}", remedy_prompt_line(source));
        }
        for p in &prepared {
            if replay::is_replay_shape(p.shape) {
                println!(
                    "dry-run: arm {} {}: prompt {} bytes (max {}) · argv same · corpus lines {}",
                    p.arm,
                    p.shape,
                    p.prompt.len(),
                    MAX_PROMPT_BYTES,
                    replay::corpus_line_count(&p.prompt),
                );
                continue;
            }
            println!(
                "dry-run: arm {} {}: prompt {} bytes (max {}) · schema {} bytes · extra args {} · dropped args {}",
                p.arm,
                p.shape,
                p.prompt.len(),
                MAX_PROMPT_BYTES,
                p.schema.len(),
                listed(p.extra_args.clone()),
                listed(p.drop_flags.iter().map(|f| f.to_string()).collect()),
            );
        }
        for entry in &left_out {
            println!("dry-run: {}", left_out_line(entry));
        }
        if args.product_path {
            return product_path_mode(&args, &replays);
        }
        return ExitCode::SUCCESS;
    }

    let (binary, model, ngl, binary_kind) = match resolve_runtime() {
        Ok(runtime) => runtime,
        Err(why) => return inconclusive(&why),
    };
    println!(
        "l4-decision-probe: sampling {}",
        if args.sampling.is_empty() {
            "default".to_string()
        } else {
            args.sampling.join(" ")
        }
    );
    println!(
        "l4-decision-probe: binary {} ({binary_kind}, -ngl {ngl}) · model {} · arms {} · shapes {} · n {} per shape",
        basename(&binary),
        basename(&model),
        args.arms.join(","),
        shape_ids.join(","),
        args.n
    );
    if let Some(source) = &remedy {
        println!("{}", remedy_prompt_line(source));
    }
    for entry in &left_out {
        println!("  {}", left_out_line(entry));
    }
    // The remedy prompt runs at the remedy reading's own n; every other
    // source at `--n`.
    let runs_of = |p: &Prepared| match &remedy {
        Some(source) if p.shape == MISS_SHAPE => source.n,
        _ => args.n,
    };

    if std::fs::create_dir_all(&args.out).is_err() {
        return inconclusive("output directory not creatable");
    }
    let grammar_path = match write_grammar_file(&args.out) {
        Ok(path) => path,
        Err(why) => return inconclusive(&why),
    };

    let mut rows: Vec<Value> = Vec::new();
    let mut total = 0u32;
    let mut total_create = 0u32;
    let mut total_rank1 = 0u32;
    let mut arm_bars: BTreeMap<String, BarCounts> = BTreeMap::new();
    let mut miss_counts: BTreeMap<&'static str, MissCounts> = BTreeMap::new();
    let mut tallies: BTreeMap<(String, &'static str), Tally> = BTreeMap::new();
    for arm in &args.arms {
        let started = Instant::now();
        let mut n_arm = 0u32;
        let mut create = 0u32;
        let mut decisions: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut severity_none = 0u32;
        let mut resolution_summary = 0u32;
        let mut key_orders: BTreeMap<String, u32> = BTreeMap::new();
        let mut distinct: BTreeMap<&'static str, BTreeSet<u64>> = BTreeMap::new();
        let mut per_shape: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut names_counts: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut rank1_per_shape: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut stem_counts: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut stem_rank1_per_shape: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut identifies_counts: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut both_per_shape: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut runs_per_shape: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut named_per_shape: BTreeMap<&'static str, NamedCounts> = BTreeMap::new();
        let mut thinking_present = 0u32;
        let mut elapsed: Vec<u64> = Vec::new();
        let mut rss_max: Option<u64> = None;
        let mut vram_max: Option<u64> = None;
        for p in prepared.iter().filter(|p| &p.arm == arm) {
            for run in 1..=runs_of(p) {
                let outcome =
                    generate(&binary, &model, ngl, p, &grammar_path, args.footprint).await;
                if let Spawned::Failed("spawn_failed", _) = outcome {
                    return inconclusive("llama-cli did not spawn");
                }
                let (thinking, metrics) = match &outcome {
                    Spawned::Output(text, m) => (thinking_label(Some(text)), *m),
                    Spawned::Failed(_, m) => (thinking_label(None), *m),
                };
                if thinking == "present" {
                    thinking_present += 1;
                }
                elapsed.push(metrics.elapsed_ms);
                rss_max = rss_max.max(metrics.peak_rss_kib);
                vram_max = vram_max.max(metrics.peak_vram_mib);
                let labels = read_labels(&outcome, p);
                *names_counts.entry(labels.names_trigger).or_default() += 1;
                if labels.names_trigger == "rank1" {
                    *rank1_per_shape.entry(p.shape).or_default() += 1;
                }
                *stem_counts.entry(labels.names_trigger_stem).or_default() += 1;
                if labels.names_trigger_stem == "rank1" {
                    *stem_rank1_per_shape.entry(p.shape).or_default() += 1;
                }
                *identifies_counts.entry(labels.identifies).or_default() += 1;
                fold_miss(miss_counts.entry(p.shape).or_default(), labels.identifies);
                fold_tally(
                    tallies.entry((arm.clone(), p.shape)).or_default(),
                    labels.identifies,
                );
                *runs_per_shape.entry(p.shape).or_default() += 1;
                if labels.identifies == "both" {
                    *both_per_shape.entry(p.shape).or_default() += 1;
                }
                n_arm += 1;
                if labels.would_create() {
                    create += 1;
                    *per_shape.entry(p.shape).or_default() += 1;
                }
                *decisions.entry(labels.decision).or_default() += 1;
                if labels.severity == "none" {
                    severity_none += 1;
                }
                if labels.is_resolution_summary {
                    resolution_summary += 1;
                }
                *key_orders.entry(labels.first_keys.join(">")).or_default() += 1;
                if let Some(d) = labels.output_hash {
                    distinct.entry(p.shape).or_default().insert(d);
                }
                eprintln!(
                    "{}",
                    run_line(p, run, &labels, thinking, metrics.elapsed_ms)
                );
                let row = row_json(p, run, &labels, thinking, metrics);
                rows.push(match args.count_naming.as_deref() {
                    Some(other) => {
                        let label = names_other(parsed_output(&outcome).as_ref(), other);
                        fold_named(named_per_shape.entry(p.shape).or_default(), label);
                        with_names_other(row, label)
                    }
                    None => row,
                });
            }
        }
        total += n_arm;
        total_create += create;
        let names_count = |k: &str| names_counts.get(k).copied().unwrap_or(0);
        total_rank1 += names_count("rank1");
        let count = |k: &str| decisions.get(k).copied().unwrap_or(0);
        let other: u32 = decisions
            .iter()
            .filter(|(k, _)| !matches!(**k, "surface" | "dismiss" | "watch"))
            .map(|(_, v)| *v)
            .sum();
        println!(
            "arm {arm}: would_create {create}/{n_arm} · decision {}/{}/{} · severity_none {severity_none} · resolution_summary {resolution_summary}",
            count("surface"),
            count("dismiss"),
            count("watch"),
        );
        println!(
            "  arm {arm}: per shape would_create {} · failed {other} · first_keys {} · distinct outputs {} · wall {}s",
            per_shape_counts(&shape_ids, |id| per_shape.get(id).copied().unwrap_or(0)
                as usize),
            key_orders
                .iter()
                .map(|(k, v)| format!("{}={v}", if k.is_empty() { "none" } else { k }))
                .collect::<Vec<_>>()
                .join(" "),
            per_shape_counts(&shape_ids, |id| distinct
                .get(id)
                .map(|d| d.len())
                .unwrap_or(0)),
            started.elapsed().as_secs()
        );
        println!(
            "  arm {arm}: names_trigger rank1 {}/{n_arm} · elsewhere {} · none {} · unparsed {} · per shape rank1 {}",
            names_count("rank1"),
            names_count("elsewhere"),
            names_count("none"),
            names_count("unparsed"),
            per_shape_counts(
                &shape_ids,
                |id| rank1_per_shape.get(id).copied().unwrap_or(0) as usize
            ),
        );
        let stem_count = |k: &str| stem_counts.get(k).copied().unwrap_or(0);
        println!(
            "  arm {arm}: names_trigger_stem rank1 {}/{n_arm} · elsewhere {} · none {} · unparsed {} · per shape rank1 {}",
            stem_count("rank1"),
            stem_count("elsewhere"),
            stem_count("none"),
            stem_count("unparsed"),
            per_shape_counts(
                &shape_ids,
                |id| stem_rank1_per_shape.get(id).copied().unwrap_or(0) as usize
            ),
        );
        println!(
            "{}",
            identifies_line(
                arm,
                &identifies_counts,
                n_arm,
                &per_shape_counts(
                    &shape_ids,
                    |id| both_per_shape.get(id).copied().unwrap_or(0) as usize
                ),
            )
        );
        if let Some(k) = args.reproduce {
            for id in &shape_ids {
                let counts = miss_counts.get(id).copied().unwrap_or_default();
                println!("{}", reproduce_line(id, counts, k));
            }
        }
        if args.count_naming.is_some() {
            for id in &shape_ids {
                let counts = named_per_shape.get(id).copied().unwrap_or_default();
                println!("{}", second_count_line(arm, id, counts));
            }
        }
        let counts = bar_counts(&both_per_shape, &runs_per_shape);
        // In the remedy reading an arm's guard counts are read after every
        // arm ran, with the shapes it takes from `shipped`.
        if let (Some(bar), None) = (args.bar, &remedy) {
            println!("{}", bar_line(arm, counts, bar));
        }
        arm_bars.insert(arm.clone(), counts);
        println!(
            "  arm {arm}: footprint thinking present {thinking_present}/{n_arm} · elapsed_ms p50 {} max {} · peak_rss_kib max {} · peak_vram_mib max {}",
            reading(p50(&elapsed)),
            reading(elapsed.iter().copied().max()),
            reading(rss_max),
            reading(vram_max),
        );
    }

    let runs_path = args.out.join("runs.json");
    match serde_json::to_string_pretty(&rows) {
        Ok(text) if std::fs::write(&runs_path, &text).is_ok() => {}
        _ => return inconclusive("runs.json not writable"),
    }

    if let Some(k) = args.reproduce {
        let (line, exit) = if replays.is_empty() {
            reproduction_verdict(&miss_counts, k)
        } else {
            replay_verdict(&miss_counts, k)
        };
        println!("{line}");
        return ExitCode::from(exit);
    }

    if let (Some(source), Some(bar)) = (&remedy, args.bar) {
        let derived = shape_ids.contains(&DERIVED_SHAPE);
        let shipped = remedy_counts(&tallies, "shipped", &[], derived);
        let candidates: Vec<(&'static str, RemedyArm)> = replay::CANDIDATE_ARMS
            .into_iter()
            .filter(|arm| args.arms.iter().any(|a| a == arm))
            .map(|arm| {
                let state = match skipped_arms.get(arm) {
                    Some(why) => RemedyArm::Skipped(why),
                    None => {
                        let reused = reused.get(arm).map(Vec::as_slice).unwrap_or_default();
                        RemedyArm::Read(remedy_counts(&tallies, arm, reused, derived))
                    }
                };
                (arm, state)
            })
            .collect();
        let (lines, exit) = remedy_reading(shipped, &candidates, source.n, bar);
        for line in lines {
            println!("{line}");
        }
        return ExitCode::from(exit);
    }

    let verdict = |pass: bool| if pass { "PASS" } else { "FAIL" };
    let mut failed = false;
    if let Some(min) = args.min {
        let pass = total_create >= min;
        failed |= !pass;
        println!(
            "l4-decision-probe: verdict: {} · would_create {total_create}/{total}",
            verdict(pass)
        );
    }
    if let Some(min_rank1) = args.min_rank1 {
        let pass = total_rank1 >= min_rank1;
        failed |= !pass;
        println!(
            "l4-decision-probe: names-trigger verdict: {} · rank1 {total_rank1}/{total}",
            verdict(pass)
        );
    }
    if let Some(bar) = args.bar {
        let (lines, pass) = service_verdict_lines(&arm_bars, bar);
        failed |= !pass;
        for line in lines {
            println!("{line}");
        }
    }
    if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use interpretation::schema::{Confidence, Hypothesis};

    type LineTarget = fn(&str) -> bool;

    fn output(title: &str, symptom: &str, statements: &[&str]) -> L4Output {
        L4Output {
            schema_version: "2.0".to_string(),
            prompt_version: "v2.4".to_string(),
            decision: Decision::Surface,
            severity: Severity::Suggested,
            title: title.to_string(),
            symptom: symptom.to_string(),
            timeline: "Started 2m ago".to_string(),
            hypotheses: statements
                .iter()
                .map(|s| Hypothesis {
                    statement: s.to_string(),
                    confidence: Confidence::High,
                    justification: "observed".to_string(),
                })
                .collect(),
            investigation_steps: Vec::new(),
            evidence_refs: Vec::new(),
            fingerprint: String::new(),
            model_tier: "primary".to_string(),
            hardware_profile: "gpu".to_string(),
            is_resolution_summary: false,
        }
    }

    fn shape(id: &str) -> Shape {
        shapes()
            .into_iter()
            .find(|s| s.id == id)
            .unwrap_or_else(|| panic!("shape {id} exists"))
    }

    fn count_lines(text: &str, target: LineTarget) -> usize {
        text.lines().filter(|l| target(l)).count()
    }

    #[test]
    fn names_trigger_reads_rank1_when_the_first_hypothesis_names_the_retry() {
        let out = output(
            "Retry storm on checkout-api",
            "Clients retry checkout-api calls",
            &["Clients retry failed checkout-api calls in a tight loop"],
        );
        assert_eq!(names_trigger(&out, CueKind::RetryStorm), "rank1");
    }

    #[test]
    fn names_trigger_reads_elsewhere_when_only_the_title_or_symptom_names_it() {
        let cases = [
            (
                "title-only",
                output("Retry storm", "Errors rose", &["A deploy broke checkout"]),
            ),
            (
                "symptom-only",
                output(
                    "Checkout errors",
                    "Clients retry",
                    &["A deploy broke checkout"],
                ),
            ),
            (
                "second-hypothesis-only",
                output(
                    "Checkout errors",
                    "Errors rose",
                    &[
                        "A deploy broke checkout",
                        "A client retry loop amplifies it",
                    ],
                ),
            ),
        ];
        for (case, out) in cases {
            assert_eq!(
                names_trigger(&out, CueKind::RetryStorm),
                "elsewhere",
                "{case}"
            );
        }
    }

    #[test]
    fn names_trigger_reads_none_when_nothing_names_it() {
        let out = output(
            "Error Rate Spike in demo-shop",
            "payment-service errors rose",
            &["A deploy broke payment-service"],
        );
        assert_eq!(names_trigger(&out, CueKind::RetryStorm), "none");
    }

    #[test]
    fn names_trigger_matches_case_insensitively() {
        let out = output("x", "y", &["Clients RETRY failed checkout-api calls"]);
        assert_eq!(names_trigger(&out, CueKind::RetryStorm), "rank1");
    }

    #[test]
    fn names_trigger_label_set_is_exactly_the_closed_set() {
        const CLOSED: [&str; 4] = ["rank1", "elsewhere", "none", "unparsed"];
        let outputs = [
            output("x", "y", &["retry"]),
            output("retry", "y", &["z"]),
            output("x", "y", &["z"]),
            output("x", "y", &[]),
        ];
        let mut seen = BTreeSet::new();
        for kind in [
            CueKind::ErrorRateSpike,
            CueKind::LatencyRegression,
            CueKind::RestartEvent,
            CueKind::ServiceWentSilent,
            CueKind::RetryStorm,
            CueKind::ReflectionTrend,
        ] {
            seen.insert(names_trigger_label(None, kind));
            for out in &outputs {
                seen.insert(names_trigger_label(Some(out), kind));
            }
        }
        assert_eq!(seen, CLOSED.into_iter().collect::<BTreeSet<_>>());
    }

    #[test]
    fn nf_arm_strips_exactly_the_three_framing_lines() {
        // S8: a shape whose corpus block the product keeps under its cue.
        let s8 = shape("S8");
        let shipped = prepare("shipped", &s8).expect("shipped composes").prompt;
        let nf = prepare("nf", &s8).expect("nf composes").prompt;
        let targets: [(&str, LineTarget); 3] = [
            ("TRIGGER line", is_trigger_line),
            ("framing note", is_framing_note_line),
            ("framing instruction", is_framing_instruction_line),
        ];
        let mut stripped = shipped.clone();
        for (what, target) in targets {
            assert_eq!(
                count_lines(&shipped, target),
                1,
                "shipped carries the {what}"
            );
            assert_eq!(count_lines(&nf, target), 0, "nf carries no {what}");
            stripped = stripped
                .split_inclusive('\n')
                .filter(|l| !target(l.trim_end_matches('\n')))
                .collect();
        }
        assert_eq!(
            stripped, nf,
            "nf is shipped minus exactly those three lines"
        );
    }

    #[test]
    fn s4_renders_a_framed_corpus_match_beside_the_trigger() {
        // Under `nb`: S4's one line is another service's, so `shipped` drops it.
        let prompt = prepare(BASELINE_ARM, &shape("S4"))
            .expect("nb composes")
            .prompt;
        let lines: Vec<&str> = prompt.lines().collect();
        assert!(lines.contains(&"TRIGGER: Retry storm"));
        let note = format!("  {CORPUS_MATCHES_FRAMING_NOTE}");
        let at = lines
            .iter()
            .position(|l| *l == note)
            .expect("the corpus framing note renders");
        let next = lines.get(at + 1).copied().unwrap_or_default();
        assert!(
            next.starts_with("  - [") && next.contains("Error-rate spike:"),
            "a corpus match line follows the note: {next}"
        );
    }

    #[test]
    fn every_arm_and_shape_composes_within_the_production_bound() {
        let ids: Vec<&str> = shapes().iter().map(|s| s.id).collect();
        assert_eq!(
            ids,
            [
                "S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8", "S9", "S10", "S11", "S12", "S13",
                "S14", "S15", "S16", "S17"
            ]
        );
        for arm in ARMS {
            for s in shapes() {
                let prepared = prepare(arm, &s);
                assert!(prepared.is_ok(), "{arm} {}: {:?}", s.id, prepared.err());
                let bytes = prepared.map(|p| p.prompt.len()).unwrap_or_default();
                assert!(bytes <= MAX_PROMPT_BYTES, "{arm} {}: {bytes} bytes", s.id);
            }
        }
    }

    fn argv(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    fn selected_ids(args: &Args) -> Vec<&'static str> {
        select_shapes(&args.shapes).iter().map(|s| s.id).collect()
    }

    #[test]
    fn shapes_default_is_s1_through_s4() {
        let Ok(args) = parse_args_from(argv(&[])) else {
            panic!("no flags parse");
        };
        assert_eq!(selected_ids(&args), ["S1", "S2", "S3", "S4"]);
    }

    #[test]
    fn shapes_flag_selects_the_listed_shapes_in_shapes_order() {
        let Ok(args) = parse_args_from(argv(&["--shapes", "S6,S5", "--n", "3"])) else {
            panic!("--shapes S6,S5 parses");
        };
        assert_eq!(selected_ids(&args), ["S5", "S6"]);
        assert_eq!(args.n, 3);
    }

    #[test]
    fn shapes_flag_refuses_an_unknown_shape() {
        let refused = parse_args_from(argv(&["--shapes", "S1,S18"])).err();
        assert_eq!(refused.as_deref(), Some("unknown shape S18"));
    }

    #[test]
    fn stem_grader_label_set_is_exactly_the_closed_set() {
        const CLOSED: [&str; 4] = ["rank1", "elsewhere", "none", "unparsed"];
        let outputs = [
            output("x", "y", &["retries"]),
            output("retried", "y", &["z"]),
            output("x", "y", &["z"]),
            output("x", "y", &[]),
        ];
        let mut seen = BTreeSet::new();
        for kind in [
            CueKind::ErrorRateSpike,
            CueKind::LatencyRegression,
            CueKind::RestartEvent,
            CueKind::ServiceWentSilent,
            CueKind::RetryStorm,
            CueKind::ReflectionTrend,
        ] {
            seen.insert(names_trigger_stem_label(None, kind));
            for out in &outputs {
                seen.insert(names_trigger_stem_label(Some(out), kind));
            }
        }
        assert_eq!(seen, CLOSED.into_iter().collect::<BTreeSet<_>>());
    }

    #[test]
    fn stem_reads_rank1_for_retries_and_retried_where_the_strict_grader_does_not() {
        for statement in [
            "Excessive retries on payment-service",
            "Failed calls are retried",
        ] {
            let out = output("Payment errors", "Errors rose", &[statement]);
            assert_eq!(
                names_trigger_stem(&out, CueKind::RetryStorm),
                "rank1",
                "{statement}"
            );
            assert_eq!(
                names_trigger(&out, CueKind::RetryStorm),
                "none",
                "{statement}"
            );
        }
    }

    fn candidate_text_count(prompt: &str, part: &str) -> usize {
        match part {
            "R1" => count_lines(prompt, is_known_framing_line),
            "R3" => prompt.matches(R3_CONVENTIONS_SENTENCE).count(),
            _ => prompt.matches(R2_SCHEMA_SENTENCE).count(),
        }
    }

    #[test]
    fn candidate_arms_carry_their_text_exactly_once_within_the_bound() {
        for arm in ["R1", "R3", "R2", "R1R3", "R1R2", "R3R2"] {
            for s in shapes() {
                let Ok(p) = prepare(arm, &s) else {
                    panic!("{arm} {} composes", s.id);
                };
                for part in candidate_parts(arm) {
                    assert_eq!(
                        candidate_text_count(&p.prompt, part),
                        1,
                        "{arm} {}: {part} text",
                        s.id
                    );
                }
                if candidate_parts(arm).contains(&"R2") {
                    assert_eq!(p.schema.matches(R2_SCHEMA_SENTENCE).count(), 1, "{arm}");
                }
                assert!(p.prompt.len() <= MAX_PROMPT_BYTES, "{arm} {}", s.id);
            }
        }
    }

    #[test]
    fn candidate_transforms_are_identity_once_their_text_is_present() {
        let s2 = shape("S2");
        let Ok(r1) = prepare("R1", &s2) else {
            panic!("R1 composes");
        };
        assert_eq!(apply_r1(&r1.prompt), Ok(r1.prompt.clone()));
        let Ok(r3) = prepare("R3", &s2) else {
            panic!("R3 composes");
        };
        assert_eq!(apply_r3(&r3.prompt), Ok(r3.prompt.clone()));
        let Ok(r2) = prepare("R2", &s2) else {
            panic!("R2 composes");
        };
        assert_eq!(
            apply_r2(&r2.prompt, &r2.schema),
            Ok((r2.prompt.clone(), r2.schema.clone()))
        );
    }

    #[test]
    fn r2_schema_parses_and_changes_only_the_hypotheses_description() {
        let base = L4_OUTPUT_JSON_SCHEMA.replace(R2_SCHEMA_SENTENCE, "");
        let Ok(edited) = r2_schema(&base) else {
            panic!("R2 edits the schema");
        };
        assert_ne!(edited, base);
        let Ok(mut parsed) = serde_json::from_str::<Value>(&edited) else {
            panic!("the edited schema is JSON");
        };
        let Ok(original) = serde_json::from_str::<Value>(&base) else {
            panic!("the base schema is JSON");
        };
        let at = "/properties/hypotheses/description";
        assert_eq!(
            parsed.pointer(at),
            Some(&Value::String(format!("{R2_ANCHOR}{R2_SCHEMA_SENTENCE}")))
        );
        if let Some(d) = parsed.pointer_mut(at) {
            *d = Value::String(R2_ANCHOR.to_string());
        }
        assert_eq!(parsed, original);
    }

    #[test]
    fn thinking_label_set_is_exactly_the_closed_set() {
        const CLOSED: [&str; 3] = ["present", "absent", "unread"];
        let seen: BTreeSet<&str> = [
            Some("[Start thinking]\nhmm\n[End thinking]\n{}"),
            Some("{}"),
            None,
        ]
        .into_iter()
        .map(thinking_label)
        .collect();
        assert_eq!(seen, CLOSED.into_iter().collect::<BTreeSet<_>>());
    }

    #[test]
    fn thinking_reads_present_on_the_b9305_marker() {
        let stdout =
            "> prompt\n[Start thinking]\nThe user wants {json}\n[End thinking]\n\n{\"a\":1}";
        assert_eq!(thinking_label(Some(stdout)), "present");
    }

    #[test]
    fn thinking_reads_absent_without_the_marker() {
        let stdout = "> prompt\n{\"decision\":\"surface\",\"thinking\":\"no\"}\n[ Prompt: 1 t/s ]";
        assert_eq!(thinking_label(Some(stdout)), "absent");
    }

    #[test]
    fn vm_hwm_parser_reads_the_peak_resident_set() {
        let status =
            "Name:\tllama-cli\nVmPeak:\t 9876543 kB\nVmHWM:\t 2345678 kB\nVmRSS:\t 2000000 kB\n";
        assert_eq!(parse_vm_hwm_kib(status), Some(2_345_678));
    }

    #[test]
    fn vm_hwm_parser_reads_none_without_the_line() {
        assert_eq!(
            parse_vm_hwm_kib("Name:\tllama-cli\nState:\tZ (zombie)\n"),
            None
        );
    }

    #[test]
    fn vram_parser_keeps_the_max_for_the_child_pid_only() {
        let csv = "4242, 1834\n999, 9000\n4242, 2100\nbad line\n";
        assert_eq!(parse_vram_mib(csv, 4242), Some(2100));
        assert_eq!(parse_vram_mib(csv, 7), None);
    }

    #[test]
    fn footprint_flag_is_off_by_default() {
        let Ok(args) = parse_args_from(argv(&["--n", "2"])) else {
            panic!("--n 2 parses");
        };
        assert!(!args.footprint);
    }

    #[test]
    fn footprint_flag_sets_it_without_taking_a_value() {
        let Ok(args) = parse_args_from(argv(&["--footprint", "--n", "2"])) else {
            panic!("--footprint --n 2 parses");
        };
        assert!(args.footprint);
        assert_eq!(args.n, 2);
    }

    #[test]
    fn nr_arm_removes_exactly_the_reasoning_switch() {
        let s1 = shape("S1");
        let (Ok(shipped), Ok(nr)) = (prepare("shipped", &s1), prepare("nr", &s1)) else {
            panic!("shipped and nr compose");
        };
        assert_eq!(nr.prompt, shipped.prompt, "nr varies the argv only");
        assert_eq!(nr.schema, shipped.schema, "nr varies the argv only");
        let (model, grammar) = (Path::new("/m.gguf"), Path::new("/g.gbnf"));
        let shipped_argv = compose_argv(&shipped, model, 99, grammar);
        let Some(at) = shipped_argv
            .windows(2)
            .position(|w| w[0] == "-rea" && w[1] == "off")
        else {
            panic!("the shipped argv carries -rea off");
        };
        let mut expected = shipped_argv.clone();
        expected.drain(at..at + 2);
        assert_eq!(compose_argv(&nr, model, 99, grammar), expected);
    }

    #[test]
    fn shipped_argv_carries_the_grammar_file_and_no_schema_file() {
        let Ok(shipped) = prepare("shipped", &shape("S1")) else {
            panic!("shipped composes");
        };
        let composed = compose_argv(&shipped, Path::new("/m.gguf"), 99, Path::new("/g.gbnf"));
        assert!(
            composed
                .windows(2)
                .any(|w| w[0] == "--grammar-file" && w[1] == "/g.gbnf")
        );
        assert!(!composed.iter().any(|a| a == "--json-schema-file"));
        let retired = parse_args_from(argv(&["--arms", "gb"])).err();
        assert_eq!(retired.as_deref(), Some("unknown arm gb"));
        let retired = parse_args_from(argv(&["--gbnf", "g.gbnf"])).err();
        assert_eq!(retired.as_deref(), Some("unknown flag --gbnf"));
    }

    #[test]
    fn sampling_replaces_the_production_pairs_rather_than_duplicating() {
        let Ok(mut shipped) = prepare("shipped", &shape("S1")) else {
            panic!("shipped composes");
        };
        let (model, grammar) = (Path::new("/m.gguf"), Path::new("/g.gbnf"));
        let production = compose_argv(&shipped, model, 99, grammar);
        let Ok(args) = parse_args_from(argv(&[
            "--sampling",
            "--temp 0.7 --top-k 20 --presence-penalty 1.5",
        ])) else {
            panic!("the sampling parses");
        };
        shipped.extra_args.extend(args.sampling);
        let composed = compose_argv(&shipped, model, 99, grammar);
        for flag in [
            "--temp",
            "--top-p",
            "--top-k",
            "--min-p",
            "--presence-penalty",
        ] {
            assert_eq!(
                composed.iter().filter(|a| *a == flag).count(),
                1,
                "{flag} passed once"
            );
        }
        let value = |argv: &[String], flag: &str| {
            argv.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
        };
        assert_eq!(value(&composed, "--temp").as_deref(), Some("0.7"));
        assert_eq!(value(&composed, "--top-k").as_deref(), Some("20"));
        assert_eq!(value(&composed, "--top-p"), value(&production, "--top-p"));
        assert_eq!(
            value(&composed, "--presence-penalty").as_deref(),
            Some("1.5")
        );
        assert_eq!(composed.len(), production.len() + 2);
    }

    #[test]
    fn sampling_flag_keeps_the_allowlisted_pairs_in_order() {
        let value = "--temp 0.7 --top-p 0.8 --top-k 20 --min-p 0 --presence-penalty 1.5 \
                     --chat-template-kwargs {\"enable_thinking\":false}";
        let Ok(args) = parse_args_from(argv(&["--sampling", value])) else {
            panic!("the Qwen sampling parses");
        };
        assert_eq!(
            args.sampling,
            [
                "--temp",
                "0.7",
                "--top-p",
                "0.8",
                "--top-k",
                "20",
                "--min-p",
                "0",
                "--presence-penalty",
                "1.5",
                "--chat-template-kwargs",
                "{\"enable_thinking\":false}",
            ]
        );
        let Ok(none) = parse_args_from(argv(&[])) else {
            panic!("no flags parse");
        };
        assert!(none.sampling.is_empty());
    }

    #[test]
    fn sampling_flag_refuses_anything_outside_the_allowlist() {
        let refused = |v: &str| parse_args_from(argv(&["--sampling", v])).err();
        assert_eq!(
            refused("-m /x.gguf").as_deref(),
            Some("--sampling: unsupported flag -m")
        );
        assert_eq!(
            refused("--grammar-file g").as_deref(),
            Some("--sampling: unsupported flag --grammar-file")
        );
        assert_eq!(
            refused("--temp hot").as_deref(),
            Some("--sampling: bad value for --temp")
        );
        assert_eq!(
            refused("--chat-template-kwargs [1]").as_deref(),
            Some("--sampling: bad value for --chat-template-kwargs")
        );
        assert_eq!(
            refused("--temp").as_deref(),
            Some("--sampling takes flag value pairs")
        );
    }

    #[test]
    fn pattern_shape_ids_parse_and_an_unknown_one_is_refused() {
        let ids = patterns::PATTERN_IDS.join(",");
        let Ok(args) = parse_args_from(argv(&["--shapes", &ids])) else {
            panic!("every pattern shape id parses");
        };
        assert_eq!(args.shapes, patterns::PATTERN_IDS);
        let refused = parse_args_from(argv(&["--shapes", "A1,A8"])).err();
        assert_eq!(refused.as_deref(), Some("unknown shape A8"));
    }

    #[test]
    fn pattern_and_s_shapes_never_mix_in_one_run() {
        let refused = parse_args_from(argv(&["--shapes", "S1,A1"])).err();
        assert_eq!(
            refused.as_deref(),
            Some("--shapes mixes S and pattern shapes")
        );
    }

    #[test]
    fn renders_flag_defaults_to_today_and_takes_both() {
        let Ok(none) = parse_args_from(argv(&[])) else {
            panic!("no flags parse");
        };
        assert_eq!(none.renders, [Render::Today]);
        let Ok(both) = parse_args_from(argv(&["--renders", "today,enriched"])) else {
            panic!("--renders today,enriched parses");
        };
        assert_eq!(both.renders, [Render::Today, Render::Enriched]);
        let refused = parse_args_from(argv(&["--renders", "richer"])).err();
        assert_eq!(refused.as_deref(), Some("unknown render richer"));
    }

    #[test]
    fn audit_draw_grade_and_table_modes_parse_and_exclude_each_other() {
        let mode = |items: &[&str]| parse_args_from(argv(items)).map(|a| a.mode);
        assert_eq!(mode(&[]), Ok(Mode::Run));
        assert_eq!(
            mode(&["--audit-draw", "target/s", "--audit-seed", "17"]),
            Ok(Mode::AuditDraw(PathBuf::from("target/s"), 17))
        );
        assert_eq!(
            mode(&["--audit-grade", "target/s"]),
            Ok(Mode::AuditGrade(PathBuf::from("target/s")))
        );
        assert_eq!(
            mode(&["--table", "target/s"]),
            Ok(Mode::Table(PathBuf::from("target/s")))
        );
        assert_eq!(
            mode(&["--audit-draw", "target/s"]).err().as_deref(),
            Some("--audit-draw needs --audit-seed")
        );
        assert_eq!(
            mode(&["--audit-seed", "17"]).err().as_deref(),
            Some("--audit-seed needs --audit-draw")
        );
        assert_eq!(
            mode(&["--audit-grade", "target/s", "--table", "target/s"])
                .err()
                .as_deref(),
            Some("--audit-draw, --audit-grade and --table are exclusive")
        );
        assert_eq!(
            mode(&["--audit-draw", "target/s", "--audit-seed", "x"])
                .err()
                .as_deref(),
            Some("--audit-seed takes a number")
        );
    }

    #[test]
    fn pattern_arms_vary_the_argv_only() {
        let a1 = patterns::select_pattern_shapes(&["A1".to_string()]);
        let Ok((nr, _)) = prepare_pattern("nr", &a1[0], Render::Today) else {
            panic!("nr composes A1");
        };
        let Ok((shipped, payload)) = prepare_pattern("shipped", &a1[0], Render::Today) else {
            panic!("shipped composes A1");
        };
        assert_eq!(nr.prompt, shipped.prompt);
        assert_eq!(nr.drop_flags, ["-rea"]);
        assert!(shipped.prompt.contains(&payload));
        for arm in ["A2", "A3", "A4", "A5", "nf", "R1", "R3", "R2", "R1R3"] {
            assert_eq!(
                prepare_pattern(arm, &a1[0], Render::Today).err(),
                Some(format!("arm {arm} does not apply to pattern shapes"))
            );
        }
    }

    const SERVICE: Option<&str> = Some("conductor");
    const BAR: Bar = Bar {
        sibling_min: 19,
        ordinary_min: 36,
    };

    fn first(statement: &str) -> L4Output {
        output("x", "y", &[statement])
    }

    fn identified(statement: &str) -> &'static str {
        identifies(Some(&first(statement)), SERVICE)
    }

    fn prompt_of(arm: &str, id: &str) -> String {
        match prepare(arm, &shape(id)) {
            Ok(p) => p.prompt,
            Err(why) => panic!("{arm} {id} composes: {why}"),
        }
    }

    fn trigger_lines(prompt: &str) -> Vec<&str> {
        prompt.lines().filter(|l| is_trigger_line(l)).collect()
    }

    /// The corpus match lines that follow the framing note, in render order.
    fn corpus_lines(prompt: &str) -> Vec<&str> {
        let note = format!("  {CORPUS_MATCHES_FRAMING_NOTE}");
        prompt
            .lines()
            .skip_while(|l| *l != note)
            .skip(1)
            .take_while(|l| l.starts_with("  - ["))
            .collect()
    }

    fn counts(sibling_both: u32, ordinary_both: u32) -> BarCounts {
        BarCounts {
            sibling_both,
            sibling_n: 20,
            ordinary_both,
            ordinary_n: 40,
        }
    }

    fn arm_counts(arms: &[(&str, BarCounts)]) -> BTreeMap<String, BarCounts> {
        arms.iter().map(|(arm, c)| (arm.to_string(), *c)).collect()
    }

    #[test]
    fn identifies_label_set_is_exactly_the_closed_five() {
        const CLOSED: [&str; 5] = ["both", "service_only", "signal_only", "neither", "unparsed"];
        let outputs = [
            first("A retry storm hit conductor"),
            first("conductor is failing"),
            first("A retry storm hit the gateway"),
            first("The gateway is failing"),
            output("x", "y", &[]),
        ];
        let mut seen = BTreeSet::new();
        for scope in [SERVICE, Some(""), None] {
            seen.insert(identifies(None, scope));
            for out in &outputs {
                seen.insert(identifies(Some(out), scope));
            }
        }
        assert_eq!(seen, CLOSED.into_iter().collect::<BTreeSet<_>>());
    }

    #[test]
    fn identifies_reads_each_label_off_the_first_statement() {
        assert_eq!(identified("A retry storm hit conductor"), "both");
        assert_eq!(identified("conductor is failing"), "service_only");
        assert_eq!(identified("A retry storm hit the gateway"), "signal_only");
        assert_eq!(identified("The gateway is failing"), "neither");
        assert_eq!(
            identifies(
                Some(&first("A RETRY storm hit CONDUCTOR.")),
                Some("Conductor")
            ),
            "both",
            "the statement and the scope_id are ASCII-lowercased"
        );
        assert_eq!(
            identifies(Some(&first("A retry storm hit conductor")), None),
            "signal_only",
            "a cue without a scope_id names no service"
        );
    }

    #[test]
    fn identifies_reads_the_service_alone_where_a_joined_sibling_fails() {
        assert_eq!(
            identified("A retry storm is occurring in conductor"),
            "both"
        );
        for sibling in ["conductor-canary", "conductor_canary"] {
            assert_eq!(
                identified(&format!("A retry storm is occurring in {sibling}")),
                "signal_only",
                "{sibling}"
            );
        }
    }

    #[test]
    fn identifies_passes_a_statement_naming_both_the_service_and_its_sibling() {
        for statement in [
            "Retry storm on conductor or conductor-canary",
            "Retry storm on conductor-canary, spreading to conductor",
        ] {
            assert_eq!(identified(statement), "both", "{statement}");
        }
    }

    #[test]
    fn identifies_reads_a_retry_token_as_a_whole_run_of_letters() {
        for (statement, expected) in [
            ("conductor calls are retried", "service_only"),
            ("conductor returns non-retryable errors", "service_only"),
            ("retry_storm on conductor", "both"),
            ("conductor keeps retrying its calls", "both"),
            ("Excessive retries on conductor", "both"),
        ] {
            assert_eq!(identified(statement), expected, "{statement}");
        }
    }

    #[test]
    fn identifies_does_not_count_a_later_hypothesis_or_the_title() {
        let out = output(
            "Retry storm on conductor",
            "conductor retries its calls",
            &[
                "A retry storm hit the gateway",
                "conductor is retrying its calls",
            ],
        );
        assert_eq!(identifies(Some(&out), SERVICE), "signal_only");
    }

    #[test]
    fn identifies_is_unparsed_without_a_parse_or_a_first_hypothesis() {
        assert_eq!(identifies(None, SERVICE), "unparsed");
        let empty = output("Retry storm on conductor", "conductor retries", &[]);
        assert_eq!(identifies(Some(&empty), SERVICE), "unparsed");
    }

    #[test]
    fn identifies_limit_negation_is_not_read() {
        assert_eq!(identified("The retry storm is not on conductor"), "both");
    }

    #[test]
    fn identifies_limit_a_space_separated_sibling_passes() {
        assert_eq!(identified("Retry storm on conductor canary"), "both");
    }

    #[test]
    fn identifies_limit_retried_is_not_a_retry_token() {
        assert_eq!(identified("Calls to conductor are retried"), "service_only");
    }

    #[test]
    fn identifies_limit_a_joining_neighbour_rejects_wherever_it_stands() {
        for statement in [
            "Retry storm on pre-conductor",
            "Retry storm on edge_conductor",
            "Retry storm on conductor2",
            "Retry storm on semiconductor",
        ] {
            assert_eq!(identified(statement), "signal_only", "{statement}");
        }
    }

    #[test]
    fn sibling_shapes_render_the_kind_label_trigger_and_both_services_rows() {
        for id in SIBLING_SHAPES {
            let prompt = prompt_of("shipped", id);
            assert_eq!(trigger_lines(&prompt), ["TRIGGER: Retry storm"], "{id}");
            for service in [SIBLING_SERVICE, SIBLING_CANARY] {
                let row = format!("  {service}     ");
                let rows = prompt
                    .lines()
                    .filter(|l| l.starts_with(&row) && l.contains("| 100.0% |"))
                    .count();
                assert_eq!(rows, 1, "{id}: one {service} row at a 100 % error rate");
            }
            let s = shape(id);
            assert_eq!(s.cue.scope_id.as_deref(), Some(SIBLING_SERVICE), "{id}");
            assert_eq!(s.cue.kind, CueKind::RetryStorm, "{id}");
        }
    }

    #[test]
    fn s8_carries_two_sibling_corpus_lines_and_s7_none() {
        let s7 = prompt_of("shipped", "S7");
        assert!(corpus_lines(&s7).is_empty());
        assert!(!s7.lines().any(|l| l == "CORPUS MATCHES:"));
        assert_eq!(count_lines(&s7, is_framing_note_line), 0);

        let s8 = prompt_of("shipped", "S8");
        assert_eq!(count_lines(&s8, is_framing_note_line), 1);
        let lines = corpus_lines(&s8);
        let opening = format!("  - [{STORM_FINGERPRINT}] Retry storm: ");
        assert_eq!(lines.len(), 2);
        for line in &lines {
            assert!(line.starts_with(&opening), "{line}");
            assert_eq!(line.matches("Conductor-Canary").count(), 1, "{line}");
        }
        assert!(lines[0].ends_with("3m ago, active"), "{}", lines[0]);
        assert!(lines[1].ends_with("9m ago, resolved"), "{}", lines[1]);
        assert_eq!(s8.lines().filter(|l| l.starts_with("  - [")).count(), 2);
    }

    #[test]
    fn the_shipped_arm_carries_the_scope_sentence_once_and_the_baselines_do_not() {
        assert_eq!(
            TRIGGER_FRAMING_INSTRUCTION,
            format!("{V25_FRAMING_INSTRUCTION} {SCOPE_SENTENCE}")
        );
        for id in ["S1", "S4", "S7", "S8"] {
            for (arm, expected) in [("shipped", 1), ("ns", 0), ("L", 0), ("LI", 1)] {
                assert_eq!(
                    prompt_of(arm, id).matches(SCOPE_SENTENCE).count(),
                    expected,
                    "{arm} {id}"
                );
            }
        }
    }

    #[test]
    fn the_line_arms_render_the_scope_on_the_trigger_line() {
        for (arm, expected) in [
            ("shipped", "TRIGGER: Retry storm"),
            ("ns", "TRIGGER: Retry storm"),
            ("L", "TRIGGER: Retry storm on conductor"),
            ("LI", "TRIGGER: Retry storm on conductor"),
        ] {
            assert_eq!(trigger_lines(&prompt_of(arm, "S7")), [expected], "{arm}");
        }
    }

    #[test]
    fn ns_is_shipped_minus_exactly_the_sentence() {
        for id in ["S1", "S7", "S8"] {
            let (shipped, ns) = (prompt_of("shipped", id), prompt_of("ns", id));
            assert_ne!(shipped, ns, "{id}");
            assert_eq!(
                shipped.replacen(&format!(" {SCOPE_SENTENCE}"), "", 1),
                ns,
                "{id}"
            );
        }
    }

    #[test]
    fn each_line_arm_differs_from_its_base_in_the_trigger_line_alone() {
        for (arm, base) in [("LI", "shipped"), ("L", "ns")] {
            for id in ["S2", "S8"] {
                let (with, without) = (prompt_of(arm, id), prompt_of(base, id));
                assert_eq!(with.lines().count(), without.lines().count(), "{arm} {id}");
                let changed: Vec<(&str, &str)> = without
                    .lines()
                    .zip(with.lines())
                    .filter(|(a, b)| a != b)
                    .collect();
                let scope = shape(id).cue.scope_id.unwrap_or_default();
                assert_eq!(
                    changed,
                    [(
                        "TRIGGER: Retry storm",
                        format!("TRIGGER: Retry storm on {scope}").as_str()
                    )],
                    "{arm} {id}"
                );
            }
        }
    }

    #[test]
    fn r1_composes_as_the_shipped_tree() {
        for s in shapes() {
            assert_eq!(
                prompt_of("R1", s.id),
                prompt_of("shipped", s.id),
                "{}",
                s.id
            );
        }
    }

    #[test]
    fn the_scope_arms_compose_on_a_tree_with_or_without_the_sentence() {
        let tree = prompt_of("shipped", "S7");
        let (Ok(with), Ok(without)) = (
            set_scope_sentence(&tree, true),
            set_scope_sentence(&tree, false),
        ) else {
            panic!("the tree's framing instruction is a known form");
        };
        assert_eq!(with.matches(SCOPE_SENTENCE).count(), 1);
        assert_eq!(without.matches(SCOPE_SENTENCE).count(), 0);
        assert_eq!(count_lines(&without, |l| l == V25_FRAMING_INSTRUCTION), 1);
        assert_eq!(set_scope_sentence(&without, true), Ok(with.clone()));
        assert_eq!(set_scope_sentence(&with, false), Ok(without.clone()));
        assert_eq!(set_scope_sentence(&with, true), Ok(with.clone()));
        assert_eq!(set_scope_sentence(&without, false), Ok(without.clone()));
        assert_eq!(apply_r1(&with), Ok(with.clone()));
        assert_eq!(apply_r1(&without), Ok(without.clone()));
        let unknown = without.replacen(V25_FRAMING_INSTRUCTION, "Some other instruction.", 1);
        assert_eq!(
            set_scope_sentence(&unknown, true).err().as_deref(),
            Some("scope sentence: known framing instruction found 0 times, expected 1")
        );
    }

    #[test]
    fn the_line_rewrite_refuses_a_cue_without_a_scope_or_a_digest_without_one_trigger() {
        assert_eq!(
            scope_trigger_line("TRIGGER: Retry storm\n", None)
                .err()
                .as_deref(),
            Some("L: the cue carries no scope_id")
        );
        assert_eq!(
            scope_trigger_line("OVERALL: nominal\n", SERVICE)
                .err()
                .as_deref(),
            Some("L: TRIGGER line found 0 times, expected 1")
        );
        assert_eq!(
            scope_trigger_line("TRIGGER: Retry storm\nSERVICES:\n", SERVICE),
            Ok("TRIGGER: Retry storm on conductor\nSERVICES:\n".to_string())
        );
    }

    #[test]
    fn bar_flags_parse_together_and_a_lone_one_is_refused() {
        let bar = |items: &[&str]| parse_args_from(argv(items)).map(|a| a.bar);
        let run = ["--arms", "shipped,ns,L,LI", "--shapes", "S1,S7"];
        assert_eq!(bar(&run), Ok(None));
        assert_eq!(
            bar(&[
                &run[..],
                &["--bar-sibling", "19", "--bar-ordinary", "36"][..]
            ]
            .concat()),
            Ok(Some(BAR))
        );
        for lone in [["--bar-sibling", "19"], ["--bar-ordinary", "36"]] {
            assert_eq!(
                bar(&[&run[..], &lone[..]].concat()).err().as_deref(),
                Some("--bar-sibling and --bar-ordinary go together")
            );
        }
        assert_eq!(
            bar(&["--bar-sibling", "many"]).err().as_deref(),
            Some("--bar-sibling takes a number")
        );
        assert_eq!(
            bar(&["--bar-ordinary", "-1"]).err().as_deref(),
            Some("--bar-ordinary takes a number")
        );
    }

    #[test]
    fn bar_flags_need_the_shipped_and_ns_arms_and_a_shape_of_each_half() {
        let refused = |arms: &str, shapes: &str| {
            parse_args_from(argv(&[
                "--arms",
                arms,
                "--shapes",
                shapes,
                "--bar-sibling",
                "19",
                "--bar-ordinary",
                "36",
            ]))
            .err()
        };
        assert_eq!(refused("shipped,ns", "S1,S7"), None);
        assert_eq!(
            refused("ns,L,LI", "S1,S7").as_deref(),
            Some("the bar needs arm shipped")
        );
        assert_eq!(
            refused("shipped,L,LI", "S1,S7").as_deref(),
            Some("the bar needs arm ns")
        );
        assert_eq!(
            refused("shipped,ns", "S1,S2,S3,S4").as_deref(),
            Some("the bar needs a sibling shape (S7, S8)")
        );
        for shapes in ["S7,S8", "S5,S6,S7"] {
            assert_eq!(
                refused("shipped,ns", shapes).as_deref(),
                Some("the bar needs an ordinary shape (S1-S4)"),
                "{shapes}"
            );
        }
    }

    #[test]
    fn bar_counts_fold_each_shape_onto_its_half_and_s5_s6_onto_neither() {
        let both: BTreeMap<&'static str, u32> = [
            ("S1", 9),
            ("S4", 10),
            ("S5", 10),
            ("S6", 7),
            ("S7", 8),
            ("S8", 10),
        ]
        .into();
        let runs: BTreeMap<&'static str, u32> = [
            ("S1", 10),
            ("S2", 10),
            ("S4", 10),
            ("S5", 10),
            ("S6", 10),
            ("S7", 10),
            ("S8", 10),
        ]
        .into();
        assert_eq!(
            bar_counts(&both, &runs),
            BarCounts {
                sibling_both: 18,
                sibling_n: 20,
                ordinary_both: 19,
                ordinary_n: 30,
            }
        );
    }

    #[test]
    fn the_bar_is_met_only_at_both_minimums() {
        assert!(bar_met(counts(19, 36), BAR));
        assert!(bar_met(counts(20, 40), BAR));
        assert!(!bar_met(counts(18, 40), BAR));
        assert!(!bar_met(counts(20, 35), BAR));
    }

    #[test]
    fn selection_takes_the_first_arm_in_order_whose_bar_is_met() {
        let (met, missed) = (counts(19, 36), counts(18, 36));
        let select = |shipped, l, li| {
            select_arm(
                &arm_counts(&[("LI", li), ("L", l), ("ns", missed), ("shipped", shipped)]),
                BAR,
            )
        };
        assert_eq!(select(met, met, met), "shipped");
        assert_eq!(select(missed, met, met), "L");
        assert_eq!(select(missed, missed, met), "LI");
        assert_eq!(select(missed, missed, missed), "none");
    }

    #[test]
    fn selection_never_takes_the_baseline_arm() {
        let (met, missed) = (counts(20, 40), counts(0, 0));
        let arms = arm_counts(&[("ns", met), ("shipped", missed), ("L", missed)]);
        assert_eq!(select_arm(&arms, BAR), "none");
        assert_eq!(select_arm(&arm_counts(&[("ns", met)]), BAR), "none");
    }

    #[test]
    fn the_regression_guard_trips_only_when_shipped_reads_below_the_baseline() {
        assert_eq!(regression_guard(counts(17, 38), counts(17, 38)), "HOLDS");
        assert_eq!(regression_guard(counts(20, 40), counts(17, 38)), "HOLDS");
        assert_eq!(regression_guard(counts(16, 40), counts(17, 38)), "TRIPPED");
        assert_eq!(regression_guard(counts(20, 37), counts(17, 38)), "TRIPPED");
        let (shipped, ns) = (counts(12, 30), counts(12, 29));
        assert!(!bar_met(shipped, BAR));
        assert_eq!(regression_guard(shipped, ns), "HOLDS");
    }

    #[test]
    fn the_verdict_lines_print_the_selection_then_the_verdict_and_the_guard_last() {
        let arms = arm_counts(&[
            ("shipped", counts(20, 38)),
            ("ns", counts(17, 38)),
            ("L", counts(20, 40)),
            ("LI", counts(20, 40)),
        ]);
        let (lines, pass) = service_verdict_lines(&arms, BAR);
        assert!(pass);
        assert_eq!(
            lines,
            [
                "l4-decision-probe: selection: shipped · order shipped,L,LI",
                "l4-decision-probe: service verdict: PASS · arm shipped",
                "l4-decision-probe: regression guard: HOLDS · sibling shipped 20 vs ns 17 · ordinary shipped 38 vs ns 38",
            ]
        );

        let arms = arm_counts(&[
            ("shipped", counts(15, 38)),
            ("ns", counts(17, 38)),
            ("L", counts(18, 40)),
            ("LI", counts(20, 40)),
        ]);
        let (lines, pass) = service_verdict_lines(&arms, BAR);
        assert!(!pass);
        assert_eq!(
            lines,
            [
                "l4-decision-probe: selection: LI · order shipped,L,LI",
                "l4-decision-probe: service verdict: FAIL · arm shipped",
                "l4-decision-probe: regression guard: TRIPPED · sibling shipped 15 vs ns 17 · ordinary shipped 38 vs ns 38",
            ]
        );
    }

    #[test]
    fn the_per_arm_bar_and_identifies_lines_take_their_printed_form() {
        assert_eq!(
            bar_line("shipped", counts(19, 36), BAR),
            "  arm shipped: bar MET · sibling both 19/20 (min 19) · ordinary both 36/40 (min 36)"
        );
        assert_eq!(
            bar_line("LI", counts(18, 36), BAR),
            "  arm LI: bar NOT MET · sibling both 18/20 (min 19) · ordinary both 36/40 (min 36)"
        );
        let labels: BTreeMap<&'static str, u32> = [
            ("both", 5),
            ("service_only", 1),
            ("signal_only", 2),
            ("unparsed", 2),
        ]
        .into();
        assert_eq!(
            identifies_line("ns", &labels, 10, "S1 3 S7 2"),
            "  arm ns: identifies both 5/10 · service_only 1 · signal_only 2 · neither 0 · unparsed 2 · per shape both S1 3 S7 2"
        );
    }

    #[test]
    fn an_s_shape_row_and_run_line_hold_labels_and_no_model_text() {
        const ROW_KEYS: [&str; 16] = [
            "arm",
            "decision",
            "elapsed_ms",
            "first_keys",
            "identifies",
            "is_resolution_summary",
            "names_trigger",
            "names_trigger_stem",
            "output_hash",
            "peak_rss_kib",
            "peak_vram_mib",
            "run",
            "severity",
            "shape",
            "thinking",
            "would_create",
        ];
        let mut out = output(
            "zq-title-41",
            "zq-symptom-41",
            &["zq-statement-41: a retry storm on conductor"],
        );
        out.timeline = "zq-timeline-41".to_string();
        out.hypotheses[0].justification = "zq-justification-41".to_string();
        out.fingerprint = STORM_FINGERPRINT.to_string();
        out.hardware_profile = "gpu_primary".to_string();
        let Ok(stdout) = serde_json::to_string(&out) else {
            panic!("the synthetic output serializes");
        };
        let Ok(p) = prepare("shipped", &shape("S7")) else {
            panic!("shipped composes S7");
        };
        let metrics = Metrics {
            elapsed_ms: 1234,
            peak_rss_kib: None,
            peak_vram_mib: None,
        };
        let labels = read_labels(&Spawned::Output(stdout, metrics), &p);
        assert_eq!(labels.decision, "surface", "the synthetic output parsed");
        assert_eq!(labels.identifies, "both");
        assert_eq!(labels.names_trigger, "rank1");

        let row = row_json(&p, 3, &labels, "absent", metrics);
        let line = run_line(&p, 3, &labels, "absent", metrics.elapsed_ms);
        let keys: BTreeSet<&str> = row
            .as_object()
            .map(|o| o.keys().map(String::as_str).collect())
            .unwrap_or_default();
        assert_eq!(keys, ROW_KEYS.into_iter().collect::<BTreeSet<_>>());
        assert_eq!(row["identifies"], "both");
        assert!(line.contains(" names_trigger_stem rank1 identifies both thinking absent "));
        let row_text = row.to_string();
        for text in [&row_text, &line] {
            assert!(!text.contains("zq-"), "model text reached a record: {text}");
        }

        let failed = read_labels(&Spawned::Failed("timeout", metrics), &p);
        assert_eq!(
            (failed.decision, failed.identifies, failed.would_create()),
            ("timeout", "unparsed", false)
        );
    }

    const STORM: &str = "Retry storm";
    const SPIKE: &str = "Error-rate spike";

    /// One rendered corpus match line.
    fn block_line(fingerprint: &str, cause: &str, title: &str, tail: &str) -> String {
        format!("  - [{fingerprint}] {cause}: {title} — {tail}")
    }

    /// A prompt with its corpus block dropped: the header, the framing note
    /// and every match line.
    fn without_corpus_block(prompt: &str) -> String {
        let note = format!("  {CORPUS_MATCHES_FRAMING_NOTE}");
        prompt
            .split_inclusive('\n')
            .filter(|l| {
                let body = l.trim_end_matches('\n');
                !(body == "CORPUS MATCHES:" || body == note || body.starts_with("  - ["))
            })
            .collect()
    }

    /// Each corpus line split at its position: what the incident is, and the
    /// age and status its slot gives it.
    fn heads_and_tails(prompt: &str) -> (Vec<String>, Vec<String>) {
        corpus_lines(prompt)
            .iter()
            .map(|l| l.rsplit_once(" — ").unwrap_or((l, "")))
            .map(|(head, tail)| (head.to_string(), tail.to_string()))
            .unzip()
    }

    fn miss(misses: u32, unparsed: u32) -> MissCounts {
        MissCounts {
            misses,
            unparsed,
            n: 20,
        }
    }

    fn all_clean() -> BTreeMap<&'static str, MissCounts> {
        REPRODUCTION_SHAPES
            .into_iter()
            .map(|id| (id, miss(0, 0)))
            .collect()
    }

    #[test]
    fn s1_to_s8_keep_their_corpus_blocks_through_the_baseline_selection() {
        let (mut baseline, mut remedied) = (0, 0);
        for id in ["S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8"] {
            let s = shape(id);
            let selected = corpus_match_lines(&baseline_corpus_incidents(&s));
            assert_eq!(selected, corpus_match_lines(&s.corpus_incidents), "{id}");
            baseline += selected.len();
            remedied += selected_corpus_incidents(&s).len();
        }
        assert_eq!(baseline, 3, "S4 holds one line and S8 two");
        assert_eq!(
            remedied, 2,
            "under its cue S8 keeps its two lines, which carry the cue's fingerprint, and S4 none"
        );
    }

    // The sizes prompt v2.6 composes at, the corpus block as the earlier
    // readings measured it (`nb`). A prompt-text edit moves all three
    // together; a change to how a corpus block is built moves S4 or S8 alone.
    #[test]
    fn the_baseline_arm_composes_s4_s7_and_s8_at_their_recorded_sizes() {
        for (id, bytes) in [("S4", 7654), ("S7", 7367), ("S8", 7693)] {
            assert_eq!(prompt_of(BASELINE_ARM, id).len(), bytes, "{id}");
        }
        for id in ["S7", "S8"] {
            assert_eq!(
                prompt_of("shipped", id),
                prompt_of(BASELINE_ARM, id),
                "{id}"
            );
        }
        assert_eq!(
            prompt_of("shipped", "S4"),
            without_corpus_block(&prompt_of(BASELINE_ARM, "S4")),
            "S4's one line is another service's: the product drops the block"
        );
    }

    #[test]
    fn the_new_shape_ids_parse_and_select_in_shapes_order() {
        let Ok(args) = parse_args_from(argv(&["--shapes", "S16,S15,S14,S13,S12,S11,S10,S9"]))
        else {
            panic!("the new shape ids parse");
        };
        assert_eq!(selected_ids(&args), REPRODUCTION_SHAPES);
    }

    #[test]
    fn each_new_shape_differs_from_s7_in_the_corpus_block_alone() {
        let s7 = prompt_of("shipped", "S7");
        assert_eq!(without_corpus_block(&s7), s7, "S7 carries no block");
        for id in REPRODUCTION_SHAPES {
            let prompt = prompt_of(BASELINE_ARM, id);
            assert_eq!(count_lines(&prompt, is_framing_note_line), 1, "{id}");
            assert_eq!(without_corpus_block(&prompt), s7, "{id}");
            assert_eq!(
                without_corpus_block(&prompt_of("shipped", id)),
                s7,
                "{id}: the product's selection moves the block alone"
            );
        }
    }

    #[test]
    fn each_new_shape_holds_exactly_the_lines_its_row_names_in_order() {
        let (cue, other) = (STORM_FINGERPRINT, CORPUS_FINGERPRINT);
        let loop_back = "Conductor-Canary calls fail and loop back";
        let saturated = "Conductor-Canary saturated by repeated calls";
        let failing = "Conductor-Canary failing a rising share of calls";
        let canary_storm = "Retry Storm Observed on conductor-canary";
        let spikes = [
            "Error Rate Spike Observed on conductor-canary",
            "Elevated Error Rate Reported on conductor-canary",
            "Failing Requests Rising on conductor-canary",
            "Error Share Climbing on conductor-canary",
        ];
        let expected = [
            (
                "S9",
                vec![
                    block_line(cue, STORM, loop_back, "3m ago, active"),
                    block_line(cue, STORM, saturated, "9m ago, resolved"),
                    block_line(
                        cue,
                        STORM,
                        "Conductor-Canary requests re-sent after timeouts",
                        "12m ago, resolved",
                    ),
                    block_line(
                        cue,
                        STORM,
                        "Conductor-Canary flooded by repeated attempts",
                        "15m ago, resolved",
                    ),
                    block_line(
                        cue,
                        STORM,
                        "Conductor-Canary callers looping on failed calls",
                        "18m ago, resolved",
                    ),
                ],
            ),
            (
                "S10",
                vec![
                    block_line(cue, STORM, loop_back, "3m ago, active"),
                    block_line(cue, SPIKE, failing, "9m ago, resolved"),
                ],
            ),
            (
                "S11",
                vec![
                    block_line(cue, SPIKE, failing, "3m ago, active"),
                    block_line(cue, STORM, loop_back, "9m ago, resolved"),
                ],
            ),
            (
                "S12",
                vec![
                    block_line(other, STORM, loop_back, "3m ago, active"),
                    block_line(other, STORM, saturated, "9m ago, resolved"),
                ],
            ),
            (
                "S13",
                vec![
                    block_line(cue, STORM, canary_storm, "3m ago, active"),
                    block_line(
                        cue,
                        STORM,
                        "Repeated Call Retries Seen on conductor-canary",
                        "9m ago, resolved",
                    ),
                ],
            ),
            (
                "S14",
                vec![
                    block_line(other, STORM, canary_storm, "3m ago, active"),
                    block_line(other, SPIKE, spikes[0], "9m ago, active"),
                    block_line(other, SPIKE, spikes[1], "12m ago, resolved"),
                    block_line(other, SPIKE, spikes[2], "15m ago, resolved"),
                    block_line(other, SPIKE, spikes[3], "18m ago, resolved"),
                ],
            ),
            (
                "S15",
                vec![
                    block_line(other, SPIKE, spikes[0], "3m ago, active"),
                    block_line(other, SPIKE, spikes[1], "9m ago, active"),
                    block_line(other, SPIKE, spikes[2], "12m ago, resolved"),
                    block_line(other, SPIKE, spikes[3], "15m ago, resolved"),
                    block_line(other, STORM, canary_storm, "18m ago, resolved"),
                ],
            ),
            (
                "S16",
                vec![
                    block_line(other, STORM, canary_storm, "3m ago, active"),
                    block_line(
                        cue,
                        STORM,
                        "Retry Storm Observed on conductor",
                        "9m ago, resolved",
                    ),
                    block_line(other, SPIKE, spikes[0], "12m ago, active"),
                    block_line(other, SPIKE, spikes[1], "15m ago, resolved"),
                    block_line(
                        cue,
                        STORM,
                        "Repeated Call Retries Seen on conductor",
                        "18m ago, resolved",
                    ),
                ],
            ),
        ];
        assert_eq!(expected.each_ref().map(|(id, _)| *id), REPRODUCTION_SHAPES);
        // `nb` holds each block as the first reading measured it, newest first.
        // `shipped` keeps of it the lines that are the triggering scope's own
        // or carry the cue's fingerprint: all of S9-S11 and S13, none of S12,
        // S14 and S15, the two conductor lines of S16.
        for (id, lines) in expected {
            assert_eq!(corpus_lines(&prompt_of(BASELINE_ARM, id)), lines, "{id}");
            let kept: Vec<String> = lines
                .iter()
                .filter(|line| line.starts_with(&format!("  - [{cue}] ")))
                .cloned()
                .collect();
            let count = match id {
                "S12" | "S14" | "S15" => 0,
                "S16" => 2,
                _ => lines.len(),
            };
            assert_eq!(kept.len(), count, "{id}");
            assert_eq!(corpus_lines(&prompt_of("shipped", id)), kept, "{id}");
        }
    }

    #[test]
    fn the_position_pairs_hold_the_same_lines_in_a_different_order() {
        for (first, second) in [("S10", "S11"), ("S14", "S15")] {
            let (heads, tails) = heads_and_tails(&prompt_of(BASELINE_ARM, first));
            let (moved_heads, moved_tails) = heads_and_tails(&prompt_of(BASELINE_ARM, second));
            assert_ne!(heads, moved_heads, "{second} reorders {first}");
            assert_eq!(
                heads.iter().collect::<BTreeSet<_>>(),
                moved_heads.iter().collect::<BTreeSet<_>>(),
                "{second} holds the lines of {first}"
            );
            assert_eq!(tails, moved_tails, "each position keeps its age and status");
            let storm = |heads: &[String]| heads.iter().position(|h| h.contains("] Retry storm: "));
            assert_eq!(storm(&heads), Some(0), "{first}: the retry storm is newest");
            assert_eq!(
                storm(&moved_heads),
                Some(heads.len() - 1),
                "{second}: the retry storm is oldest"
            );
        }
    }

    #[test]
    fn s16_holds_two_lines_for_the_triggering_service_and_three_for_the_sibling() {
        let kept: Vec<Option<String>> = selected_corpus_incidents(&shape("S16"))
            .into_iter()
            .map(|i| i.scope_id)
            .collect();
        assert_eq!(
            kept,
            [
                Some(SIBLING_SERVICE.to_string()),
                Some(SIBLING_SERVICE.to_string())
            ],
            "under its cue the product keeps the triggering service's two"
        );
        let selected = baseline_corpus_incidents(&shape("S16"));
        let scoped = |scope: &str| {
            selected
                .iter()
                .filter(|i| i.scope_id.as_deref() == Some(scope))
                .count()
        };
        assert_eq!(
            (
                selected.len(),
                scoped(SIBLING_SERVICE),
                scoped(SIBLING_CANARY)
            ),
            (5, 2, 3)
        );
        for incident in &selected {
            assert_eq!(
                incident.fingerprint == STORM_FINGERPRINT,
                incident.scope_id.as_deref() == Some(SIBLING_SERVICE),
                "only the triggering service's lines carry the cue's fingerprint"
            );
        }
    }

    #[test]
    fn the_new_shapes_count_in_neither_half_of_the_bar() {
        let per_shape: BTreeMap<&'static str, u32> =
            REPRODUCTION_SHAPES.into_iter().map(|id| (id, 20)).collect();
        assert_eq!(bar_counts(&per_shape, &per_shape), BarCounts::default());
    }

    #[test]
    fn the_reproduction_reading_takes_the_shipped_arm_a_new_shape_and_no_other_verdict_flag() {
        let reproduce = |items: &[&str]| parse_args_from(argv(items)).map(|a| a.reproduce);
        let refused = |items: &[&str]| reproduce(items).err();
        assert_eq!(
            reproduce(&["--arms", "shipped", "--shapes", "S7,S9"]),
            Ok(None)
        );
        let reading = [
            "--arms",
            "shipped",
            "--shapes",
            "S7,S8,S9",
            "--reproduce-misses",
            "2",
        ];
        assert_eq!(reproduce(&reading), Ok(Some(2)));
        for arms in ["shipped,ns", "ns", "A0"] {
            assert_eq!(
                refused(&["--arms", arms, "--shapes", "S9", "--reproduce-misses", "2"]).as_deref(),
                Some("the reproduction reading takes arm shipped alone"),
                "{arms}"
            );
        }
        assert_eq!(
            refused(&["--shapes", "S9", "--reproduce-misses", "2"]).as_deref(),
            Some("the reproduction reading takes arm shipped alone"),
            "the default arm"
        );
        assert_eq!(
            refused(&[
                "--arms",
                "shipped",
                "--shapes",
                "S7,S8",
                "--reproduce-misses",
                "2"
            ])
            .as_deref(),
            Some("the reproduction reading needs a shape of S9-S16")
        );
        let beside: [&[&str]; 3] = [
            &["--min", "5"],
            &["--min-rank1", "5"],
            &["--bar-sibling", "19", "--bar-ordinary", "36"],
        ];
        for flags in beside {
            assert_eq!(
                refused(&[&reading[..], flags].concat()).as_deref(),
                Some("--reproduce-misses takes no other verdict flag"),
                "{flags:?}"
            );
        }
        assert_eq!(
            refused(&[
                "--arms",
                "shipped",
                "--shapes",
                "S9",
                "--reproduce-misses",
                "0"
            ])
            .as_deref(),
            Some("--reproduce-misses takes a number of at least 1")
        );
        assert_eq!(
            refused(&["--reproduce-misses", "two"]).as_deref(),
            Some("--reproduce-misses takes a number")
        );
    }

    #[test]
    fn a_shape_reproduces_at_k_misses_and_is_clean_only_below_k_with_its_unparsed_rows() {
        let read = |misses, unparsed| reproduce_shape(miss(misses, unparsed), 2);
        for (misses, unparsed) in [(2, 0), (20, 0), (2, 18)] {
            assert_eq!(read(misses, unparsed), "REPRODUCES", "{misses} {unparsed}");
        }
        for (misses, unparsed) in [(0, 0), (1, 0), (0, 1)] {
            assert_eq!(read(misses, unparsed), "CLEAN", "{misses} {unparsed}");
        }
        for (misses, unparsed) in [(1, 1), (0, 2), (0, 20)] {
            assert_eq!(read(misses, unparsed), "UNREAD", "{misses} {unparsed}");
        }
        assert_eq!(reproduce_shape(MissCounts::default(), 2), "UNREAD");
        assert_eq!(reproduce_shape(miss(2, 0), 3), "CLEAN");
        assert_eq!(reproduce_shape(miss(3, 0), 3), "REPRODUCES");
    }

    #[test]
    fn an_unparsed_generation_is_counted_apart_and_never_as_a_miss() {
        let mut counts = MissCounts::default();
        for label in [
            "both",
            "service_only",
            "signal_only",
            "neither",
            "unparsed",
            "unparsed",
        ] {
            fold_miss(&mut counts, label);
        }
        assert_eq!(
            counts,
            MissCounts {
                misses: 2,
                unparsed: 2,
                n: 6
            }
        );

        let mut unparsed = MissCounts::default();
        for _ in 0..20 {
            fold_miss(&mut unparsed, "unparsed");
        }
        assert_eq!(unparsed, miss(0, 20));
        assert_eq!(reproduce_shape(unparsed, 2), "UNREAD");
    }

    #[test]
    fn the_reproduction_verdict_reads_the_eight_new_shapes() {
        let verdict = |shapes: &BTreeMap<&'static str, MissCounts>| reproduction_verdict(shapes, 2);
        assert_eq!(
            verdict(&all_clean()),
            (
                "l4-decision-probe: reproduction verdict: NOT REPRODUCED · shapes none".to_string(),
                1
            )
        );

        let mut reproduced = all_clean();
        reproduced.insert("S14", miss(3, 0));
        reproduced.insert("S9", miss(2, 1));
        reproduced.insert("S12", miss(1, 1));
        assert_eq!(
            verdict(&reproduced),
            (
                "l4-decision-probe: reproduction verdict: REPRODUCED · shapes S9,S14".to_string(),
                0
            )
        );

        let mut unread = all_clean();
        unread.insert("S12", miss(1, 1));
        assert_eq!(
            verdict(&unread),
            (
                "l4-decision-probe: INCONCLUSIVE - no shape reproduces and S12 unread".to_string(),
                2
            )
        );

        let mut not_run = all_clean();
        not_run.remove("S16");
        not_run.remove("S10");
        assert_eq!(
            verdict(&not_run),
            (
                "l4-decision-probe: INCONCLUSIVE - no shape reproduces and S10,S16 unread"
                    .to_string(),
                2
            )
        );
    }

    #[test]
    fn a_control_changes_no_reproduction_verdict() {
        let not_reproduced = reproduction_verdict(&all_clean(), 2);
        for control in [miss(20, 0), miss(0, 20)] {
            let mut shapes = all_clean();
            shapes.insert("S7", control);
            shapes.insert("S8", control);
            assert_eq!(reproduction_verdict(&shapes, 2), not_reproduced);
        }
        let controls_alone: BTreeMap<&'static str, MissCounts> =
            [("S7", miss(20, 0)), ("S8", miss(20, 0))].into();
        assert_eq!(reproduction_verdict(&controls_alone, 2).1, 2);
    }

    #[test]
    fn the_reproduce_line_takes_its_printed_form() {
        assert_eq!(
            reproduce_line("S7", miss(20, 0), 2),
            "  arm shipped: reproduce S7 REPRODUCES · misses 20/20 · unparsed 0"
        );
        assert_eq!(
            reproduce_line("S16", miss(1, 2), 2),
            "  arm shipped: reproduce S16 UNREAD · misses 1/20 · unparsed 2"
        );
        assert_eq!(
            reproduce_line("S9", miss(1, 0), 2),
            "  arm shipped: reproduce S9 CLEAN · misses 1/20 · unparsed 0"
        );
    }

    const REMEDY_BAR: Bar = Bar {
        sibling_min: 38,
        ordinary_min: 72,
    };
    const REPLAY_FLAGS: [&str; 4] = [
        "--replay",
        "miss=/captures/a,control=/captures/b",
        "--replay-scope",
        "conductor",
    ];

    fn parsed(items: &[&str]) -> Args {
        match parse_args_from(argv(items)) {
            Ok(args) => args,
            Err(why) => panic!("{items:?} parse: {why}"),
        }
    }

    fn refused(items: &[&str]) -> String {
        match parse_args_from(argv(items)) {
            Ok(_) => panic!("{items:?} is refused"),
            Err(why) => why,
        }
    }

    fn tally(both: u32, misses: u32, unparsed: u32, n: u32) -> Tally {
        Tally {
            both,
            misses,
            unparsed,
            n,
        }
    }

    /// An arm's remedy counts: `not_both` generations of `n` on the remedy
    /// prompt, `misses` of them service misses, and `both` of 40 and of 80
    /// over the guard halves.
    fn remedy(n: u32, not_both: u32, misses: u32, sibling: u32, ordinary: u32) -> RemedyCounts {
        RemedyCounts {
            remedy: tally(n - not_both, misses, 0, n),
            sibling: tally(sibling, 0, 0, 40),
            ordinary: tally(ordinary, 0, 0, 80),
            derived: None,
        }
    }

    fn composed(arm: &str, shape: &'static str, prompt: &str) -> Prepared {
        Prepared {
            arm: arm.to_string(),
            shape,
            trigger: CueKind::RetryStorm,
            scope_id: None,
            prompt: prompt.to_string(),
            schema: String::new(),
            extra_args: Vec::new(),
            drop_flags: Vec::new(),
        }
    }

    #[test]
    fn the_replay_flags_parse_and_a_replay_brings_no_s_shape_of_its_own() {
        let args = parsed(&[&["--arms", "shipped,CR"], &REPLAY_FLAGS[..]].concat());
        assert_eq!(
            args.replays,
            [
                ("miss".to_string(), PathBuf::from("/captures/a")),
                ("control".to_string(), PathBuf::from("/captures/b")),
            ]
        );
        assert_eq!(args.replay_scope.as_deref(), Some("conductor"));
        assert_eq!(args.own_lines, OwnLines::Unknown);
        assert!(args.shapes.is_empty(), "no S shape beside a replay");
        assert_eq!(args.mode, Mode::Run);

        let beside = parsed(
            &[
                &[
                    "--arms",
                    "shipped",
                    "--shapes",
                    "S8,S7",
                    "--own-lines",
                    "2,5",
                ],
                &REPLAY_FLAGS[..],
            ]
            .concat(),
        );
        assert_eq!(selected_ids(&beside), ["S7", "S8"]);
        assert_eq!(beside.own_lines, OwnLines::Positions(vec![2, 5]));
        assert_eq!(
            replay_own_lines(&beside, "replay:miss"),
            OwnLines::Positions(vec![2, 5])
        );
        assert_eq!(
            replay_own_lines(&beside, "replay:control"),
            OwnLines::Unknown,
            "the positions were read against the miss block alone"
        );
        assert_eq!(
            refused(&[
                "--arms",
                "shipped,CX",
                "--replay",
                "control=/captures/b",
                "--replay-scope",
                "conductor",
                "--own-lines",
                "1",
            ]),
            "--own-lines describes the replay labelled miss"
        );

        let with = |extra: &[&str]| refused(&[extra, &REPLAY_FLAGS[..]].concat());
        assert_eq!(with(&[]), "arm A0 does not apply to a replay");
        for arm in ["ns", "nb"] {
            assert_eq!(
                with(&["--arms", &format!("shipped,{arm}")]),
                format!("arm {arm} does not apply to a replay")
            );
        }
        let path = |extra: &[&str]| {
            let items = [&["--arms", "shipped,CX"], extra, &REPLAY_FLAGS[..]].concat();
            parse_args_from(argv(&items)).map(|args| args.product_path)
        };
        assert_eq!(path(&["--own-lines", "2,5", "--dry-run"]), Ok(false));
        for own in ["2,5", "none"] {
            assert_eq!(
                path(&["--own-lines", own, "--dry-run", "--product-path"]),
                Ok(true),
                "{own}"
            );
        }
        for missing in [
            &["--own-lines", "2,5", "--product-path"][..],
            &["--dry-run", "--product-path"],
            &["--own-lines", "unknown", "--dry-run", "--product-path"],
        ] {
            assert_eq!(
                path(missing).err().as_deref(),
                Some("--product-path needs --dry-run and the own lines of a replay"),
                "{missing:?}"
            );
        }
        assert_eq!(
            with(&["--arms", "shipped", "--shapes", "A1"]),
            "--replay never runs beside a pattern shape"
        );
        assert_eq!(
            with(&["--arms", "shipped", "--sampling", "--temp 0"]),
            "--sampling does not apply beside --replay"
        );
        assert_eq!(
            refused(&["--arms", "shipped", "--replay", "miss=/captures/a"]),
            "--replay needs --replay-scope"
        );
        for alone in [
            ["--replay-scope", "conductor"],
            ["--own-lines", "none"],
            ["--remedy-from", "target/x"],
        ] {
            assert_eq!(
                refused(&alone),
                "--replay-scope, --own-lines and --remedy-from need --replay",
                "{alone:?}"
            );
        }
    }

    #[test]
    fn the_replay_reading_takes_the_shipped_arm_and_a_replay_labelled_miss() {
        let reading = [
            &["--arms", "shipped", "--reproduce-misses", "2"],
            &REPLAY_FLAGS[..],
        ]
        .concat();
        let args = parsed(&reading);
        assert_eq!(args.reproduce, Some(2));
        assert!(args.shapes.is_empty(), "the reading needs no shape");
        assert_eq!(
            refused(&[
                "--arms",
                "shipped",
                "--reproduce-misses",
                "2",
                "--replay",
                "control=/captures/b",
                "--replay-scope",
                "conductor",
            ]),
            "the replay reading needs a replay labelled miss"
        );
        assert_eq!(
            refused(
                &[
                    &["--arms", "shipped,CR", "--reproduce-misses", "2"],
                    &REPLAY_FLAGS[..]
                ]
                .concat()
            ),
            "the reproduction reading takes arm shipped alone"
        );
        assert_eq!(
            refused(&[&reading[..], &["--min", "5"]].concat()),
            "--reproduce-misses takes no other verdict flag"
        );
    }

    #[test]
    fn the_replay_verdict_reads_the_miss_replay_and_no_control() {
        let verdict = |counts: &[(&'static str, MissCounts)]| {
            replay_verdict(&counts.iter().copied().collect(), 2)
        };
        let reproduced = (
            "l4-decision-probe: reproduction verdict: REPRODUCED · replay:miss".to_string(),
            0,
        );
        let not_reproduced = (
            "l4-decision-probe: reproduction verdict: NOT REPRODUCED · replay:miss".to_string(),
            1,
        );
        let unread = (
            "l4-decision-probe: INCONCLUSIVE - replay:miss unread".to_string(),
            2,
        );
        assert_eq!(verdict(&[("replay:miss", miss(2, 0))]), reproduced);
        assert_eq!(verdict(&[("replay:miss", miss(1, 0))]), not_reproduced);
        assert_eq!(verdict(&[("replay:miss", miss(1, 1))]), unread);
        assert_eq!(verdict(&[("replay:control", miss(20, 0))]), unread);
        for control in [miss(20, 0), miss(0, 20), miss(0, 0)] {
            assert_eq!(
                verdict(&[("replay:miss", miss(1, 0)), ("replay:control", control)]),
                not_reproduced,
                "a control that reproduces changes nothing"
            );
            assert_eq!(
                verdict(&[
                    ("replay:miss", miss(5, 0)),
                    ("replay:control", control),
                    ("S16", control)
                ]),
                reproduced
            );
        }
    }

    #[test]
    fn the_replay_verdict_reads_every_replay_but_the_control() {
        let verdict = |counts: &[(&'static str, MissCounts)]| {
            replay_verdict(&counts.iter().copied().collect(), 2)
        };
        let line = |tail: &str| format!("l4-decision-probe: reproduction verdict: {tail}");
        assert_eq!(
            verdict(&[("replay:miss", miss(0, 0)), ("replay:d2", miss(2, 0))]),
            (line("REPRODUCED · replay:d2"), 0),
            "a second prompt reproduces where miss reads clean"
        );
        assert_eq!(
            verdict(&[("replay:miss", miss(1, 1)), ("replay:d2", miss(3, 0))]),
            (line("REPRODUCED · replay:d2"), 0),
            "and where miss is unread"
        );
        assert_eq!(
            verdict(&[("replay:miss", miss(4, 0)), ("replay:d2", miss(2, 0))]),
            (line("REPRODUCED · replay:miss,replay:d2"), 0),
            "miss is named first"
        );
        for control in [miss(20, 0), miss(0, 20), miss(0, 0)] {
            assert_eq!(
                verdict(&[
                    ("replay:miss", miss(1, 0)),
                    ("replay:d2", miss(0, 1)),
                    ("replay:control", control),
                    ("S16", miss(20, 0)),
                ]),
                (line("NOT REPRODUCED · replay:miss,replay:d2"), 1),
                "all clean, whatever the control and an S shape read"
            );
        }
        assert_eq!(
            verdict(&[("replay:miss", miss(0, 0)), ("replay:d2", miss(1, 1))]),
            (
                "l4-decision-probe: INCONCLUSIVE - replay:d2 unread".to_string(),
                2
            ),
            "one prompt unread and none reproducing"
        );
        assert_eq!(
            verdict(&[("replay:d2", miss(0, 0))]),
            (
                "l4-decision-probe: INCONCLUSIVE - replay:miss unread".to_string(),
                2
            ),
            "miss is read even when it did not run"
        );
    }

    #[test]
    fn the_second_count_reads_the_first_statement_for_the_other_id() {
        const OTHER: &str = "conductor-canary";
        let label = |statement: &str| names_other(Some(&first(statement)), OTHER);
        for statement in [
            "Retry storm on conductor is active in conductor-canary.",
            "A retry storm hit Conductor-Canary",
            "The conductor canary service keeps retrying",
            "retry_storm on conductor_canary",
        ] {
            assert_eq!(label(statement), "named", "{statement}");
        }
        for statement in [
            "Retry storm in conductor scope_id=conductor is active",
            "Retry storm on conductor-canary-2",
            "Retry storm on pre-conductor-canary",
            "Retry storm on conductor, canary unaffected",
        ] {
            assert_eq!(label(statement), "not_named", "{statement}");
        }
        assert_eq!(
            names_other(
                Some(&first("Retry storm on conductor canary")),
                "conductor_canary"
            ),
            "named",
            "a joint in the id is read as any of the three too"
        );

        let later = output(
            "Retry storm on conductor-canary",
            "conductor-canary retries",
            &["Retry storm on conductor", "It spread to conductor-canary"],
        );
        assert_eq!(names_other(Some(&later), OTHER), "not_named");
        assert_eq!(names_other(None, OTHER), "unparsed");
        let empty = output("Retry storm on conductor-canary", "x", &[]);
        assert_eq!(names_other(Some(&empty), OTHER), "unparsed");
    }

    #[test]
    fn the_second_count_folds_prints_and_rides_a_row_as_a_closed_label() {
        let mut counts = NamedCounts::default();
        for label in ["named", "not_named", "unparsed", "named", "not_named"] {
            fold_named(&mut counts, label);
        }
        assert_eq!(
            counts,
            NamedCounts {
                named: 2,
                unparsed: 1,
                n: 5
            }
        );
        assert_eq!(
            second_count_line("shipped", "replay:miss", counts),
            "  arm shipped: second count replay:miss · other scope named 2/5 · unparsed 1"
        );

        let mut out = output("t", "s", &["zq-statement-77 in conductor-canary"]);
        out.fingerprint = STORM_FINGERPRINT.to_string();
        out.hardware_profile = "gpu_primary".to_string();
        let Ok(stdout) = serde_json::to_string(&out) else {
            panic!("the synthetic output serializes");
        };
        let metrics = Metrics {
            elapsed_ms: 1,
            peak_rss_kib: None,
            peak_vram_mib: None,
        };
        let parsed = parsed_output(&Spawned::Output(stdout, metrics));
        assert_eq!(names_other(parsed.as_ref(), "conductor-canary"), "named");
        assert!(parsed_output(&Spawned::Failed("timeout", metrics)).is_none());
        assert!(parsed_output(&Spawned::Output("no json".to_string(), metrics)).is_none());

        let row = with_names_other(json!({"identifies": "both"}), "named");
        assert_eq!(row, json!({"identifies": "both", "names_other": "named"}));
    }

    #[test]
    fn count_naming_parses_as_an_id_and_moves_no_reading() {
        let reading = [
            &["--arms", "shipped", "--reproduce-misses", "2"],
            &REPLAY_FLAGS[..],
        ]
        .concat();
        let plain = parsed(&reading);
        let counted = parsed(&[&reading[..], &["--count-naming", "conductor-canary"]].concat());
        assert_eq!(plain.count_naming, None);
        assert_eq!(counted.count_naming.as_deref(), Some("conductor-canary"));
        assert_eq!(
            (counted.reproduce, counted.bar, counted.min, counted.n),
            (plain.reproduce, plain.bar, plain.min, plain.n)
        );
        assert_eq!(counted.replays, plain.replays);
        for bad in ["", "conductor canary", "a/b", "x\ny"] {
            assert_eq!(
                refused(&[&reading[..], &["--count-naming", bad]].concat()),
                "--count-naming takes a service id",
                "{bad:?}"
            );
        }
        assert_eq!(
            refused(&[&reading[..], &["--count-naming"]].concat()),
            "--count-naming needs a value"
        );
    }

    #[test]
    fn the_remedy_reading_parses_its_flags_and_selects_its_own_shapes() {
        let flags = [
            "--own-lines",
            "2,5",
            "--remedy-from",
            "target/l4-decision-probe/replay-x",
            "--bar-sibling",
            "38",
            "--bar-ordinary",
            "72",
        ];
        let reading = [
            &["--arms", "shipped,CR,CO,CC,CX"],
            &REPLAY_FLAGS[..],
            &flags[..],
        ]
        .concat();
        let args = parsed(&reading);
        assert_eq!(
            args.remedy_from,
            Some(PathBuf::from("target/l4-decision-probe/replay-x"))
        );
        assert_eq!(
            args.shapes,
            ["S1", "S2", "S3", "S4", "S7", "S8", "S17"],
            "the guards, then the derived shape the probe defines"
        );
        assert_eq!(args.bar, Some(REMEDY_BAR), "graded with no ns arm");
        assert_eq!(args.own_lines, OwnLines::Positions(vec![2, 5]));

        let without = |dropped: &str, width: usize| {
            let at = reading
                .iter()
                .position(|item| *item == dropped)
                .unwrap_or_else(|| panic!("{dropped} is in the reading"));
            let mut items = reading.clone();
            items.drain(at..at + width);
            refused(&items)
        };
        assert_eq!(
            without("--own-lines", 2),
            "the remedy reading needs --own-lines"
        );
        assert_eq!(
            without("--bar-sibling", 4),
            "the remedy reading needs --bar-sibling and --bar-ordinary"
        );
        assert_eq!(
            without("--bar-ordinary", 2),
            "--bar-sibling and --bar-ordinary go together"
        );
        let with_arms =
            |arms: &str| refused(&[&["--arms", arms], &REPLAY_FLAGS[..], &flags[..]].concat());
        for arms in ["shipped", "CR,CX"] {
            assert_eq!(
                with_arms(arms),
                "the remedy reading needs arm shipped and a candidate arm",
                "{arms}"
            );
        }
        assert_eq!(
            refused(
                &[
                    &[
                        "--arms",
                        "shipped,CR",
                        "--replay",
                        "miss=/captures/a",
                        "--replay-scope",
                        "conductor"
                    ],
                    &flags[..]
                ]
                .concat()
            ),
            "the remedy reading needs a replay labelled control"
        );
        assert_eq!(
            refused(&[&reading[..], &["--shapes", "S7"]].concat()),
            "the remedy reading selects its own shapes"
        );
        for other in [
            ["--reproduce-misses", "2"],
            ["--min", "5"],
            ["--min-rank1", "5"],
        ] {
            assert_eq!(
                refused(&[&reading[..], &other[..]].concat()),
                "--remedy-from takes no other reading's flag",
                "{other:?}"
            );
        }
        assert_eq!(
            refused(&[
                "--arms",
                "shipped,CR",
                "--shapes",
                "S1,S7",
                "--bar-sibling",
                "19",
                "--bar-ordinary",
                "36",
            ]),
            "the bar needs arm ns",
            "without --remedy-from the bar is the predecessor's"
        );
    }

    #[test]
    fn the_sections_flag_is_a_mode_of_its_own() {
        assert_eq!(
            parsed(&["--sections", "S8,S16"]).mode,
            Mode::Sections(vec![
                SectionSource::Shape("S8".to_string()),
                SectionSource::Shape("S16".to_string()),
            ])
        );
        assert_eq!(
            parsed(&["--sections", "miss=/captures/a,S14"]).mode,
            Mode::Sections(vec![
                SectionSource::Capture("miss".to_string(), PathBuf::from("/captures/a")),
                SectionSource::Shape("S14".to_string()),
            ])
        );
        for unknown in ["S99", "A1"] {
            assert_eq!(
                refused(&["--sections", &format!("S8,{unknown}")]),
                format!("unknown shape {unknown}")
            );
        }
        assert_eq!(
            refused(&["--sections", "S8,S16", "--table", "target/s"]),
            "--sections takes no other mode flag"
        );
        let rows = match section_rows(&[
            SectionSource::Shape("S8".to_string()),
            SectionSource::Shape("S16".to_string()),
            SectionSource::Shape("S7".to_string()),
        ]) {
            Ok(rows) => rows,
            Err(why) => panic!("the shapes are read: {why}"),
        };
        let summaries: Vec<&String> = rows
            .iter()
            .filter(|r| r.contains(": differs in "))
            .collect();
        assert_eq!(
            summaries,
            [
                "sections: S8 vs S16: differs in digest.corpus-matches",
                "sections: S8 vs S7: differs in digest.corpus-matches",
            ],
            "every source after the first, against the first"
        );
    }

    #[test]
    fn the_remedy_prompt_runs_at_the_n_that_expects_five_baseline_misses() {
        for (misses, n) in [(2, 50), (3, 34), (4, 25), (5, 20), (6, 20), (20, 20)] {
            assert_eq!(remedy_n(misses), Some(n), "m = {misses}");
            assert!(n * misses >= 100, "m = {misses}: an expected 5 of n");
        }
        for misses in [0, 1] {
            assert_eq!(remedy_n(misses), None, "m = {misses}: not reproduced");
        }
        for (n, allowance) in [(20, 1), (25, 1), (34, 1), (50, 2)] {
            assert_eq!(remedy_allowance(n), allowance, "n = {n}");
        }
    }

    #[test]
    fn the_known_positive_holds_at_the_allowance_plus_two_and_not_at_plus_one() {
        for (n, min) in [(20, 3), (25, 3), (34, 3), (50, 4)] {
            assert!(known_positive_held(min, n), "n = {n}: {min} misses hold");
            assert!(known_positive_held(n, n), "n = {n}");
            assert!(
                !known_positive_held(min - 1, n),
                "n = {n}: one miss over the allowance is lost"
            );
        }
    }

    #[test]
    fn a_candidates_bar_needs_the_allowance_and_both_guard_minimums() {
        let met = |counts| remedy_bar_met(counts, remedy_allowance(50), REMEDY_BAR);
        assert!(met(remedy(50, 2, 2, 38, 72)));
        assert!(met(remedy(50, 0, 0, 40, 80)));
        assert!(!met(remedy(50, 3, 0, 40, 80)), "over the allowance");
        assert!(!met(remedy(50, 2, 0, 37, 80)), "the sibling half");
        assert!(!met(remedy(50, 2, 0, 40, 71)), "the ordinary half");
        let mut unparsed = remedy(20, 0, 0, 40, 80);
        unparsed.remedy = tally(18, 0, 2, 20);
        assert!(
            !remedy_bar_met(unparsed, remedy_allowance(20), REMEDY_BAR),
            "an unparsed generation is not both"
        );
        unparsed.remedy = tally(19, 0, 1, 20);
        assert!(remedy_bar_met(unparsed, remedy_allowance(20), REMEDY_BAR));
    }

    #[test]
    fn the_remedy_guard_trips_only_below_shipped_on_a_guard_half() {
        let shipped = remedy(50, 6, 6, 39, 78);
        assert_eq!(remedy_guard(remedy(50, 0, 0, 39, 78), shipped), "HOLDS");
        assert_eq!(remedy_guard(remedy(50, 9, 9, 40, 80), shipped), "HOLDS");
        assert_eq!(remedy_guard(remedy(50, 0, 0, 38, 80), shipped), "TRIPPED");
        assert_eq!(remedy_guard(remedy(50, 0, 0, 40, 77), shipped), "TRIPPED");
    }

    #[test]
    fn the_remedy_selection_takes_the_first_candidate_in_order_that_is_selectable() {
        let shipped = remedy(50, 6, 6, 39, 78);
        let select = |candidates: &[(&'static str, RemedyArm)]| {
            remedy_selection(candidates, shipped, remedy_allowance(50), REMEDY_BAR)
        };
        let good = RemedyArm::Read(remedy(50, 1, 1, 40, 80));
        let not_met = RemedyArm::Read(remedy(50, 3, 3, 40, 80));
        let tripped = RemedyArm::Read(remedy(50, 0, 0, 38, 80));
        assert_eq!(
            select(&[("CX", good), ("CC", good), ("CO", tripped), ("CR", not_met)]),
            Some("CC"),
            "the order is the candidates', not the list's"
        );
        assert_eq!(
            select(&[("CR", good), ("CO", good), ("CC", good), ("CX", good)]),
            Some("CR")
        );
        assert_eq!(
            select(&[
                (
                    "CR",
                    RemedyArm::Skipped("composes as shipped on replay:miss")
                ),
                ("CX", good)
            ]),
            Some("CX"),
            "a skipped arm is never selectable"
        );
        assert_eq!(select(&[("CR", not_met), ("CO", tripped)]), None);
        assert_eq!(select(&[]), None);
    }

    #[test]
    fn a_candidate_composing_as_shipped_is_skipped_or_takes_shippeds_counts() {
        let prepared = [
            composed("shipped", "replay:miss", "m"),
            composed("shipped", "replay:control", "c"),
            composed("shipped", "S1", "one"),
            composed("shipped", "S7", "seven"),
            composed("shipped", "S8", "eight"),
            composed("CR", "replay:miss", "m restated"),
            composed("CR", "replay:control", "c"),
            composed("CR", "S1", "one"),
            composed("CR", "S7", "seven"),
            composed("CR", "S8", "eight restated"),
            composed("CC", "replay:miss", "m"),
            composed("CC", "replay:control", "c capped"),
            composed("CC", "S8", "eight"),
            composed("CO", "S8", "eight"),
        ];
        assert_eq!(
            remedy_arm_plan(&prepared, "CR"),
            ArmPlan::Run(vec!["S1", "S7"]),
            "the control replay is generated whatever it composes as"
        );
        assert_eq!(
            remedy_arm_plan(&prepared, "CC"),
            ArmPlan::Skipped("composes as shipped on replay:miss")
        );
        assert_eq!(
            remedy_arm_plan(&prepared, "CO"),
            ArmPlan::Skipped("own lines unknown")
        );

        let tallies: BTreeMap<(String, &'static str), Tally> = [
            (("shipped", "replay:miss"), tally(44, 6, 0, 50)),
            (("shipped", "S1"), tally(19, 1, 0, 20)),
            (("shipped", "S7"), tally(20, 0, 0, 20)),
            (("shipped", "S8"), tally(19, 0, 1, 20)),
            (("CR", "replay:miss"), tally(49, 1, 0, 50)),
            (("CR", "S8"), tally(18, 2, 0, 20)),
        ]
        .into_iter()
        .map(|((arm, shape), counts)| ((arm.to_string(), shape), counts))
        .collect();
        let read = remedy_counts(&tallies, "CR", &["S1", "S7"], false);
        assert_eq!(read.remedy, tally(49, 1, 0, 50));
        assert_eq!(
            read.sibling,
            tally(38, 2, 0, 40),
            "S7 from shipped, S8 its own"
        );
        assert_eq!(read.ordinary, tally(19, 1, 0, 20), "S1 from shipped");
        assert_eq!(read.derived, None);
        assert_eq!(
            remedy_counts(&tallies, "shipped", &[], true).derived,
            Some(Tally::default()),
            "a defined derived shape is read, here with no generation"
        );
    }

    #[test]
    fn a_generation_folds_into_its_tally_by_its_identifies_label() {
        let mut counts = Tally::default();
        for label in [
            "both",
            "both",
            "service_only",
            "signal_only",
            "neither",
            "unparsed",
        ] {
            fold_tally(&mut counts, label);
        }
        assert_eq!(counts, tally(2, 2, 1, 6));
        assert_eq!(counts.not_both(), 4);
    }

    #[test]
    fn the_derived_shape_reproduces_at_two_misses_and_is_covered_within_one_not_both() {
        let reproducing = tally(18, 2, 0, 20);
        assert_eq!(
            derived_reading(reproducing, Some(tally(19, 1, 0, 20))),
            ("REPRODUCES", "COVERED")
        );
        assert_eq!(
            derived_reading(reproducing, Some(tally(18, 1, 1, 20))),
            ("REPRODUCES", "NOT COVERED")
        );
        assert_eq!(
            derived_reading(reproducing, None),
            ("REPRODUCES", "not read against a selection")
        );
        assert_eq!(
            derived_reading(tally(19, 1, 0, 20), Some(tally(20, 0, 0, 20))),
            ("CLEAN", "not read against a selection")
        );
        assert_eq!(
            derived_reading(tally(18, 1, 1, 20), Some(tally(20, 0, 0, 20))),
            ("UNREAD", "not read against a selection")
        );
    }

    #[test]
    fn the_remedy_reading_prints_its_lines_in_order_and_exits_by_its_verdict() {
        let shipped = remedy(50, 7, 6, 39, 78);
        let candidates = [
            ("CR", RemedyArm::Read(remedy(50, 3, 2, 40, 80))),
            ("CO", RemedyArm::Skipped("own lines unknown")),
            ("CC", RemedyArm::Read(remedy(50, 1, 1, 38, 79))),
            ("CX", RemedyArm::Read(remedy(50, 2, 0, 40, 78))),
        ];
        let (lines, exit) = remedy_reading(shipped, &candidates, 50, REMEDY_BAR);
        assert_eq!(exit, 0);
        assert_eq!(
            lines,
            [
                "  arm CR: remedy bar NOT MET · guard HOLDS · not both 3/50 (allowance 2) · sibling both 40/40 (min 38, shipped 39) · ordinary both 80/80 (min 72, shipped 78)",
                "  arm CO: skipped · own lines unknown",
                "  arm CC: remedy bar MET · guard TRIPPED · not both 1/50 (allowance 2) · sibling both 38/40 (min 38, shipped 39) · ordinary both 79/80 (min 72, shipped 78)",
                "  arm CX: remedy bar MET · guard HOLDS · not both 2/50 (allowance 2) · sibling both 40/40 (min 38, shipped 39) · ordinary both 78/80 (min 72, shipped 78)",
                "l4-decision-probe: known-positive: HELD · shipped misses 6/50 on replay:miss (min 4)",
                "l4-decision-probe: remedy selection: CX · order CR,CO,CC,CX",
                "l4-decision-probe: derived shape: absent",
                "l4-decision-probe: remedy verdict: SELECTED · arm CX",
            ]
        );

        let (none, exit) = remedy_reading(shipped, &candidates[..3], 50, REMEDY_BAR);
        assert_eq!(exit, 1);
        assert_eq!(
            none[3..],
            [
                "l4-decision-probe: known-positive: HELD · shipped misses 6/50 on replay:miss (min 4)",
                "l4-decision-probe: remedy selection: none · order CR,CO,CC,CX",
                "l4-decision-probe: derived shape: absent",
                "l4-decision-probe: remedy verdict: NONE",
            ]
        );

        let (lost, exit) = remedy_reading(remedy(50, 5, 3, 39, 78), &candidates, 50, REMEDY_BAR);
        assert_eq!(exit, 2);
        assert_eq!(
            lost[4..],
            [
                "l4-decision-probe: known-positive: LOST · shipped misses 3/50 on replay:miss (min 4)",
                "l4-decision-probe: derived shape: absent",
                "l4-decision-probe: INCONCLUSIVE - the known-positive is lost on replay:miss",
            ]
        );
    }

    #[test]
    fn the_derived_shape_is_recorded_and_moves_no_selection_and_no_exit() {
        let with =
            |counts: RemedyCounts, derived: Option<Tally>| RemedyCounts { derived, ..counts };
        let line = |lines: &[String]| {
            lines
                .iter()
                .find(|l| l.starts_with("l4-decision-probe: derived shape: "))
                .cloned()
                .unwrap_or_default()
        };
        let others = |lines: &[String]| -> Vec<String> {
            lines
                .iter()
                .filter(|l| !l.starts_with("l4-decision-probe: derived shape: "))
                .cloned()
                .collect()
        };
        let shipped = remedy(20, 4, 4, 39, 78);
        let selectable = remedy(20, 1, 1, 40, 80);
        let not_met = remedy(20, 2, 2, 40, 80);
        let read = |shipped_on: Option<Tally>,
                    cr_on: Option<Tally>,
                    cx_on: Option<Tally>,
                    cx: RemedyCounts| {
            remedy_reading(
                with(shipped, shipped_on),
                &[
                    ("CR", RemedyArm::Read(with(not_met, cr_on))),
                    ("CX", RemedyArm::Read(with(cx, cx_on))),
                ],
                20,
                REMEDY_BAR,
            )
        };

        let (absent, exit) = read(None, None, None, selectable);
        assert_eq!(exit, 0);
        assert_eq!(line(&absent), "l4-decision-probe: derived shape: absent");

        let reproducing = Some(tally(17, 3, 0, 20));
        let (covered, covered_exit) = read(
            reproducing,
            Some(tally(20, 0, 0, 20)),
            Some(tally(19, 1, 0, 20)),
            selectable,
        );
        assert_eq!(
            line(&covered),
            "l4-decision-probe: derived shape: S17 REPRODUCES · COVERED · shipped misses 3/20 · unparsed 0 · not both CR 0/20 CX 1/20"
        );
        let (uncovered, uncovered_exit) = read(
            reproducing,
            Some(tally(20, 0, 0, 20)),
            Some(tally(10, 10, 0, 20)),
            selectable,
        );
        assert_eq!(
            line(&uncovered),
            "l4-decision-probe: derived shape: S17 REPRODUCES · NOT COVERED · shipped misses 3/20 · unparsed 0 · not both CR 0/20 CX 10/20",
            "the selected candidate's reading, not the best one"
        );
        let (clean, clean_exit) = read(
            Some(tally(20, 0, 0, 20)),
            Some(tally(0, 20, 0, 20)),
            Some(tally(0, 20, 0, 20)),
            selectable,
        );
        assert_eq!(
            line(&clean),
            "l4-decision-probe: derived shape: S17 CLEAN · not read against a selection · shipped misses 0/20 · unparsed 0 · not both CR 20/20 CX 20/20"
        );
        for (lines, exit) in [
            (&covered, covered_exit),
            (&uncovered, uncovered_exit),
            (&clean, clean_exit),
        ] {
            assert_eq!(others(lines), others(&absent));
            assert_eq!(exit, 0);
        }

        let (none, exit) = read(
            reproducing,
            Some(tally(20, 0, 0, 20)),
            Some(tally(20, 0, 0, 20)),
            not_met,
        );
        assert_eq!(exit, 1, "a derived shape every arm clears selects nothing");
        assert_eq!(
            line(&none),
            "l4-decision-probe: derived shape: S17 REPRODUCES · not read against a selection · shipped misses 3/20 · unparsed 0 · not both CR 0/20 CX 0/20"
        );
    }

    #[test]
    fn the_remedy_reading_takes_m_from_the_replay_readings_shipped_miss_rows() {
        let row = |arm: &str, shape: &str, identifies: &str| json!({"arm": arm, "shape": shape, "identifies": identifies});
        let labels = [
            "signal_only",
            "signal_only",
            "signal_only",
            "neither",
            "unparsed",
            "service_only",
        ];
        let mut rows: Vec<Value> = labels
            .iter()
            .map(|label| row("shipped", "replay:miss", label))
            .collect();
        rows.extend((0..14).map(|_| row("shipped", "replay:miss", "both")));
        rows.extend((0..20).map(|_| row("shipped", "replay:control", "neither")));
        rows.extend((0..20).map(|_| row("CR", "replay:miss", "neither")));
        assert_eq!(replay_reading_misses(&rows, "miss"), Ok(4));
        assert_eq!(
            replay_reading_misses(&rows, "control"),
            Ok(20),
            "m is read off the label the reading read the prompt under"
        );
        assert_eq!(
            replay_reading_misses(&rows, "d2"),
            Err("the replay reading holds 0 rows for replay:d2, expected 20".to_string())
        );
        rows.remove(0);
        assert_eq!(
            replay_reading_misses(&rows, "miss"),
            Err("the replay reading holds 19 rows for replay:miss, expected 20".to_string())
        );
        assert_eq!(
            remedy_prompt_line(&RemedySource {
                misses: 4,
                n: 25,
                dir_name: "replay-20261007T140000Z".to_string(),
                read_as: "miss".to_string(),
            }),
            "l4-decision-probe: remedy prompt: replay:miss n 25 (replay reading 4/20) · from replay-20261007T140000Z"
        );
        assert_eq!(
            remedy_prompt_line(&RemedySource {
                misses: 11,
                n: 20,
                dir_name: "replay-x".to_string(),
                read_as: "d2".to_string(),
            }),
            "l4-decision-probe: remedy prompt: replay:miss n 20 (replay reading 11/20 as replay:d2) · from replay-x"
        );
    }

    #[test]
    fn remedy_read_as_names_a_replay_label_and_needs_the_remedy_reading() {
        let remedy = [
            &[
                "--arms",
                "shipped,CR",
                "--own-lines",
                "2",
                "--remedy-from",
                "target/l4-decision-probe/replay-x",
                "--bar-sibling",
                "38",
                "--bar-ordinary",
                "72",
            ],
            &REPLAY_FLAGS[..],
        ]
        .concat();
        assert_eq!(parsed(&remedy).remedy_read_as, "miss");
        let as_d2 = parsed(&[&remedy[..], &["--remedy-read-as", "d2"]].concat());
        assert_eq!(as_d2.remedy_read_as, "d2");
        assert_eq!(as_d2.shapes, parsed(&remedy).shapes);
        assert_eq!(
            refused(&[&remedy[..], &["--remedy-read-as", "a=b"]].concat()),
            "--remedy-read-as takes a replay label"
        );
        assert_eq!(
            refused(
                &[
                    &["--arms", "shipped", "--remedy-read-as", "d2"],
                    &REPLAY_FLAGS[..]
                ]
                .concat()
            ),
            "--remedy-read-as needs --remedy-from"
        );
    }

    #[test]
    fn the_derived_shape_holds_its_three_lines_its_services_order_and_one_active_incident() {
        // Under `nb`, the block the remedy reading measured: newest first, the
        // triggering service's own line second. `shipped` keeps that line alone.
        let prompt = prompt_of(BASELINE_ARM, DERIVED_SHAPE);
        let own = block_line(
            STORM_FINGERPRINT,
            STORM,
            DERIVED_TITLES[1],
            "9m ago, resolved",
        );
        assert_eq!(
            corpus_lines(&prompt),
            [
                block_line(
                    CORPUS_FINGERPRINT,
                    STORM,
                    DERIVED_TITLES[0],
                    "3m ago, active"
                ),
                own.clone(),
                block_line(
                    THIRD_FINGERPRINT,
                    STORM,
                    DERIVED_TITLES[2],
                    "12m ago, resolved"
                ),
            ]
        );
        assert_eq!(corpus_lines(&prompt_of("shipped", DERIVED_SHAPE)), [own]);
        let lines: Vec<&str> = prompt.lines().collect();
        let Some(services) = lines.iter().position(|l| l.starts_with("SERVICES (")) else {
            panic!("the derived shape renders a services table");
        };
        assert_eq!(
            lines[services + 1..services + 3],
            [
                "  conductor-canary     0.3/s | 100.0% | 0ms",
                "  conductor     0.3/s | 100.0% | 0ms",
            ],
            "the sibling's row first, both rows at the same values"
        );
        let overall = |prompt: &str| -> String {
            prompt
                .lines()
                .find(|l| l.starts_with("OVERALL: "))
                .unwrap_or_default()
                .to_string()
        };
        assert_eq!(
            overall(&prompt),
            "OVERALL: anomalous (1 active incident(s); 1 cue(s))"
        );
        for s in shapes().iter().filter(|s| s.id != DERIVED_SHAPE) {
            assert_eq!(s.active_incidents, 0, "{}", s.id);
            assert_eq!(
                overall(&prompt_of("shipped", s.id)),
                "OVERALL: anomalous (0 active incident(s); 1 cue(s))",
                "{}",
                s.id
            );
        }
        let derived = shape(DERIVED_SHAPE);
        assert_eq!(
            own_lines(&derived, &baseline_corpus_incidents(&derived)),
            OwnLines::Positions(vec![2]),
            "the triggering service's own line stands second by age"
        );
    }

    #[test]
    fn the_derived_shape_composes_at_its_recorded_size_and_differs_from_s16_in_three_sections() {
        assert_eq!(prompt_of(BASELINE_ARM, DERIVED_SHAPE).len(), 7824);
        let rows = match section_rows(&[
            SectionSource::Shape(DERIVED_SHAPE.to_string()),
            SectionSource::Shape("S16".to_string()),
        ]) {
            Ok(rows) => rows,
            Err(why) => panic!("S17 and S16 are cut: {why}"),
        };
        assert_eq!(
            rows.last().map(String::as_str),
            Some(
                "sections: S17 vs S16: differs in digest.overall,digest.services,digest.corpus-matches"
            )
        );
        assert!(remedy_shape_ids().contains(&DERIVED_SHAPE.to_string()));
    }
}
