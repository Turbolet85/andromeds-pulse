
## 2026-10-10-capability-record-re-based — the capability record named as a contract another project reads
**Section:** §Standard Contracts
**Change:** A new bullet, "Capability record — the form another project reads". `docs/capability-record.json`
(`schema_version` 1; header `product` · `version` · `as_of` · `supersedes` · `legend`; `capabilities`, 82 entries
P-001…P-082 in id order, each `id` · `title` · `disposition` · optional `note`) is the one current record of the
capability ids. The accepted set is every entry whose `disposition` is the string `claimed`; a `retired` entry says
with what in `surfaces` (closed words `window` · `model` · `desktop` · `workspace` · `training-export` ·
`corpus-encryption`) and by which working-route entry in `removed_by`. Those three fields — `disposition`,
`surfaces`, `removed_by` — are what the external harness (Conductor) derives its accepted set from; a change of
their form is a change another repository reads, and is made known to it. The record is validated by `cargo xtask
verify:capability-matrix` and supersedes `docs/v0_2_0/pulse-capability-spec.md`,
`docs/v0_2_0/capability-verification-matrix.json` and `andromeda-pulse-0.3.0/verification-matrix.json`, which
stand unchanged until the working route retires or re-homes them. Was: no master named the record or any contract
with the external harness's accepted set.
**Why:** The record's form is now relied on outside this repository: Conductor's accepted set is derived from it.
The operator directed, in the wrap directive of 2026-10-10 (the pc overseer, relayed by file), that the record be
named with its three fields where the masters list what an external reader relies on, so that a later change of the
form is heard of there. Standing rule: a chunk that changes the form of `disposition`, `surfaces` or `removed_by`
says so to the operator for Conductor's side before it lands.
**Kept:** The other fields of an entry (`carried_by`, `changed_form`, `provisional`, `scenarios`, `guard`,
`kept_half_owner`) are this project's own and are not part of the contract.
**Ref:** .andromeda/runs/2026-10-10T12-20-47Z-wrap/
