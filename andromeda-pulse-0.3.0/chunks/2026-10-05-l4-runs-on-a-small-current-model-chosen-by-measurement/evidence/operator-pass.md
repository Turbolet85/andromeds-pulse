# Operator pass (plan entries 28–38)

Authority: overseer (founder-delegated), 2026-10-05: "Go: the operator pass, entries 28-38 as planned … Report the CI
verdict and stop before the wrap." Each entry was driven once by hand in its plan text from the repo root. Exit codes
were read from the bare command.

| entry | stage | exit | reading |
|---|---|---|---|
| 28 | `gate.py hygiene` | 0 | **refused 1 file** (P1 host path): `.andromeda/runs/2026-10-05T06-18-42Z-phase/p5-dryrun.txt`, a saved phase-run `gate.py` dry-run capture. Its header lines 3–4 (repo root, tmp gate-log dir) were placeholdered. A re-run still refuses it at line 41 (`home`). Further edits to that phase-run file were blocked by the session's permission layer as audit-trail tampering, so it is OPEN for the operator. 0 host paths in this chunk's evidence or inputs. |
| 29 | stage 1/6 script-modes: `git ls-files -s scripts/agent-run.sh` | 0 | `100755` |
| 30 | stage 2/6 source-lint: `cargo xtask check:english-sources` (env -i isolation) | 0 | `"verdict": "clean"` |
| 31 | stage 3/6 npm: `npm ci` + `npm run build`, fresh `PUPPETEER_CACHE_DIR` `target/pre-push/puppeteer-20261005T090636Z` | 0 | built (the chunk-size advisory only) |
| 32 | stage 4/6 clippy `--workspace --all-targets --all-features -D warnings` | 0 | finished, 0 warnings |
| 33 | stage 5/6 `cargo xtask test` | 0 | `2654 tests run: 2654 passed, 0 skipped` |
| 34 | stage 6/6 `cargo xtask ci-gates`, fresh seeded data dir `target/pre-push/data-20261005T090821Z` | 0 | heartbeat-gap PASS (max gap 15000 ms); perf-budget NEUTRAL over the seed log, as designed |
| 35 | regen re-fired after stage 5: `nextest -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` | 0 | 1 passed |
| 36 | base-identity close: `git diff --quiet 4e5b595… -- pulse-app/ui/src/bindings/index.ts` | 0 | byte-identical to the chunk base |
| 28 (re-fired) | `gate.py hygiene` | 0 | **`hygiene: clean`** (read 53: runs 32 · evidence 13 · inputs 8; 0 host paths kept) |

**Provenance of the `p5-dryrun.txt` scrub.**
- /implement placeholdered header lines 3–4 (repo root → `{repo-root}`, tmp gate-log dir → `{tmp}/…`). It stopped
  there when the permission layer blocked further edits to that phase-run file.
- The remaining host paths (lines 41 and 51) were scrubbed by the OVERSEER at the FOUNDER's explicit instruction
  (2026-10-05, about 10:00, the founder away from the desk). Those lines now read `~/.claude/skills/…`, the form of
  the committed `2026-10-04T00-48-08Z-phase` instance, with 0 home paths remaining (overseer relay). This
  implement session did not edit those lines.

