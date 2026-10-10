
## 2026-10-10-capability-record-re-based — the capability gate reads the one record
**Section:** §9 CI Integration → Capability verification matrix (CI step, runs after capability-drift)
**Change:** The paragraph now says: `cargo xtask verify:capability-matrix` reads two fixed in-repo files, no path
argument and no environment variable — `docs/capability-record.json`, all 82 ids P-001–P-082, each `claimed` or
`retired` (36 claimed, 46 retired, as measured at this chunk), and `andromeda-pulse-0.4.0/working-route.md` — and
exits 0 clean · 1 findings · 2 cannot-evaluate. It holds all 82 ids to the form and validates proofs of claimed ids
only: `carried_by` in P-083…P-129 (empty only with a `note`), at least one scenario that is not `source-evidence`,
eight scenario kinds (five with a file ref that must exist and its `contains` anchor found; `by-construction` ·
`external` · `manual` needing the entry's `note`), nine verification modes (the old seven plus `dynamic-external`
and `manual`). A retired entry carries no scenario, mode or `carried_by`; it names `surfaces` (six closed words),
`removed_by` (working-route entry titles the route carries) and `guard` (`runs` · `part` · `none`, with `by` and
`unrun`); no proof file of a retired entry is checked. The id set is closed in `xtask/src/capability_record.rs`, so
a change of it lands in the module and the record in one chunk. Was: the gate validates
`docs/v0_2_0/capability-verification-matrix.json`, all 60 ids P-001–P-060, four modes named with P-040 as the
by-construction example, exit on dangling ids, paths or anchors, and ids from P-061 on extend the matrix JSON. The
`xtask-gate` scenario kind is no longer known to the gate. Both older records are superseded and read by no gate.
**Why:** The chunk made the old gate read the one current record, which gives each of the 82 ids one of two
dispositions; the paragraph's input, id range, exits and extension rule were each measured false. Standing trap: the
gate reads the working route, so a route-resolve that renames, retires or splits an entry the record names reddens
the verb and the pin `the_committed_record_reads_clean_over_the_committed_route` until the record is corrected.
**Ref:** .andromeda/runs/2026-10-10T12-20-47Z-wrap/
