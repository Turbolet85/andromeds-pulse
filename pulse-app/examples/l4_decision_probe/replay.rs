//! Replay input, section reader and corpus-block edits for the L4 decision
//! probe.
//!
//! A replay is one captured model invocation, read from a directory given at
//! run time and never from the tree. The directory holds `prompt.txt` (the
//! bare `-p` operand) and `argv.nul` (the argv, NUL-terminated, `argv[0]`
//! first). The prompt runs unchanged as the `-p` operand of the probe's own
//! argv, and its graders read the cue kind and `scope_id` of its first cue
//! line.
//!
//! Nothing of a captured file is printed or stored. A refusal is a closed
//! phrase, the section reader prints labels with byte and line counts, and a
//! replayed generation keeps the labels an S shape's does.
//!
//! The four candidate arms each make one edit of a composed prompt's corpus
//! block, the lines from `CORPUS MATCHES:` to the end of the digest, whether
//! the prompt is a shape's or a replay's. The product path composes the same
//! block through the product's selection, line format and render, to be read
//! against the `CX` edit.

use std::path::{Path, PathBuf};

use interpretation::prompt::{
    CITABLE_CLOSE_MARKER, CITABLE_OPEN_MARKER, DIGEST_CLOSE_MARKER, DIGEST_OPEN_MARKER,
};
use interpretation::schema::L4_OUTPUT_JSON_SCHEMA;
use pulse_app::llamacli_inference::validate_prompt_bounded;
use triage::contract::{
    AttentionCue, CORPUS_MATCHES_FRAMING_NOTE, CueKind, CueScope, DIGEST_CORPUS_RETRIEVAL_LIMIT,
    Incident, IncidentStatus, PriorityTier, TRIGGER_LINE_PREFIX, cue_summary,
    format_corpus_match_line, select_corpus_matches,
};

use super::{
    NANOS_PER_MINUTE, Prepared, RENDER_NOW_UNIX_NANO, compose_argv, corpus_match_incident,
    rendered_corpus_block,
};

/// The remedy candidates, in selection order: least change first.
pub const CANDIDATE_ARMS: [&str; 4] = ["CR", "CO", "CC", "CX"];
/// The line `CR` appends after the last corpus line. Static, by reference and
/// kind-generic: it names no service, no `scope_id` value and no cue kind.
pub const RESTATE_LINE: &str = "(end of other incidents - the signal this digest reports is the \
cue line under ATTENTION CUES, for the scope_id written there)";
/// The corpus lines `CC` keeps: the count S8 passes with.
const CAP_LINES: usize = 2;
pub const REPLAY_SHAPE_PREFIX: &str = "replay:";
pub const OWN_LINES_UNKNOWN: &str = "own lines unknown";

const MAX_CAPTURE_FILE_BYTES: u64 = 64 * 1024;
const PROMPT_FILE: &str = "prompt.txt";
const ARGV_FILE: &str = "argv.nul";
const DIGEST_HEADER: &str = "# Current Digest";
const SERVICES_HEADER: &str = "SERVICES (rate, error%, p99 vs baselines):";
const CUES_HEADER: &str = "ATTENTION CUES:";
const CORPUS_HEADER: &str = "CORPUS MATCHES:";
const MATCH_LINE_PREFIX: &str = "  - ";
const NO_DIGEST: &str = "the prompt has no <DIGEST> section";
const BLOCK_FORM: &str = "the corpus block is not in the rendered form";
const LINE_FORM: &str = "a corpus line is not in the product's line form";
const NO_OTHER_SCOPE: &str = "the digest names no service beside the triggering scope";
const CUE_KINDS: [CueKind; 6] = [
    CueKind::ErrorRateSpike,
    CueKind::LatencyRegression,
    CueKind::RestartEvent,
    CueKind::ServiceWentSilent,
    CueKind::RetryStorm,
    CueKind::ReflectionTrend,
];

/// The section reader's closed labels, in prompt order: the builder's
/// sections, the digest cut into the render's, then text under no known
/// header.
pub const SECTION_LABELS: [&str; 16] = [
    "role",
    "conventions",
    "output-schema",
    "project-context",
    "digest.window",
    "digest.project",
    "digest.recent-changes",
    "digest.overall",
    "digest.trigger",
    "digest.services",
    "digest.attention-cues",
    "digest.corpus-matches",
    "citable-evidence-ids",
    "corpus-retrieval",
    "output-instructions",
    "other",
];
const OTHER: usize = 15;
const BUILDER_HEADERS: [(&str, usize); 7] = [
    ("# Role", 0),
    ("# Conventions", 1),
    ("# Output Schema (JSON Schema draft 2020-12)", 2),
    ("# Project Context", 3),
    ("# Citable Evidence Ids", 12),
    ("# Corpus Retrieval (past similar incidents)", 13),
    ("# Output Instructions", 14),
];

