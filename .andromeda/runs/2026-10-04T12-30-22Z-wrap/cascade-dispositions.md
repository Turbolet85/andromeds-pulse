# Cascade dispositions — 2026-10-04-linux-launch-stays-up-on-nvidia-wayland

Step 2 sweep: `cascade.py sweep --patterns-file cascade-patterns.toml` (cascade v1.1), baseline `ffb62f08` (the parent
of the operator pre-CI commit `2099998c`); every pattern's control fired on the pre-pass masters. Patterns (derived from
all eight body amendments before the first grep): `count92` · `webview2` · `gpuposture` · `xdgsys` · `setvar` ·
`reservedenv` · `trayleaf` · `productread` · `tokiort` · `bareapp`. Phrasings NOT sweepable (0 pre-pass master hits, so
no control): `first statement`, `WEBKIT_DISABLE`, `Error 71`, `env var NAME`, `NAME, which is safe` — checked by hand
over the leaves instead: `grep -rlF` over CLAUDE.md + `.claude/docs` + `.claude/rules` → `WEBKIT` 1 hit
(`.claude/docs/workflow.md:99`, `WEBKIT_DISABLE_COMPOSITING_MODE` for `cargo tauri dev` DevTools — a different
variable, true, no change); `Wayland` 0 · `NVIDIA` 0.

## Rows
| row | disposition |
|---|---|
| `.claude/rules/testing.md:25` count92 leaf | **re-derived** — "92 targets" → "101 files as of 2026-10-04" (test-plan §4 amended) |
| `architecture.md:245` webview2 edited / gpuposture edited ×2 | amended this pass (the reconcile sentence added); the remaining "stay out of product config" is the kept ban — no further change |
| `architecture.md:246` webview2 @c14572 | no change — the `perf:frame-sample` xtask-surface prose (window read via the row's offset context: the leg sets the var on the app child only); true |
| `test-plan.md:326` webview2 | no change — `perf:frame-sample` prose; true |
| `.claude/rules/verification-harness.md:90` webview2 leaf | no change — the same `perf:frame-sample` fact; true |
| `architecture.md:219` xdgsys @c1506 | no change — the corpus-key lock-file row (`cascade.py window --line 219 --at 1400` read: "No `ANDROMEDA_PULSE_*` variable was added; the system `XDG_RUNTIME_DIR` is read") — a true claim about that chunk |
| `security-plan.md:84` xdgsys/reservedenv edited | amended this pass (the CLI-input vector's entry point gained `__NV_DISABLE_EXPLICIT_SYNC`) |
| `security-plan.md:138` xdgsys edited | amended this pass (the CLI / env var row) |
| `security-plan.md:393` xdgsys @c1989 / productread @c1802 | no change — §Security Anti-Patterns → Input, the PATH-var carve-outs (`window --line 393 --at 1700` read: the corpus-key lock-dir carve-out); `__NV_DISABLE_EXPLICIT_SYNC` is not a path var, so the canonicalize-and-confine rule does not reach it |
| `CLAUDE.md:44` xdgsys leaf / productread leaf ×2 | no change — the path-var Critical Warning; not a path var |
| `.claude/rules/security.md:17` xdgsys leaf @c3586 | no change — the same path-var rule |
| `architecture.md:237` setvar new · `security-plan.md:138` setvar new · `test-plan.md:149` setvar new ×2 | this pass's own text |
| `test-plan.md:801` setvar standing | no change — the §11 anti-pattern against `set_var` mid-TEST without cleanup; the chunk's tests honour it (the real `set_var` runs only in re-exec children) |
| `obs-plan.md:132` trayleaf standing | no change — the multi-platform tag row; true |
| `obs-plan.md:537` trayleaf new | this pass's own text |
| `architecture.md:17` / `:48` tokiort | no change — the shared Tokio runtime of the receivers; true |
| `architecture.md:237` / `test-plan.md:149` tokiort new | this pass's own text |
| `obs-plan.md:48` / `:63` tokiort | no change — tokio as an instrumentable stack member; true |
| `.claude/rules/security.md:156` + `.claude/docs/session-learnings.md:1758,1760,1764,1766,1773,1818` tokiort curation | no change — preserve-verbatim curation homes; each states a true runtime-context fact (taurpc needs a runtime in scope; a `.setup` closure has no entered runtime); none is contradicted (the posture apply runs BEFORE the runtime by design) |
| `CLAUDE.md:35` tokiort leaf | **re-derived** — the `pulse-app` module line gained the launch-posture invariant (arch §Occupied Resources amended) |
| `.claude/docs/services/config-watcher.md:21` / `.claude/docs/stack.md:14` tokiort leaf | no change — true runtime facts, outside the amended sections |
| `security-plan.md:447` / `obs-plan.md:535,536` bareapp standing | no change — sibling leaves' no-bare-`app` statements; still true (bare `app` 0 at the worktree, `grep -A1 'by_target.insert('`) |
| `.claude/rules/observability.md:65` / `.claude/docs/obs-summary.md:116` bareapp leaf | no change at those rows; the new leaf's own bullet/paragraph added beside the `app.exit` entries (re-derived below) |

## Step 3 — leaves re-derived (provenance: the four amended masters)
- CLAUDE.md `GENERATED:setup:modules` — the `pulse-app` line (arch §Occupied Resources → Environment variables).
  `GENERATED:setup:warnings` re-read: its path-var bullet is unaffected (not a path var); no new anti-pattern section moved.
- `.claude/docs/security-summary.md` §8 CLI / env vars — the presence-read + product-set system variable and its residual.
- `.claude/docs/obs-summary.md` — the launch render-posture leaf paragraph beside the process-end leaf.
- `.claude/rules/observability.md` — the `app.boot.render.posture` bullet beside the `app.exit` bullet.
- `.claude/rules/testing.md:25` — the pulse-app test-file count.
- `.claude/docs/tests-summary.md` — re-read: its pending-trigger section carries the amendment-born trigger families, never
  test-plan §1's table rows (`discovery-observer-wiring-coverage` / `exit-hook-main-composition-coverage` absent there
  too), so the new row's omission is consistent — no change.
- `.claude/docs/stack.md` / `.claude/docs/conventions.md` (provenance "Extracted from" architecture) — the amended section
  is §Occupied Resources, neither §Stack nor §Conventions; `grep -n 'Environment variables\|__NV\|env var'` → only the
  stack.md L4 binary-distribution note, unaffected — no change.
- Lateral binds: test-plan §3 ↔ obs-plan §3 untouched (no harness/log-format change); a11y ↔ obs schema untouched.
- Judgment bases (`playbook.md`, `drift-base.md`): 0 `base` rows on every pattern.
