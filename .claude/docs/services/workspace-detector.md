# `workspace-detector` — Host Project Context Detection

## Responsibility
Detects host project context for telemetry correlation: `.andromeda/` marker + VCS metadata (git root, project name, cargo manifest). Backs the one implemented procedure `workspace.detect` (resolver at `crates/ui-bridge/src/workspace_ipc.rs`; `workspace.list` is deferred — no runtime emitter). Used by the shared engine boot (`engine_boot::start` detects the host workspace and publishes `run/workspace-key` for both programs) and read cross-process by the `andromeda-pulse-mcp` sidecar.

## Key integrations

### Consumes from
- Filesystem (paths canonicalized via `std::fs::canonicalize`, both-sides on confinement checks).
- VCS metadata (`git rev-parse --show-toplevel`, `Cargo.toml` package.name).

### Publishes to
- TauRPC procedure: `workspace.detect` (resolver in `crates/ui-bridge/src/workspace_ipc.rs`); `workspace.list` deferred — no runtime emitter.
- `tracing` events: `workspace.detect`, `app.boot.workspace.detect`, `filesystem.scan.gitroot`, `workspace.marker.check`.

### Dependencies
- `std` both-sides-canonicalize — the canonicalize-and-confine primitive for env-var path overrides (this crate's `publish_workspace_key` is the repo's reference pattern; `strict-path` dropped 2026-08-29).
- `git2` (or shell-out to `git`) for VCS metadata.
- `serde` for `Workspace` struct serialization.

## Internal conventions
- **Path canonicalization REQUIRED** — all input paths (env vars `ANDROMEDA_PULSE_*_PATH` / `*_DIR`, current working directory) canonicalize via `std::fs::canonicalize` (`strict-path` was measured 2026-08-26 at zero `.rs` users repo-wide and DROPPED from the dependency graph at chunk 2026-08-29-advisory-backlog; this crate's `publish_workspace_key` canonicalizes BOTH sides then `starts_with`, the repo's reference confinement pattern); assert resolved path lives under the resolved data dir or current project root.
- **`.andromeda/` marker detection** — walk parents from cwd looking for `.andromeda/` directory; first match wins.
- **VCS detection** — try git first (`git rev-parse --show-toplevel`); fall back to filesystem scan if git not available.
- **Anonymized logging** — `tracing` events emit `workspace.root_path_basename` and `workspace.project_name` only; NEVER full canonicalized path (security plan vector 6).
- **Errors** collapse to `AppError::NotFound { resource }` (workspace) or `AppError::Validation { field, reason }` (path traversal attempt).

## Workspace struct (binding)
```rust
struct Workspace {
    project_name: String,    // From Cargo.toml package.name OR directory name
    root_path: PathBuf,      // Canonicalized absolute path
    vcs_type: Option<String>, // "git" if .git/ found
    vcs_root: Option<PathBuf>, // .git/ parent
    has_andromeda: bool,     // .andromeda/ marker present
}
```

## Service-specific gotchas
- **Symlink chains** — must resolve canonically; TOCTOU race attempts must be atomic. Negative test: symlink chain in plugin dir → assert resolved canonically.
- **Out-of-repo test scenarios untestable** — agent-driven harness assumes CWD is repository root with `.andromeda/` marker (test-plan §1 P7 limitation).

## Entry points for modification
- **Detection logic:** `crates/workspace-detector/src/detect.rs`
- **Path canonicalization:** `workspace_detector::contract::publish_workspace_key` (std both-sides-canonicalize + `starts_with` confinement)
- **VCS metadata:** `crates/workspace-detector/src/vcs.rs`
- **TauRPC resolver:** `crates/ui-bridge/src/workspace_ipc.rs` (`WorkspaceApiImpl`, `workspace.detect`)
- **Tests:** colocated per module + `tests/fixtures/workspaces/` for fixture project structures

## Testing this service
- **Unit tests:** `cargo nextest run --filter-expr 'package(workspace-detector)'`
- **Integration (test-plan §6 P7):** boot app with CWD set to repo root (has `.andromeda/` marker) → invoke `workspace.detect()` → assert response contains `project_name`, `root_path` (canonical absolute), `vcs_type: "git"`, `vcs_root: PathBuf`, `has_andromeda: true`.
- **Negative test (security-plan vector 5):** set `ANDROMEDA_PULSE_PLUGIN_DIR=~/.andromeda-pulse/plugins/../../../etc` (escape attempt) → assert canonicalization rejects + confinement blocks traversal + only basename `"etc"` appears in error log.

## Local development
- **Test workspace fixtures:** `tests/fixtures/workspaces/with-andromeda/` (has marker), `tests/fixtures/workspaces/no-andromeda/` (no marker), `tests/fixtures/workspaces/symlink-chain/` (negative test).

## References
- `.andromeda/architecture.md` §Occupied Resources (workspace.* TauRPC routes) + §Workspace crates
- `.andromeda/security-plan.md` §Anti-Patterns Input (path env var canonicalization, CWE-22 defense) + §Logging Vector 6 (basename-only logging)
- `.andromeda/test-plan.md` §6 P7 + §5 (path canonicalization negative test)
- `.andromeda/obs-plan.md` §1 P7 (must-trace `app.boot.workspace.detect`)
