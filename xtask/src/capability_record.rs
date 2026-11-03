//! The `verify:capability-matrix` verdict over the capability record
//! (test-plan 9, Capability verification matrix): `docs/capability-record.json`
//! gives each of P-001..P-082 one disposition, `claimed` or `retired`.
//!
//! The verb holds all 82 ids to the form, validates the proofs of the claimed
//! ones and resolves every route entry a retired one names against the working
//! route. It validates no proof of a retired id: the entries that remove a
//! surface delete those files by design. An absent or unreadable input is an
//! exit of its own, never a pass.

use std::collections::BTreeSet;
use std::path::Path;

use serde_json::Value;

pub const RECORD_PATH: &str = "docs/capability-record.json";
pub const ROUTE_PATH: &str = "andromeda-pulse-0.4.0/working-route.md";

pub const DISPOSITIONS: [&str; 2] = ["claimed", "retired"];
pub const SURFACES: [&str; 6] = [
    "window",
    "model",
    "desktop",
    "workspace",
    "training-export",
    "corpus-encryption",
];
pub const GUARD_STATES: [&str; 3] = ["runs", "part", "none"];

#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    /// An input is absent, unreadable or not a record; the reason names which.
    CannotEvaluate(String),
    Read(Reading),
}

/// What the record holds, with every finding. The counts follow the record.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Reading {
    pub ids: usize,
    pub claimed: usize,
    pub retired: usize,
    pub findings: Vec<String>,
}

impl Verdict {
    pub fn state(&self) -> &'static str {
        match self {
            Verdict::CannotEvaluate(_) => "cannot-evaluate",
            Verdict::Read(reading) if reading.findings.is_empty() => "clean",
            Verdict::Read(_) => "violations",
        }
    }

    pub fn exit_code(&self) -> u8 {
        match self.state() {
            "clean" => 0,
            "violations" => 1,
            _ => 2,
        }
    }

    /// The one line the verb prints.
    pub fn line(&self) -> String {
        let state = self.state();
        match self {
            Verdict::CannotEvaluate(reason) => {
                format!("verify:capability-matrix: {state} ({reason})")
            }
            Verdict::Read(reading) => format!(
                "verify:capability-matrix: {state} ({} ids: {} claimed, {} retired, {} violation(s))",
                reading.ids,
                reading.claimed,
                reading.retired,
                reading.findings.len()
            ),
        }
    }
}

/// Scenario kinds whose `ref` is a file under the root.
const FILE_KINDS: [&str; 5] = [
    "nextest-file",
    "ui-test",
    "a11y-spec",
    "ci-script",
    "source-evidence",
];
/// Scenario kinds with no file to check: the entry's `note` says where the
/// proof lives.
const NOTED_KINDS: [&str; 3] = ["by-construction", "external", "manual"];
const MODES: [&str; 9] = [
    "automated-nextest",
    "automated-a11y",
    "automated-e2e",
    "xtask-gate",
    "env-gated-runtime",
    "manual-sr-supplemental",
    "by-construction",
    "dynamic-external",
    "manual",
];

const RECORD_IDS: std::ops::RangeInclusive<u32> = 1..=82;
const CARRYING_IDS: std::ops::RangeInclusive<u32> = 83..=129;

/// The number of a `P-NNN` id.
fn id_number(id: &str) -> Option<u32> {
    let digits = id.strip_prefix("P-")?;
    (digits.len() == 3 && digits.bytes().all(|byte| byte.is_ascii_digit()))
        .then(|| digits.parse().ok())
        .flatten()
}

fn text<'a>(entry: &'a Value, key: &str) -> &'a str {
    entry.get(key).and_then(Value::as_str).unwrap_or("")
}

/// The strings of an array member; an element that is no string reads empty.
fn words<'a>(entry: &'a Value, key: &str) -> Vec<&'a str> {
    entry
        .get(key)
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .map(|item| item.as_str().unwrap_or(""))
                .collect()
        })
        .unwrap_or_default()
}

