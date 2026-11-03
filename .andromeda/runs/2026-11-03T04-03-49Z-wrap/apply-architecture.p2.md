# Apply — architecture · part 2 of 2

## Verdicts
- fanout-architecture.md 16 · applied · E1
- fanout-architecture.md 17 · folded into fanout-architecture.md 16
- fanout-architecture.md 29 · applied · E2
- fanout-architecture.md 30 · applied · E3

## Edits
@fact id=fct-z32zdd7c expect_rev=1
old: `boot` (the mcp-feature release build → `Build the exit witness`
new: `boot` (the release build `cargo build --workspace --release` → `Build the exit witness`
old: `boot`'s `cargo build --workspace --release --features mcp-server`, pinned by the self-lint test `ci_workflow_keeps_the_linux_release_build_witnesses`;
new: `boot`'s `cargo build --workspace --release`, pinned by the self-lint test `ci_workflow_keeps_the_linux_release_build_witnesses`, and no step names `--features mcp-server` and no job is `mcp-test`, pinned by `ci_workflow_holds_no_mcp_test_job_and_no_mcp_server_feature`;
old: as six independent jobs, each on
new: as five independent jobs (the count pinned by `data_dir_export_precedes_every_consumer`, `("ci.yml", 5)`), each on
old: `logs-perf-samples-{os}` upload) · `mcp-test` (nextest with `--features mcp-server`) · `a11y`
new: `logs-perf-samples-{os}` upload) · `a11y`
old: `mcp-test` and `a11y` restore `lint-test-{os}` read-only
new: `a11y` restores `lint-test-{os}` read-only

@fact id=fct-35cgfrz4 expect_rev=1
old: Network ports, the door). Plus one optional `andromeda-pulse-mcp` sidecar process (only when `--features mcp-server` is enabled at build time and `ANDROMEDA_PULSE_MCP_ENABLED=true` at runtime).
new: Network ports, the door).

@fact id=fct-ahat5ytg expect_rev=1
old: linked by pulse-app in every build; the stdio sidecar binary and the rmcp anchor stay feature-gated
new: linked by pulse-app in every build and served by the door; no binary, no feature

## Patterns
[[pattern]]
id = "arch2-01"
fixed = 'the mcp-feature release build'

[[pattern]]
id = "arch2-02"
fixed = 'cargo build --workspace --release --features mcp-server'

[[pattern]]
id = "arch2-03"
fixed = 'six independent jobs'

[[pattern]]
id = "arch2-04"
fixed = '(nextest with `--features mcp-server`)'

[[pattern]]
id = "arch2-05"
fixed = '`mcp-test` and `a11y` restore'

[[pattern]]
id = "arch2-06"
fixed = 'optional `andromeda-pulse-mcp` sidecar process'

[[pattern]]
id = "arch2-07"
fixed = 'the stdio sidecar binary and the rmcp anchor stay feature-gated'
