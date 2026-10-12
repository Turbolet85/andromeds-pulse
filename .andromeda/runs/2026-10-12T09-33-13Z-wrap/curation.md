CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "a blank test input uses only bytes the code's own whitespace predicate counts" (confidence 0.8)
                                              + testing.md: "a Unix socket path of 108 bytes or more is made in a test by hard-linking a socket bound at a short path" (confidence 0.8)
                                              + testing.md: "a pin that matches a string constant in Rust source text breaks on a `\` line continuation" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 1 task-specific · 0 conflict · 0 deferred · 1 below threshold
  No-other-home: the three Tier-2 entries (each at 0.6 before the signal; none carried on the route, in a master, a
    playbook rule, a ledger note or a contract document of this chunk)

Proof (blank bytes): the unit entry's first firing at implement — `TIMEOUT [ 180.014s]
  whitespace_only_lines_open_no_connection` — fed `\x0b` as a blank byte; the relay opened a connection to a stand-in
  serving nothing and blocked; fixed with `\x0c` (report, Decisions & corrections). +0.4 measured (a real gate failure)
  · +0.2 technical detail · +0.2 no other home.
Proof (hard link): the 108-byte unreachable case of `unit_door_bridge.rs` reaches a real socket under a second
  hard-link name so the path canonicalizes inside the data dir (report, Decisions & corrections; Deviations step 3);
  green in the workspace run and on `ci#38227488889`. +0.4 · +0.2 · +0.2.
Proof (line continuation): the relay arm's sentence constant was rewritten on one source line because the workflow
  pin compares `arm.split_whitespace().join(" ")` against the source text and the continuation's backslash stayed in
  it (report, Decisions & corrections). +0.4 · +0.2 · +0.2.

Rejected:
- "On Linux a listener that drops a connection unread leaves the client with EPIPE on write or ECONNRESET on read" —
  0.6 (measured +0.4, detail +0.2); amended into security-plan §Authentication & Authorization and architecture
  §Standard Contracts → Engine door by this wrap's P2, so the no-other-home signal does not apply → below threshold.
- "A Python edit script anchored on a multi-line string failed its count == 1 assertion on indentation" — task-specific
  (a one-off edit's indentation); the host rule already directs validated, file-read-first payloads.

Recurrences (handoff, Deferred learnings): the Bash guard refused a leading `cd` three times in this window
  (host leaf 2026-10-05) and, in the first window, a `cat` heredoc with a file target (host-linux.md, Transports) —
  each re-issued once in the form the guard passes.
