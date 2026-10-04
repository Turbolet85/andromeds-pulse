
## 2026-10-02-incident-events-readable-through-mcp — the dev host moves to Linux; pre-push:linux cannot run there
**Section:** §3 Per-chunk gate discipline → `cargo xtask pre-push:linux` · §3 Process-end witness form (integration tier)
**Change:**
- `pre-push:linux`: was "dev-host only (a Windows host with the WSL `Ubuntu` distro)"; now it requires a Windows host with WSL (`wsl.exe`), so since the dev host moved to Omarchy Linux (2026-10-03) it cannot run on the dev host — its six stages ran natively and env-isolated in its place under a founder ruling (green re-run, 2575/2575), and the native port is its own route entry, placed after "Retry-storm interpretation names its cause".
- Process-end witness form: was "`cfg(unix)` arms … unrunnable on the Windows dev host, run by `cargo xtask pre-push:linux` and CI lint-test Linux/macOS" (per the 2026-10-01-conductor-return entry, whose other claims stand); now they run natively in `cargo nextest run --workspace` on the Linux dev host (2575 = 2573 + the two arms) and in CI lint-test Linux/macOS, unrunnable only on a Windows host.
**Why:** the founder moved the dev host to Linux (2026-10-03); the two places that named Windows as the dev host and `pre-push:linux` as its Linux runner no longer described where the gates run.
**Ref:** .andromeda/runs/2026-10-03T23-46-09Z-wrap/
