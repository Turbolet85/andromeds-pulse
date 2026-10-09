# Operator pass — entries 34 · 35 · 36

Run in this session on the overseer's word (founder-delegated, 2026-10-05: "Go: the operator pass as planned: hygiene
(34), the pre-CI commit and clean-tree push (35), the ci.py CI read (36)").

## 34 — `python -X utf8 {tools_dir}/gate.py hygiene`
`hygiene: clean — read 39 (runs 28 · evidence 6 · inputs 5) · trails 14 not read · copies 3 not read by P1 — 0 host
paths kept · binary 0 not read by P1` (control fired on every predicate first). `expect = ['exit 0', 'contains
hygiene: clean']` holds.

## 35 — the pre-CI commit, then `git diff --quiet && git diff --cached --quiet && git push origin chore/migrate-pulse-to-v3`
- The commit: `git add -A`, then `bc1946d chore(2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings):
  operator pre-CI commit, for the run this chunk's verdict reads` — the whole tree, the predecessor pre-CI commit's
  shape; `git status --short` empty after it.
- The push: the clean-tree guard held, `7663dd9..bc1946d  chore/migrate-pulse-to-v3 -> chore/migrate-pulse-to-v3`,
  exit 0; `git rev-parse HEAD origin/chore/migrate-pulse-to-v3` → both `bc1946de3484a0fcc038174f07c1396ac7e1855d`.

## 36 — `python -X utf8 {tools_dir}/ci.py conclusion --sha HEAD --wait 2400`
```
repo Turbolet85/andromeds-pulse (the push remote `origin`) · polled 56× over 1713 s
bc1946de3484 verdict: green · checks 13/13 · wall 1686 s · runs secret-scan#37327847087 completed/success ci#3732784682…
runs: secret-scan#37327847087 pull_request completed/success · ci#37327846820 pull_request completed/success
```
exit 0; `expect = ['exit 0', 'contains verdict: green']` holds. `gh run view 37327846820 --json jobs`: `lint / test
(ubuntu-22.04)` · `lint / test (macos-latest)` · `lint / test (windows-latest)` all `success` — `unit_l4_grammar` ran
green with each runner's own Python (it fails, never skips, without one): the Python proof (overseer, P4).

The chunk's wrap commit follows this pass; it adds only spec, route, state, handoff and run-dir files, plus this
evidence (no source), so the CI verdict above is read on the final source tree.
