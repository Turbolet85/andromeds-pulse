sweep: base: 50216d7e — the previous sweep (2026-10-10T19-50-04Z-wrap) · re-pointed 2 · changed 0 · stretched 0

# Citation dispositions — 2026-10-10-shared-telemetry-test-data

The sweep (`cites.py apply`, cites v1.4, the second window of this wrap) wrote two master citations, digits only:
`.andromeda/architecture.md` `Cargo.lock:1393` → `1394`, and `.andromeda/test-plan.md` `xtask/src/main.rs:362` → `364`.
It printed six rows, none in a master. `held 0`. Each row was read at its citing line; the section it stands in was
read from the file's headings.

moved .claude/rules/frontend.md:99 xtask/src/main.rs:550 → 552 — listed, not written: leaf → curation home — P3
  (line 99 stands under `## Session Additions`, which opens at line 94; a preserve-verbatim home)

out-of-range .claude/rules/observability.md:149@c2263 pulse-app/src/heartbeat.rs:1078 — the file held 435 lines at the base · naked: bound by position → curation home — P3
  (line 149 stands under `## Session Additions`, which opens at line 124. Read by offset, chars 2063-2463: the entry
  itself says the two numbers are lines of `pulse-app/src/observability.rs` as it stood on 2026-08-26, not of any
  file as it stands now; the position binding to `heartbeat.rs` is the tool's, not the sentence's)

out-of-range .claude/rules/observability.md:149@c2272 pulse-app/src/heartbeat.rs:1934 — the file held 435 lines at the base · naked: bound by position → curation home — P3
  (the same line and the same reading as the row above)

moved .claude/rules/security.md:160 xtask/src/main.rs:813 → 815 — listed, not written: leaf → curation home — P3
  (line 160 stands under `## Session Additions`, which opens at line 95; the entry's own 2026-10-10 extension says
  the `:813` it cites is no longer the function's line and that the function is read by its name)

moved .claude/rules/security.md:160 xtask/src/main.rs:813 → 815 — listed, not written: leaf → curation home — P3
  (the second citation of the same number on the same line)

moved .claude/docs/session-learnings.md:1324 Cargo.toml:42 → 43 — listed, not written: leaf → curation home — P3
  (`docs/session-learnings.md` is a preserve-verbatim home whole. Read: the sentence says the `specta` workspace
  dependency is declared at the root `Cargo.toml:42`; in the work tree `specta` stands at line 48, and neither 42
  nor 43 holds it, so the number was stale before this chunk moved the file by one line)