fn has_note(entry: &Value) -> bool {
    !text(entry, "note").trim().is_empty()
}

/// A route entry's title: its line without a leading `[marker] ` stamp, up
/// to the first ` -- ` (an em dash). Headers, note lines and the arrows
/// between entries are no entries.
fn route_titles(route: &str) -> BTreeSet<&str> {
    route
        .lines()
        .filter(|line| !line.starts_with(['#', '_', ' ', '\t']))
        .filter_map(|line| {
            let unstamped = match line.strip_prefix('[') {
                Some(rest) => rest.split_once("] ")?.1,
                None => line,
            };
            Some(unstamped.split_once(" \u{2014} ")?.0)
        })
        .collect()
}

fn legend_findings(legend: Option<&Value>, findings: &mut Vec<String>) {
    let Some(legend) = legend.and_then(Value::as_object) else {
        findings.push("legend: absent".to_owned());
        return;
    };
    let sets: [(&str, &[&str]); 3] = [
        ("disposition", &DISPOSITIONS),
        ("surface", &SURFACES),
        ("guard", &GUARD_STATES),
    ];
    for (key, closed) in sets {
        let stated: BTreeSet<&str> = legend
            .get(key)
            .and_then(Value::as_object)
            .map(|words| words.keys().map(String::as_str).collect())
            .unwrap_or_default();
        if stated != closed.iter().copied().collect() {
            findings.push(format!(
                "legend: `{key}` is not the closed set {}",
                closed.join(", ")
            ));
        }
    }
    if legend.len() != sets.len() {
        findings.push("legend: a member beyond disposition, surface and guard".to_owned());
    }
}

fn claimed_findings(id: &str, entry: &Value, root: &Path, findings: &mut Vec<String>) {
    match entry.get("carried_by").and_then(Value::as_array) {
        None => findings.push(format!("{id}: `carried_by` is absent")),
        Some(carrying) if carrying.is_empty() && !has_note(entry) => {
            findings.push(format!("{id}: empty `carried_by` and no `note`"));
        }
        Some(_) => {
            for carrying in words(entry, "carried_by") {
                if !id_number(carrying).is_some_and(|n| CARRYING_IDS.contains(&n)) {
                    findings.push(format!(
                        "{id}: carried_by `{carrying}` is outside P-083..P-129"
                    ));
                }
            }
        }
    }
    if entry.get("provisional") == Some(&Value::Bool(true)) && !has_note(entry) {
        findings.push(format!("{id}: provisional with no `note`"));
    }
    let mode = text(entry, "verification_mode");
    if !MODES.contains(&mode) {
        findings.push(format!("{id}: verification_mode `{mode}` is unknown"));
    }

    let scenarios = entry
        .get("scenarios")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default();
    if scenarios.is_empty() {
        findings.push(format!("{id}: no scenario"));
        return;
    }
    for scenario in scenarios {
        let kind = text(scenario, "kind");
        let reference = text(scenario, "ref");
        if NOTED_KINDS.contains(&kind) {
            if !has_note(entry) {
                findings.push(format!("{id}: `{kind}` scenario with no `note`"));
            }
        } else if FILE_KINDS.contains(&kind) {
            let path = root.join(reference);
            if reference.is_empty() || !path.is_file() {
                findings.push(format!("{id}: scenario ref `{reference}` does not exist"));
            } else if let Some(anchor) = scenario.get("contains").and_then(Value::as_str) {
                match std::fs::read_to_string(&path) {
                    Ok(content) if content.contains(anchor) => {}
                    Ok(_) => findings.push(format!(
                        "{id}: anchor `{anchor}` not found in `{reference}`"
                    )),
                    Err(_) => {
                        findings.push(format!("{id}: scenario ref `{reference}` is unreadable"));
                    }
                }
            }
        } else {
            findings.push(format!("{id}: scenario kind `{kind}` is unknown"));
        }
    }
    if scenarios
        .iter()
        .all(|scenario| text(scenario, "kind") == "source-evidence")
    {
        findings.push(format!("{id}: source-evidence alone is no proof"));
    }
}

