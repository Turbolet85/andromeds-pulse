# Curation — 2026-10-04-supply-chain-advisories-on-wasmtime-resolved

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   corrected verification-harness.md: "An `npm ci` exit 0 that prints `npm warn install-scripts … held back` …" — the remedy (remove the partial `~/.cache/puppeteer` folder) replaced by "run the stage with `PUPPETEER_CACHE_DIR` under `target/`; never delete the shared cache", tagged `[corrected 2026-10-04: …]` (a correction — exempt from the cap)
    Proof: PC22 at this chunk's operator pass — a fresh `target/pre-push/puppeteer` reproduced the partial v148 folders (chrome 21 MB, chrome-headless-shell 1.6 MB, no binary; the v148 zip intact, 308 entries incl. `chrome-linux64/chrome`, `testzip()` clean) while v152 installed whole; `npm ci` exited 0 over it (`chunks/2026-10-04-supply-chain-advisories-on-wasmtime-resolved/evidence/operator-pass.md` §PC22); founder ruling 2026-10-04 (2): do not delete the cache, isolate it.
  Tier 3 (.claude/docs/session-learnings.md): + "`grep` on the Linux dev host is ugrep, and a bounded-context pattern dies silently" (confidence 0.8)
    Proof: P1 of this wrap — seven `grep -oE '.{0,220}48\.0\.3.{0,260}'` site reads all exited 2 with `ugrep: error … exceeds complexity limits` and no match line; the same reads succeeded through a python extractor.
  Filters: 0 dup · 2 task-specific/low-confidence (`cargo audit` fetches the DB before scanning, 0.4; the cited run-file rewrite authority, a one-off operator decision about the pipeline's own gate, 0.4) · 0 conflict · 0 deferred by cap
  Recurrence (→ handoff Deferred learnings): `recurrence-despite-learning` — the bindings clobber by a default-features test run (`.claude/rules/security.md` 2026-05-19 / 2026-06-12; playbook gate-ORDER rule) recurred in the hand-run pre-push stages: stage 5 (`cargo xtask test`) re-emitted the no-mcp bindings after the implement-phase regen and the first cut of the pre-CI commit carried them; caught by reading the commit's stat before the push. The fact also rides the route entry "pre-push:linux runs natively on Linux" (P5 CARRY, overseer instruction), whose port must order the regen after its test stage.
  PC22 also rides that route entry (P5 CARRY, overseer instruction).
  CLAUDE.md size: see the health check 1 row in the P7 console.
