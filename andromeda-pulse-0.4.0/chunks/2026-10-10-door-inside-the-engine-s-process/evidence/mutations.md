# Mutation checks — 2026-10-10-door-inside-the-engine-s-process

Step 12 of the plan, run once at /implement, on 2026-10-10 (all four put back by 23:52:00Z, read from `date -u`).
Each mutation was applied with the Edit tool, confirmed present by a grep of the mutated line, run, read red, and
put back; the put-back was confirmed by a grep and by a green run of the four door binaries at the end. Every run
was `cargo test -p pulse-app --test {binary} {filter}` (libtest, the binary alone), read from its redirected log.

## 1. The owner predicate made to admit everyone

- **Mutation** (`pulse-app/src/door.rs`, `admits`): the body `peer_account == own_account` replaced by `true`.
- **Present** (grep of the function): `let _ = (peer_account, own_account);` / `true`.
- **Run:** `--test unit_door the_owner_predicate`.
- **Reading: red.** `the_owner_predicate_admits_the_engines_own_account_and_no_other ... FAILED`, panicked at
  `pulse-app/tests/unit_door.rs:53:5`: `a different account id`. `test result: FAILED. 0 passed; 1 failed`.

## 2. The `door.call` leaf narrowed by one field

- **Mutation** (`pulse-app/src/observability.rs`, the `door.call` leaf): the member `"duration_ms",` removed.
- **Present** (the leaf's lines): seven members, `"answer_bytes",` the last.
- **Run:** `--test unit_observability_allowlist_door` (the whole binary, six tests).
- **Reading: the set-equality pin red, the resolve pin green.**
  `the_call_record_emits_exactly_its_leaf_and_is_info_only_for_ok ... FAILED`, panicked at
  `pulse-app/tests/unit_observability_allowlist_door.rs:208:9`, left (the emit site) eight names with
  `"duration_ms"`, right (the leaf) seven without it.
  `each_door_target_resolves_and_no_fallback_key_could_resolve_it ... ok`, as were the other four.
  `test result: FAILED. 5 passed; 1 failed`.

## 3. The door handed a fresh in-memory database in place of the engine's connection

- **Mutation** (`pulse-app/src/engine_boot.rs`, where `start` seats the door): `EngineTools::new` given a
  connection opened in memory there, with the buffer schema created on it, in place of `buffer_conn.as_ref()`.
  This is the shape the stdio sidecar has: a well-formed database nothing was ingested into.
- **Present** (grep): `let fresh = Connection::open_in_memory().ok().map(|conn| {` … `fresh.as_ref(),`.
- **Run:** `--test integration_engine_boot the_door_` (four arms match the filter).
- **Reading: red.** `the_door_answers_from_the_stores_the_boot_built ... FAILED`, thread `door-arm-stores` panicked
  at `pulse-app/tests/integration_engine_boot.rs:567:9`: `the span the receiver took is what the door returns:
  {"id":1,"jsonrpc":"2.0","result":{"items":[],"next_cursor":null,"total":0}}`. The door answered a well-formed
  empty result. The three corpus arms the filter also matched stayed green (`3 passed; 1 failed`): they read the
  corpus, which the mutation did not touch.

## 4. The switch ignored

- **Mutation** (`pulse-app/src/door.rs`, the call path): `if write && !self.resolve.allows_write() {` given a
  trailing `&& false`.
- **Present** (grep): `if write && !self.resolve.allows_write() && false {`.
- **Run:** `--test integration_engine_boot the_door_refuses_the_write_with_the_switch_off`.
- **Reading: red.** `the_door_refuses_the_write_with_the_switch_off ... FAILED`, thread `door-arm-write-off`
  panicked at `pulse-app/tests/integration_engine_boot.rs:665:9`: the answer was
  `{"id":1,"jsonrpc":"2.0","result":{"incident_id":1,"resolved":true}}` where error `-32001` was asserted. The
  write was applied with the switch off.

## Put back

- Greps after the last restore: `admits` reads `peer_account == own_account`; `&& false` 0 times in
  `pulse-app/src/door.rs`; `let fresh` 0 times in `pulse-app/src/engine_boot.rs`; the `door.call` leaf holds
  `"duration_ms",` again. `cargo fmt --check` exit 0.
- One run of the four binaries after it, exit 0: `integration_door` 5 passed, `integration_engine_boot` 11 passed,
  `unit_door` 19 passed, `unit_observability_allowlist_door` 6 passed.

## Limits

- Each mutation was read on the one arm the plan names for it; no other arm was run under it, so nothing is said
  here of which further arms each mutation would redden.
- Mutation 3 was made with the schema created on the fresh database. A fresh database with no schema was not run.
