# Cascade dispositions — 2026-10-04-corpus-key-creation-is-race-free

The search: `cascade.py sweep` over `cascade-patterns.toml` (9 patterns, every control fired on the pre-pass masters,
baseline `76d6cca`), listing `.sweep.txt`. Patterns cover the amended claims' tokens AND their phrasings: the Rust floor
(`rustc 1.8x`, a toolchain-adjacent `1.8x`, `rust[-_]version`), the export sink's exclusivity (`ONE|only|sole|single
(deliberate )?exception`, `Downloads`), key persistence / creation (`persists across processes`, `key on first boot`),
the full-path NEVER-log enumeration (`L4_ALLOW_ROOT` root), the CLI-input entry point (`Reserved env vars`). Sections
read beyond the rows: architecture §Occupied Resources (Corpus SQLite, Filesystem locations), security-plan §Threat
Model, §Input Validation, §Data Protection → At rest, §Secret Management, §Logging & Monitoring, §Security
Anti-Patterns → Input / Logging, test-plan §4. Not looked for: obs-plan / a11y / design / layout prose (their agents
returned no proposal and no sweep pattern hit them except design-system's unrelated "only exception").

## Masters
| row | disposition |
|---|---|
| architecture.md:14 toolchain-floor / rust-version (new) | amended (A4) |
| architecture.md:47 toolchain-floor (new) | amended (A5) |
| architecture.md:353 toolchain-floor / rust-version (new) | amended (A7) |
| architecture.md:269 (control, now `# pin rustc 1.95.0`) | amended (A6) — no longer a row |
| architecture.md:103 rust-version (standing) — `app_info` example `"rust_version": "1.84.0"` | no change — illustrative envelope; the report carries no fact about what `app_info` reports (A8 rejected); named for the route entry "The declared Rust floor matches the code" |
| architecture.md:186 downloads-sink (standing) — `storage.export_for_training` route line, "the default `~/Downloads` egress sink" | no change — names the sink, asserts no exclusivity (window read, chars 0-379) |
| architecture.md:211 persists-proc (standing, edited) | amended in place (A3) — "persists across processes" kept true, concurrent convergence added on the same line; no intra-line duplicate of a retired claim |
| architecture.md:218 downloads-sink (standing, edited) | amended (A2) — "ONE deliberate exception" → "one of TWO" |
| security-plan.md:46 persists-proc (edited) | amended (S8) |
| security-plan.md:84 env-entrypoint (edited) | amended (S3) |
| security-plan.md:167 downloads-sink (new) | amended (S9, the new lock-file medium bullet) |
| security-plan.md:275 first-boot-key (edited) | amended (S5) — "derives … on first boot" kept, ONE-key-under-concurrency added |
| security-plan.md:296 persists-proc (edited) | amended (S6) |
| security-plan.md:341 full-path-set (edited) | amended (S11) |
| security-plan.md:393 downloads-sink (new, @c2605) | amended (S2, the Anti-Patterns → Input carve-out) |
| security-plan.md:454 full-path-set (edited) | amended (S10) |
| security-plan.md:470 toolchain-floor (standing) — "NEVER let the rust-toolchain drift below 1.85.0" | no change — a true minimum-pin ban, not a floor-of-code claim; named for the route entry "The declared Rust floor matches the code" |
| test-plan.md:343 toolchain-floor / rust-version (new) | amended (T2) |
| test-plan.md:422 rust-version (standing) — `rust_version` named as an `app_info` field | no change — a true field name |
| design-system.md:125, :369 one-exception (standing) — the Halo radial-gradient visual exception | no change — a different claim sharing the token |

## Leaves (re-derived from the now-current masters)
| row | disposition |
|---|---|
| CLAUDE.md:9 rustc-floor (GENERATED:setup:overview Stack line) | re-derived — pinned 1.95.0, code ≥ 1.89, stale declared floor + owner |
| CLAUDE.md:31 persists-proc (GENERATED:setup:modules corpus bullet) | re-derived — locked create-or-read, lock-file location, convergence |
| CLAUDE.md:44 (GENERATED:setup:warnings path-env line; no sweep row) | re-derived — the ratified `XDG_RUNTIME_DIR` carve-out appended (security-plan §Anti-Patterns → Input amended) |
| .claude/docs/stack.md:6, :58 rustc-floor | re-derived |
| .claude/docs/stack.md:84 toolchain-floor ("minimum was 1.84 — bump to 1.85.0") | re-derived — the old reconciliation closed (pin 1.95.0); the open half named with its owner |
| .claude/docs/commands.md:119-120 toolchain-floor ("rustup install 1.85.0 && override set 1.85.0") | re-derived — a non-amendment staleness the recompute fixes: `rust-toolchain.toml` pins 1.95.0 |
| .claude/rules/testing.md:17 rustc-floor (body, not Session Additions) | re-derived |
| .claude/rules/security.md:51 full-path-set (body) | re-derived — lock dir / lock file added to the covered set |
| .claude/rules/security.md:17 (body path-env rule; no sweep row) | re-derived — the ratified carve-out appended |
| .claude/rules/security.md:86-87 toolchain-floor ("MUST pin to 1.85.0 minimum") | no change — true minimum; mirrors security-plan :470 (unamended); named for the Rust-floor route entry |
| .claude/docs/security-summary.md:50 toolchain-floor ("NEVER let rust-toolchain drift below 1.85.0") | no change — same as :470 |
| .claude/docs/security-summary.md:13 (no sweep row) | re-derived — locked create-or-read + lock-file location (security-plan §Secret Management amended) |
| .claude/docs/services/corpus.md (no sweep row; provenance: arch modules) | re-derived — key-creation posture, first-launch prompt (stale `get_or_create_encryption_key` → `fetch_or_create_key`), skip-under-`env -i` gotcha, test count 35 → 87 |
| .claude/docs/gotchas.md:13-14 toolchain-floor ("1.84 → 1.85 bump") | no change — a dated historical gotcha, true as history |
| .claude/rules/design-tokens.md:41, :93 one-exception | no change — the Halo visual exception, a different claim |

## Curation homes and judgment bases
| row | disposition |
|---|---|
| .claude/docs/session-learnings.md:2399 rustc-floor (curation) — "pushes the effective floor to rustc 1.88+ for any project that compiles Tauri 2" | never edited by the cascade; still true as written (a floor of ≥ 1.88 from Tauri's tree); routed to P3 — no extension owed |
| base (playbook / drift-base) | 0 rows |

Provenance re-scan (`grep -l 'Extracted from .*(architecture|security-plan|test-plan)'` leaves × the retired wordings) → 0 remaining hits.
