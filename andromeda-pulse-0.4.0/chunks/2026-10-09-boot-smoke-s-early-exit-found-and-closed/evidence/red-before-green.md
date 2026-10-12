# The pins, read red before the code existed (plan step 1)

Both test targets were run on the tree holding the pins and nothing they pin: `xtask/src/harness_ready.rs` held
its test module alone, and `.github/workflows/ci.yml` was at the chunk base. The raw outputs are not kept; the
figures below were read from them.

## `cargo test -p xtask harness_ready` — 2026-10-09T19:10:49Z

- Exit 101. `error: could not compile xtask (bin "xtask" test) due to 90 previous errors`.
- 90 errors: 67 `E0425`, 21 `E0433`, 2 `E0422`. No test ran.
- The functions the pins call, each absent (`E0425: cannot find function`), with the count of call sites:
  `decide_ready` 8 · `decide_settled` 7 · `exit_status` 6 · `resolve_port` 6 · `parse_display` 8 ·
  `parse_session_bus` 8 · `probe_label` 5 · `timeout_supports_verdict` 3 · `log_evidence` 2 ·
  `receiver_accepts` 2 · `receiver_label` 2 · `settled_payload` 1 · `socket_label` 1.
- So the red here is a compile red: the per-arm pins could not fail on a value, because the decision they pin did
  not exist.

## `cargo test -p pulse-app --test quality_gate_workflow` — 2026-10-09T19:10:56Z

- Exit 101. `test result: FAILED. 22 passed; 3 failed`.
- Red, three of the four new pins:
  - `ci_workflow_boot_smoke_reads_the_app_past_its_settle_inside_one_display` — the step holds no line with
    `cargo xtask harness:settled` (0 lines, 1 expected).
  - `ci_workflow_boot_smoke_runs_cleanup_whatever_settled_and_status_returned` — the same first assertion.
  - `ci_workflow_boot_smoke_keeps_the_display_server_output_in_the_logs_artifact` — the `xvfb-run` line holds no
    `-e "$ANDROMEDA_PULSE_DATA_DIR/logs/xvfb.log"`.
- **Green on arrival, one of the four:** `ci_workflow_boot_smoke_carries_no_soft_fail`. The step at the base
  carries no soft-fail key, so this pin cannot read red before the edit. It is a regression guard, and its
  discrimination is shown by a mutation instead (see `mutation-soft-fail.md`).
- The 21 tests that were in the file before this chunk all passed.
