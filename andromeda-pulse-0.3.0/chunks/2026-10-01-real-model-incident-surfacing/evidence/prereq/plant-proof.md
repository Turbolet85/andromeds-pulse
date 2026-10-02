# PREREQ one-shot RED/GREEN proof - check:english-sources (plan Step 4)

Command: `cargo xtask check:english-sources`, run from the repository root.
Plant: `crates/english_sources_plant.rs`, a 2-line `.rs` file whose line 2 is
`// send \u{043A} the model` (U+043A written here escaped). Removed after reading 1.

## Reading 1 - planted
- exit: 1
- stdout: decodes as strict ASCII (211 bytes)
- annotations (1):
  - `::error file=crates/english_sources_plant.rs,line=2::Cyrillic character in source - crates/english_sources_plant.rs:2: // send \u{043A} the model`
- verdict line: `"verdict": "findings"`

## Reading 2 - plant removed
- exit: 0
- stdout: decodes as strict ASCII (62 bytes)
- verdict line: `"verdict": "clean"`

Strict ASCII means the bytes are identical under cp1252 or any other ANSI code
page, so the Windows runner's pipe encoding cannot fail the step.
