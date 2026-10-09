# Light gate (P7.1) — 2026-10-02-incident-events-readable-through-mcp

`gate.py run --plan …/plan.md --run-dir .andromeda/runs/2026-10-03T23-46-09Z-wrap --marker …` (no `--skip`, no
`--only`), Linux dev host, 2026-10-04. Summary line: `entries 22 · green 15 · red 1 (6) · recorded 0 · timeout 0 ·
not-run 6`. Workspace nextest 2575/2575 (entry 13); targeted `incident_events` 16 run, `lacks [skip]` held (entry 3);
`package(mcp-server)` 87/87 (entry 5); bindings byte-identical to `a69030a` (entry 15); release build green (entry 16).

## The ASSERT, per entry
- 1–5, 7–16 — green by their `expect`.
- 6 `git diff --name-only a69030a… -- Cargo.lock deny.toml pulse-app crates/corpus/src/schema.rs crates/ui-bridge` —
  `red · no output`: prints `pulse-app/src/inference_runtime.rs` and `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs`,
  exactly the two pulse-app `widening` lines of `scope-record.md`. **Owned red, superseded by the founder ruling of
  2026-10-03 (option A — the shared event vocabulary written by the producer, proven by the real-producer leg; the
  widening approval)**, recorded so on the operator's direction of 2026-10-04 (relayed by the overseer). The rest of
  the probe holds: no `Cargo.lock` / `deny.toml` / schema / ui-bridge path is printed.
- 17–22 — `not run — leg operator`; re-verified by the results recorded in `chunks/{marker}/evidence/` for the final
  HEAD S2 `cdb6c1e` (no commit since but this wrap's docs-only one):
  - 17 hygiene — `hygiene: clean` (`operator-pass.md`, path remap on the operator's word).
  - 18 `pre-push:linux` — DEVIATION (founder ruling 2026-10-03): the verb needs `wsl.exe`; its six stages ran natively,
    env-isolated, verdict green on the re-run (`operator-pass.md`). Owner of the port: route entry "pre-push:linux runs
    natively on Linux".
  - 19 `git rev-parse HEAD` — S2 `cdb6c1ed572761ae384597a7ed437222e3a1d1fc` (`round-binary.md`).
  - 20 push — exit 0, `4a26ad8..cdb6c1e` (`round-binary.md`).
  - 21 `ci.py conclusion` — `ci#37157540938` 11/12, red `supply-chain` only. **red — not this chunk's: two-sided
    probe — `cargo deny check advisories` FAILED on the parent `a69030a` in a clean worktree AND on HEAD with the same
    three vulnerabilities (RUSTSEC-2026-0325 / -0326 / -0327 on `wasmtime` 48.0.3; the RUSTSEC-2024-04xx rows are the
    standing ignore-listed warnings), `Cargo.lock` + `deny.toml` byte-identical between them → owner: route entry
    "Supply-chain advisories on wasmtime resolved"** (founder ruling 2026-10-04: it does not block this chunk).
  - 22 Conductor evidence names S2 — exit 0, 3 hits; the seven verdicts read from Conductor's committed ledger at
    `e6e1eef` (`round-result.md`), 7/7 PASS.
