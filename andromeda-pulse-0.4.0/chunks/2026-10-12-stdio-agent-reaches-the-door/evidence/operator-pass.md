# Operator pass — 2026-10-12-stdio-agent-reaches-the-door

On the operator's word (the pc overseer, the founder's delegate, 2026-10-12; inputs#I5): entry 20, the pre-CI
commit, entries 21, 22 and 23. The step-6 departure (a refused account reads exit 1 with the fixed line; the plan
said exit 0) was accepted as built; the wrap reconciles the plan.

## Entry 20 — hygiene (fired as written, before the pre-CI commit)
```
hygiene: clean — read 68 (runs 61 · evidence 0 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
exit 0.

## Pre-CI commit
`2595b5cc47a9cb51b4e03d904209a12f49fdf125` — `chore(2026-10-12-stdio-agent-reaches-the-door): operator pre-CI commit`.

## Entry 21 — merge-base probe (`gate.py run --operator 21`, no run dir)
```
operator entry 21 · history tripwire: none of 7 forms matched — not a clearance
 21 probe       green · exit 0 · 0.47s · 0 B → 21.log · history: unmoved · m="$(git ls-remote origin refs/heads/main | cut -f1)" && t… (111 chars)
entries 23 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 22
```

## Entry 22 — the push (`gate.py run --operator 22`, no run dir)
```
operator entry 22 · history tripwire: git
 22 probe       green · exit 0 · 2.43s · 135 B → 22.log · history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0 d3e694d2→2595b5cc · git diff --quiet && git diff --cached --quiet && git push … (92 chars)
entries 23 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 22
```
The one ref moved is the intended one.

## Entry 23 — the CI read (fired as written)
```
2595b5cc47a9 verdict: green · checks 7/7 · wall 1628 s · runs secret-scan#38227488989 completed/success ci#38227488889 …
runs: secret-scan#38227488989 pull_request completed/success · ci#38227488889 pull_request completed/success
checks: a11y (ubuntu-22.04) · boot smoke (ubuntu-22.04) · coverage gate (line ≥75% / function ≥85%) · gitleaks
  lint / test (ubuntu-22.04) · mcp-server tests (ubuntu-22.04) · supply-chain (audit + deny + auditable)
```

## The second account, read on the runner
From the `lint / test` job `114738471070` of `ci#38227488889`, step `Door refuses another account` (escape codes and
timestamps stripped):
```
        PASS [   0.513s] (1/2) pulse-app::integration_door another_account_through_the_bridge_receives_nothing
    door bridge other-account arm: nobody as made exit Some(1), 0 byte(s) on stdout
    door bridge other-account arm: root as made exit Some(1), 0 byte(s) on stdout
    door bridge other-account arm: nobody opened exit Some(1), 0 byte(s) on stdout
    door bridge other-account arm: root opened exit Some(1), 0 byte(s) on stdout
    door bridge other-account arm: no byte reached another account through the bridge, the account check refused
        PASS [   0.533s] (2/2) pulse-app::integration_door another_account_is_refused_at_each_of_three_layers
    door other-account arm: readings as [nobody, root]: directory 0700 [Kernel(13), Closed], socket 0600 [Kernel(13), Closed], both opened [Closed, Closed]
    door other-account arm: directory refused, socket mode refused, account check refused
     Summary [   0.533s] 2 tests run: 2 passed, 5 skipped
```
The relay arm passed its assertion of exactly three `door.refused` records (root as made, nobody opened, root
opened); the records themselves are in no kept artifact. The workspace run in the same job: 3186 of 3186 passed.

## Limits
One CI run, attempt 1, on the pre-CI commit; the second account read on one runner; the run's artifacts not
downloaded.
