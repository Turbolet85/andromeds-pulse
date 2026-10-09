
## 2026-10-07T09-50-55Z-wrap — Critical CVE response SLA: not in force before a first release
**Section:** §Dependency Security → CI integration, the `Critical CVE response SLA` clause
**Change:**
- The clause keeps its commitment unchanged: 72h from public advisory disclosure to Dependabot PR merge + tagged patch release, distributed by the Tauri updater + Homebrew/Scoop pipelines within the same window.
- It now states when that commitment binds. Was: no stated start, so it read as binding today. Now: not in force before a first release; the 72h window starts to bind with the first tagged release, and until then a fix lands on the version's build branch and reaches `main` with the version.
- The measured state is recorded beside it, dated 2026-10-07: 0 tags, 0 GitHub releases, `main` last moved 2026-06-12 (`93d9670`), so no window is running.
- The one advisory the question was raised on is named: GHSA-p6vx-979v-rg4c (seroval, critical, published 2026-10-05), fixed on the build branch `chore/migrate-pulse-to-v3` only, at chunk `2026-10-06-npm-supply-chain-gate-is-green-again`.
**Why:** The founder ruled it by dialog on 2026-10-07 (relayed by the pc overseer; this wrap was invoked on that relay). The SLA's end condition is a tagged patch release reaching users through the update channels, and none of those exists before a first release, so the window had nothing to bind to. Standing rule for later chunks: before the first tagged release a critical advisory is closed on the build branch and is not a reason to merge to `main` early; from the first tagged release the 72h window binds as written.
**Kept:** The 72h figure and the distribution sentence, word for word. What happens to an advisory disclosed before the first release and still open at it was not ruled and is not stated.
**Ref:** .andromeda/runs/2026-10-07T09-50-55Z-wrap/
