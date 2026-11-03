# Apply — security-plan

## Verdicts
- fanout-security-plan.md 1 · applied · E1
- fanout-security-plan.md 2 · applied · E2
- fanout-security-plan.md 3 · applied · E3
- fanout-security-plan.md 4 · applied · E4
- fanout-security-plan.md 5 · applied · E5
- fanout-security-plan.md 6 · applied · E6
- fanout-security-plan.md 7 · applied · E7
- fanout-security-plan.md 8 · applied · E8
- fanout-security-plan.md 9 · applied · E9
- fanout-security-plan.md 10 · applied · E10
- fanout-security-plan.md 11 · applied · E11
- fanout-security-plan.md 12 · applied · E12
- fanout-security-plan.md 13 · applied · E13
- fanout-security-plan.md 14 · applied · E14

## Edits
@fact id=fct-tg5z4vbe expect_rev=2
old: involve crates not in this project's dependency graph but document the same symlink-chain / TOCTOU traversal class that `tauri-plugin-fs`-style file APIs and any user-overridable path env-var construct are equally exposed to.
new: document the same symlink-chain / TOCTOU traversal class that `tauri-plugin-fs`-style file APIs and any user-overridable path env-var construct are equally exposed to; RustFS is not in this project's dependency graph, while the zip crate is — zip 9.3.0, no zip below 9 in `Cargo.lock` (as measured at chunk `2026-11-03-advisories-since-2026-10-12-closed`, its gate `grep -A1 '^name = "zip"$' Cargo.lock`).

@fact id=fct-3ey2oony expect_rev=1
old: Current state (chunk `2026-09-29-p-025-hue-shift-observable-made-gradable`): exit 0 `green-with-dispositions` — 2 advisory + 2 license exceptions; both advisory exceptions are extract-zip, and neither has a fixed release anywhere: GHSA-jmr9-qjv8-65gv and GHSA-7pqw-9j4j-h8q3 (vulnerable range `*`, 2.0.1 is the latest release; npm's remedy is a pa11y-ci downgrade, rejected; reached via the wdio 9 and pa11y-ci-nested-pa11y-9 puppeteer chains, dev-only) — GHSA-7pqw is the one residual accepted at that chunk.
new: Current state: exit 0 `green-with-dispositions`, `unexcepted: []` — 5 advisory + 2 license exceptions, as measured at chunk `2026-11-03-advisories-since-2026-10-12-closed` (its gate `cargo xtask check:npm-supply-chain`). Two of the advisory exceptions are extract-zip, and neither has a fixed release anywhere: GHSA-jmr9-qjv8-65gv and GHSA-7pqw-9j4j-h8q3 (vulnerable range `*`, 2.0.1 is the latest release; npm's remedy is a pa11y-ci downgrade, rejected; reached via the wdio 9 and pa11y-ci-nested-pa11y-9 puppeteer chains, dev-only). The other three are GHSA-g3hv-75wg-m249 (minimatch), GHSA-3g3m-2fjr-6pw4 (glob) and GHSA-mrhx-c7xc-m3w5 (zod), each with its reason, owner and a closing condition ending «prune when the gate runs green without this entry».
old: was PRUNED there because
new: was PRUNED at chunk `2026-09-29-p-025-hue-shift-observable-made-gradable` because

@fact id=fct-kmsyqgta expect_rev=2
old: **PROVISIONAL — a boundary widening, one item:** the endpoint, the owner check, the reach of the nine tools and of the one write, the write-switch variable and the workflow step that runs a client as a second account are the operator's answers (the pc overseer: at that chunk's P4 dialog, 2026-10-10, and the operator's reading of 2026-10-11 at its wrap); the founder's own word is owed on the surface itself, and nothing of it is ratified. Since chunk `2026-10-12-stdio-agent-reaches-the-door` the same item also covers the console program's third word `door` (the stdio relay) and the step's second arm: the operator's answers at that chunk's P4 (the pc overseer, 2026-10-12) and at its operator pass, and the operator's answer as the founder's delegate at its wrap; the founder's own word is still owed. Every other
new: **A boundary widening, one item, ratified:** the endpoint, the owner check, the reach of the nine tools and of the one write, the write-switch variable, the workflow step that runs a client as a second account, the console program's third word `door` (the stdio relay) and the step's second arm. Every other

@fact id=fct-6geshnvr expect_rev=2
old: (the next row; PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector);
new: (the next row);

@fact id=fct-gn744k3z expect_rev=1
old: compiled out off Unix). PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector |
new: compiled out off Unix) |

@fact id=fct-lwwyrpa6 expect_rev=1
old: **`door` is not covered by that reading: it is a boundary widening, PROVISIONAL with the door's one item** (the operator's answers at that chunk's P4 and wrap; §Threat Model Summary → Attack surface, the door's vector).
new: **`door` is not covered by that reading: it is a boundary widening, part of the door's one item, ratified** (§Threat Model Summary → Attack surface, the door's vector).

@fact id=fct-ty7icrgc expect_rev=2
old: Env-layer only. PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector. |
new: Env-layer only. |

@fact id=fct-j4oj6ftt expect_rev=2
old: `door`; PROVISIONAL — the door's vector).
new: `door`).
old: the-door`, PROVISIONAL with the door's item) — any other
new: the-door`) — any other
old: (§Input Validation; PROVISIONAL — the door's vector above).
new: (§Input Validation).

@fact id=fct-inw2zo7r expect_rev=1
old: (§Authentication & Authorization; PROVISIONAL — Attack surface, the door's vector).
new: (§Authentication & Authorization).

@fact id=fct-ashdoovw expect_rev=1
old: records held in no kept artifact. PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector.
new: records held in no kept artifact.

@fact id=fct-xsadt7mu expect_rev=1
old: -process`; PROVISIONAL with the door — §Threat Model Summary → Attack surface, the door's vector):
new: -process`):

@fact id=fct-ixrmtmoe expect_rev=2
old: — classification PROVISIONAL, part of the door's one item (§Threat Model Summary
new: — part of the door's one item, ratified (§Threat Model Summary

@fact id=fct-cvhwrox6 expect_rev=2
old: reading through either (PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector).
new: reading through either.

@fact id=fct-y6svlsyp expect_rev=2
old: only when the write switch is on; PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector).
new: only when the write switch is on).

## Patterns
[[pattern]]
id = "sec-01"
fixed = 'involve crates not in this project'

[[pattern]]
id = "sec-02"
fixed = 'both advisory exceptions are extract-zip'

[[pattern]]
id = "sec-03"
fixed = 'nothing of it is ratified'

[[pattern]]
id = "sec-04"
regex = 'PROVISIONAL[^.;:()]{0,60}the door'
