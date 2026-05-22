//! Chunk #77 — Drain LogHub-style golden corpus regression harness.
//!
//! Per pulse-v0_2_0-route §77 Part 2 final bullet + acceptance criterion #7 +
//! audit Section 3.S ("algorithm correctness verified via unit tests but no
//! golden directory visible"):
//!
//! Loads committed LogHub-style log line fixtures от `tests/fixtures/drain/`
//! and drives the Drain mining algorithm via the public `DrainMiner` API
//! (per arch §Cross-cutting Test-time telemetry injection — NOT pre-seeded
//! DuckDB rows). Asserts the mined template output matches committed
//! golden expected JSON structurally (template count + per-template token
//! patterns + occurrence counts).
//!
//! Coverage scope: NUM masking, IPv4 masking, identical-line clustering.
//! Per chunk #69 e2e_drain_template_assignment.rs precedent + chunk #77
//! research.md open question 4 recommendation (synthetic fixtures vs
//! vendoring LogHub corpus directly — synthetic is sufficient for
//! regression coverage of Drain mining-output stability).

use buffer::{DrainConfig, DrainMiner};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct ExpectedGolden {
    fixture: String,
    description: String,
    total_input_lines: usize,
    expected_template_count: usize,
    expected_templates: Vec<ExpectedTemplate>,
}

#[derive(Debug, Deserialize)]
struct ExpectedTemplate {
    description: String,
    expected_occurrence_count: u64,
    must_contain_tokens: Vec<String>,
    must_contain_mask: Option<String>,
}

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("drain")
}

#[test]
fn drain_mines_synthetic_basic_matches_golden_expected() {
    let dir = fixtures_dir();
    let input_path = dir.join("synthetic_basic.log");
    let expected_path = dir.join("synthetic_basic.expected.json");

    let input = std::fs::read_to_string(&input_path)
        .unwrap_or_else(|e| panic!("read input fixture {input_path:?}: {e}"));
    let expected_json = std::fs::read_to_string(&expected_path)
        .unwrap_or_else(|e| panic!("read expected fixture {expected_path:?}: {e}"));
    let expected: ExpectedGolden =
        serde_json::from_str(&expected_json).expect("parse expected.json");

    let _ = expected.description; // sanity-touch the description field
    let _ = expected.fixture;

    let input_lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(
        input_lines.len(),
        expected.total_input_lines,
        "input fixture line count mismatch (golden drift): expected {} got {}",
        expected.total_input_lines,
        input_lines.len()
    );

    // Drive Drain via the public DrainMiner API. No persistence — pure
    // in-memory regression test of the mining algorithm.
    let miner = DrainMiner::new(DrainConfig::default_config(), None);
    let mut ts_nano = 1_700_000_000_000_000_000_i64;
    for line in &input_lines {
        let _ = miner.assign_at(line, ts_nano);
        ts_nano += 1_000_000;
    }

    // Read back the template distribution. top_n large enough к surface ALL
    // templates (we expect ≤10; choose 50 для safety margin).
    let distribution = miner.template_distribution(50);

    // Assert 1: template count matches expected.
    assert_eq!(
        distribution.len(),
        expected.expected_template_count,
        "Drain mining produced unexpected template count: expected {} got {}. \
         Got templates: {:#?}",
        expected.expected_template_count,
        distribution.len(),
        distribution
            .iter()
            .map(|t| (t.id, &t.content, t.occurrence_count))
            .collect::<Vec<_>>()
    );

    // Assert 2: each expected template matches at least one mined template
    // by structural pattern (must_contain_tokens + must_contain_mask +
    // expected_occurrence_count).
    for expected_template in &expected.expected_templates {
        let matched = distribution.iter().find(|mined| {
            // All required tokens present в the mined template content.
            let tokens_match = expected_template
                .must_contain_tokens
                .iter()
                .all(|token| mined.content.contains(token.as_str()));
            // Mask requirement (if specified) present в mined content.
            let mask_match = match &expected_template.must_contain_mask {
                Some(mask) => mined.content.contains(mask.as_str()),
                None => true,
            };
            // Occurrence count matches.
            let count_match = mined.occurrence_count == expected_template.expected_occurrence_count;
            tokens_match && mask_match && count_match
        });
        assert!(
            matched.is_some(),
            "Expected template `{}` (occurrence {}, tokens {:?}, mask {:?}) NOT found in mined distribution. \
             Mined: {:#?}",
            expected_template.description,
            expected_template.expected_occurrence_count,
            expected_template.must_contain_tokens,
            expected_template.must_contain_mask,
            distribution
                .iter()
                .map(|t| (t.id, t.content.clone(), t.occurrence_count))
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn drain_template_count_matches_input_template_diversity() {
    // Sanity invariant: after mining the synthetic_basic fixture, the miner
    // reports the expected total template count via the public
    // `template_count()` API. Pairs с the structural test above for
    // belt-and-suspenders regression coverage.
    let dir = fixtures_dir();
    let input = std::fs::read_to_string(dir.join("synthetic_basic.log")).expect("read fixture");
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();

    let miner = DrainMiner::new(DrainConfig::default_config(), None);
    let mut ts_nano = 1_700_000_000_000_000_000_i64;
    for line in &lines {
        let _ = miner.assign_at(line, ts_nano);
        ts_nano += 1_000_000;
    }
    assert_eq!(
        miner.template_count(),
        3,
        "template_count() mismatch: expected 3 (NUM / IP / exact-match clusters), got {}",
        miner.template_count()
    );
}
