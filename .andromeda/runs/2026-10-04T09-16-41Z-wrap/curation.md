# Curation — 2026-10-04-corpus-key-creation-is-race-free

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A CLEAN-SKIP IS INVISIBLE TO NEXTEST'S COUNTS … `env -i` drops DBUS_SESSION_BUS_ADDRESS" (confidence 1.1)
    Proof: native pre-push stage 5 (`cargo xtask test` under `env -i`) read `2580 tests run: 2580 passed, 0 skipped`
    while a one-shot re-run of the same `race_free` + process-boundary selection under the stage's exact `env -i` with
    `--success-output immediate` printed three `[skip] no OS credential store on this host; …` lines (6 passed); the
    plan's claim that stage 5 exercised the temp-dir arm was falsified (`evidence/operator-pass.md` §Spec claim
    disproved). Signals: verified by measurement +0.4 · specific technical detail +0.2 · the overseer's explicit
    direction to record the finding +0.5.
  Tier 3 (.claude/docs/session-learnings.md): none
  Extended: T2/host-win32.md: "2026-10-01 rust-analyzer flycheck entry" + "on Linux select by comm==cargo and --message-format=json; never pgrep -f / pkill -f on a pattern in your own command line" (confidence 1.0)
    Proof: at /implement, `pkill -TERM -P …` over `pgrep -f 'cargo check --workspace --message-format=json'` killed the
    Bash tool's own shell (exit 144) because the pattern sat in that shell's command line; the `ps -eo pid,comm,args |
    awk '$2=="cargo" && /message-format=json/'` form then stopped the flycheck tree cleanly (stopped pid 2683069 at
    the wrap-prep re-check). Signals: operator directive to stop the flycheck +0.4 · verified by measurement +0.4 ·
    specific technical detail +0.2.
  Filters: 0 dup · 1 task-specific (the skip-arm lock-file residue of `corpus_key_survives_a_real_process_boundary` → route-resolve CARRY) · 0 conflict · 0 deferred
  Rejected at Filter 4 (exactly 0.6): "clippy `incompatible_msrv` checks APIs, not syntax — a green clippy is not evidence the declared floor builds" (measured +0.4 · specific +0.2; its fact rides the route entry "The declared Rust floor matches the code" as CONTEXT, and the masters now state the floor)
  CLAUDE.md size: see the P7 health row
