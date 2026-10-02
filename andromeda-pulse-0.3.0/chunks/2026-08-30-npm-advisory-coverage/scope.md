# Scope — 2026-08-30-npm-advisory-coverage

## Intent (working entry, surface)
npm advisory coverage — the webview devDependency channel is scanned for advisories, not only
version-bumped. Owner-named by the security-plan §Dependency Security amendment applied at the
`2026-08-23-webview-self-verify` wrap; the gap PRE-EXISTS that chunk and was widened by it.

## What this chunk builds
- **Advisory / license / ban scanning for the npm channel** at `pulse-app/ui` (the workspace's only
  npm manifest), so an advisory against any npm dependency surfaces as a **failing or visibly
  dispositioned gate**, never only as a Dependabot version bump. The triad mirrors the cargo side:
  `cargo audit` + `cargo deny check bans licenses sources` + `cargo-auditable` all stop at the Rust
  boundary today.
- **CI wiring**: `.github/workflows/ci.yml` runs `npm ci` (:92) and `npm run build` (:114) with no
  scanning step — the gate must run per-PR there, and be runnable locally with a deterministic,
  machine-parseable invocation (Development Style: agent-driven). VERIFIED (P3): the local
  invocation is a `cargo xtask` subcommand — arch locks the task runner to cargo-xtask;
  `Cmd::Audit` / `Cmd::DenyBans` are the existing supply-chain verb precedents and
  `run_npm_script` (xtask/src/main.rs:1093, 3 dispatch callers) the npm-spawn precedent.
- **Disposition discipline, ported**: per security-plan §Supply chain + CI — a finding WITH a stated
  safe upgrade is NEVER ignore-listed (named owner, stays red until upgraded); only no-safe-upgrade
  findings take an ID-scoped documented exception with provenance + closing condition; designed-red
  observed signals stay SEPARATE from pass/fail gates (the 2026-08-17 two-invocation lesson).
  VERIFIED (P3): `npm audit` has NO native ID-scoped exception mechanism (`--audit-level` is
  threshold-only), so the exceptions live in a project-owned policy file evaluated by the xtask
  gate — the selection criterion the security extract predicted, resolved by measurement.
- **Native-binary coverage**: the scan must reach the lockfile-delivered platform packages
  (`@crabnebula/tauri-driver-*`, incl. `-win32-x64-msvc` — a native `.exe` via npm) that no
  cargo-side gate inspects.
- **First-scan findings get dispositions in-chunk** — the gate lands meaningful, not
  undifferentiated red. Mirrors the cargo advisories visible-disposition rule and the operator's
  standing in-chunk-fix preference. MEASURED (P3, 2026-08-30): the first scan is `npm audit`
  exit 1 with **39 findings — 1 critical (vitest) / 20 high / 16 moderate / 2 low**; direct-dep
  hits: vitest (critical, fix within ^3), vite (fix within ^7), lighthouse, pa11y (only fix =
  10.0.0, a MAJOR crossing the a11y-plan §3 pin), pa11y-ci, webdriverio (suggested "fix" is a
  major DOWNGRADE to 8.14.6 — exception-class).

## Verified coordinates (re-derived at HEAD, 2026-08-30)
- `.github/dependabot.yml` — npm ecosystem at `/pulse-app/ui`, weekly: present. Bumps only.
- `.github/workflows/ci.yml` — `npm ci` :92 · `npm run build` :114 · no audit/license/ban step.
- `pulse-app/ui/package.json` — **29 direct devDependencies + 9 runtime dependencies**; no
  manifest-level `optionalDependencies`; `@crabnebula/tauri-driver ^2.0.9` (devDep) resolves 5
  platform packages in `package-lock.json` (910 lockfile packages total).
- Zero existing npm scanning tooling anywhere (xtask sources, workflows, package.json scripts).

## Premise correction (measured; outcome unchanged, case strengthens)
The entry's CONTEXT states "Everything in the channel is dev/harness-only with no runtime or bundle
path." **Measured FALSE as a whole-channel claim**: `package.json` carries 9 runtime `dependencies`
(react / react-dom 19, `@tauri-apps/api`, `@tauri-apps/plugin-clipboard-manager`,
`@tanstack/react-router`, `react-aria-components`, `focus-trap-react`, `motion`, `tabbable`) that
Vite bundles into the shipped webview — a direct runtime path into the product. The dev/harness-only
bound holds for the 29 devDependencies only. The scan must cover BOTH classes; the blast-radius
framing in any artifact this chunk touches must not repeat the whole-channel claim.

## PREREQ folded (audit pin #22 — verified against security-plan §Supply chain + CI)
The `cargo audit` standing deferral rides this entry. Session arithmetic: `state.yaml`
`session_count: 58` → this chunk's wrap is session **59 = BETWEEN-POINT** (next FULL-FORM interval
point: session 61). Owed at this chunk's wrap, and recorded in its report — never a silent skip:
- re-verify the basis first-hand (`cargo audit` true exit read directly; expected: exit 1,
  `duplicate advisory ID: RUSTSEC-2026-0244`, cargo-audit 0.22.2);
- re-verify the overlap (`cargo deny check advisories`) re-enumerating DISTINCT `RUSTSEC-` ids from
  scratch — currently GREEN (owned upgradeable set EMPTY); a NEW finding gets a visible disposition,
  never a quiet return to red;
- record `probe skipped per ratified interval (next: 61)`. No "Nth consecutive" ordinal.

## Boundaries (not in scope)
- No runtime/bundle code changes; no `.rs` production changes expected.
- No Rust-side gate rework (`cargo deny` pair green at HEAD; the designed-red posture is settled).
- `dependabot.yml` untouched — bumps continue; this chunk adds the missing scanning half. (P3
  closure: no research finding requires touching it.)
- Not the other markerless tail entries (Diagnostics un-muting · Staged-bindings assertion ·
  ACL-rejection logging) — adjacent supply-chain/harness work stays theirs.
