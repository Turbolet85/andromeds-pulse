# P-075 round binary — 2026-10-01-conductor-return

- **S = `03ec94481b0d6c3ba574626e7acb39e33fd40141`** — `chore(2026-10-01-conductor-return): operator pre-CI commit`, on
  `chore/migrate-pulse-to-v3`, parent `a2addb3`. Printed by plan gate 22 (`git rev-parse HEAD`) on 2026-10-02.
- Build command: `cargo build --workspace --release --features mcp-server` (plan gate 17, green, 645.82 s).
- Artifacts: `target/release/pulse-app.exe` and `target/release/andromeda-pulse-mcp.exe` (Windows dev host).
- Source identity: gate 17 built the working tree whose source equals S. Between that build and the S commit, only
  evidence documents, run-dir trails and the friction ledger changed; no `.rs`, manifest, lockfile, capability or webview
  file. A rebuild at S is a no-op for these binaries.
- Relay: to conductor-builder by the overseer, with `round-request.md` (the six graded assertions, the deterministic mode,
  the grading posture).
