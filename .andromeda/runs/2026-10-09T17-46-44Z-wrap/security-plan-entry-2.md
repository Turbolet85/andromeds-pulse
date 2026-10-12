
## 2026-10-09-supply-chain-job-same-on-push-and-pull-request — one dated departure from "reaches main with the version"
**Section:** §Dependency Security → Critical CVE response SLA
**Change:** The sentence "until then a fix lands on the version's build branch and reaches `main` with the version" stands, and is followed by one dated departure: on 2026-10-09 the repair of the `supply-chain` job's `cargo audit` step reached `main` ahead of the version, by a separate pull request (#41, merge commit `178ebac`) that the founder merged himself; `main` had read red on its push run since `60ef43c`, and that step was the cause.
**Why:** Ratified by the founder, 2026-10-09, by dialog — relayed verbatim by the pc overseer: the red `main` is repaired now by a separate small pull request he merges, and the push his merge makes is the witness of the repair. The departure is this one repair's; it sets no rule that fixes go to `main` early. A later chunk that wants the same asks him again.
**Kept:** The 72h window, "not in force before a first release" and the 2026-10-07 measurements in the paragraph are untouched.
**Ref:** .andromeda/runs/2026-10-09T17-46-44Z-wrap/