pub fn is_label(label: &str) -> bool {
    (1..=32).contains(&label.len())
        && label
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

/// `--replay LABEL=DIR[,LABEL=DIR…]`: each label a short ASCII word, none
/// repeated.
pub fn parse_sources(value: &str) -> Result<Vec<(String, PathBuf)>, String> {
    let mut sources: Vec<(String, PathBuf)> = Vec::new();
    for item in value.split(',') {
        let Some((label, dir)) = item.trim().split_once('=') else {
            return Err("--replay takes LABEL=DIR items".to_string());
        };
        if !is_label(label) || dir.is_empty() {
            return Err(
                "--replay takes LABEL=DIR items, each label a short ASCII word".to_string(),
            );
        }
        if sources.iter().any(|(known, _)| known == label) {
            return Err("a replay label is repeated".to_string());
        }
        sources.push((label.to_string(), PathBuf::from(dir)));
    }
    Ok(sources)
}

/// One `--sections` source: a capture directory under a label, or an S shape
/// id (its `shipped` composition).
#[derive(Debug, PartialEq)]
pub enum SectionSource {
    Capture(String, PathBuf),
    Shape(String),
}

impl SectionSource {
    pub fn label(&self) -> &str {
        match self {
            Self::Capture(label, _) | Self::Shape(label) => label,
        }
    }
}

/// `--sections SRC[,SRC…]`: at least two sources, no label repeated.
pub fn parse_section_sources(value: &str) -> Result<Vec<SectionSource>, String> {
    let mut sources: Vec<SectionSource> = Vec::new();
    for item in value.split(',').map(str::trim) {
        let source = match item.split_once('=') {
            Some((label, dir)) if is_label(label) && !dir.is_empty() => {
                SectionSource::Capture(label.to_string(), PathBuf::from(dir))
            }
            Some(_) => return Err("--sections takes LABEL=DIR or shape id items".to_string()),
            None => SectionSource::Shape(item.to_string()),
        };
        if sources.iter().any(|known| known.label() == source.label()) {
            return Err("a section source label is repeated".to_string());
        }
        sources.push(source);
    }
    if sources.len() < 2 {
        return Err("--sections takes at least two sources".to_string());
    }
    Ok(sources)
}

/// The work tree holding `cwd`: its nearest ancestor with a `.git` entry, or
/// `cwd` itself.
pub fn work_tree(cwd: &Path) -> PathBuf {
    cwd.ancestors()
        .find(|dir| dir.join(".git").exists())
        .unwrap_or(cwd)
        .to_path_buf()
}

pub fn is_replay_shape(shape: &str) -> bool {
    shape.starts_with(REPLAY_SHAPE_PREFIX)
}

/// Byte range of the digest payload inside a composed prompt: what the
/// builder was handed, between its opening frame and the newline it writes
/// ahead of the closing marker.
fn digest_span(prompt: &str) -> Option<(usize, usize)> {
    let open = format!("\n{DIGEST_HEADER}\n{DIGEST_OPEN_MARKER}\n");
    let close = format!("\n{DIGEST_CLOSE_MARKER}\n");
    let start = prompt.find(&open)? + open.len();
    let end = start + prompt[start..].find(&close)?;
    Some((start, end))
}

fn kind_label(kind: CueKind) -> String {
    cue_summary(&AttentionCue {
        kind,
        scope: CueScope::Service,
        scope_id: None,
        magnitude: 0.0,
        absolute_value: 0.0,
        persistence: 0,
        confidence: 0.0,
        priority_tier: PriorityTier::Autonomous,
        suppression_bypassed: false,
        fingerprint: None,
    })
}

/// The cue kind and `scope_id` of a prompt's first cue line,
/// `  [tier] kind — kind scope_id=value`: the cue its graders read against.
fn first_cue(prompt: &str) -> Result<(CueKind, Option<String>), String> {
    let (start, end) = digest_span(prompt).ok_or(NO_DIGEST)?;
    let line = prompt[start..end]
        .lines()
        .skip_while(|l| *l != CUES_HEADER)
        .nth(1)
        .ok_or("the digest carries no cue line")?;
    let unknown = || "the first cue line names no known cue kind".to_string();
    let (_, rest) = line
        .strip_prefix("  [")
        .and_then(|l| l.split_once("] "))
        .ok_or_else(unknown)?;
    let (label, summary) = rest.split_once(" — ").ok_or_else(unknown)?;
    let kind = CUE_KINDS
        .into_iter()
        .find(|kind| kind_label(*kind) == label)
        .ok_or_else(unknown)?;
    let scope_id = summary
        .strip_prefix(label)
        .and_then(|rest| rest.strip_prefix(" scope_id="))
        .filter(|id| !id.is_empty())
        .map(str::to_string);
    Ok((kind, scope_id))
}

fn read_capture_file(dir: &Path, name: &str) -> Result<Vec<u8>, String> {
    let path = dir.join(name);
    let meta = std::fs::symlink_metadata(&path)
        .ok()
        .filter(|meta| meta.file_type().is_file())
        .ok_or_else(|| format!("{name} is missing or not a regular file"))?;
    if meta.len() > MAX_CAPTURE_FILE_BYTES {
        return Err(format!("{name} is larger than 64 KiB"));
    }
    std::fs::read(&path).map_err(|_| format!("{name} is not readable"))
}

/// A capture directory's prompt and its recorded operands (`argv[0]`
/// dropped), refused unless the directory lies outside this work tree or
/// under its `target/`, and the operand after the one `-p` is `prompt.txt`
/// byte for byte.
fn read_capture(dir: &Path, tree: &Path) -> Result<(String, Vec<Vec<u8>>), String> {
    let resolved = dir
        .canonicalize()
        .ok()
        .filter(|resolved| resolved.is_dir())
        .ok_or("the capture directory is missing or not a directory")?;
    let prompt_bytes = read_capture_file(&resolved, PROMPT_FILE)?;
    let argv_bytes = read_capture_file(&resolved, ARGV_FILE)?;
    let prompt = String::from_utf8(prompt_bytes).map_err(|_| "prompt.txt is not UTF-8")?;
    let tree = tree
        .canonicalize()
        .map_err(|_| "the work tree is not resolvable")?;
    if resolved.starts_with(&tree) && !resolved.starts_with(tree.join("target")) {
        return Err(
            "the capture directory resolves inside this work tree and outside target/".to_string(),
        );
    }
    let Some(fields) = argv_bytes.strip_suffix(&[0]) else {
        return Err("argv.nul is not NUL-terminated".to_string());
    };
    let operands: Vec<Vec<u8>> = fields
        .split(|b| *b == 0)
        .skip(1)
        .map(<[u8]>::to_vec)
        .collect();
    let mut flags = operands
        .iter()
        .enumerate()
        .filter(|(_, operand)| operand.as_slice() == b"-p");
    let Some((at, _)) = flags.next() else {
        return Err("argv.nul has no -p".to_string());
    };
    if flags.next().is_some() {
        return Err("argv.nul has more than one -p".to_string());
    }
    if operands.get(at + 1).map(Vec::as_slice) != Some(prompt.as_bytes()) {
        return Err("the operand after -p is not prompt.txt".to_string());
    }
    validate_prompt_bounded(&prompt)
        .map_err(|rejection| format!("the prompt is rejected: {}", rejection.label()))?;
    Ok((prompt, operands))
}

/// The first place a recorded argv leaves the probe's own, the values after
/// `-m` and `--grammar-file` apart, named by the probe's flag there. The
/// phrase carries no recorded text.
fn argv_difference(recorded: &[Vec<u8>], own: &[String]) -> Option<String> {
    let mut flag = "";
    for (at, mine) in own.iter().enumerate() {
        let before = at.checked_sub(1).map(|i| own[i].as_str());
        let apart = matches!(before, Some("-m" | "--grammar-file"));
        if mine.starts_with('-') && !apart && before != Some("-p") {
            flag = mine;
        }
        let same = recorded
            .get(at)
            .is_some_and(|theirs| apart || theirs.as_slice() == mine.as_bytes());
        if !same {
            return Some(format!(
                "the recorded argv differs from the probe's at {flag}"
            ));
        }
    }
    (recorded.len() > own.len())
        .then(|| "the recorded argv carries operands the probe's does not".to_string())
}

/// One captured invocation, loaded and checked.
pub struct Replay {
    /// `replay:LABEL`, the shape id its rows and lines carry.
    pub shape: &'static str,
    pub prompt: String,
    trigger: CueKind,
    scope_id: Option<String>,
}

/// A replay under one arm: a composition to run, or the arm skipped with its
/// reason.
pub enum Composed {
    Run(Prepared),
    Skipped(&'static str),
}

impl Replay {
    fn prepared(&self, arm: &str, prompt: String) -> Prepared {
        Prepared {
            arm: arm.to_string(),
            shape: self.shape,
            trigger: self.trigger,
            scope_id: self.scope_id.clone(),
            prompt,
            schema: L4_OUTPUT_JSON_SCHEMA.to_string(),
            extra_args: Vec::new(),
            drop_flags: Vec::new(),
        }
    }

    /// `shipped` is the prompt verbatim; a candidate arm is `shipped` with
    /// its one edit of the corpus block. No other arm applies.
    pub fn compose(&self, arm: &str, own: &OwnLines) -> Result<Composed, String> {
        if arm == "shipped" {
            return Ok(Composed::Run(self.prepared(arm, self.prompt.clone())));
        }
        let candidate =
            candidate(arm).ok_or_else(|| format!("arm {arm} does not apply to a replay"))?;
        let Some(prompt) = edit_block(&self.prompt, candidate, own)? else {
            return Ok(Composed::Skipped(OWN_LINES_UNKNOWN));
        };
        validate_prompt_bounded(&prompt)
            .map_err(|rejection| format!("{arm}: prompt rejected: {}", rejection.label()))?;
        Ok(Composed::Run(self.prepared(arm, prompt)))
    }
}

/// Loads the capture in `dir` as `replay:{label}`. `scope` is the `scope_id`
/// its first cue line must carry (`None`: not checked); `ngl` is the probe's
/// own `-ngl` value, for the argv comparison.
pub fn load(
    label: &str,
    dir: &Path,
    tree: &Path,
    scope: Option<&str>,
    ngl: u32,
) -> Result<Replay, String> {
    let (prompt, operands) = read_capture(dir, tree)?;
    let (trigger, scope_id) = first_cue(&prompt)?;
    if scope.is_some_and(|scope| scope_id.as_deref() != Some(scope)) {
        return Err("the first cue line's scope_id is not --replay-scope".to_string());
    }
    // One short string per `--replay` label, for the life of the process: a
    // shape id is `&'static str` everywhere a row or a count is keyed.
    let shape: &'static str = Box::leak(format!("{REPLAY_SHAPE_PREFIX}{label}").into_boxed_str());
    let replay = Replay {
        shape,
        prompt,
        trigger,
        scope_id,
    };
    let shipped = replay.prepared("shipped", replay.prompt.clone());
    let own = compose_argv(&shipped, Path::new("model"), ngl, Path::new("grammar"));
    match argv_difference(&operands, &own) {
        Some(why) => Err(why),
        None => Ok(replay),
    }
}

/// A prompt cut into `SECTION_LABELS`, one text per label. The digest's frame
/// (its header, its two markers and the newline ahead of the closing one) is
/// structure and belongs to no label; so does a blank line under no header.
fn cut(prompt: &str) -> Result<Vec<String>, String> {
    let (start, end) = digest_span(prompt).ok_or(NO_DIGEST)?;
    let frame_open = DIGEST_HEADER.len() + DIGEST_OPEN_MARKER.len() + 2;
    let frame_close = DIGEST_CLOSE_MARKER.len() + 2;
    let mut texts = vec![String::new(); SECTION_LABELS.len()];
    cut_builder_sections(&prompt[..start - frame_open], &mut texts);
    cut_digest(&prompt[start..end], &mut texts);
    cut_builder_sections(&prompt[end + frame_close..], &mut texts);
    Ok(texts)
}

fn cut_builder_sections(text: &str, texts: &mut [String]) {
    let mut current: Option<usize> = None;
    for line in text.split_inclusive('\n') {
        let body = line.strip_suffix('\n').unwrap_or(line);
        if let Some((_, at)) = BUILDER_HEADERS.iter().find(|(header, _)| *header == body) {
            current = Some(*at);
        } else if body.starts_with("# ") {
            current = Some(OTHER);
        }
        match current {
            Some(at) => texts[at].push_str(line),
            None if body.is_empty() => {}
            None => texts[OTHER].push_str(line),
        }
    }
}

fn cut_digest(digest: &str, texts: &mut [String]) {
    let (mut current, mut takes_indented) = (OTHER, false);
    for line in digest.split_inclusive('\n') {
        let body = line.strip_suffix('\n').unwrap_or(line);
        let header = if body.starts_with("WINDOW: ") {
            Some((4, false))
        } else if body.starts_with("PROJECT: ") {
            Some((5, false))
        } else if body == "RECENT CHANGES:" {
            Some((6, true))
        } else if body.starts_with("OVERALL: ") {
            Some((7, false))
        } else if body.starts_with(TRIGGER_LINE_PREFIX) {
            Some((8, false))
        } else if body == SERVICES_HEADER {
            Some((9, true))
        } else if body == CUES_HEADER {
            Some((10, true))
        } else if body == CORPUS_HEADER {
            Some((11, true))
        } else {
            None
        };
        match header {
            Some(next) => (current, takes_indented) = next,
            None if takes_indented && body.starts_with("  ") => {}
            None => (current, takes_indented) = (OTHER, false),
        }
        texts[current].push_str(line);
    }
}

/// A prompt cut into its sections, under the label its rows print.
pub struct Sectioned {
    label: String,
    texts: Vec<String>,
}

pub fn sectioned(label: &str, prompt: &str) -> Result<Sectioned, String> {
    Ok(Sectioned {
        label: label.to_string(),
        texts: cut(prompt)?,
    })
}

/// One row per label, then the labels that differ. Sameness is exact byte
/// equality; a row holds byte and line counts and no section text.
pub fn compare(first: &Sectioned, other: &Sectioned) -> Vec<String> {
    let (a, b) = (&first.label, &other.label);
    let size = |text: &str| format!("{} B {} L", text.len(), text.lines().count());
    let mut differing = Vec::new();
    let mut rows: Vec<String> = SECTION_LABELS
        .iter()
        .zip(first.texts.iter().zip(&other.texts))
        .map(|(label, (mine, theirs))| {
            let word = if mine == theirs {
                "same"
            } else {
                differing.push(*label);
                "differs"
            };
            format!(
                "sections: {a} vs {b}: {label} {word} · {a} {} · {b} {}",
                size(mine),
                size(theirs)
            )
        })
        .collect();
    rows.push(format!(
        "sections: {a} vs {b}: differs in {}",
        if differing.is_empty() {
            "none".to_string()
        } else {
            differing.join(",")
        }
    ));
    rows
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Candidate {
    /// `CR`: the restate line appended after the last corpus line.
    Restate,
    /// `CO`: the own lines first, then the others; each group's order kept.
    Reorder,
    /// `CC`: the first `CAP_LINES` lines kept.
    Cap,
    /// `CX`: the own lines kept and the others removed; with no line left the
    /// header and its note go too.
    Exclude,
}

pub fn candidate(arm: &str) -> Option<Candidate> {
    match arm {
        "CR" => Some(Candidate::Restate),
        "CO" => Some(Candidate::Reorder),
        "CC" => Some(Candidate::Cap),
        "CX" => Some(Candidate::Exclude),
        _ => None,
    }
}

/// Which corpus lines are the triggering scope's own, by 1-based position in
/// the block. A rendered line carries no scope, so a shape derives the set
/// from its incidents and a replay is given it.
#[derive(Clone, Debug, PartialEq)]
pub enum OwnLines {
    Unknown,
    Positions(Vec<usize>),
}

/// `--own-lines`: `unknown`, `none`, or 1-based positions `1,3`.
pub fn parse_own_lines(value: &str) -> Result<OwnLines, String> {
    match value.trim() {
        "unknown" => return Ok(OwnLines::Unknown),
        "none" => return Ok(OwnLines::Positions(Vec::new())),
        _ => {}
    }
    let mut positions = Vec::new();
    for item in value.split(',') {
        let position = item
            .trim()
            .parse::<usize>()
            .ok()
            .filter(|position| *position >= 1 && !positions.contains(position))
            .ok_or("--own-lines takes unknown, none or distinct 1-based positions")?;
        positions.push(position);
    }
    Ok(OwnLines::Positions(positions))
}

/// A prompt's corpus block: its byte range, from the header to the end of the
/// digest, and its match lines.
struct Block<'a> {
    start: usize,
    end: usize,
    lines: Vec<&'a str>,
    restated: bool,
    /// Whether the digest ends on a newline, as the product renders it.
    terminated: bool,
}

fn corpus_block(prompt: &str) -> Result<Option<Block<'_>>, String> {
    let (digest_start, end) = digest_span(prompt).ok_or(NO_DIGEST)?;
    let digest = &prompt[digest_start..end];
    let mut offset = 0;
    let mut found = None;
    for line in digest.split_inclusive('\n') {
        if line.strip_suffix('\n').unwrap_or(line) == CORPUS_HEADER {
            found = Some(offset);
            break;
        }
        offset += line.len();
    }
    let Some(offset) = found else {
        return Ok(None);
    };
    let text = &digest[offset..];
    let mut bodies = text
        .split_inclusive('\n')
        .map(|line| line.strip_suffix('\n').unwrap_or(line))
        .skip(1);
    if bodies.next().and_then(|note| note.strip_prefix("  ")) != Some(CORPUS_MATCHES_FRAMING_NOTE) {
        return Err(BLOCK_FORM.to_string());
    }
    let mut lines = Vec::new();
    let mut restated = false;
    for body in bodies {
        if !restated && body.starts_with(MATCH_LINE_PREFIX) {
            lines.push(body);
        } else if !restated && body.strip_prefix("  ") == Some(RESTATE_LINE) {
            restated = true;
        } else {
            return Err(BLOCK_FORM.to_string());
        }
    }
    if lines.is_empty() {
        return Err(BLOCK_FORM.to_string());
    }
    Ok(Some(Block {
        start: digest_start + offset,
        end,
        lines,
        restated,
        terminated: text.ends_with('\n'),
    }))
}

/// The corpus lines of a composed prompt: 0 without a block.
pub fn corpus_line_count(prompt: &str) -> usize {
    corpus_block(prompt)
        .ok()
        .flatten()
        .map_or(0, |block| block.lines.len())
}

/// `prompt` with one candidate's edit made to its corpus block and every
/// other byte kept. A prompt without a block is left as it is. `None` when
/// the edit needs the own lines and they are unknown.
pub fn edit_block(
    prompt: &str,
    candidate: Candidate,
    own: &OwnLines,
) -> Result<Option<String>, String> {
    let needs_own = matches!(candidate, Candidate::Reorder | Candidate::Exclude);
    if needs_own && *own == OwnLines::Unknown {
        return Ok(None);
    }
    let Some(block) = corpus_block(prompt)? else {
        return Ok(Some(prompt.to_string()));
    };
    let positions: &[usize] = match own {
        OwnLines::Positions(positions) => positions,
        OwnLines::Unknown => &[],
    };
    if positions.iter().any(|at| *at > block.lines.len()) {
        return Err("--own-lines names a line the block does not hold".to_string());
    }
    let is_own = |at: usize| positions.contains(&(at + 1));
    let numbered = || block.lines.iter().copied().enumerate();
    let mut restated = block.restated;
    let kept: Vec<&str> = match candidate {
        Candidate::Restate => {
            restated = true;
            block.lines.clone()
        }
        Candidate::Cap => block.lines.iter().copied().take(CAP_LINES).collect(),
        Candidate::Reorder => numbered()
            .filter(|(at, _)| is_own(*at))
            .chain(numbered().filter(|(at, _)| !is_own(*at)))
            .map(|(_, line)| line)
            .collect(),
        Candidate::Exclude => numbered()
            .filter(|(at, _)| is_own(*at))
            .map(|(_, line)| line)
            .collect(),
    };
    let mut text = String::new();
    if !kept.is_empty() {
        text.push_str(&format!(
            "{CORPUS_HEADER}\n  {CORPUS_MATCHES_FRAMING_NOTE}\n"
        ));
        for line in kept {
            text.push_str(line);
            text.push('\n');
        }
        if restated {
            text.push_str(&format!("  {RESTATE_LINE}\n"));
        }
    }
    Ok(Some(spliced(prompt, &block, text)))
}

/// `prompt` with its corpus block replaced by `text` and every other byte
/// kept.
fn spliced(prompt: &str, block: &Block<'_>, mut text: String) -> String {
    let mut head = &prompt[..block.start];
    if !block.terminated {
        // A digest that did not end on a newline does not gain one.
        match text.pop() {
            Some(_) => {}
            None => head = head.strip_suffix('\n').unwrap_or(head),
        }
    }
    format!("{head}{text}{}", &prompt[block.end..])
}

/// What the product's own selection and render make of a composed prompt's
/// corpus block, read against the bytes the probe holds.
#[derive(Debug, PartialEq)]
pub struct ProductPath {
    /// The block selected without a triggering scope and rendered again is
    /// the prompt as given: the lines read back are the block.
    pub unremedied_same: bool,
    /// The block selected under the cue's scope is the `CX` edit.
    pub remedied_same: bool,
    pub kept: usize,
    pub lines: usize,
}

/// One rendered corpus line read back into the fields the product's selection
/// and line format read. `at` keeps the block's order among lines of one age.
fn incident_of(line: &str, at: usize, scope: &str) -> Option<Incident> {
    let rest = line.strip_prefix(MATCH_LINE_PREFIX)?.strip_prefix('[')?;
    let (fingerprint, rest) = rest.split_once("] ")?;
    let (title, tail) = rest.rsplit_once(" — ")?;
    let (age, outcome) = tail.split_once("m ago, ")?;
    let status = match outcome {
        "resolved" => IncidentStatus::Resolved,
        "acknowledged" => IncidentStatus::Acknowledged,
        "active" => IncidentStatus::Active,
        _ => return None,
    };
    let age = age.parse::<i64>().ok().filter(|age| *age >= 0)?;
    let incident = Incident {
        fingerprint: fingerprint.to_string(),
        title: title.to_string(),
        status,
        scope_id: Some(scope.to_string()),
        opened_at_unix_nano: RENDER_NOW_UNIX_NANO - age * NANOS_PER_MINUTE - at as i64,
        ..corpus_match_incident()
    };
    let again = format_corpus_match_line(&incident, RENDER_NOW_UNIX_NANO);
    (line.strip_prefix(MATCH_LINE_PREFIX) == Some(again.as_str())).then_some(incident)
}

/// The services a digest's table names: the scopes its window held.
fn service_rows(digest: &str) -> Vec<String> {
    digest
        .lines()
        .skip_while(|line| *line != SERVICES_HEADER)
        .skip(1)
        .take_while(|line| line.starts_with("  "))
        .filter_map(|line| line[2..].split_once("     "))
        .map(|(service, _)| service.to_string())
        .collect()
}

/// The fingerprints a prompt lists as citable: its cues' own, the part of the
/// window's set a prompt carries.
fn cited_fingerprints(prompt: &str) -> Vec<String> {
    prompt
        .lines()
        .skip_while(|line| *line != CITABLE_OPEN_MARKER)
        .skip(1)
        .take_while(|line| *line != CITABLE_CLOSE_MARKER)
        .filter(|line| !line.starts_with('('))
        .map(str::to_string)
        .collect()
}

/// The product path over a composed prompt whose block is the unremedied
/// one, a capture's or a shape's under `nb`. Each line is read back into an
/// incident, scoped to `trigger` at the own positions and to another of the
/// digest's services elsewhere; the product's `select_corpus_matches` runs
/// over them with the prompt's citable fingerprints as the window's, its line
/// format and its render write the block, and the result is compared byte for
/// byte. `None` when the own lines are unknown.
pub fn product_path(
    prompt: &str,
    own: &OwnLines,
    trigger: &str,
) -> Result<Option<ProductPath>, String> {
    let OwnLines::Positions(positions) = own else {
        return Ok(None);
    };
    let Some(edited) = edit_block(prompt, Candidate::Exclude, own)? else {
        return Ok(None);
    };
    let Some(block) = corpus_block(prompt)? else {
        return Ok(Some(ProductPath {
            unremedied_same: true,
            remedied_same: true,
            kept: 0,
            lines: 0,
        }));
    };
    let (start, end) = digest_span(prompt).ok_or(NO_DIGEST)?;
    let active = service_rows(&prompt[start..end]);
    let other = active
        .iter()
        .find(|service| *service != trigger)
        .ok_or(NO_OTHER_SCOPE)?;
    let window = cited_fingerprints(prompt);
    let mut candidates = Vec::new();
    for (at, line) in block.lines.iter().enumerate() {
        let scope = if positions.contains(&(at + 1)) {
            trigger
        } else {
            other
        };
        candidates.push(incident_of(line, at, scope).ok_or(LINE_FORM)?);
    }
    let composed = |triggering_scope: Option<&str>| -> Result<(String, usize), String> {
        let kept = select_corpus_matches(
            candidates.clone(),
            &window,
            &active,
            triggering_scope,
            DIGEST_CORPUS_RETRIEVAL_LIMIT,
        );
        let lines: Vec<String> = kept
            .iter()
            .map(|incident| format_corpus_match_line(incident, RENDER_NOW_UNIX_NANO))
            .collect();
        let text = rendered_corpus_block(&lines)?;
        Ok((spliced(prompt, &block, text), kept.len()))
    };
    let (unremedied, _) = composed(None)?;
    let (remedied, kept) = composed(Some(trigger))?;
    Ok(Some(ProductPath {
        unremedied_same: unremedied == prompt,
        remedied_same: remedied == edited,
        kept,
        lines: block.lines.len(),
    }))
}

/// The dry-run line of a product path: closed words and two counts.
pub fn product_path_line(shape: &str, path: &ProductPath) -> String {
    let word = |same: bool| if same { "same" } else { "differs" };
    format!(
        "dry-run: product path {shape}: unremedied {} · under the cue scope {} as CX · kept {} of {} lines",
        word(path.unremedied_same),
        word(path.remedied_same),
        path.kept,
        path.lines
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use interpretation::schema::{Confidence, Decision, Hypothesis, L4Output, Severity};
    use triage::contract::{Incident, format_corpus_match_line};

    use super::super::{
        Metrics, RENDER_NOW_UNIX_NANO, SIBLING_SERVICE, STORM_FINGERPRINT, Shape, Spawned,
        baseline_corpus_incidents, is_trigger_line, own_lines, prepare, prepare_rendered,
        read_labels, row_json, run_line, selected_corpus_incidents, shapes,
    };
    use super::*;

    const NGL: u32 = 99;

    fn temp() -> tempfile::TempDir {
        tempfile::tempdir().unwrap_or_else(|_| panic!("a temp dir"))
    }

    fn shape(id: &str) -> Shape {
        shapes()
            .into_iter()
            .find(|s| s.id == id)
            .unwrap_or_else(|| panic!("shape {id} exists"))
    }

    fn prompt_of(arm: &str, id: &str) -> String {
        match prepare(arm, &shape(id)) {
            Ok(p) => p.prompt,
            Err(why) => panic!("{arm} {id} composes: {why}"),
        }
    }

    /// The operands the probe's own argv holds for `prompt`.
    fn own_operands(prompt: &str) -> Vec<String> {
        let replay = Replay {
            shape: "replay:own",
            prompt: prompt.to_string(),
            trigger: CueKind::RetryStorm,
            scope_id: None,
        };
        let shipped = replay.prepared("shipped", prompt.to_string());
        compose_argv(
            &shipped,
            Path::new("/models/m.gguf"),
            NGL,
            Path::new("/tmp/g.gbnf"),
        )
    }

    /// Writes a capture directory as the operator's wrapper does: the bare
    /// prompt, and the argv NUL-terminated with `argv[0]` first.
    fn write_capture(dir: &Path, prompt: &[u8], operands: &[Vec<u8>]) {
        let mut argv = b"llama-cli\0".to_vec();
        for operand in operands {
            argv.extend_from_slice(operand);
            argv.push(0);
        }
        for (name, bytes) in [(PROMPT_FILE, prompt), (ARGV_FILE, argv.as_slice())] {
            if std::fs::write(dir.join(name), bytes).is_err() {
                panic!("{name} is written");
            }
        }
    }

    fn bytes_of(operands: &[String]) -> Vec<Vec<u8>> {
        operands.iter().map(|o| o.as_bytes().to_vec()).collect()
    }

    /// A capture of `prompt` under the probe's own argv, in a fresh temp dir.
    fn capture_of(prompt: &str) -> tempfile::TempDir {
        let dir = temp();
        write_capture(
            dir.path(),
            prompt.as_bytes(),
            &bytes_of(&own_operands(prompt)),
        );
        dir
    }

    fn loaded(dir: &Path, scope: Option<&str>) -> Result<Replay, String> {
        load("x", dir, temp().path(), scope, NGL)
    }

    fn refusal(dir: &Path, scope: Option<&str>) -> String {
        match loaded(dir, scope) {
            Ok(_) => panic!("the capture is refused"),
            Err(why) => why,
        }
    }

    fn run_of(composed: Result<Composed, String>) -> Prepared {
        match composed {
            Ok(Composed::Run(p)) => p,
            Ok(Composed::Skipped(why)) => panic!("the arm runs: skipped, {why}"),
            Err(why) => panic!("the arm runs: {why}"),
        }
    }

    #[test]
    fn a_shapes_own_composition_replays_to_that_prompt_with_the_same_argv() {
        // S16 under `nb`: the five-line block a capture taken before the
        // remedy holds.
        for (arm, id) in [("shipped", "S7"), ("nb", "S16")] {
            let prompt = prompt_of(arm, id);
            let dir = capture_of(&prompt);
            let replay = match loaded(dir.path(), Some(SIBLING_SERVICE)) {
                Ok(replay) => replay,
                Err(why) => panic!("{id} replays: {why}"),
            };
            assert_eq!(replay.shape, "replay:x");
            let p = run_of(replay.compose("shipped", &OwnLines::Unknown));
            assert_eq!(p.prompt, prompt, "{id}: byte for byte");
            assert_eq!(p.trigger, CueKind::RetryStorm, "{id}");
            assert_eq!(p.scope_id.as_deref(), Some(SIBLING_SERVICE), "{id}");
            assert_eq!(
                compose_argv(
                    &p,
                    Path::new("/models/m.gguf"),
                    NGL,
                    Path::new("/tmp/g.gbnf")
                ),
                own_operands(&prompt),
                "{id}: the argv a generation runs is the probe's own"
            );
        }
        assert_eq!(corpus_line_count(&prompt_of("nb", "S16")), 5);
        assert_eq!(corpus_line_count(&prompt_of("shipped", "S16")), 2);
        assert_eq!(corpus_line_count(&prompt_of("shipped", "S7")), 0);
    }

    #[test]
    fn a_recorded_model_and_grammar_path_are_not_compared() {
        let prompt = prompt_of("shipped", "S8");
        let mut operands = own_operands(&prompt);
        for flag in ["-m", "--grammar-file"] {
            let at = operands
                .iter()
                .position(|o| o == flag)
                .unwrap_or_else(|| panic!("the argv carries {flag}"));
            operands[at + 1] = format!("/elsewhere/{}", flag.trim_start_matches('-'));
        }
        let dir = temp();
        write_capture(dir.path(), prompt.as_bytes(), &bytes_of(&operands));
        assert!(loaded(dir.path(), Some(SIBLING_SERVICE)).is_ok());
    }

    #[test]
    fn a_recorded_argv_that_differs_is_refused_at_the_first_differing_flag() {
        let prompt = prompt_of("shipped", "S8");
        let own = own_operands(&prompt);
        let at = |flag: &str| {
            own.iter()
                .position(|o| o == flag)
                .unwrap_or_else(|| panic!("the argv carries {flag}"))
        };
        let refused = |operands: &[String]| {
            let dir = temp();
            write_capture(dir.path(), prompt.as_bytes(), &bytes_of(operands));
            refusal(dir.path(), Some(SIBLING_SERVICE))
        };

        let mut value = own.clone();
        value[at("-c") + 1] = "4096".to_string();
        assert_eq!(
            refused(&value),
            "the recorded argv differs from the probe's at -c"
        );

        let mut dropped = own.clone();
        dropped.drain(at("-rea")..at("-rea") + 2);
        assert_eq!(
            refused(&dropped),
            "the recorded argv differs from the probe's at -rea"
        );

        let mut sampled = own.clone();
        sampled[at("--temp") + 1] = "0".to_string();
        assert_eq!(
            refused(&sampled),
            "the recorded argv differs from the probe's at --temp"
        );

        let mut extra = own.clone();
        extra.push("--version".to_string());
        assert_eq!(
            refused(&extra),
            "the recorded argv carries operands the probe's does not"
        );

        let mut other_ngl = own.clone();
        other_ngl[at("-ngl") + 1] = "0".to_string();
        assert_eq!(
            refused(&other_ngl),
            "the recorded argv differs from the probe's at -ngl"
        );
    }

    #[test]
    fn a_missing_directory_or_file_is_refused() {
        let prompt = prompt_of("shipped", "S8");
        let gone = temp().path().join("absent");
        assert_eq!(
            refusal(&gone, None),
            "the capture directory is missing or not a directory"
        );
        for (name, phrase) in [
            (PROMPT_FILE, "prompt.txt is missing or not a regular file"),
            (ARGV_FILE, "argv.nul is missing or not a regular file"),
        ] {
            let dir = capture_of(&prompt);
            assert!(std::fs::remove_file(dir.path().join(name)).is_ok());
            assert_eq!(refusal(dir.path(), None), phrase, "absent");
            assert!(std::fs::create_dir(dir.path().join(name)).is_ok());
            assert_eq!(refusal(dir.path(), None), phrase, "a directory");
        }
    }

    #[test]
    fn an_oversized_or_non_utf8_file_is_refused() {
        let prompt = prompt_of("shipped", "S8");
        let operands = bytes_of(&own_operands(&prompt));
        let big = vec![b'a'; 64 * 1024 + 1];
        let dir = temp();
        write_capture(dir.path(), &big, &operands);
        assert_eq!(
            refusal(dir.path(), None),
            "prompt.txt is larger than 64 KiB"
        );
        let dir = temp();
        write_capture(dir.path(), prompt.as_bytes(), &[big]);
        assert_eq!(refusal(dir.path(), None), "argv.nul is larger than 64 KiB");
        let dir = temp();
        write_capture(dir.path(), &[0xff, 0xfe, b'a'], &operands);
        assert_eq!(refusal(dir.path(), None), "prompt.txt is not UTF-8");
    }

    #[test]
    fn a_capture_inside_the_work_tree_is_refused_outside_its_target() {
        let prompt = prompt_of("shipped", "S8");
        let tree = temp();
        let place = |sub: &str| {
            let dir = tree.path().join(sub);
            assert!(std::fs::create_dir_all(&dir).is_ok());
            write_capture(&dir, prompt.as_bytes(), &bytes_of(&own_operands(&prompt)));
            load("x", &dir, tree.path(), Some(SIBLING_SERVICE), NGL).err()
        };
        assert_eq!(
            place("captures/one").as_deref(),
            Some("the capture directory resolves inside this work tree and outside target/")
        );
        assert_eq!(place("target/captures/one"), None);
        assert!(
            std::fs::create_dir_all(tree.path().join("crates/a/.git")).is_ok(),
            "a nested marker"
        );
        assert_eq!(
            work_tree(&tree.path().join("crates/a/src")),
            tree.path().join("crates/a")
        );
    }

    #[test]
    fn argv_nul_needs_exactly_one_p_with_the_prompt_after_it() {
        let prompt = prompt_of("shipped", "S8");
        let own = own_operands(&prompt);
        let refused = |operands: Vec<String>| {
            let dir = temp();
            write_capture(dir.path(), prompt.as_bytes(), &bytes_of(&operands));
            refusal(dir.path(), Some(SIBLING_SERVICE))
        };
        let without: Vec<String> = own.iter().filter(|o| *o != "-p").cloned().collect();
        assert_eq!(refused(without), "argv.nul has no -p");
        let mut twice = own.clone();
        twice.extend(["-p".to_string(), prompt.clone()]);
        assert_eq!(refused(twice), "argv.nul has more than one -p");
        let mut other = own.clone();
        if let Some(last) = other.last_mut() {
            last.push('x');
        }
        assert_eq!(refused(other), "the operand after -p is not prompt.txt");
        let mut bare = own.clone();
        bare.pop();
        assert_eq!(refused(bare), "the operand after -p is not prompt.txt");

        let dir = capture_of(&prompt);
        let unterminated = b"llama-cli\0-p\0x".to_vec();
        assert!(std::fs::write(dir.path().join(ARGV_FILE), unterminated).is_ok());
        assert_eq!(
            refusal(dir.path(), Some(SIBLING_SERVICE)),
            "argv.nul is not NUL-terminated"
        );
    }

    #[test]
    fn a_prompt_the_production_bound_rejects_is_refused() {
        let prompt = prompt_of("shipped", "S8");
        let long = format!("{prompt}{}", "a".repeat(16 * 1024));
        assert_eq!(
            refusal(capture_of(&long).path(), None),
            "the prompt is rejected: prompt_too_long"
        );
        let control = prompt.replacen("WINDOW: ", "WINDOW: \u{1}", 1);
        assert_eq!(
            refusal(capture_of(&control).path(), None),
            "the prompt is rejected: prompt_control_character"
        );
    }

    #[test]
    fn a_prompt_without_a_digest_a_known_first_cue_or_the_given_scope_is_refused() {
        let prompt = prompt_of("shipped", "S8");
        let cue = "  [autonomous] retry_storm — retry_storm scope_id=conductor\n";
        assert_eq!(prompt.matches(cue).count(), 1, "the cue line as rendered");
        let refused = |text: String, scope: Option<&str>| refusal(capture_of(&text).path(), scope);

        assert_eq!(
            refused(prompt.replacen("<DIGEST>", "<DIGESTED>", 1), None),
            NO_DIGEST
        );
        assert_eq!(
            refused(prompt.replacen(CUES_HEADER, "ATTENTION:", 1), None),
            "the digest carries no cue line"
        );
        let unknown_kind = cue.replace("retry_storm", "retry_squall");
        assert_eq!(
            refused(prompt.replacen(cue, &unknown_kind, 1), None),
            "the first cue line names no known cue kind"
        );
        assert_eq!(
            refused(prompt.clone(), Some("conductor-canary")),
            "the first cue line's scope_id is not --replay-scope"
        );
        let unscoped = cue.replace(" scope_id=conductor", "");
        assert_eq!(
            refused(prompt.replacen(cue, &unscoped, 1), Some(SIBLING_SERVICE)),
            "the first cue line's scope_id is not --replay-scope"
        );
        assert!(
            loaded(capture_of(&prompt).path(), None).is_ok(),
            "no scope asked"
        );
    }

    #[test]
    fn replay_labels_are_short_ascii_words_and_none_is_repeated() {
        assert_eq!(
            parse_sources("miss=/a/b,control=/c"),
            Ok(vec![
                ("miss".to_string(), PathBuf::from("/a/b")),
                ("control".to_string(), PathBuf::from("/c")),
            ])
        );
        assert_eq!(
            parse_sources("miss=/a,miss=/b").err().as_deref(),
            Some("a replay label is repeated")
        );
        assert_eq!(
            parse_sources("/a/b").err().as_deref(),
            Some("--replay takes LABEL=DIR items")
        );
        for bad in ["a b=/x", "=/x", "miss="] {
            assert_eq!(
                parse_sources(bad).err().as_deref(),
                Some("--replay takes LABEL=DIR items, each label a short ASCII word"),
                "{bad}"
            );
        }
    }

    #[test]
    fn a_replay_row_and_run_line_hold_labels_and_no_prompt_text() {
        let title = "zq-capture-title-77";
        let prompt = prompt_of("shipped", "S8").replacen("Conductor-Canary", title, 1);
        assert!(prompt.contains(title), "the marker is in the prompt");
        let dir = capture_of(&prompt);
        let Ok(replay) = load(
            "miss",
            dir.path(),
            temp().path(),
            Some(SIBLING_SERVICE),
            NGL,
        ) else {
            panic!("the capture loads");
        };
        let p = run_of(replay.compose("shipped", &OwnLines::Unknown));
        let metrics = Metrics {
            elapsed_ms: 1234,
            peak_rss_kib: None,
            peak_vram_mib: None,
        };
        let out = L4Output {
            schema_version: "2.0".to_string(),
            prompt_version: "v2.6".to_string(),
            decision: Decision::Surface,
            severity: Severity::Suggested,
            title: "t".to_string(),
            symptom: "s".to_string(),
            timeline: "now".to_string(),
            hypotheses: vec![Hypothesis {
                statement: "a retry storm on conductor".to_string(),
                confidence: Confidence::High,
                justification: "observed".to_string(),
            }],
            investigation_steps: Vec::new(),
            evidence_refs: Vec::new(),
            fingerprint: STORM_FINGERPRINT.to_string(),
            model_tier: "primary".to_string(),
            hardware_profile: "gpu_primary".to_string(),
            is_resolution_summary: false,
        };
        let Ok(stdout) = serde_json::to_string(&out) else {
            panic!("the synthetic output serializes");
        };
        let labels = read_labels(&Spawned::Output(stdout, metrics), &p);
        assert_eq!(
            labels.identifies, "both",
            "graded against the cue line's scope"
        );
        let row = row_json(&p, 1, &labels, "absent", metrics);
        let line = run_line(&p, 1, &labels, "absent", metrics.elapsed_ms);
        assert_eq!(row["shape"], "replay:miss");
        assert_eq!(row["arm"], "shipped");
        assert!(line.starts_with("l4-decision-probe: shipped replay:miss run 1: "));
        let keys: BTreeSet<String> = row
            .as_object()
            .map(|o| o.keys().cloned().collect())
            .unwrap_or_default();
        let Ok(s8) = prepare("shipped", &shape("S8")) else {
            panic!("shipped composes S8");
        };
        let s_shape = row_json(&s8, 1, &labels, "absent", metrics);
        let s_keys: BTreeSet<String> = s_shape
            .as_object()
            .map(|o| o.keys().cloned().collect())
            .unwrap_or_default();
        assert_eq!(keys, s_keys, "the keys an S shape's row holds");
        for text in [row.to_string(), line] {
            assert!(
                !text.contains("zq-"),
                "prompt text reached a record: {text}"
            );
        }
    }

    fn rows_of(a: &str, b: &str) -> Vec<String> {
        let cut_of = |id: &str| match sectioned(id, &prompt_of("shipped", id)) {
            Ok(cut) => cut,
            Err(why) => panic!("{id} is cut: {why}"),
        };
        compare(&cut_of(a), &cut_of(b))
    }

    #[test]
    fn s8_against_s16_differs_in_the_corpus_block_alone() {
        let rows = rows_of("S8", "S16");
        assert_eq!(rows.len(), SECTION_LABELS.len() + 1);
        assert_eq!(
            rows.last().map(String::as_str),
            Some("sections: S8 vs S16: differs in digest.corpus-matches")
        );
        let differing: Vec<&String> = rows.iter().filter(|r| r.contains(" differs · ")).collect();
        assert_eq!(differing.len(), 1);
        assert!(
            differing[0].starts_with("sections: S8 vs S16: digest.corpus-matches differs · S8 "),
            "{}",
            differing[0]
        );
        // A shape source is its `shipped` composition: S16's block holds the
        // header, the note and its two conductor lines, as S8's holds its two.
        assert!(differing[0].contains(" B 4 L · S16 "), "{}", differing[0]);
        assert!(differing[0].ends_with(" B 4 L"), "{}", differing[0]);
        assert!(
            rows.iter()
                .any(|r| r.starts_with("sections: S8 vs S16: digest.services same · S8 "))
        );
    }

    #[test]
    fn s7_against_s8_reads_the_block_absent_on_one_side() {
        let rows = rows_of("S7", "S8");
        assert_eq!(
            rows.last().map(String::as_str),
            Some("sections: S7 vs S8: differs in digest.corpus-matches")
        );
        assert!(
            rows.iter().any(|r| r.starts_with(
                "sections: S7 vs S8: digest.corpus-matches differs · S7 0 B 0 L · S8 "
            ) && r.ends_with(" B 4 L")),
            "{rows:?}"
        );
        assert_eq!(
            rows_of("S8", "S8").last().map(String::as_str),
            Some("sections: S8 vs S8: differs in none")
        );
    }

    #[test]
    fn a_section_under_an_unknown_header_lands_in_other() {
        let s8 = prompt_of("shipped", "S8");
        let extra = "# Field Notes\nzq-unknown-section-line\n\n";
        let with_top = s8.replacen(
            "# Output Instructions\n",
            &format!("{extra}# Output Instructions\n"),
            1,
        );
        assert_ne!(with_top, s8);
        let Ok(texts) = cut(&with_top) else {
            panic!("the prompt is cut");
        };
        assert_eq!(texts[OTHER], extra);

        let stray = "UPTIME: 4h\n  zq-unknown-digest-line\n";
        let with_digest_line = s8.replacen("OVERALL: ", &format!("{stray}OVERALL: "), 1);
        let Ok(texts) = cut(&with_digest_line) else {
            panic!("the prompt is cut");
        };
        assert_eq!(texts[OTHER], stray);
        let (Ok(plain), Ok(edited)) = (sectioned("a", &s8), sectioned("b", &with_digest_line))
        else {
            panic!("both are cut");
        };
        let rows = compare(&plain, &edited);
        assert_eq!(
            rows.last().map(String::as_str),
            Some("sections: a vs b: differs in other")
        );
        assert!(
            rows.iter().all(|r| !r.contains("zq-")),
            "a row holds no section text"
        );
    }

    #[test]
    fn every_shape_is_cut_whole_with_nothing_under_other() {
        // The digest's header and two markers, the newline after each, the
        // newline ahead of the closing marker and the blank line after it.
        let frame = DIGEST_HEADER.len() + DIGEST_OPEN_MARKER.len() + DIGEST_CLOSE_MARKER.len() + 5;
        for s in shapes() {
            let prompt = prompt_of("shipped", s.id);
            let Ok(texts) = cut(&prompt) else {
                panic!("{} is cut", s.id);
            };
            assert_eq!(texts[OTHER], "", "{}", s.id);
            let cut_bytes: usize = texts.iter().map(String::len).sum();
            assert_eq!(cut_bytes + frame, prompt.len(), "{}", s.id);
            for label in [
                "role",
                "conventions",
                "output-schema",
                "project-context",
                "digest.window",
                "digest.project",
                "digest.overall",
                "digest.trigger",
                "digest.services",
                "digest.attention-cues",
                "citable-evidence-ids",
                "output-instructions",
            ] {
                let at = SECTION_LABELS.iter().position(|l| *l == label);
                assert!(
                    at.is_some_and(|at| !texts[at].is_empty()),
                    "{}: {label} is read",
                    s.id
                );
            }
        }
    }

    #[test]
    fn section_sources_are_captures_or_shape_ids_two_or_more_and_none_repeated() {
        assert_eq!(
            parse_section_sources("miss=/a,S14, S16"),
            Ok(vec![
                SectionSource::Capture("miss".to_string(), PathBuf::from("/a")),
                SectionSource::Shape("S14".to_string()),
                SectionSource::Shape("S16".to_string()),
            ])
        );
        assert_eq!(
            parse_section_sources("S8").err().as_deref(),
            Some("--sections takes at least two sources")
        );
        assert_eq!(
            parse_section_sources("S8,S8").err().as_deref(),
            Some("a section source label is repeated")
        );
        assert_eq!(
            parse_section_sources("S8=/a,S8").err().as_deref(),
            Some("a section source label is repeated")
        );
        assert_eq!(
            parse_section_sources("a b=/x,S8").err().as_deref(),
            Some("--sections takes LABEL=DIR or shape id items")
        );
    }

    /// Whether an incident's line is the triggering scope's own, read off the
    /// incident: its scope is the cue's, or its fingerprint the cue's.
    fn is_own(s: &Shape, incident: &Incident) -> bool {
        incident.scope_id == s.cue.scope_id
            || Some(&incident.fingerprint) == s.cue.fingerprint.as_ref()
    }

    fn lines_of(incidents: &[Incident]) -> Vec<String> {
        incidents
            .iter()
            .map(|incident| format_corpus_match_line(incident, RENDER_NOW_UNIX_NANO))
            .collect()
    }

    fn rendered(s: &Shape, lines: &[String]) -> String {
        match prepare_rendered("shipped", s, lines) {
            Ok(p) => p.prompt,
            Err(why) => panic!("{} composes: {why}", s.id),
        }
    }

    /// The block's match lines, and the restate line when it is there.
    fn block_lines(prompt: &str) -> Vec<&str> {
        let note = format!("  {CORPUS_MATCHES_FRAMING_NOTE}");
        prompt
            .lines()
            .skip_while(|l| *l != note)
            .skip(1)
            .take_while(|l| l.starts_with("  "))
            .collect()
    }

    fn without_block(prompt: &str) -> String {
        let note = format!("  {CORPUS_MATCHES_FRAMING_NOTE}");
        let restate = format!("  {RESTATE_LINE}");
        prompt
            .split_inclusive('\n')
            .filter(|l| {
                let body = l.trim_end_matches('\n');
                !(body == CORPUS_HEADER
                    || body == note
                    || body == restate
                    || body.starts_with(MATCH_LINE_PREFIX))
            })
            .collect()
    }

    fn edited(prompt: &str, arm: &str, own: &OwnLines) -> String {
        let Some(candidate) = candidate(arm) else {
            panic!("{arm} is a candidate");
        };
        match edit_block(prompt, candidate, own) {
            Ok(Some(edited)) => edited,
            other => panic!("{arm} applies: {other:?}"),
        }
    }

    #[test]
    fn each_edit_equals_the_same_change_made_before_rendering() {
        let mut changed = [0usize; 4];
        for s in shapes() {
            let incidents = baseline_corpus_incidents(&s);
            let (own, others): (Vec<Incident>, Vec<Incident>) =
                incidents.iter().cloned().partition(|i| is_own(&s, i));
            let baseline = rendered(&s, &lines_of(&incidents));
            assert_eq!(prompt_of("nb", s.id), baseline, "{}", s.id);

            let mut restated = lines_of(&incidents);
            if let Some(last) = restated.last_mut() {
                last.push_str(&format!("\n  {RESTATE_LINE}"));
            }
            let reordered: Vec<Incident> = own.iter().chain(&others).cloned().collect();
            let capped: Vec<Incident> = incidents.iter().take(2).cloned().collect();
            let expected = [
                ("CR", rendered(&s, &restated)),
                ("CO", rendered(&s, &lines_of(&reordered))),
                ("CC", rendered(&s, &lines_of(&capped))),
                ("CX", rendered(&s, &lines_of(&own))),
            ];
            let own_positions = own_lines(&s, &incidents);
            for (at, (arm, expected)) in expected.iter().enumerate() {
                let edited = edited(&baseline, arm, &own_positions);
                assert_eq!(&edited, expected, "{arm} {}", s.id);
                changed[at] += usize::from(edited != baseline);
            }
            // The exclude is the product's now: the tree's block is the
            // block that edit produced, so the arm composes as `shipped`.
            assert_eq!(prompt_of("shipped", s.id), expected[3].1, "{}", s.id);
            assert_eq!(
                prompt_of("CX", s.id),
                prompt_of("shipped", s.id),
                "{}",
                s.id
            );
        }
        // Over the baseline blocks: S4 and S8-S17 hold a block; S9 and
        // S14-S17 hold more than two lines; S16 and S17 hold an own line
        // behind another's; S4, S12 and S14-S17 hold a line that is not the
        // triggering scope's own.
        assert_eq!(changed, [11, 2, 5, 6], "shapes each edit changes");
    }

    #[test]
    fn a_candidate_arm_is_its_edit_of_the_baseline_composition() {
        for s in shapes() {
            let baseline = prompt_of("nb", s.id);
            let own = own_lines(&s, &baseline_corpus_incidents(&s));
            for arm in CANDIDATE_ARMS {
                assert_eq!(
                    prompt_of(arm, s.id),
                    edited(&baseline, arm, &own),
                    "{arm} {}",
                    s.id
                );
            }
        }
    }

    #[test]
    fn an_edit_changes_nothing_outside_the_block_and_never_the_trigger_line() {
        for s in shapes() {
            let shipped = prompt_of("shipped", s.id);
            let trigger: Vec<&str> = shipped.lines().filter(|l| is_trigger_line(l)).collect();
            assert_eq!(trigger.len(), 1, "{}", s.id);
            for arm in CANDIDATE_ARMS {
                let edited = prompt_of(arm, s.id);
                assert_eq!(
                    without_block(&edited),
                    without_block(&shipped),
                    "{arm} {}",
                    s.id
                );
                assert_eq!(
                    edited
                        .lines()
                        .filter(|l| is_trigger_line(l))
                        .collect::<Vec<_>>(),
                    trigger,
                    "{arm} {}",
                    s.id
                );
            }
        }
    }

    #[test]
    fn cap_and_reorder_change_no_lines_text() {
        for s in shapes() {
            let baseline = prompt_of("nb", s.id);
            let before = block_lines(&baseline);
            let reordered = prompt_of("CO", s.id);
            let mut sorted = (before.clone(), block_lines(&reordered));
            sorted.0.sort_unstable();
            sorted.1.sort_unstable();
            assert_eq!(sorted.0, sorted.1, "CO {}: the same lines", s.id);
            let capped = prompt_of("CC", s.id);
            let kept: Vec<&str> = before.iter().copied().take(2).collect();
            assert_eq!(block_lines(&capped), kept, "CC {}", s.id);
        }
        let base = prompt_of("nb", "S16");
        let before = block_lines(&base);
        assert_eq!(before.len(), 5);
        assert_eq!(
            block_lines(&prompt_of("CO", "S16")),
            [before[1], before[4], before[0], before[2], before[3]],
            "the two conductor lines first, each group in its order"
        );
        assert_eq!(
            block_lines(&prompt_of("shipped", "S16")),
            [before[1], before[4]],
            "the product keeps the two conductor lines alone, in their order"
        );
    }

    #[test]
    fn exclude_keeps_a_line_the_fingerprint_arm_selects_from_another_scope() {
        let s8 = shape("S8");
        let incidents = baseline_corpus_incidents(&s8);
        assert_eq!(incidents.len(), 2);
        assert!(
            incidents
                .iter()
                .all(|i| i.scope_id != s8.cue.scope_id && is_own(&s8, i)),
            "S8's lines are the sibling's and carry the cue's fingerprint"
        );
        assert_eq!(own_lines(&s8, &incidents), OwnLines::Positions(vec![1, 2]));
        assert_eq!(prompt_of("CX", "S8"), prompt_of("nb", "S8"));
        assert_eq!(
            lines_of(&selected_corpus_incidents(&s8)),
            lines_of(&incidents),
            "and the product's selection keeps both under the cue's scope"
        );

        let s12 = shape("S12");
        let others = baseline_corpus_incidents(&s12);
        assert_eq!(others.len(), 2);
        assert_eq!(
            own_lines(&s12, &others),
            OwnLines::Positions(Vec::new()),
            "S12's lines carry another fingerprint"
        );
        assert!(selected_corpus_incidents(&s12).is_empty());
        assert_eq!(
            prompt_of("CX", "S12"),
            prompt_of("shipped", "S7"),
            "with no line left the header and its note go too"
        );
        assert_eq!(
            block_lines(&prompt_of("CX", "S16")).len(),
            2,
            "S16 keeps its two conductor lines"
        );
    }

    #[test]
    fn a_prompt_without_a_block_is_left_as_it_is() {
        let s7 = prompt_of("shipped", "S7");
        for arm in CANDIDATE_ARMS {
            assert_eq!(prompt_of(arm, "S7"), s7, "{arm}");
            let Some(candidate) = candidate(arm) else {
                panic!("{arm} is a candidate");
            };
            assert_eq!(
                edit_block(&s7, candidate, &OwnLines::Positions(Vec::new())),
                Ok(Some(s7.clone())),
                "{arm}"
            );
        }
    }

    #[test]
    fn the_restate_edit_adds_its_line_once_and_is_identity_after() {
        let s8 = prompt_of("shipped", "S8");
        let restated = prompt_of("CR", "S8");
        let line = format!("  {RESTATE_LINE}");
        assert_eq!(restated.lines().filter(|l| *l == line).count(), 1);
        assert_eq!(restated.len(), s8.len() + line.len() + 1);
        assert_eq!(block_lines(&restated).last().copied(), Some(line.as_str()));
        assert!(RESTATE_LINE.is_ascii());
        let own = OwnLines::Positions(vec![1, 2]);
        assert_eq!(
            edit_block(&restated, Candidate::Restate, &own),
            Ok(Some(restated.clone())),
            "identity once the line is present"
        );
        let Ok(Some(capped)) = edit_block(&restated, Candidate::Cap, &own) else {
            panic!("the cap applies to a restated block");
        };
        assert_eq!(capped, restated, "two lines and the restate line kept");
        let Ok(Some(emptied)) = edit_block(
            &restated,
            Candidate::Exclude,
            &OwnLines::Positions(Vec::new()),
        ) else {
            panic!("the exclude applies to a restated block");
        };
        assert_eq!(emptied, prompt_of("shipped", "S7"));
    }

    #[test]
    fn a_replay_takes_its_own_lines_by_position_and_skips_the_two_edits_without_them() {
        // S16 under `nb`, its own lines behind another's: the block a capture
        // taken before the remedy holds.
        let shape16 = shape("S16");
        let s16 = prompt_of("nb", "S16");
        let from_incidents = own_lines(&shape16, &baseline_corpus_incidents(&shape16));
        let of_shape = |arm: &str| edited(&s16, arm, &from_incidents);
        let dir = capture_of(&s16);
        let Ok(replay) = loaded(dir.path(), Some(SIBLING_SERVICE)) else {
            panic!("the capture loads");
        };
        let given = OwnLines::Positions(vec![2, 5]);
        assert_eq!(from_incidents, given);
        for arm in CANDIDATE_ARMS {
            assert_eq!(
                run_of(replay.compose(arm, &given)).prompt,
                of_shape(arm),
                "{arm}: the edit of the shape, from positions alone"
            );
        }
        for arm in ["CO", "CX"] {
            assert!(
                matches!(
                    replay.compose(arm, &OwnLines::Unknown),
                    Ok(Composed::Skipped(OWN_LINES_UNKNOWN))
                ),
                "{arm}"
            );
        }
        for arm in ["CR", "CC"] {
            assert_eq!(
                run_of(replay.compose(arm, &OwnLines::Unknown)).prompt,
                of_shape(arm),
                "{arm} needs no own lines"
            );
        }
        for arm in ["ns", "nb"] {
            assert_eq!(
                replay.compose(arm, &given).err(),
                Some(format!("arm {arm} does not apply to a replay"))
            );
        }
        assert_eq!(
            replay
                .compose("CX", &OwnLines::Positions(vec![6]))
                .err()
                .as_deref(),
            Some("--own-lines names a line the block does not hold")
        );
    }

    fn path_of(prompt: &str, own: &[usize]) -> Result<Option<ProductPath>, String> {
        product_path(prompt, &OwnLines::Positions(own.to_vec()), SIBLING_SERVICE)
    }

    fn read(unremedied_same: bool, remedied_same: bool, kept: usize, lines: usize) -> ProductPath {
        ProductPath {
            unremedied_same,
            remedied_same,
            kept,
            lines,
        }
    }

    #[test]
    fn the_product_path_reads_a_baseline_block_back_and_composes_the_exclude_edit() {
        let mut dropped = Vec::new();
        for s in shapes() {
            let Some(scope) = s.cue.scope_id.as_deref() else {
                panic!("{}: the cue is scoped", s.id);
            };
            let baseline = prompt_of("nb", s.id);
            let own = own_lines(&s, &baseline_corpus_incidents(&s));
            let lines = corpus_line_count(&baseline);
            let kept = corpus_line_count(&prompt_of("shipped", s.id));
            assert_eq!(
                product_path(&baseline, &own, scope),
                Ok(Some(read(true, true, kept, lines))),
                "{}",
                s.id
            );
            if kept < lines {
                dropped.push(s.id);
            }
        }
        assert_eq!(
            dropped,
            ["S4", "S12", "S14", "S15", "S16", "S17"],
            "the shapes holding a line the cue's scope drops"
        );
    }

    #[test]
    fn the_product_path_reads_differs_where_the_selection_and_the_edit_part() {
        // A sibling's line named own: the edit keeps it alone, the selection
        // keeps it and the two lines that carry the cited fingerprint.
        let s16 = prompt_of("nb", "S16");
        assert_eq!(path_of(&s16, &[1]), Ok(Some(read(true, false, 3, 5))));
        // S8's lines are the sibling's and carry the cited fingerprint: with
        // no own position the edit drops them and the selection keeps them.
        let s8 = prompt_of("nb", "S8");
        assert_eq!(path_of(&s8, &[]), Ok(Some(read(true, false, 2, 2))));
        // A block that is not newest first is not read back as itself.
        let reordered = prompt_of("CO", "S16");
        assert_ne!(reordered, s16);
        assert_eq!(
            path_of(&reordered, &[1, 2]),
            Ok(Some(read(false, true, 2, 5)))
        );
    }

    #[test]
    fn the_product_path_refuses_what_it_cannot_read_back() {
        let s8 = prompt_of("nb", "S8");
        assert_eq!(
            product_path(&s8, &OwnLines::Unknown, SIBLING_SERVICE),
            Ok(None)
        );
        assert_eq!(
            path_of(&prompt_of("nb", "S7"), &[]),
            Ok(Some(read(true, true, 0, 0))),
            "no block: nothing to select"
        );
        assert_eq!(s8.matches("m ago, active\n").count(), 1);
        let unknown_status = s8.replacen("m ago, active\n", "m ago, open\n", 1);
        assert_eq!(
            path_of(&unknown_status, &[1, 2]),
            Err(LINE_FORM.to_string())
        );
        assert_eq!(s8.matches(" — 3m ago, active\n").count(), 1);
        let padded_age = s8.replacen(" — 3m ago, active\n", " — 03m ago, active\n", 1);
        assert_eq!(
            path_of(&padded_age, &[1, 2]),
            Err(LINE_FORM.to_string()),
            "a line the product's format does not write back"
        );
        let sibling_row = |line: &&str| line.starts_with("  conductor-canary     ");
        assert_eq!(s8.lines().filter(sibling_row).count(), 1);
        let alone: String = s8
            .split_inclusive('\n')
            .filter(|line| !sibling_row(&line.trim_end_matches('\n')))
            .collect();
        assert_eq!(path_of(&alone, &[1, 2]), Err(NO_OTHER_SCOPE.to_string()));
        assert_eq!(
            path_of(&s8, &[3]).err().as_deref(),
            Some("--own-lines names a line the block does not hold")
        );
        assert_eq!(
            product_path_line("replay:miss", &read(true, false, 2, 5)),
            "dry-run: product path replay:miss: unremedied same · under the cue scope differs as CX · kept 2 of 5 lines"
        );
    }

    #[test]
    fn own_lines_parse_as_unknown_none_or_distinct_positions() {
        assert_eq!(parse_own_lines("unknown"), Ok(OwnLines::Unknown));
        assert_eq!(parse_own_lines("none"), Ok(OwnLines::Positions(Vec::new())));
        assert_eq!(parse_own_lines("2, 5"), Ok(OwnLines::Positions(vec![2, 5])));
        for bad in ["0", "2,2", "two", "", "1,,2"] {
            assert_eq!(
                parse_own_lines(bad).err().as_deref(),
                Some("--own-lines takes unknown, none or distinct 1-based positions"),
                "{bad}"
            );
        }
    }

    #[test]
    fn a_digest_that_ends_without_a_newline_gains_none() {
        let s8 = prompt_of("shipped", "S8");
        let close = format!("\n\n{DIGEST_CLOSE_MARKER}\n");
        assert_eq!(s8.matches(&close).count(), 1);
        let bare = s8.replacen(&close, &format!("\n{DIGEST_CLOSE_MARKER}\n"), 1);
        let none = OwnLines::Positions(Vec::new());
        let Ok(Some(restated)) = edit_block(&bare, Candidate::Restate, &none) else {
            panic!("the restate applies");
        };
        assert!(restated.contains(&format!("  {RESTATE_LINE}\n{DIGEST_CLOSE_MARKER}\n")));
        let Ok(Some(emptied)) = edit_block(&bare, Candidate::Exclude, &none) else {
            panic!("the exclude applies");
        };
        let s7 = prompt_of("shipped", "S7");
        assert_eq!(
            emptied,
            s7.replacen(&close, &format!("\n{DIGEST_CLOSE_MARKER}\n"), 1)
        );
    }
}
