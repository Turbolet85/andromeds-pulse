# security extract

## Relevance
Partial — P-062 touches only Tauri window configuration + runtime resize behavior; no untrusted input acceptance, no API surface, no secrets, no dependencies involved.

## Constraints
- Window-size constraint parameters (min-width, min-height, aspect-ratio bounds) are configuration-only; no runtime re-binding from external input (per security-plan §Input Validation boundary discipline — static Tauri config is the authoritative source).
- Window coordinate values are never logged during constraint enforcement (per security-plan §Logging & Monitoring redaction rule + §Data Protection at-rest row established in P-061 amendment).
- No new TauRPC procedures, capability JSON entries, or IPC surfaces introduced (per P-062 scope — window operations are Rust/config-side only).

## Patterns to follow
- Window geometry read/write follows the bounded-integer + graceful-default pattern established by P-061 (security-plan §Input Validation "Persisted window geometry" row).
- Tauri `tauri.conf.json` window definition (static `minWidth`/`minHeight`) is the configuration source of truth per P-062 scope.
- Runtime resize-clamping code (if aspect-ratio enforcement requires event-handler logic) mirrors P-061's `on_window_event` Moved handler discipline (Rust-only, no geometry-file schema changes).

## Anti-patterns to avoid
- Do not log window coordinates or computed dimension values (security-plan §Logging & Monitoring NEVER-log list: "Full plugin file paths… DuckDB query parameters… Snapshot file contents…" — extend to window geometry state during constraint operations).
- Do not accept user-configurable aspect-ratio bounds at runtime (P-062 scope specifies "a sane fixed constant" not user-tunable Settings extension).

## Contract bindings
Window-geometry.json persisted-file contract established by P-061 (§Input Validation row); P-062 constrains valid runtime state-space but does not alter the file format or deserialization boundary.

## Acceptance criteria contributions
- (security) Window-size constraint parameters (min-width, min-height, aspect-ratio constants) are configuration sources-of-truth only; no dynamic external input modifies these values at runtime (compliance with security-plan §Input Validation bounded-config discipline).
- (security) No coordinate or dimension values logged during constraint-clamping operations (compliance with security-plan §Logging & Monitoring redaction rule; verify via grep for coordinate field names in new Rust code).

## Relevant amendment history
**2026-06-29-window-geometry-movable-shell (P-061)** — Registered `<data_dir>/window-geometry.json` as an input boundary: integer x/y per `serde`; missing/corrupt → default via `unwrap_or_default`; atomic write; **no coordinate values logged** (security-plan §Input Validation row + §Logging redaction). P-062 builds on this P-061 foundation; no regression of the no-logging rule expected.
