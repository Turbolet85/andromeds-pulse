# Operator pass — 2026-09-30-span-level-redaction

Fired on the overseer's word (2026-10-01), by /implement, in plan order.

- **Entry 22** — `python -X utf8 {tools_dir}/gate.py hygiene` → exit 0 ·
  `hygiene: clean — read 39 (runs 32 · evidence 7) · trails 13 not read · binary 0 not read by P1`
  (expect `exit 0` + `contains hygiene: clean` → held).
- LSP flycheck census before pre-push: no flycheck cargo tree running (tree size 0); rust-analyzer left alone.
- **Entry 23** — `cargo xtask pre-push:linux` → exit 0 · verdict document `"verdict": "green"`,
  tree `a78e822f2e530ba4f729aaa88c28d07c91fe82cf`; stages script-modes (92 ms) · npm (18120 ms) ·
  clippy (13015 ms) · test (75634 ms) · ci-gates (1724 ms), all `ok: true`
  (expect `exit 0` + `contains "verdict": "green"` → held).
- **Pre-CI commit** — `9d14166 chore(2026-09-30-span-level-redaction): operator pre-CI commit`
  (66 files; tree clean after; hygiene re-read clean with this file included: read 40).
- **Entry 24** — clean-tree guard held; `git push origin chore/migrate-pulse-to-v3` → exit 0,
  `7bb56ea..9d14166`; HEAD = origin.
- **Entry 25** — `ci.py conclusion --sha HEAD --wait 2400` → exit 0 · `9d141665f3bb verdict: red` ·
  first fail +127 s `lint / test (ubuntu-22.04)` (expect `contains verdict: green` → **failed**).
  Final run `ci#36842111417`: **completed / failure**; `secret-scan#36842111445` success.
  - failure: `lint / test` on ubuntu-22.04, macos-latest, windows-latest — all one cause: the
    ci.yml "no Cyrillic in source" step flags `crates/security/src/scrubber.rs:745-746`
    (`span_mask_handles_multibyte_text_around_a_secret`, Russian sample text). On Windows the step
    found the same lines and then crashed printing them (`UnicodeEncodeError` under cp1252), so that
    job shows only `exit code 1`.
  - success: supply-chain · mcp-server tests · coverage gate · a11y ×3 · release build macOS/Windows ·
    boot smoke.
  - `pre-push:linux` stayed green because it does not run the ci.yml inline Cyrillic step.

## Fix round (overseer's word)
- `span_mask_handles_multibyte_text_around_a_secret` sample → `échec für bob@example.com — réessai`
  (`é`/`ü` 2-byte, `—` 3-byte). A local scan with the CI step's own rule (U+0400–U+04FF over
  crates · pulse-app/src · pulse-app/tests · pulse-app/ui/src · xtask/src, `.rs`/`.ts`/`.tsx`) → 0 hits.
- Gates 1 fmt + 2 clippy green. Gate 3 first fired into `timeout` at its 1800 s bound: the recompile
  ran alongside another session's nextest build on the host; the bound's SIGTERM surfaced as
  `rust-lld … exit code: 143`, and the entry's `cargo nextest` tree outlived its shell and was stopped by
  PID (8 processes; the other session's tree untouched). After an unbounded throttled
  `cargo build --workspace --tests` (3 m 35 s), gate 3 re-fired green: 46/46.
- Bindings byte-identical to the base (no default-features bindings emission ran).
- `cargo xtask pre-push:linux` → exit 0, `"verdict": "green"`, tree
  `65385c91092393411776803710f88f72ffb9f1b3`; script-modes · npm · clippy · test · ci-gates all ok.
- Hygiene clean (read 2) → commit `7949d81 chore(2026-09-30-span-level-redaction): operator pre-CI commit`
  (the project's fix-push form, as `d708ad7` → `c6eb395`; the fix is named in the body).
- Entry 24 — clean-tree guard held; push exit 0, `9d14166..7949d81`; HEAD = origin.
- Entry 25 — `ci.py conclusion --sha HEAD --wait 2400` → exit 0 · **`7949d8173ea0 verdict: green`** ·
  checks 13/13 · wall 1680 s · `ci#36851508616` completed/success · `secret-scan#36851508610`
  completed/success (expect `exit 0` + `contains verdict: green` → held).
- For the wrap's route-resolve (overseer's word): (1) the ci.yml Cyrillic step crashes with
  `UnicodeEncodeError` (cp1252) on the Windows runner whenever it has hits to print, so there it can only
  fail as a bare `exit code 1`; (2) `cargo xtask pre-push:linux` does not mirror that inline lint.
