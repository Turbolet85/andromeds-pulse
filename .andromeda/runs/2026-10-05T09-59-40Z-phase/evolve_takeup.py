import json
TS = "2026-10-05T10:01:23Z"
M = "2026-10-05-l4-model-chosen-by-pattern-discrimination"
base = {"v": 1, "ts": TS, "version": "andromeda-pulse-0.3.0", "epoch": "Epoch 4 — Polish & ship: verification",
        "chunk": M, "skill": "andromeda-phase", "step": "take-up"}
rs = [dict(base, kind="step", id=TS + "-a", outcome="ok",
    counts={"dialogue_rounds": 0, "halted": 0},
    consumed=[
        {"artifact": "working-entry", "quality": "ok", "note": "2771-char entry, 4 CONTEXT + 2 CARRY, all foldable; design snapshot sha matched the entry's"},
        {"artifact": "handoff", "quality": "ok"},
        {"artifact": "master-route", "quality": "ok"},
        {"artifact": "working-route", "quality": "ok"},
        {"artifact": "ci-verdict", "quality": "ok", "note": "2dc099a in progress 13/13; carried as verdict not yet available"}],
    produced=[
        {"artifact": "scope", "signals": ["scope-inferred", "carry-folded", "verdict-pending"], "note": "6 [inferred] premises for P3; directive coordinate ':170' re-verified as the predecessor entry, not this entry's line"},
        {"artifact": "master-route", "signals": ["pending-append"]},
        {"artifact": "working-route", "signals": ["stamp"]}],
    problem=None)]
sites = [("working-route.md:52", "UNPARSED: separator-boundary token outside the letters' introducer shape: 'PREMISE CORRECTED 2026…'"),
         ("working-route.md:54", "UNPARSED: separator-boundary token outside the letters' introducer shape (two tokens)"),
         ("working-route.md:60", "UNPARSED: separator-boundary token outside the letters' introducer shape: 'MEASURED, not inferred…'"),
         ("working-route.md:100", "INDETERMINATE: after ONE space — freight or prose, no structural split: 'GPU:'"),
         ("working-route.md:116", "UNPARSED: entry line carrying `↓` inside it; read as an ENTRY"),
         ("working-route.md:125", "UNPARSED: entry line carrying `↓` inside it; read as an ENTRY")]
for i, (s, w) in enumerate(sites):
    rs.append(dict(base, kind="friction", id=TS + "-" + "bcdefg"[i], type="contract.grammar-irregularity",
                   what=w, impact={}, artifacts=[s], evidence="route.py cursor at phase Setup (no run-dir trail)"))
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
