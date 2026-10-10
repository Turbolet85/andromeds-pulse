
- 2026-10-10: SWEEP HAZARD — a GitHub Actions job log echoes each step's script before its output, so a token count over fetched job logs counts the script's own `echo` and comment lines beside what the step printed; a probe that must read 0 for a retired token needs the token gone from the workflow's script text too, and its known-positive control counts both kinds of line.

- 2026-10-10: SWEEP HAZARD — `cargo xtask quarantine-tracking` greps line-anchored for the ignore attribute in every `.rs` file under `crates`, `pulse-app/src`, `pulse-app/tests` and `xtask/src`, so a test fixture written there must not START a source line with that attribute: keep it inside a one-line string literal, or the check flags the test's own file.
