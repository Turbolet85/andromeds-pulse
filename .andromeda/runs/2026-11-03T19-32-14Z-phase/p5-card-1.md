# P5 review — 2026-11-03-advisories-since-2026-10-12-closed · «Advisories since 2026-10-12 closed»

Plan: `doc:2026-11-03-advisories-since-2026-10-12-closed/plan` rev 4, rendered for an editor at
`.andromeda/cache/docs/2026-11-03-advisories-since-2026-10-12-closed/plan.md`. No plan stands in a chunk's folder;
writing in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py
diff --root . 2026-11-03-advisories-since-2026-10-12-closed` is a diff of the chunk's records. Scope rev 4, research rev 3.

## Goal
Turn the CI `supply-chain` job green again. At HEAD 0e13e255, `cargo audit` reads 42 vulnerabilities over 38 distinct
ids, the same as the job did (`ci#41582102601`, its log read at P5). All of them, plus the tokio, event-listener and
zstd-safe unsound warnings, close by upgrade, with no ignore where an advisory states one. The npm gate behind the
audit is red too (22 GHSA findings): 19 close by in-range updates, and 3 take exceptions. No product behaviour changes.

## The founder's four answers (`inputs#I2`, his own word, P4 dialog)
1. duckdb / libduckdb-sys 1.10505.0 → 1.10506.1: zip 6 → 9 as a build dependency, DuckDB 1.5.x kept.
2. tauri-plugin-updater 2.10.1 → 2.14.1: zip 4 → 9, with Tauri 2.11.0 → 2.12.5 and tao 0.35 → 0.37, all lockfile moves.
3. zbus_names / zvariant / zvariant_derive (RUSTSEC-2026-0371 / -0370 / -0372, unmaintained, no solution) get ID-scoped
   `deny.toml` ignores with provenance.
4. npm minimatch / glob / zod (GHSA-g3hv-75wg-m249 / -3g3m-2fjr-6pw4 / -mrhx-c7xc-m3w5, dev-only, no forward fix in
   their holders' ranges) get GHSA-scoped exceptions in `npm-policy.json`.

## Touchpoints (6 files; no source file planned)
`Cargo.toml` (wasmtime requirement "48.0.4" → "48.0.8" and its provenance comment) · `Cargo.lock` · `deny.toml` ·
`pulse-app/ui/package-lock.json` · `pulse-app/ui/package.json` (only if an update needs it) ·
`pulse-app/ui/npm-policy.json`. A code fix that a bumped crate forces lands where the build shows it, recorded as an
out-of-list edit (the scope guard reads it).

## Found at P5, folded in (validation-1: intent-incomplete, scope amended to rev 4)
- `cargo deny check advisories` already reads ten `advisory-not-detected` at the chunk base. The ten GTK3 ignores
  (RUSTSEC-2024-0411 … -0420) name advisories the database withdrew on 2026-08-14. Step 6 prunes them (prune-not-keep),
  along with the five unic-* ignores the Tauri bump makes undetected.
- In a scratch clone carrying the two founder-ruled updates, `cargo deny check bans` reads six unnecessary skips
  (zip, phf ×4, png) and eight new Windows-target duplicates (`windows`, `windows-collections`, -core, -future,
  -numerics, -result, -strings, -threading). Step 6 prunes the skips and adds ID-scoped skips with provenance for the
  duplicates. `multiple-versions = "deny"` and the tonic canary are untouched.

## Top acceptance criteria
- `cargo audit` exits 0. None of the 38 vulnerability ids remains, nor the tokio / event-listener / zstd-safe unsound
  ids, and no new id appears.
- No zip below 9 and no unic-* crate remains in the lockfile. wasmtime's requirement and resolution are 48.0.8, and
  the Cranelift assertion holds.
- `cargo deny check bans licenses sources` exits 0 with no `unnecessary-skip`. `cargo deny check advisories` reads no
  `advisory-not-detected`.
- The npm gate exits 0, and `npm-policy.json` holds exactly 5 exceptions (2 standing + the founder's 3).
- Keyring's four platform features are byte-for-byte unchanged, and nothing outside the six files changes.
- The standard gates, the bindings close, the window boot series (Tauri 2.12.5), the console engine cycle (DuckDB
  1.10506.1, tokio 1.54) and `pre-push:linux` are green.
- The pushed tip's CI run reads `verdict: green` on every check, with supply-chain's later steps run.

## Test commands
37 entries; 16 are new and baseline-run at P5:
- **Red on the subject:** the audit, the zip / unic / wasmtime reads, the npm gate and the policy count.
- **Red on an existing property, now owned by step 6:** deny advisories (the withdrawn GTK3 ignores).
- **Green:** the guards (each with a scratch-clone control on both sides), the Cranelift and OTel SDK reads, deny
  bans, the bindings close, the a11y list, the boot series (95.1 s) and the engine cycle (18.35 s).
- **Must-pass readings:** zip, unic, deny bans and deny advisories, each over a clone carrying the subject.
- **Operator legs (4):** hygiene, merge-base, push, CI read.
- **Not run locally:** `test:a11y` (no Playwright headless shell on this host; the CI a11y job reads it) and
  `cargo auditable build` (CI's supply-chain job).

## Capabilities
None claimed: the 0.4.0 pool holds no supply-chain acceptance (P-083 and P-121, the audit-worded caps, belong to other
entries). sampler: off.

## Expected amendments (wrap)
- **architecture:** wasmtime 48.0.8 and the closed ids; DuckDB 1.10506.1; Tauri 2.12.5 where a version is named.
- **security-plan:** the pinned-version sentences, the new ignores and exceptions on the founder's word, the pruned
  GTK3 / unic ignores and skips, the Windows duplicate skips, and the false «zip is not in the graph» sentence.
- **test-plan §4 / §5:** wasmtime / cranelift / pulley versions.
- **Carry `cry-rdhtyshr` rev 3:** rides this chunk's wrap.

## Beyond the plan
none
