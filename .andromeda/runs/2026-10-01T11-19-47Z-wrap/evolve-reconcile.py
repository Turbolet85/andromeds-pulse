import json

ts = "2026-10-01T11:35:33Z"
base = {
    "v": 1, "ts": ts, "version": "andromeda-pulse-0.3.0",
    "epoch": "Epoch 4 — Polish & ship: verification",
    "chunk": "2026-09-30-span-level-redaction", "skill": "andromeda-wrap-session", "step": "reconcile",
}
specs = ["architecture", "security-plan", "design-system", "layout-templates", "test-plan", "obs-plan", "a11y-plan"]
rs = [
    dict(base, id=ts + "-a", kind="step", outcome="ok",
         counts={"retries": 0, "dialogue_rounds": 0, "halted": 0},
         consumed=[{"artifact": "report", "quality": "thin", "note": "its expected-amendment line forecast 0 change at security-plan:166, which names the retired actor; the security detector caught it"}]
         + [{"artifact": "spec:" + s, "quality": "ok"} for s in specs]
         + [{"artifact": "drift-base", "quality": "ok"}, {"artifact": "wrap-playbook", "quality": "ok"}],
         produced=[
             {"artifact": "spec:security-plan", "signals": ["escalation-shaped"], "note": "6 amendments; Boundary widening resolved by the recorded P4 ratification"},
             {"artifact": "spec:architecture", "signals": [], "note": "2 amendments"},
             {"artifact": "spec:test-plan", "signals": [], "note": "1 amendment"},
             {"artifact": "spec:obs-plan", "signals": [], "note": "1 amendment"},
             {"artifact": "sidecar:security-plan", "signals": []}, {"artifact": "sidecar:architecture", "signals": []},
             {"artifact": "sidecar:test-plan", "signals": []}, {"artifact": "sidecar:obs-plan", "signals": []},
         ],
         problem=[{"nature": "process", "solution": "overridden",
                   "note": "sidecar-contract says a ratification is named by who ratified, never quoted; the operator's wrap directive asked to quote the founder span-extent word in every widening amendment, so all 4 entries quote it"}]),
    dict(base, id=ts + "-b", kind="friction", type="input.report-insufficient",
         what="report's expected-amendments line said security-plan:166 (At rest corpus) needed 0 change because it states no whole-value claim, but it names scrub_attribute as the corpus-write mechanism the chunk retired; the detector proposed it and it was applied",
         impact={"extra_reads": 1}, artifacts=[".andromeda/security-plan.md:166"]),
    dict(base, id=ts + "-c", kind="friction", type=None, untyped=True,
         what="re-deriving .claude/docs/services/security.md found its module-layout line already false before the pass (lib.rs re-exports scrubber items; measured: lib.rs is only `pub mod scrubber;`); corrected in the re-derive",
         impact={"extra_reads": 1}, artifacts=[".claude/docs/services/security.md"]),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
