import json

ts = "2026-10-01T08:34:10Z"
base = {
    "v": 1,
    "ts": ts,
    "version": "andromeda-pulse-0.3.0",
    "epoch": "Epoch 4 — Polish & ship: verification",
    "chunk": "2026-09-30-span-level-redaction",
    "skill": "andromeda-implement",
    "step": "fix-loop",
}
rs = [
    dict(base, id=ts + "-a", kind="step", outcome="ok",
         counts={"iterations": 1, "retries": 1, "soft_exit": 0, "deferred": 0},
         consumed=[
             {"artifact": "plan", "quality": "ok", "note": "all 25 entries ran as written; 15 green, 6 env/live + 4 operator not run by design"},
             {"artifact": "research", "quality": "ok"},
             {"artifact": "tests", "quality": "ok"},
             {"artifact": "source", "quality": "ok"},
         ],
         produced=[
             {"artifact": "source", "signals": ["mutation-discriminates"],
              "note": "m1 12 red, m2 7 red, m3 26 red, extra m4 (no fixpoint) 2 red incl. a proptest shrinking to the fused card+password case"},
             {"artifact": "matrix", "signals": [], "note": "0 capabilities claimed by this chunk; no ledger write"},
             {"artifact": "implement-outcome", "signals": ["green"]},
         ],
         problem=[
             {"nature": "environment", "solution": "workaround",
              "note": "this session's rust-analyzer flycheck respawned 6 times during gates/mutations and took the target lock; each respawn stopped by PID per the operator's standing word"},
         ]),
    dict(base, id=ts + "-b", kind="friction", type="contract.vacuous-check-found",
         what="plan predicted m1 (keyed extent narrowed to the match) would redden the e2e keyed column, but the e2e keyed canary ended its line, so exact-match and to-end-of-line extents stored identical text; strengthened the value with words after the keyed canary, then m1 reddened it",
         impact={"iterations": 1}, artifacts=["pulse-app/tests/e2e_pii_span_masking_stored_field.rs"]),
    dict(base, id=ts + "-c", kind="friction", type=None, untyped=True,
         what="mutation runs made proptest write crates/security/proptest-regressions/scrubber.txt with a provider-key canary in a seed comment; kept for replay (out-of-list mechanical) and annotated gitleaks:allow so CI secret-scan does not flag it",
         impact={"extra_reads": 2}, artifacts=["crates/security/proptest-regressions/scrubber.txt"]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