fn retired_findings(id: &str, entry: &Value, titles: &BTreeSet<&str>, findings: &mut Vec<String>) {
    for key in ["scenarios", "verification_mode", "carried_by"] {
        if entry.get(key).is_some() {
            findings.push(format!("{id}: a retired entry carries `{key}`"));
        }
    }
    let surfaces = words(entry, "surfaces");
    if surfaces.is_empty() {
        findings.push(format!("{id}: `surfaces` is empty"));
    }
    for surface in surfaces {
        if !SURFACES.contains(&surface) {
            findings.push(format!("{id}: surface `{surface}` is unknown"));
        }
    }
    let removing = words(entry, "removed_by");
    if removing.is_empty() {
        findings.push(format!("{id}: `removed_by` is empty"));
    }
    for title in removing {
        if !titles.contains(title) {
            findings.push(format!(
                "{id}: removed_by `{title}` is no entry of the route"
            ));
        }
    }
    if entry.get("kept_half_owner").is_some() {
        let owner = text(entry, "kept_half_owner");
        if !titles.contains(owner) {
            findings.push(format!(
                "{id}: kept_half_owner `{owner}` is no entry of the route"
            ));
        }
    }
    let Some(guard) = entry.get("guard").filter(|guard| guard.is_object()) else {
        findings.push(format!("{id}: `guard` is absent"));
        return;
    };
    let state = text(guard, "state");
    if !GUARD_STATES.contains(&state) {
        findings.push(format!("{id}: guard state `{state}` is unknown"));
        return;
    }
    if state != "none" && words(guard, "by").is_empty() {
        findings.push(format!("{id}: guard `{state}` with an empty `by`"));
    }
    if state != "runs" && words(guard, "unrun").is_empty() {
        findings.push(format!("{id}: guard `{state}` with an empty `unrun`"));
    }
}

/// The verdict over the record's text and the route's text. A file a
/// claimed entry names is looked up under `root`.
pub fn judge(record: &str, route: &str, root: &Path) -> Verdict {
    let Ok(doc) = serde_json::from_str::<Value>(record) else {
        return Verdict::CannotEvaluate(format!("{RECORD_PATH}: not JSON"));
    };
    let Some(capabilities) = doc.get("capabilities").and_then(Value::as_array) else {
        return Verdict::CannotEvaluate(format!("{RECORD_PATH}: no `capabilities` array"));
    };
    let titles = route_titles(route);
    let mut reading = Reading::default();
    let findings = &mut reading.findings;
    legend_findings(doc.get("legend"), findings);

    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for entry in capabilities {
        let id = text(entry, "id");
        if id.is_empty() {
            findings.push("entry: no `id`".to_owned());
            continue;
        }
        if !seen.insert(id) {
            findings.push(format!("{id}: duplicate id"));
        }
        if !id_number(id).is_some_and(|n| RECORD_IDS.contains(&n)) {
            findings.push(format!("{id}: id outside P-001..P-082"));
        }
        match text(entry, "disposition") {
            "claimed" => {
                reading.claimed += 1;
                claimed_findings(id, entry, root, findings);
            }
            "retired" => {
                reading.retired += 1;
                retired_findings(id, entry, &titles, findings);
            }
            other => findings.push(format!(
                "{id}: disposition `{other}` is neither `claimed` nor `retired`"
            )),
        }
    }
    for number in RECORD_IDS {
        let id = format!("P-{number:03}");
        if !seen.contains(id.as_str()) {
            findings.push(format!("{id}: missing from the record"));
        }
    }
    if reading.claimed == 0 {
        findings.push("record: no claimed id".to_owned());
    }
    reading.ids = seen.len();
    Verdict::Read(reading)
}

