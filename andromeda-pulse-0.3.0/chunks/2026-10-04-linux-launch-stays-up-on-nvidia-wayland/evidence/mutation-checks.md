# Mutation checks — 2026-10-04-linux-launch-stays-up-on-nvidia-wayland

Plan Step 7, one-shot at /implement (2026-10-04, run `.andromeda/runs/2026-10-04T11-47-01Z-implement`). Each mutation
was applied with the anchored Edit tool, its application confirmed by a `grep` of the mutated line before the run,
and then reverted the same way. Every run used the plan's targeted `unit` entry:
`cargo nextest run --workspace --profile ci -E 'binary(unit_render_posture) | binary(unit_observability_allowlist_render_posture)'`.
A red run is the expected outcome; a green run would have been a failed mutation.

| # | Mutation | Exit | Summary | Pins that went RED | Pins that stayed green (expected) |
|---|---|---|---|---|---|
| a | `decide`'s Linux-absent arm `(true, false) => PresetHonoured` | 100 | 12 run: 10 passed, 2 failed | `unit_render_posture::linux_without_a_preset_applies_the_default` · `unit_render_posture::absent_lever_is_set_to_one_in_a_fresh_process` (child arm A) | the preset arm B and the leaf pins (not on this path) |
| b | `apply_linux_default` sets the lever whenever `cfg!(target_os = "linux")` (unconditional on Linux) | 100 | 12 run: 11 passed, 1 failed | `unit_render_posture::preset_lever_is_honoured_and_never_overwritten` (child arm B: `left: "1"`, `right: "0"` — the preset was overwritten) | arm A (the absent arm sets `1` in both worlds — the conditional-property asymmetry) |
| c | the `app.boot.render.posture` leaf deleted from `AllowList::production()` | 100 | 12 run: 10 passed, 2 failed | `unit_observability_allowlist_render_posture::render_posture_resolves_to_an_exact_leaf_with_exactly_its_fields` · `::every_posture_emits_exactly_the_leaf_fields_at_its_level` | `::render_posture_has_no_bare_app_fallback` held — `for_target("app")` stays `None`, so the deleted leaf resolves to nothing rather than to a fallback set |

After the three reverts: `git diff ffb62f0` over the three modified sources showed only the chunk's intended
edits, `cargo fmt --check` exited 0, and the full non-leg gate block was re-run green (17/17, workspace nextest
2592 passed, 0 skipped). That re-run also rebuilt `target/debug/pulse-app` from the restored source before the
live legs: a workspace nextest during mutation (c) builds the package's bin target.
