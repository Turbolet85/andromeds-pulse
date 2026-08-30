# Development Workflow

## Branch model
- **`main`** — protected; all commits land via PR; never force-push.
- **Feature branches:** `feat/<short-name>`, `fix/<short-name>`, `chore/<short-name>` per task. Created from `main`, merged via squash+rebase PR.
- **Release branches:** `release/v<X.Y.Z>` only when needed for a release stabilization window; tagged from this branch.

## Commit conventions (Conventional Commits)
- `feat: …` — new feature
- `fix: …` — bug fix
- `refactor: …` — code change without behavior change
- `chore: …` — tooling, deps, CI, formatting
- `docs: …` — documentation only
- `test: …` — test additions / changes
- `perf: …` — performance improvement

Scope optional: `feat(ingest): add OTAP wire format flag`.

Body wrapped at 72 cols; "why" not "what."

Co-Authored-By footer when AI-assisted: `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>`.

## PR process
- Use `gh` CLI for ALL GitHub operations (do NOT suggest MCP or web UI).
- Title: short imperative (under 70 chars). Body has Summary + Test plan sections.
- One topic per PR — split if it touches multiple unrelated concerns.
- All CI gates must pass before merge: lint + clippy + nextest + coverage thresholds + supply-chain + a11y + capability-drift + staged-artifacts.
- Squash on merge (single conventional commit per PR).

## Release flow
1. Open release PR: `chore: prepare v<X.Y.Z>` — bumps `Cargo.toml` workspace version + updates `CHANGELOG.md`.
2. Merge to `main`.
3. Tag `v<X.Y.Z>` on `main`.
4. Push tag — triggers `release.yml` (tauri-action build matrix → Azure Key Vault EV signing → Apple Developer ID notarization → GitHub Releases upload).
5. `update-channels.yml` triggers on `release.yml` completion (Homebrew tap + Scoop manifest).
6. `production-release` GitHub Environment requires manual approval before signing secrets are accessed.
7. Verify auto-update flow on a test installation before announcing.

## CI gates (PR cannot merge if any fails)
- `cargo fmt --check` clean
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` clean
- `cargo nextest run --workspace --profile ci` passes (zero failures, zero flakes — flake = real bug per Standard tier)
- Coverage ≥75% line / ≥70% branch / ≥85% function (`cargo llvm-cov`)
- Performance budgets respected (snapshot p99 ≤500ms, WebGPU frame p99 ≤33ms — measured via `criterion` + log-tail aggregation)
- `cargo audit` zero findings above warning
- `cargo deny check bans licenses sources` clean (catches `tonic` 0.14/0.13 duplicate)
- `xtask capability-drift` passes (TauRPC procedures match the `EXPECTED_PROCEDURES` pin — worktree AND staged git-index copies; staged `capabilities/*.json` grants match `staged_gate::EXPECTED_GRANTS`; `cargo xtask check:staged-artifacts` is the direct verb)
- A11y suite passes (axe-core no critical/serious, Lighthouse a11y ≥90, no per-PR regression vs base branch)
- Heartbeat-stall detection passes (no `{module}.tick` gap >45s during test run)
- Zero `app.panic.fatal` spans in test logs

## Local development loop
```bash
# 1. Branch from main
git checkout main && git pull && git checkout -b feat/my-task

# 2. Iterate
cargo tauri dev                                  # hot reload
cargo nextest run --filter-expr 'package(my-crate)'

# 3. Pre-commit checks (mirror CI)
cargo fmt
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --profile ci
cargo audit

# 4. Commit (conventional)
git add <files>
git commit -m "feat(ingest): …"

# 5. Push + open PR
git push -u origin feat/my-task
gh pr create --fill
```

## Context management (Claude Code)
- `/clear` between unrelated tasks to reset context.
- `/compact preserve: branch=feat/x, status=mid-refactor, next=fix-foo` for long sessions.
- Delegate research / codebase exploration to subagents (Explore / general-purpose) to keep main context clean. Subagents cannot run Bash — only Read/Glob/Grep tasks.
- For complex tasks: use Plan mode first, then switch to implementation.

## Session handoff
- At session end: update `.claude/session-handoff.md` (decisions made, files modified, current branch, remaining tasks, blockers, last failed command if any).
- At session start: ALWAYS read `.claude/session-handoff.md` if it exists.
- SessionStart hook auto-injects on resume/compact, but NOT on brand-new sessions — read manually at start of fresh `claude` invocation.
- If handoff contains "Last Failed Command" — propose an alternative approach BEFORE doing anything else; never retry the same command.
- `/wrap-session` curates session learnings + reconciles living artifacts (dependency-tree.md + api-surface.md) at session end.

## Andromeda planning loop
- Plans in `.andromeda/` are the source of truth: arch + 6 specialist plans + route → CLAUDE.md ecosystem materialization via `/andromeda-setup-project`.
- Re-run `/andromeda-setup-project` after any plan regeneration to refresh `CLAUDE.md` + `.claude/rules/` + `.claude/docs/`.
- Per-chunk implementation: `/andromeda-phase` plans the chunk; phase loop runs `/implement` to execute.
- `/wrap-session` at session end → reconciles living artifacts + curates Tier 2/3 learnings.
- `/andromeda-help` for full Andromeda system docs.

## Browser debugging
- Never ask for manual browser console / network copy.
- If `chrome-devtools-mcp` is available, open the browser directly via Chrome DevTools MCP and read the console.
- For Tauri webview debugging in dev: `cargo tauri dev` opens DevTools automatically when `WEBKIT_DISABLE_COMPOSITING_MODE=1` is set on Linux; Cmd+Opt+I on macOS; F12 / Ctrl+Shift+I on Windows.
