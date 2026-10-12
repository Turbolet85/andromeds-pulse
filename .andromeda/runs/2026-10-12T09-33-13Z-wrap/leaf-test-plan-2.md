group: test-plan-2
read: .claude/rules/testing.md · 324 · «- 2026-10-11: A LOCK GUARD TAKEN IN AN `if let` SCRUTINEE LI»

- leaf: .claude/rules/testing.md:25   class: stale
  leaf-text: «112 files as of chunk 2026-10-10-door-inside-the-engine-s-process; 109 at chunk 2026-10-10-console-engine-entry-point»
  source: .andromeda/test-plan.md:206 «(113 top-level `pulse-app/tests/*.rs` files as of chunk 2026-10-12-stdio-agent-reaches-the-door — the one new `unit_door_bridge.rs` — 112 at its base;»
  replace-with: «113 files as of chunk 2026-10-12-stdio-agent-reaches-the-door; 112 at chunk 2026-10-10-door-inside-the-engine-s-process»

- leaf: .claude/rules/testing.md:22   class: omission
  leaf-text: «which need `bash`, a Linux `/proc` and the engine built in the test profile.»
  source: .andromeda/test-plan.md:63 «Since chunk 2026-10-12-stdio-agent-reaches-the-door a fourth depth, the stdio relay `andromeda-pulse-engine door` (a client of `run/door/sock`, one connection per request line, no record of its own)»
  replace-with: «which need `bash`, a Linux `/proc` and the engine built in the test profile. **Console engine door (Unix only):** a `tokio::net::UnixStream` client at `{data dir}/run/door/sock` — over a stand-in `Tools` in process (`unit_door.rs`), the door seated by `engine_boot::start` in process (`integration_engine_boot.rs`) and a spawned `andromeda-pulse-engine run` (`integration_door.rs`) — and, since chunk 2026-10-12-stdio-agent-reaches-the-door, the stdio relay `andromeda-pulse-engine door` (`unit_door_bridge.rs` and two relay arms of `integration_door.rs`); the two other-account arms print a skip sentence and pass where `sudo -n true` fails, so the `lint-test` step `Door refuses another account` runs both on the runner and fails unless it reads both arms' sentences (PROVISIONAL with the door).»
