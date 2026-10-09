
## 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings — Python 3 is a test-time runtime; 102 pulse-app test files
**Section:** §4 Unit Test Strategy → Framework (Rust crates) · §4 → Conventions → Test file location (Rust) · §9 CI Integration → the `lint-test` row
**Change:**
- §4 Framework: one unit binary now shells out to a non-Rust runtime. `pulse-app/tests/unit_l4_grammar.rs` runs the vendored stdlib Python 3 converter `pulse-app/vendor/llama-cpp/json_schema_to_grammar.py` (llama.cpp b9305, MIT) through the first `python3` / `python` that prints `Python 3`, and asserts the committed `l4-output.gbnf` equals its output. A missing interpreter FAILS the test; it never skips.
- §4 Test file location: was "101 top-level `pulse-app/tests/*.rs` files"; now 102, the new one being `unit_l4_grammar.rs`.
- §9 `lint-test`: the workspace tests need a Python 3 interpreter on all three runners, through the runner's own interpreter with no setup step. Green on ubuntu-22.04, macos-latest and windows-latest (`ci#37327846820`).
**Why:** The founder ruled that the GBNF test performs the conversion, and the overseer (founder-delegated) named the green CI run on all three runners as the proof that Python is present.
**Ref:** .andromeda/runs/2026-10-05T15-18-57Z-wrap/
