
## 2026-10-10-capability-record-re-based — the capability record's gate registered
**Section:** §Occupied Resources → xtask CLI surfaces (dev/CI gates)
**Change:** The bullet now registers `cargo xtask verify:capability-matrix`, which it never held. Module
`xtask/src/capability_record.rs` (`evaluate` reads both files, `judge` is the pure verdict; one caller, the verb in
`xtask/src/main.rs`). Inputs: two fixed in-repo paths held as constants, `docs/capability-record.json` and
`andromeda-pulse-0.4.0/working-route.md`; no path argument, no environment variable. Contract: exit 0 clean · 1
findings · 2 cannot-evaluate (the record absent, unreadable, not JSON or without a `capabilities` array; the route
absent or unreadable); one line `verify:capability-matrix: {clean|violations} ({n} ids: {c} claimed, {r} retired,
{k} violation(s))` or `verify:capability-matrix: cannot-evaluate ({reason})`; one JSON event line (target
`xtask.verify_capability_matrix`) and the report twin `target/capability-matrix/report.json` (`state` ·
`capability_count` · `claimed_count` · `retired_count` · `violation_count` · `reason`; the twin adds `violations`
and `generated_at`). The findings are listed in the body: the id set (exactly P-001…P-082 once each), the two
dispositions, the legend's three closed sets, a claimed entry's carrying requirement (P-083…P-129) and proofs, a
retired entry's surfaces, removing titles, kept-half owner and guard, and a record with no claimed id. A route
entry's title is its line without a leading `[marker] ` stamp, up to the first ` — `. It validates no proof of a
retired id and runs in ci.yml's `lint-test` after `capability-widening-check`. Was: unregistered; the verb read
`docs/v0_2_0/capability-verification-matrix.json` (P-001…P-060), exit 0 or 1, an absent file falling into the
generic error exit.
**Why:** The chunk re-pointed the verb at the one current record and gave it the registry's exit form; a gate the CI
runs was absent from the section that lists every other formalized xtask contract. Standing consequence for later
chunks: the verb reads the working route, so a route-resolve that renames, retires or splits an entry the record
names reddens it until the record is corrected in that wrap's chunk.
**Ref:** .andromeda/runs/2026-10-10T12-20-47Z-wrap/
