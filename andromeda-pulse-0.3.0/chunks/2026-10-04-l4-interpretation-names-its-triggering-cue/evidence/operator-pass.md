# Operator pass — entries 21-31

Driven by hand on the overseer's word (founder-delegated, 2026-10-04: "Go: run the operator pass,
entries 21-31, as planned"), after a host reboot with the tree as /implement left it (HEAD
`5d6e344`, 16 modified/untracked paths, nothing running). Entries 19-20 stay deferred on the
Linux CUDA `llama-cli` (the `gated:` block). Exit codes are read from the bare command, with
output redirected to a file.

| # | entry | exit | reading |
|---|---|---|---|
| 21 | `gate.py hygiene` | 0 | `hygiene: clean — read 27 (runs 25 · evidence 2) · trails 12 not read · binary 0 not read by P1` |
| 22 | stage 1/6 `git ls-files -s scripts/agent-run.sh` | 0 | `100755 86d56bd3… scripts/agent-run.sh` |
| 23 | stage 2/6 `cargo xtask check:english-sources` (env -i) | 0 | `"verdict": "clean"` |
| 24 | stage 3/6 `npm ci` + `npm run build` (env -i, fresh `PUPPETEER_CACHE_DIR` under `target/pre-push/`) | 0 | `added 800 packages, and audited 801 packages` · `10 high severity vulnerabilities` (the standing reading) · `built in 1.97s` |
| 25 | stage 4/6 `cargo clippy --workspace --all-targets --all-features -- -D warnings` (env -i) | 0 | `Finished dev profile` |
| 26 | stage 5/6 `cargo xtask test` (env -i) | 0 | `Summary 2628 tests run: 2628 passed, 0 skipped` |
| 27 | stage 6/6 `cargo xtask ci-gates` (fresh seeded data dir under `target/pre-push/`) | 0 | zero-spans PASS · zero-panic PASS · heartbeat-gap PASS (max 15000ms) · perf-budget NEUTRAL (frame cannot-evaluate, no adapter record; memory and snapshot NEUTRAL) |
| 28 | bindings regen `--features mcp-server` `emit_taurpc_bindings` | 0 | `1 test run: 1 passed`. Before it, `git diff --quiet 5d6e344 -- bindings/index.ts` exited 1: stage 5 had re-emitted the no-mcp bindings again |
| 29 | `git diff --quiet 5d6e344 -- pulse-app/ui/src/bindings/index.ts` | 0 | bindings byte-identical to the chunk base |

Entries 30 (the push, after the pre-CI commit) and 31 (the CI read) are recorded in the wrap's
report; they follow this commit.