/// Reads the record and the route under `root` and judges them.
pub fn evaluate(root: &Path) -> Verdict {
    let read = |relative: &str| {
        std::fs::read_to_string(root.join(relative))
            .map_err(|err| Verdict::CannotEvaluate(format!("{relative}: {}", err.kind())))
    };
    let record = match read(RECORD_PATH) {
        Ok(text) => text,
        Err(verdict) => return verdict,
    };
    let route = match read(ROUTE_PATH) {
        Ok(text) => text,
        Err(verdict) => return verdict,
    };
    judge(&record, &route, root)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    const PROOF_FILE: &str = "crates/demo/src/lib.rs";
    const PROOF_ANCHOR: &str = "proof_anchor";
    const ROUTE: &str = "# Working Route\n\n\
        _A note line \u{2014} never an entry._\n\n\
        ### Epoch 1 \u{2014} Foundation\n\
        [2026-01-01-engine-record] Engine record kept \u{2014} done (P-112)\n   \u{2193}\n\
        Window retired \u{2014} the shell leaves (P-083)\n   \u{2193}\n\
        Display-only computation retired \u{2014} stream topics leave (P-109)\n";

    fn legend() -> Value {
        let meanings = |words: &[&str]| -> Value {
            words
                .iter()
                .map(|word| ((*word).to_owned(), json!("one line of meaning")))
                .collect::<serde_json::Map<String, Value>>()
                .into()
        };
        json!({
            "disposition": meanings(&DISPOSITIONS),
            "surface": meanings(&SURFACES),
            "guard": meanings(&GUARD_STATES),
        })
    }

    fn claimed(id: &str) -> Value {
        json!({
            "id": id,
            "title": "A kept capability",
            "disposition": "claimed",
            "carried_by": ["P-086"],
            "verification_mode": "automated-nextest",
            "scenarios": [{"kind": "nextest-file", "ref": PROOF_FILE, "contains": PROOF_ANCHOR}],
        })
    }

    fn retired(id: &str) -> Value {
        json!({
            "id": id,
            "title": "A retired capability",
            "disposition": "retired",
            "surfaces": ["window"],
            "removed_by": ["Window retired"],
            "guard": {"state": "runs", "by": ["pulse-app/tests/a_proof.rs"], "unrun": []},
        })
    }

    /// P-001 claimed, P-002 to P-082 retired with the window.
    fn valid_record() -> Value {
        let mut capabilities = vec![claimed("P-001")];
        capabilities.extend((2..=82).map(|n| retired(&format!("P-{n:03}"))));
        json!({
            "schema_version": 1,
            "legend": legend(),
            "capabilities": capabilities,
        })
    }

    fn root_holding(record: Option<&str>, route: Option<&str>) -> tempfile::TempDir {
        let root = tempfile::TempDir::new().expect("tmp");
        let write = |relative: &str, text: &str| {
            let path = root.path().join(relative);
            std::fs::create_dir_all(path.parent().expect("a parent")).expect("create the dir");
            std::fs::write(path, text).expect("write the file");
        };
        write(PROOF_FILE, &format!("fn {PROOF_ANCHOR}() {{}}\n"));
        if let Some(text) = record {
            write(RECORD_PATH, text);
        }
        if let Some(text) = route {
            write(ROUTE_PATH, text);
        }
        root
    }

    fn judge_record(record: &Value) -> Verdict {
        let root = root_holding(Some(&record.to_string()), Some(ROUTE));
        evaluate(root.path())
    }

    fn assert_cannot_evaluate(verdict: &Verdict, names: &str) {
        let Verdict::CannotEvaluate(reason) = verdict else {
            panic!("expected cannot-evaluate, read {verdict:?}");
        };
        assert!(reason.contains(names), "reason: {reason}");
        assert_eq!(verdict.exit_code(), 2);
        assert!(
            verdict
                .line()
                .starts_with("verify:capability-matrix: cannot-evaluate ("),
            "line: {}",
            verdict.line()
        );
    }

    /// The record's one finding: an arm's pin differs from the valid record
    /// in that arm alone.
    fn only_finding(record: &Value) -> String {
        let verdict = judge_record(record);
        assert_eq!(verdict.exit_code(), 1, "verdict: {verdict:?}");
        let Verdict::Read(reading) = verdict else {
            panic!("expected a reading, read {verdict:?}");
        };
        assert_eq!(
            reading.findings.len(),
            1,
            "findings: {:?}",
            reading.findings
        );
        reading.findings.into_iter().next().expect("one finding")
    }

    fn assert_finding(record: &Value, id: &str, says: &str) {
        let finding = only_finding(record);
        assert!(
            finding.starts_with(&format!("{id}: ")) && finding.contains(says),
            "finding: {finding}"
        );
    }

    /// The valid record with entry `index` changed by `change`.
    fn with_entry(index: usize, change: impl FnOnce(&mut Value)) -> Value {
        let mut record = valid_record();
        change(&mut record["capabilities"][index]);
        record
    }

    fn without_key(index: usize, key: &str) -> Value {
        with_entry(index, |entry| {
            entry.as_object_mut().expect("an object").remove(key);
        })
    }

    #[test]
    fn a_valid_record_reads_clean() {
        let verdict = judge_record(&valid_record());
        assert_eq!(verdict.state(), "clean", "verdict: {verdict:?}");
        assert_eq!(verdict.exit_code(), 0);
    }

    #[test]
    fn the_counts_on_the_line_are_read_from_the_record() {
        let verdict = judge_record(&valid_record());
        assert_eq!(
            verdict.line(),
            "verify:capability-matrix: clean (82 ids: 1 claimed, 81 retired, 0 violation(s))"
        );
    }

    #[test]
    fn the_committed_record_reads_clean_over_the_committed_route() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the workspace root");
        let verdict = evaluate(root);
        assert_eq!(
            verdict.line(),
            "verify:capability-matrix: clean (82 ids: 36 claimed, 46 retired, 0 violation(s))",
            "verdict: {verdict:?}"
        );
    }

    #[test]
    fn an_absent_record_cannot_be_evaluated() {
        let root = root_holding(None, Some(ROUTE));
        assert_cannot_evaluate(&evaluate(root.path()), RECORD_PATH);
    }

    #[test]
    fn an_unreadable_record_cannot_be_evaluated() {
        let root = root_holding(None, Some(ROUTE));
        std::fs::create_dir_all(root.path().join(RECORD_PATH)).expect("a dir in the file's place");
        assert_cannot_evaluate(&evaluate(root.path()), RECORD_PATH);
    }

    #[test]
    fn a_record_that_is_not_json_cannot_be_evaluated() {
        let root = root_holding(Some("{ not json"), Some(ROUTE));
        assert_cannot_evaluate(&evaluate(root.path()), "not JSON");
    }

    #[test]
    fn a_record_with_no_capabilities_array_cannot_be_evaluated() {
        let root = root_holding(Some(r#"{"capabilities": {}}"#), Some(ROUTE));
        assert_cannot_evaluate(&evaluate(root.path()), "capabilities");
    }

    #[test]
    fn an_absent_route_cannot_be_evaluated() {
        let root = root_holding(Some(&valid_record().to_string()), None);
        assert_cannot_evaluate(&evaluate(root.path()), ROUTE_PATH);
    }

    #[test]
    fn a_missing_id_is_a_finding() {
        let mut record = valid_record();
        record["capabilities"]
            .as_array_mut()
            .expect("an array")
            .pop();
        assert_finding(&record, "P-082", "missing");
    }

    #[test]
    fn a_duplicated_id_is_a_finding() {
        let mut record = valid_record();
        record["capabilities"]
            .as_array_mut()
            .expect("an array")
            .push(retired("P-082"));
        assert_finding(&record, "P-082", "duplicate");
    }

    #[test]
    fn an_id_outside_the_range_is_a_finding() {
        let mut record = valid_record();
        record["capabilities"]
            .as_array_mut()
            .expect("an array")
            .push(retired("P-083"));
        assert_finding(&record, "P-083", "outside P-001..P-082");
    }

    #[test]
    fn an_unknown_disposition_is_a_finding() {
        let record = with_entry(1, |entry| entry["disposition"] = json!("deferred"));
        assert_finding(&record, "P-002", "disposition `deferred`");
    }

    #[test]
    fn a_legend_that_is_not_the_three_closed_sets_is_a_finding() {
        let mut record = valid_record();
        record["legend"]["surface"]
            .as_object_mut()
            .expect("an object")
            .remove("desktop");
        assert_finding(&record, "legend", "surface");
    }

    #[test]
    fn a_claimed_entry_without_carried_by_is_a_finding() {
        assert_finding(&without_key(0, "carried_by"), "P-001", "carried_by");
    }

    #[test]
    fn a_carrying_requirement_outside_the_range_is_a_finding() {
        let record = with_entry(0, |entry| entry["carried_by"] = json!(["P-082"]));
        assert_finding(&record, "P-001", "outside P-083..P-129");
    }

    #[test]
    fn an_empty_carried_by_needs_a_note() {
        let empty = with_entry(0, |entry| entry["carried_by"] = json!([]));
        assert_finding(&empty, "P-001", "no `note`");

        let noted = with_entry(0, |entry| {
            entry["carried_by"] = json!([]);
            entry["note"] = json!("no requirement restates it");
        });
        assert_eq!(judge_record(&noted).state(), "clean");
    }

    #[test]
    fn a_provisional_entry_needs_a_note() {
        let record = with_entry(0, |entry| entry["provisional"] = json!(true));
        assert_finding(&record, "P-001", "provisional");
    }

    #[test]
    fn a_claimed_entry_with_no_scenario_is_a_finding() {
        let record = with_entry(0, |entry| entry["scenarios"] = json!([]));
        assert_finding(&record, "P-001", "no scenario");
    }

    #[test]
    fn source_evidence_alone_is_no_proof() {
        let record = with_entry(0, |entry| {
            entry["scenarios"] = json!([{"kind": "source-evidence", "ref": PROOF_FILE}]);
        });
        assert_finding(&record, "P-001", "source-evidence");
    }

    #[test]
    fn an_unknown_scenario_kind_is_a_finding() {
        let record = with_entry(0, |entry| entry["scenarios"][0]["kind"] = json!("vibes"));
        assert_finding(&record, "P-001", "scenario kind `vibes`");
    }

    #[test]
    fn an_unknown_verification_mode_is_a_finding() {
        let record = with_entry(0, |entry| entry["verification_mode"] = json!("hoped"));
        assert_finding(&record, "P-001", "verification_mode `hoped`");
    }

    #[test]
    fn a_dangling_file_ref_is_a_finding() {
        let record = with_entry(0, |entry| {
            entry["scenarios"][0]["ref"] = json!("crates/demo/src/gone.rs");
        });
        assert_finding(&record, "P-001", "does not exist");
    }

    #[test]
    fn a_missing_anchor_is_a_finding() {
        let record = with_entry(0, |entry| {
            entry["scenarios"][0]["contains"] = json!("absent_anchor");
        });
        assert_finding(&record, "P-001", "anchor `absent_anchor` not found");
    }

    #[test]
    fn a_scenario_with_no_file_needs_a_note() {
        for kind in ["external", "manual", "by-construction"] {
            let scenario = json!([{"kind": kind, "ref": "a round recorded elsewhere"}]);
            let bare = with_entry(0, |entry| entry["scenarios"] = scenario.clone());
            assert_finding(&bare, "P-001", &format!("`{kind}` scenario"));

            let noted = with_entry(0, |entry| {
                entry["scenarios"] = scenario.clone();
                entry["note"] = json!("where the proof lives");
            });
            assert_eq!(judge_record(&noted).state(), "clean", "kind: {kind}");
        }
    }

    #[test]
    fn a_retired_entry_carries_no_proof_of_its_own() {
        for (key, value) in [
            (
                "scenarios",
                json!([{"kind": "nextest-file", "ref": PROOF_FILE}]),
            ),
            ("verification_mode", json!("automated-nextest")),
            ("carried_by", json!(["P-086"])),
        ] {
            let record = with_entry(1, |entry| entry[key] = value);
            assert_finding(&record, "P-002", &format!("`{key}`"));
        }
    }

    #[test]
    fn a_retired_entry_with_no_surface_is_a_finding() {
        let record = with_entry(1, |entry| entry["surfaces"] = json!([]));
        assert_finding(&record, "P-002", "surfaces");
    }

    #[test]
    fn an_unknown_surface_is_a_finding() {
        let record = with_entry(1, |entry| entry["surfaces"] = json!(["window", "tray"]));
        assert_finding(&record, "P-002", "surface `tray`");
    }

    #[test]
    fn a_retired_entry_with_no_removing_entry_is_a_finding() {
        let record = with_entry(1, |entry| entry["removed_by"] = json!([]));
        assert_finding(&record, "P-002", "removed_by");
    }

    #[test]
    fn a_removing_entry_the_route_does_not_carry_is_a_finding() {
        let record = with_entry(1, |entry| entry["removed_by"] = json!(["Tray retired"]));
        assert_finding(&record, "P-002", "`Tray retired`");
    }

    #[test]
    fn a_title_is_read_past_its_marker_stamp_and_never_from_a_note_line() {
        let stamped = with_entry(1, |entry| {
            entry["removed_by"] = json!(["Engine record kept"]);
        });
        assert_eq!(judge_record(&stamped).state(), "clean");

        let note = with_entry(1, |entry| entry["removed_by"] = json!(["_A note line"]));
        assert_finding(&note, "P-002", "`_A note line`");
        let header = with_entry(1, |entry| entry["removed_by"] = json!(["### Epoch 1"]));
        assert_finding(&header, "P-002", "`### Epoch 1`");
    }

    #[test]
    fn a_kept_half_owner_the_route_does_not_carry_is_a_finding() {
        let named = with_entry(1, |entry| {
            entry["kept_half_owner"] = json!("Display-only computation retired");
        });
        assert_eq!(judge_record(&named).state(), "clean");

        let record = with_entry(1, |entry| entry["kept_half_owner"] = json!("Nobody"));
        assert_finding(&record, "P-002", "kept_half_owner `Nobody`");
    }

    #[test]
    fn a_retired_entry_with_no_guard_is_a_finding() {
        assert_finding(&without_key(1, "guard"), "P-002", "guard");
    }

    #[test]
    fn an_unknown_guard_state_is_a_finding() {
        let record = with_entry(1, |entry| entry["guard"]["state"] = json!("mostly"));
        assert_finding(&record, "P-002", "guard state `mostly`");
    }

    #[test]
    fn a_guard_that_runs_names_what_runs() {
        for state in ["runs", "part"] {
            let record = with_entry(1, |entry| {
                entry["guard"] =
                    json!({"state": state, "by": [], "unrun": ["a proof no gate runs"]});
            });
            assert_finding(&record, "P-002", "empty `by`");
        }
    }

    #[test]
    fn a_guard_short_of_running_names_what_does_not_run() {
        for state in ["part", "none"] {
            let record = with_entry(1, |entry| {
                entry["guard"] = json!({"state": state, "by": ["a proof a job runs"], "unrun": []});
            });
            assert_finding(&record, "P-002", "empty `unrun`");
        }
    }

    #[test]
    fn a_record_with_no_claimed_id_is_a_finding() {
        let mut record = valid_record();
        record["capabilities"][0] = retired("P-001");
        assert_finding(&record, "record", "no claimed id");
    }
}
