
## 2026-10-04-linux-launch-stays-up-on-nvidia-wayland — the launch render-posture boot record
**Section:** §6 Log Coverage → Log levels mapping (`warn` row) · §8 PII Scrubbing → the exact-leaf list
**Change:** §6's warn row gains `warn!(target: "app.boot.render.posture", posture, lever)` — EXACTLY ONCE per boot, emitted after `observability::init` (decided at the head of `main()`, where no subscriber exists, and carried as a value), WARN for `preset_honoured`, INFO for `applied` / `not_applicable`; `lever` is the variable NAME only. §8 gains its exact leaf `{posture, lever}` — both fields the emit site emits, a closed label plus the env-var name, never the value (wire-measured 0× in both live legs) — beside `app.boot.tray.init` under the no-bare-`app` invariant, guarded ×3 in `pulse-app/tests/unit_observability_allowlist_render_posture.rs` (exact-resolve with set equality, the discriminator, an emit-site capture equal to the leaf for all three postures; mutation-checked).
**Why:** a preset-honoured boot is otherwise indistinguishable from a default-applied one; registered dual-site per the once-per-boot rule.
**Ref:** .andromeda/runs/2026-10-04T12-30-22Z-wrap/
