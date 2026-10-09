import json

ts = "2026-10-01T08:37:54Z"
base = {
    "v": 1,
    "ts": ts,
    "version": "andromeda-pulse-0.3.0",
    "epoch": "Epoch 4 — Polish & ship: verification",
    "chunk": "2026-09-30-span-level-redaction",
    "skill": "andromeda-implement",
    "step": "smoke",
}
rs = [
    dict(base, id=ts + "-a", kind="step", outcome="ok",
         counts={"retries": 0, "deferred": 0},
         consumed=[{"artifact": "plan", "quality": "ok",
                    "note": "no boot-path / UI touchpoint and no smoke or self-verify entry; the P2 live leg booted the warm debug binary instead"}],
         produced=[
             {"artifact": "conversation", "signals": ["skipped", "teardown-exact"],
              "note": "P3 skipped (no boot-path / UI change); P2 live leg: pid 45024 stopped by PID, no pulse-app left, 4317/4318 released, 0 panic, 0 ERROR, redactions_applied 0->4"},
             {"artifact": "implement-outcome", "signals": ["green"]},
         ],
         problem=[
             {"nature": "process", "solution": "workaround",
              "note": "piped one gate.py --only 16 call through grep to shorten its listing, against the run-bare rule; the verdict line and trail were unaffected, later calls ran bare"},
         ]),
    dict(base, id=ts + "-b", kind="friction", type="contract.vacuous-check-found",
         what="plan entry 18's poll break (cat log | grep -q ... && break) can never fire under the gate tool's bash -o pipefail on a ~10 MB log (cat takes SIGPIPE on match), so it always waits the full 60 s; the final file read still graded 4 correctly; measured with the same predicate with and without pipefail",
         impact={"extra_reads": 2},
         artifacts=["andromeda-pulse-0.3.0/chunks/2026-09-30-span-level-redaction/plan.md"],
         evidence="andromeda-pulse-0.3.0/chunks/2026-09-30-span-level-redaction/evidence/live-leg/live-leg.log"),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
