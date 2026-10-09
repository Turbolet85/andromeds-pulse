# security extract

## Relevance
Partial — touches Tauri capability trust boundary + conditional API surface (TauRPC procedure IF chosen over direct window call); no new external input validation or data protection concerns.

## Constraints
1. **Capability gating is mandatory** — per security-plan §API Security + §Security Anti-Patterns § API line 402: any TauRPC procedure must have a matching `pulse-app/capabilities/` JSON entry, or affordance calls will be silently rejected. If direct `@tauri-apps/api/webviewWindow` route is chosen, `core:window:*` operations are similarly rejected under `pulse:default` unless explicitly granted per scope.md P-061 lesson.
2. **Error handling boundary** — if TauRPC procedure route chosen: must return `Result<(), AppError>` per security-plan §Error Handling line 305 + Conventions Error response schema; module-internal errors convert to `AppError::{Validation, NotFound, Internal, Plugin, Storage, Ingest}` variants.
3. **No core API widening without rationale** — per security-plan §Security Anti-Patterns § API line 403, do not grant `fs`, `shell`, `dialog`, or `http` core APIs; window operations are permissible if explicitly stated with rationale in capability JSON.
4. **Xtask drift check enforcement** — per security-plan §Bootstrap phases line 250 + §API Security line 193, TauRPC procedures must match `pulse-app/capabilities/` JSON entries; CI must verify via xtask drift check (no silent rejections due to capability gaps).
5. **Reuse show/focus semantics from tray** — scope.md directs reusing the existing tray show/focus pattern from `pulse-app/src/tray.rs` / `pulse-app/src/window.rs` to ensure idempotent "open if hidden, focus if shown" behavior.

## Patterns to follow
1. **Explicit capability grant with stated rationale** — per scope.md "Surfaces & contracts likely touched (confirm in research/plan)", capability JSON must enumerate either the `core:window:*` grant (direct route) OR a new TauRPC namespace entry (procedure route), with explicit rationale.
2. **Tray navigation precedent** — security-plan §API Security § Updater capability isolation (line 194) + tray/window refs in scope.md establish the pattern: capability gates must explicitly bind the procedure/permission to its intended caller (tray icon only, compact-widget affordance only), not a catch-all.
3. **Silent rejection detection in test** — scope.md §Acceptance "affordance-honesty (a real activation must be exercised)" — real click/keyboard event must drive the window operation; no silent rejections. Xtask drift check is the compile-time enforcement; affordance-level e2e is the runtime proof.

## Anti-patterns to avoid
1. **NEVER skip capability JSON entry for a TauRPC procedure** — security-plan §Security Anti-Patterns § API line 402 (the bridge call is silently rejected at runtime, becoming a hard-to-diagnose UX bug).
2. **NEVER widen `pulse:default` grant with window core APIs without explicit addition** — security-plan §Security Anti-Patterns § API line 403 (explicit per-feature capability addition with stated rationale required).

## Contract bindings
1. **IPC ↔ Tauri capability system** — if TauRPC procedure chosen, new procedure requires capability JSON entry; xtask drift check enforces mismatch detection at CI.
2. **Design ↔ affordance UX** — design owns control shape/placement/styling; security owns capability grant + error-handling boundary validation.
3. **A11y ↔ keyboard navigation** — a11y owns `tabindex`/`aria-label`/keyboard event dispatch; security owns IPC capability grant (no silent rejections due to capability gaps).

## Acceptance criteria contributions
1. **(security) Capability drift check passes** — xtask verifies TauRPC procedure ↔ capability JSON entry match (if procedure route chosen) OR verifies `core:window:*` grant present in compact-widget capability (if direct route chosen), per §API Security line 250.
2. **(security) Affordance-honesty verification** — real in-widget control activation (click / keyboard event per scope) must drive dashboard window to shown+focused; no silent capability rejections (verified by affordance-level e2e per scope §Acceptance + P-078 self-verify boot smoke).
3. **(security) Error boundary compliance** — if TauRPC procedure chosen, procedure returns `Result<(), AppError>` following Conventions Error response schema; no stack traces / internal details leak.
4. **(security) No regression of tray window semantics** — show/focus behavior reused from existing tray pattern (idempotent open/focus, unminimize if minimized, focus if already visible).

## Relevant amendment history
- **2026-06-29-window-geometry-movable-shell (chunk P-061, sibling epoch)** — established explicit capability grant + xtask drift checking precedent for window operations in the same epoch (Window & shell hygiene). Chunk #80 (P-061) added the first window-geometry-movable affordance and wired the xtask drift check. This chunk (P-066) follows the same pattern: explicit capability grant required, xtask drift check enforces.
