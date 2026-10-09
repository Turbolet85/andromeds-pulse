
## 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings — §Stack gains the test-only L4 grammar conversion check
**Section:** §Stack and Technologies (a new row after AI/ML serving)
**Change:** A new row "L4 grammar conversion check (test-only)": llama.cpp b9305 (`63248fc3`) `examples/json_schema_to_grammar.py`, vendored byte-identical at `pulse-app/vendor/llama-cpp/` (MIT, its `LICENSE` beside it; stdlib-only Python 3), plus a Python 3 interpreter (`python3` or `python`) on every host that runs the workspace tests. `pulse-app/tests/unit_l4_grammar.rs` runs it on `L4_OUTPUT_JSON_SCHEMA` and asserts the committed `L4_OUTPUT_GBNF` equals its output; a missing interpreter fails the test and never skips it. It is never in the shipped binary and sits outside cargo-deny's view.
**Why:** The P4 fork answered by the overseer (founder-delegated): the founder's ruling requires the test to PERFORM the conversion, so the converter is vendored and Python becomes a test-time runtime.
**Ref:** .andromeda/runs/2026-10-05T15-18-57Z-wrap/
