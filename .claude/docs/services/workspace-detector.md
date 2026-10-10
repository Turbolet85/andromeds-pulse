# `workspace-detector` — Host Project Context Detection

## Responsibility
Detects host project context for telemetry correlation: `.andromeda/` marker + VCS metadata (the `.git` root and a short HEAD prefix) + a project name taken from the root's basename. Backs the one implemented procedure `workspace.detect` (resolver at `crates/ui-bridge/src/workspace_ipc.rs`; `workspace.list` is deferred — no runtime emitter). Used by the shared engine boot (`engine_boot::start` detects the host workspace and publishes `run/workspace-key` for both programs) and read cross-process by the `andromeda-pulse-mcp` sidecar.

## Key integrations

### Consumes from
- Filesystem (paths canonicalized via `std::fs::canonicalize`, both-sides on confinement checks).
- VCS metadata read from the filesystem alone: a walk up at most 32 ancestors for a `.git/` directory, then `.git/HEAD` for a short commit prefix (`vcs.rs::detect_vcs`). No `git` process is started and no `Cargo.toml` is read.

### Publishes to
- TauRPC procedure: `workspace.detect` (resolver in `crates/ui-bridge/src/workspace_ipc.rs`); `workspace.list` deferred — no runtime emitter.
- `tracing` events: `workspace.detect`, `app.boot.workspace.detect`, `filesystem.scan.gitroot`, `workspace.marker.check`.

### Dependencies
- `std` both-sides-canonicalize — the canonicalize-and-confine primitive for env-var path overrides (this crate's `publish_workspace_key` is the repo's reference pattern; `strict-path` dropped 2026-08-29).
- No VCS library and no `git2`: `crates/workspace-detector/Cargo.toml` names `thiserror`, `serde` and `tracing` only.
- `serde` for `WorkspaceContext` / `VcsMetadata` serialization.

## Internal conventions
- **Path canonicalization REQUIRED** — all input paths (env vars `ANDROMEDA_PULSE_*_PATH` / `*_DIR`, current working directory) canonicalize via `std::fs::canonicalize` (`strict-path` was measured 2026-08-26 at zero `.rs` users repo-wide and DROPPED from the dependency graph at chunk 2026-08-29-advisory-backlog; this crate's `publish_workspace_key` canonicalizes BOTH sides then `starts_with`, the repo's reference confinement pattern); assert resolved path lives under the resolved data dir or current project root.
- **`.andromeda/` marker detection** — walk parents from cwd looking for `.andromeda/` directory; first match wins.
- **VCS detection** — filesystem only: the nearest ancestor (at most 32 up) holding a `.git/` directory is the VCS root; `.git/HEAD` gives an 8-character commit prefix. Git is the one VCS recognised (`VcsType::Git`).
- **Anonymized logging** — `tracing` events emit `workspace.root_path_basename` and `workspace.project_name` only; NEVER full canonicalized path (security plan vector 6).
- **Errors** collapse to `AppError::NotFound { resource }` (workspace) or `AppError::Validation { field, reason }` (path traversal attempt).

## Workspace context (as `crates/workspace-detector/src/contract.rs` declares it)
```rust
pub struct WorkspaceContext {
    pub root: PathBuf,                 // Canonicalized candidate path
    pub project_name: Option<String>,  // Best-effort, from the root's basename
    pub vcs: Option<VcsMetadata>,      // None when no .git/ is found
    pub has_andromeda_marker: bool,    // .andromeda/ directory present (content never read)
}

pub struct VcsMetadata {
    pub vcs_type: VcsType,                     // closed enum: Git
    pub vcs_root: PathBuf,                     // the ancestor holding .git/
    pub head_commit_basename: Option<String>,  // short SHA prefix only
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
- **Marker check:** `crates/workspace-detector/src/marker.rs` (existence only)
- **Tests:** colocated in the four module files, each building its project tree in a `tempfile` directory; the E2E P7 leg is `pulse-app/tests/e2e_p7_workspace_detect.rs`. There is no committed fixture directory (no `tests/fixtures/workspaces/`).

## Testing this service
- **Unit tests:** `cargo nextest run --filter-expr 'package(workspace-detector)'`
- **Integration (test-plan §6 P7):** boot app with CWD set to repo root (has `.andromeda/` marker) → invoke `workspace.detect()` → assert response contains `project_name`, `root_path` (canonical absolute), `vcs_type: "git"`, `vcs_root: PathBuf`, `has_andromeda: true`.
- **Negative test (security-plan vector 5):** set `ANDROMEDA_PULSE_PLUGIN_DIR=~/.andromeda-pulse/plugins/../../../etc` (escape attempt) → assert canonicalization rejects + confinement blocks traversal + only basename `"etc"` appears in error log.

## Local development
- **Test workspace fixtures:** none are committed. A test makes its own tree in a temp dir (a `.git/` with a `HEAD`, a `.andromeda/` directory) and removes it with the dir.

## References
- `.andromeda/architecture.md` §Occupied Resources (workspace.* TauRPC routes) + §Workspace crates
- `.andromeda/security-plan.md` §Anti-Patterns Input (path env var canonicalization, CWE-22 defense) + §Logging Vector 6 (basename-only logging)
- `.andromeda/test-plan.md` §6 P7 + §5 (path canonicalization negative test)
- `.andromeda/obs-plan.md` §1 P7 (must-trace `app.boot.workspace.detect`)
