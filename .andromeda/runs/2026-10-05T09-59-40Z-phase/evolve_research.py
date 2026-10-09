import json
TS = "2026-10-05T10:11:53Z"
M = "2026-10-05-l4-model-chosen-by-pattern-discrimination"
base = {"v": 1, "ts": TS, "version": "andromeda-pulse-0.3.0", "epoch": "Epoch 4 — Polish & ship: verification",
        "chunk": M, "skill": "andromeda-phase", "step": "research"}
rs = [dict(base, kind="step", id=TS + "-a", outcome="ok",
    counts={"reformulations": 0, "extra_reads": 2},
    consumed=[
        {"artifact": "extracts", "quality": "ok", "note": "arch/tests/obs/security signals all resolved at the named files; tests extract's 29-pin and test=true claims re-derived true"},
        {"artifact": "cookbook", "quality": "ok", "note": "canonical IMPACT query copy-adapted with an IN list"},
        {"artifact": "tree-db", "quality": "ok", "note": "1 query, rust plane, 67 rows, no regenerate"},
        {"artifact": "source", "quality": "ok"}],
    produced=[
        {"artifact": "research", "signals": ["unresolved-questions"], "note": "2 plan-decision questions (valid time budget; lightest ordering) + 1 founder-facing fact (cueless surface creates no incident)"},
        {"artifact": "scope", "signals": ["premise-verified"], "note": "6 [inferred] premises closed: 5 verified, 1 resolved at P4; CI re-read added a red fold"}],
    problem=[{"nature": "environment", "solution": "deferred", "note": "the red boot-smoke job log could not be read: gh api returned escape-sequence refusal and gh run view --log-failed refuses while run ci#37293411947 is in progress; the read is deferred to P4/P5 when the run completes"}])]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
