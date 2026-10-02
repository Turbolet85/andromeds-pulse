import json

ts = "2026-10-01T11:24:33Z"
base = {
    "v": 1, "ts": ts, "version": "andromeda-pulse-0.3.0",
    "epoch": "Epoch 4 — Polish & ship: verification",
    "chunk": "2026-09-30-span-level-redaction", "skill": "andromeda-wrap-session", "step": "report",
}
rs = [
    dict(base, id=ts + "-a", kind="step", outcome="ok", counts={"extra_reads": 7},
         consumed=[
             {"artifact": "conversation", "quality": "ok"},
             {"artifact": "implement-outcome", "quality": "thin", "note": "superseded by two later steps: the training-export residual fix and the operator pass (CI red on 9d14166, fix round, green on 7949d81)"},
             {"artifact": "operator-directive", "quality": "ok", "note": "wrap directive: route the two CI-lint observations as a PREREQ; quote the founder word in widening amendments; red run stays in the report"},
             {"artifact": "git-state", "quality": "ok"},
             {"artifact": "plan", "quality": "ok"},
         ],
         produced=[{"artifact": "report", "signals": [], "note": "all Changes families from session knowledge; expected-amendment sites located by per-master grep before authoring; security suite count re-measured (85)"}],
         problem=None),
    dict(base, id=ts + "-b", kind="friction", type="input.implement-outcome-unsettled",
         what="implement's P4 green report was followed by an operator-directed residual fix (training-export interpretation) and an operator pass whose first CI run went red on the chunk's own Cyrillic test literal; the outcome basis became the operator pass's final HEAD 7949d81 + ci#36851508616",
         impact={"iterations": 2}),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
