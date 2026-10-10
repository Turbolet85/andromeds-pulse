# Curation — 2026-10-10-capability-record-re-based

```
CLAUDE.md ecosystem curated:
  Tier 2 (.claude/rules/*):                   + testing.md: "SWEEP HAZARD — an unquoted YAML scalar ends at ` #`; a probe asserting a parsed step name states the text up to the hash; an atom that prints only after the chunk's edit cannot be vouched for by a baseline on the untouched tree"
  Tier 3 (.claude/docs/session-learnings.md): + "A hand-copied evidence line is checked against its log before the record closes"
  Filters: 1 dup · 1 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  Extended: T2/security.md: "2026-06-11 (session 183): CORRECTION to the §Tauri capability gating body block …" + "the gate's input moved to docs/capability-record.json; the function is read by name, its cited line no longer its line"
  CLAUDE.md size: 154/200 · T1 6.8 KB, 0 over 600 B
```

**Tier 2 — testing.md.**
Proof: the plan's entry-5 name atom failed on implement's first firing (`contains … (chunk #99 — P-001..P-060 scenario
mapping)` ✗) while the entry's other atoms held; `yaml.safe_load` of `ci.yml` at `279a477e` gave the step the name
`cargo xtask verify:capability-matrix (chunk`, and three step names of the base workflow end at `(chunk`
(`evidence/operator-edit-plan.md`, correction 1). The P5 baseline had read `0 True True True -` with an empty name
line, so it could not show the atom's text.

**Extension — security.md, the 2026-06-11 Session Addition.**
Proof: the cascade sweep's curation rows at `.claude/rules/security.md:160` (`verb` · `matrix-name` · `sixty-range`,
`cascade-dispositions.md`) and the citation sweep's `changed` row for the same line (`xtask/src/main.rs:813 → ?`,
`citation-dispositions.md`); `capability_widening_check` stands at another line of that file today.

**Tier 3 — session-learnings.md.**
Proof: `evidence/operator-pass.md` first carried `mcp-server tests … · 2026-10-10T12:05:54Z` where entry 25's log
reads `2026-10-10T11:55:54Z`; a loop over the log's six lines with a fixed-string grep found it, and the record was
corrected before it closed (six of six lines then matched, and the five entry lines).

**Filtered.**
- dup: "a route-resolve that renames an entry the record names reddens the capability gate" — stated by the amended
  test-plan paragraph and architecture's registration, and re-derived into `rules/testing.md`'s body this pass.
- task-specific: "one rule for the guard word" (a retired id is `part` when any automated proof its ref names
  runs) — a ruling about this record, carried by the record's legend and the plan.

**Not candidates.** The Bash guard refused a `cat` heredoc with a file target once and a leading `cd` into a
subdirectory twice in the session; each was re-issued once in the guard's form, so the two host-rule entries did
not recur.

**Surfaced, not corrected** (as at the previous wraps): three stale line citations in preserve-verbatim homes that
the citation sweep listed again — `.claude/rules/frontend.md:99` (`xtask/src/main.rs:550`, now `:551`) and
`.claude/rules/observability.md:148` twice (`pulse-app/src/heartbeat.rs:1078` and `:1934`, a file of 424 lines). The
fourth, `.claude/rules/security.md:160`, is settled by the extension above.
