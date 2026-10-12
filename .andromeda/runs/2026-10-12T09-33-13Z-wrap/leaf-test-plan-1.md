group: test-plan-1
read: .claude/docs/commands.md · 190 · «- Views: `symbol` / `refs` / `calls` / `contains` / `crate_e»
read: .claude/docs/tests-summary.md · 152 · «Full plan: `.andromeda/test-plan.md`.»

- leaf: .claude/docs/commands.md:78   class: stale
  leaf-text: «# one line, exit 0; any other command line is usage, exit 2»
  source: .andromeda/test-plan.md:62 «`door` builds no runtime and boots no engine, exits 0 when standard input ends and 1 with one fixed standard-error line when the door cannot be reached»
  replace-with: «# one line, exit 0; `andromeda-pulse-engine door` is the stdio relay to run/door/sock (boots no engine; exit 0 when stdin ends, 1 with one fixed stderr line when the door cannot be reached); any other command line is usage, exit 2 (usage: andromeda-pulse-engine <run|version|door>)»

- leaf: .claude/docs/tests-summary.md:45   class: omission
  leaf-text: «and `integration_door.rs` on a spawned `andromeda-pulse-engine run` |»
  source: .andromeda/test-plan.md:63 «Since chunk 2026-10-12-stdio-agent-reaches-the-door a fourth depth, the stdio relay `andromeda-pulse-engine door` (a client of `run/door/sock`, one connection per request line, no record of its own)»
  replace-with: «and `integration_door.rs` on a spawned `andromeda-pulse-engine run`; since chunk 2026-10-12-stdio-agent-reaches-the-door also through the stdio relay `andromeda-pulse-engine door` (a client of `run/door/sock`, one connection per request line): `unit_door_bridge.rs` (8 tests) and two relay arms of `integration_door.rs` (7 arms) |»

- leaf: .claude/docs/tests-summary.md:92   class: stale
  leaf-text: «runs the one arm `another_account_is_refused_at_each_of_three_layers` of `integration_door` directly after `cargo xtask test`»
  source: .andromeda/test-plan.md:494 «test(=another_account_through_the_bridge_receives_nothing))'` (two arms since chunk 2026-10-12-stdio-agent-reaches-the-door; the first alone until it)»
  replace-with: «runs two arms of `integration_door` directly after `cargo xtask test` — `another_account_is_refused_at_each_of_three_layers` and, since chunk 2026-10-12-stdio-agent-reaches-the-door, `another_account_through_the_bridge_receives_nothing` (the relay word and the step's second arm PROVISIONAL with the door)»

- leaf: .claude/docs/tests-summary.md:92   class: stale
  leaf-text: «and passes only when nextest exits 0 and the arm's kept output holds its three-layer sentence»
  source: .andromeda/test-plan.md:494 «and it holds `door bridge other-account arm: no byte reached another account through the bridge, the account check refused`»
  replace-with: «and passes only when nextest exits 0 and the kept output holds both arms' sentences — the three-layer sentence and the bridge sentence (`door bridge other-account arm: no byte reached another account through the bridge, the account check refused`)»

- leaf: .claude/docs/tests-summary.md:92   class: stale
  leaf-text: «The arm's client runs under `sudo -n -u nobody` and `sudo -n`»
  source: .andromeda/test-plan.md:63 «and another account through the relay (a copy of the built engine run as `door` under `sudo -n -u nobody` and `sudo -n`»
  replace-with: «The three-layer arm's client (`python3 -c`) and the relay arm's copy of the built engine, run as `door`, both run under `sudo -n -u nobody` and `sudo -n`»

- leaf: .claude/docs/tests-summary.md:92   class: stale
  leaf-text: «the arm prints its skip sentence and passes»
  source: .andromeda/test-plan.md:63 «The two other-account arms (the three-layer arm and, since chunk 2026-10-12-stdio-agent-reaches-the-door, the relay's) each print a skip sentence and pass where `sudo -n true` fails (the dev host)»
  replace-with: «each of the two arms prints its skip sentence and passes»

- leaf: .claude/docs/tests-summary.md:92   class: omission
  leaf-text: «Green on `ci#38097928056`.»
  source: .andromeda/test-plan.md:494 «green again on `ci#38227488889` (the pushed tip `2595b5cc`, chunk 2026-10-12-stdio-agent-reaches-the-door), where the relay's four other-account readings»
  replace-with: «Green on `ci#38097928056`, and again on `ci#38227488889` (chunk 2026-10-12-stdio-agent-reaches-the-door), where the relay's four other-account readings each exited 1 with 0 bytes on standard output.»

- leaf: .claude/docs/tests-summary.md:92   class: stale
  leaf-text: «`pulse-app/tests/*.rs` counts 112 files and the workspace run 3174 tests at that chunk.»
  source: .andromeda/test-plan.md:206 «113 top-level `pulse-app/tests/*.rs` files as of chunk 2026-10-12-stdio-agent-reaches-the-door — the one new `unit_door_bridge.rs` — 112 at its base»
  replace-with: «`pulse-app/tests/*.rs` counts 113 files (the new `unit_door_bridge.rs`) and the workspace run 3186 tests as of chunk 2026-10-12-stdio-agent-reaches-the-door (112 and 3174 at the door's chunk).»

- leaf: .claude/docs/tests-summary.md:93   class: stale
  leaf-text: «or the door step above not reading its three-layer sentence»
  source: .andromeda/test-plan.md:515 «or the output lacks either arm's sentence — the three-layer sentence or the bridge sentence — the case when either arm skipped for want of a second account»
  replace-with: «or the door step above not reading either arm's sentence (the three-layer sentence or the bridge sentence)»
