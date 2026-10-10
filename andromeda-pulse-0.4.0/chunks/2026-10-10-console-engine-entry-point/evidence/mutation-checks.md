# Mutation checks — 2026-10-10-console-engine-entry-point

Run by /andromeda-implement at P2 on 2026-10-10, after gate entries 1 to 8 read green. Each mutation was applied
with the anchored Edit tool, confirmed present by a grep of the file, and only then run. The command each time:
`cargo test -p pulse-app --test unit_engine_boot_seam --test unit_observability_allowlist_boot_engine --no-fail-fast`.
The lines below are copied from the three runs' output; a line beginning `test` or `test result` is verbatim.

## Run 1 — two mutations in the tree together (two different test binaries)

**Mutation A (plan step 8):** a `run_retention(` call put back into `pulse-app/src/main.rs`, as
`tokio::spawn(buffer::run_retention(Arc::clone(conn), Arc::clone(&engine.buffer_state), engine.retention_seconds))`
under `if let Some(conn) = buffer_conn.as_ref()`. Grep before the run: `159:        tokio::spawn(buffer::run_retention(`.

**Mutation B:** the `app.boot.engine` leaf of `pulse-app/src/observability.rs` renamed to `app.boot.engine-mutated`
(the exact leaf removed). Grep before the run: `1016:            "app.boot.engine-mutated",`.

cargo exit 101.

```
test no_entry_point_wires_an_engine_part ... FAILED
main.rs names the engine part `run_retention(`; it belongs to the shared boot
test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test the_emit_site_target_is_the_registered_one ... ok
test boot_engine_has_no_bare_app_fallback ... ok
test every_program_and_seat_emits_exactly_the_leaf_fields_at_its_level ... FAILED
test boot_engine_resolves_to_an_exact_leaf_with_exactly_its_fields ... FAILED
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Reading: the seam pin reddens under the mutation of step 8, and the seven other seam tests stay green. With the
exact leaf gone, both field-set tests redden; `boot_engine_has_no_bare_app_fallback` stays green, which is the
discriminator holding (no bare `app` key catches the target).

## Run 2 — mutation A restored, the leaf narrowed

**Mutation C:** the leaf back under its own name, narrowed to `["program", "interpretation"]` (`reason` dropped).
Grep before the run: `run_retention(` 0 times in `main.rs`; the leaf line reads
`["program", "interpretation"].iter().copied().collect(),`.

cargo exit 101.

```
test no_entry_point_wires_an_engine_part ... ok
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test the_emit_site_target_is_the_registered_one ... ok
test boot_engine_has_no_bare_app_fallback ... ok
test boot_engine_resolves_to_an_exact_leaf_with_exactly_its_fields ... FAILED
test every_program_and_seat_emits_exactly_the_leaf_fields_at_its_level ... FAILED
assertion `left == right` failed: the leaf must equal {program, interpretation, reason}
  left: {"interpretation", "program"}
 right: {"interpretation", "program", "reason"}
assertion `left == right` failed: Window / Some(Model): emitted fields must equal the leaf
  left: {"interpretation", "program", "reason"}
 right: {"interpretation", "program"}
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Reading: a narrowed leaf is caught from both sides, by the leaf's own set and by the emit site's capture.

## Run 3 — the tree restored

`git diff f31027a4023fd64e97bff3272d303faa628d2336 -- pulse-app/src/observability.rs` shows the one added leaf with
its three fields and nothing else; `run_retention(` occurs 0 times in `main.rs`.

cargo exit 0.

```
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Not mutated

The two seat arms of `integration_engine_boot` and the spawned-program arms of `integration_console_engine` were
not mutation-checked. The gate block run after these checks re-ran every entry on the restored tree.
