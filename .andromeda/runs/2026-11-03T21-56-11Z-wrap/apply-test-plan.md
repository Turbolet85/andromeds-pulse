# Apply — test-plan

## Verdicts
- fanout-test-plan.md 1 · applied · E1
- fanout-test-plan.md 2 · applied · E2
- fanout-test-plan.md 3 · applied · E3
- fanout-test-plan.md 4 · applied · E4
- fanout-test-plan.md 5 · applied · E5
- fanout-test-plan.md 6 · applied · E6

## Edits
@fact id=fct-ahyhfjr6 expect_rev=1
old: 4.1.11 with `@vitest/mocker` 4.1.11 since chunk 2026-09-29-p-025-hue-shift-observable-made-gradable —
new: 4.1.12 with `@vitest/mocker` 4.1.12, as resolved by `pulse-app/ui/package-lock.json` —

@fact id=fct-ylz3qbha expect_rev=1
old: (4.1.11 since chunk 2026-09-29-p-025-hue-shift-observable-made-gradable)
new: (4.1.12, as resolved by `pulse-app/ui/package-lock.json`)

@fact id=fct-uo3y5o44 expect_rev=1
old: 28 packages declare 1.95.0 (wasmtime 48.0.5, cranelift 0.135.5, pulley 48.0.5)
new: 28 packages declare 1.95.0 (a count read on the wasmtime 48.0.5 / cranelift 0.135.5 / pulley 48.0.5 graph and not re-read on the one `Cargo.lock` resolves — wasmtime 48.0.8, cranelift 0.135.8, pulley 48.0.8, requirement `"48.0.8"`)

@fact id=fct-35qjdhiv expect_rev=1
old: lockfile-resolved 48.0.5 as of 2026-10-04;
new: lockfile-resolved 48.0.8;

@fact id=fct-ep45valw expect_rev=2
old: **The door is PROVISIONAL — a boundary widening, one item:** the endpoint, the owner check, the reach of the nine tools and of the one write, the write-switch variable and this workflow step, which runs a client as a second account, are the operator's answers (the pc overseer: at that chunk's P4 dialog, 2026-10-10, and the operator's reading of 2026-10-11 at its wrap); the founder's own word is owed on the surface itself, nothing of it is ratified, and every other statement of the door in this plan rests on this one.
new: **The door is a boundary widening, one item, ratified:** the endpoint, the owner check, the reach of the nine tools and of the one write, the write-switch variable, the stdio relay word `door`, and this workflow step, which runs a client as a second account, its second arm (the relay's) included; every other statement of the door in this plan rests on this one.

@fact id=fct-mqmrzd2o expect_rev=2
old: (§9, which holds the door's PROVISIONAL sentence; the relay word and the step's second arm are PROVISIONAL with it)
new: (§9, which holds the door's item sentence)

## Patterns
[[pattern]]
id = "tp-01"
fixed = '4.1.11 with `@vitest/mocker` 4.1.11'

[[pattern]]
id = "tp-02"
regex = 'vitest`? 4\.x \(4\.1\.11'

[[pattern]]
id = "tp-03"
fixed = '(wasmtime 48.0.5, cranelift 0.135.5, pulley 48.0.5)'

[[pattern]]
id = "tp-04"
fixed = 'lockfile-resolved 48.0.5'

[[pattern]]
id = "tp-05"
fixed = 'The door is PROVISIONAL'

[[pattern]]
id = "tp-06"
fixed = "holds the door's PROVISIONAL sentence"
