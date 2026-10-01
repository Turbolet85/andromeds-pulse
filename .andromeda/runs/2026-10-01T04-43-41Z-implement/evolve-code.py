import json

ts = "2026-10-01T07:00:17Z"
base = {
    "v": 1,
    "ts": ts,
    "version": "andromeda-pulse-0.3.0",
    "epoch": "Epoch 4 — Polish & ship: verification",
    "chunk": "2026-09-30-span-level-redaction",
    "skill": "andromeda-implement",
    "step": "code",
}
rs = [
    dict(base, id=ts + "-a", kind="step", outcome="ok",
         counts={"extra_reads": 4, "soft_exit": 0},
         consumed=[
             {"artifact": "plan", "quality": "ok", "note": "Steps 1-7 executable as written; idempotence mechanism under-specified (see friction)"},
             {"artifact": "research", "quality": "ok", "note": "both lists exact: 15 modify + 2 new, 0 out-of-list edits"},
             {"artifact": "scope", "quality": "ok"},
             {"artifact": "source", "quality": "ok"},
         ],
         produced=[{"artifact": "source", "signals": ["style-inferred", "red-before-green-witnessed"],
                    "note": "the three Step 1 pins ran RED at base exactly as predicted before any production edit"}],
         problem=[
             {"nature": "environment", "solution": "overridden",
              "note": "step-0 cargo clean stalled ~45 min while this session's rust-analyzer flycheck compiled into target/debug; operator chose to stop the flycheck cargo tree by PID (45 procs), then twice more on respawn after source edits"},
             {"nature": "process", "solution": "workaround",
              "note": "bash-guard hook blocked a python heredoc edit carrying a doubled backslash; used the Edit tool instead"},
             {"nature": "product-logic", "solution": "removed-cause",
              "note": "plan's idempotence rested on merge + no-placeholder-match; a digit run fused to a key word becomes card-detectable once the key is masked, so mask_secret_spans re-masks to a fixpoint (bounded 4 passes)"},
         ]),
    dict(base, id=ts + "-b", kind="friction", type="tooling.hook-friction",
         what="PostToolUse rustfmt reformatted a scratchpad test block (dedented the fns), so the anchored insert script's tail assertion fired after writing; recovered with cargo fmt -p security",
         impact={"retries": 1}, artifacts=["crates/security/src/scrubber.rs"]),
    dict(base, id=ts + "-c", kind="friction", type="input.plan-step-ambiguous",
         what="Step 2 (g) idempotence claimed by merge + no-placeholder-match; a fused-boundary case (16 Luhn digits directly before 'password=') breaks single-pass idempotence; resolved in-impl with a bounded fixpoint and a pin",
         impact={"reformulations": 1}, artifacts=["crates/security/src/scrubber.rs"]),
    dict(base, id=ts + "-d", kind="friction", type=None, untyped=True,
         what="this session's own rust-analyzer LSP flycheck (cargo check --workspace --all-targets) runs concurrently with the skill's cargo clean/builds in the same target dir and respawns after every source edit; the clean stalled ~45 min with D: free flat until the tree was stopped",
         impact={"dialogue_rounds": 1, "retries": 2}),
]
print("\n".join(json.dumps(r, ensure_ascii=False) for r in rs))
