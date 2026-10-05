import json

ts = "2026-10-05T16:17:56Z"
base = {
    "v": 1,
    "ts": ts,
    "version": "andromeda-pulse-0.3.0",
    "epoch": "Epoch 4 — Polish & ship: verification",
    "chunk": "2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings",
    "skill": "andromeda-wrap-session",
    "step": "reconcile",
}
rs = [
    dict(base, kind="step", id=ts + "-a", outcome="halted-resolved",
         counts={"retries": 0, "dialogue_rounds": 1, "halted": 1},
         consumed=[
             {"artifact": "report", "quality": "ok", "note": "every detector ran on the report alone; the expected-amendment site greps let check 5 cover all 11 entries"},
             {"artifact": "spec:architecture", "quality": "ok"}, {"artifact": "spec:security-plan", "quality": "ok"},
             {"artifact": "spec:test-plan", "quality": "ok"}, {"artifact": "spec:obs-plan", "quality": "ok"},
             {"artifact": "spec:design-system", "quality": "ok"}, {"artifact": "spec:layout-templates", "quality": "ok"},
             {"artifact": "spec:a11y-plan", "quality": "ok"},
             {"artifact": "drift-base", "quality": "ok"}, {"artifact": "wrap-playbook", "quality": "ok"}],
         produced=[
             {"artifact": "spec:architecture", "signals": ["escalation-shaped", "cascade-wide"], "note": "10 proposals applied after the Boundary widening escalation resolved on the founder's live word"},
             {"artifact": "spec:security-plan", "signals": ["escalation-shaped"], "note": "5 detector proposals + 1 orchestrator-raised (the argv row and -p bullet, check 5)"},
             {"artifact": "spec:obs-plan", "signals": ["detector-caught-unforeseen"], "note": "D-obs-defect-narrative reopened the section 8 muted backlog, an amendment the plan's expected list did not carry"},
             {"artifact": "sidecar:architecture", "signals": []}, {"artifact": "sidecar:security-plan", "signals": []},
             {"artifact": "sidecar:test-plan", "signals": []}, {"artifact": "sidecar:obs-plan", "signals": []}],
         problem=None),
    dict(base, kind="friction", id=ts + "-b", type="contract.false-positive-proposal",
         what="D-obs-defect-narrative proposals carried basis fields citing source lines the report does not carry (xtask/ci/l4-latency-p99.sh:40, llamacli_inference.rs:974, observability.rs:2131); rejected as written by the re-derivation tell and re-raised by the orchestrator from the report's facts",
         impact={"reformulations": 2}, artifacts=["obs-plan"], evidence=".andromeda/runs/2026-10-05T15-18-57Z-wrap/fanout-results.md"),
    dict(base, kind="friction", id=ts + "-c", type="contract.cascade-miss",
         what="the sweep's two-outside pattern missed CLAUDE.md:31 'the second data-dir escape after ~/Downloads'; a manual leaf grep on data-dir escape found it and the leaf was re-derived",
         impact={"extra_reads": 1}, artifacts=["CLAUDE.md:31"], evidence=".andromeda/runs/2026-10-05T15-18-57Z-wrap/cascade-dispositions.md"),
    dict(base, kind="friction", id=ts + "-d", type="contract.in-pass-correction",
         what="two first writes corrected before commit: an architecture Fault Identity sentence claiming the shipped model's rank-1 reading unmeasured (not a report fact; removed on re-read) and an architecture sidecar Supersedes naming an entry whose claims still stand (moved to a partial retirement in Change on re-read against the contract)",
         impact={"retries": 2}, artifacts=["architecture.md:73", "architecture-amendments.md"]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
