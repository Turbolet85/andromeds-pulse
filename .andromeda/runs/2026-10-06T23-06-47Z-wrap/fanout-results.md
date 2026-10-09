# Fan-out results — 2026-10-06-npm-supply-chain-gate-is-green-again

Seven doc-agents, one parallel batch, 19 detectors (the drift-base's 19 `doc:` names: arch 2 · security-plan 4 · design-system 2 · layout-templates 2 · test-plan 3 · obs-plan 4 · a11y-plan 2). Every return parsed as YAML as sent (the commentary arrived as YAML comments, so stripping removed nothing and no raw twin is owed). Entity probe over the seven returns, scripted: `entities=0` on each.

## Verdicts
- **architecture** — `proposals: []`. Both detectors hold: no new resource, no new library or runtime. Its sweep: the one `green-with-dispositions` line is the six-arm contract list, not a current-state claim; `extract-zip`, `seroval`, `source-map-js` 0 hits. All four keyed-contract files read, none touched.
- **security-plan** — 1 proposal (below). D-security-input, D-security-auth, D-security-logging hold. It noted, and did not propose, the `Critical CVE response SLA` line: open and founder-owned per the report.
- **design-system** — `proposals: []`. No new UI element; no status claim moved. The accent token and "the radix family is lockfile-absent" still hold.
- **layout-templates** — `proposals: []`. No surface added; no status claim moved.
- **test-plan** — `proposals: []`. All three detectors hold. It reported one out-of-detector observation, not a proposal: §9 Supply chain row says "node_modules is never installed in this job", while the report reads `npm ci` at step 15 of that job. Disposition below (orchestrator-raised).
- **obs-plan** — `proposals: []`. All four detectors hold; no §10 defect narrative, no §8 backlog item and no owner pointer is touched.
- **a11y-plan** — `proposals: []`. Both detectors hold. The one site naming the exception (`:39`) still holds. It opened 2 of its 10 key files and covered the other 8 by grep.

## Proposals and dispositions

### S1 — security-plan · D-security-deps · filed `warning`
- section: §Dependency Security → npm channel (`pulse-app/ui`), the dated current-state run
- change (as proposed): replace the "Re-read at chunk `2026-09-29-scrubber-path-false-positive` … `package.json` untouched." run with a re-read at this chunk: exit 0 `green-with-dispositions`, the same four exceptions, `unexcepted []`; three advisories (seroval GHSA-p6vx-979v-rg4c, GHSA-jp82-f5mq-hwhp; source-map-js GHSA-68fv-2mgg-jv7q) closed by in-range lockfile bumps (seroval and seroval-plugins 1.5.4 → 1.6.8, source-map-js 1.2.1 → 1.2.2), no exception, `package.json` and `npm-policy.json` untouched; the "Current state (…p-025…)" description stays.
- basis: `.andromeda/security-plan.md:222` · its own sweep: `Re-read` 1 · `green-with-dispositions` 1 · `extract-zip` 1 · `four exceptions` 1 · `Current state` 1 · `seroval` / `source-map-js` / `findings-red` 0 → single site, no `dependent-of`.
- **Disposition: APPLY.**
  - Check 1 (playbook): the detector carries `escalate`, and the finding is outside the class that severity guards — the report's Dependencies bullet reads none added, the ban arm `hits []`, the license arm `violations []`, so no banned or unvetted dependency exists (the 2026-08-23 escalate-by-actual-class rule, both conditions held). Its actual class is "Accurate this-chunk addition" → routine. It is also the plan's own `Expected amendments (wrap)` entry, which names the change.
  - Check 2: no opposing proposal. Check 3: matches the chunk's intent. Check 4: line 222 is over 2 000 chars and was read whole through the Read tool before the edit; the cascade sweep re-read every hit by offset (`cascade-dispositions.md`). Check 5: this is the plan's one expected amendment — matched.
  - Applied text re-derived, not pasted. Two differences from the proposal: the reading is scoped to the build branch it was taken on, and "entered the feed after the prior green run" is narrowed to what was measured — the seroval advisories were published after that run; the source-map-js one was published 2026-09-18 and updated 2026-10-05, and why the gate first reported it then is written as recorded, not established. A one-clause pointer to the 2026-09-29 close is kept; its version numbers retire to the sidecar.

### O1 — test-plan · orchestrator-raised (Validate check 6)
- section: §9 CI Integration → Pipeline structure, the Supply chain row (`.andromeda/test-plan.md:494`)
- claim measured false: "deliberately NO `npm ci` (… so node_modules is never installed in this job)". The job ran `npm ci` and `npm run build` after the gate step on `fc4ebdf` (report, Spec claims disproved by measurement; `.github/workflows/ci.yml` read at HEAD: the two steps sit between the gate and `Install cargo-auditable`).
- **Disposition: APPLY.**
  - Check 1 (playbook): the 2026-08-14 rule — a doc claim a pre-existing reality falsifies, the implementation already correct, the fix completing in one artifact. Its one question, "is there an impl half to fix?": no. Sibling readers were read and do not carry the false form: security-plan `:222` and `:259` and architecture `:249` say "no `npm ci`" of the gate or the verb, which is true. → routine-APPLY. The "Not this chunk's drift" reject rule does not govern: its own caution sends an accurate, real, single-artifact case to the rule above.
  - Check 4: the absence of other false sites rests on `grep -rn 'never installed\|node_modules is never'` over the masters, registries, leaves and bases (test-plan 1, security-plan 1 true-of-the-gate, the rest 0) and on the cascade sweep's `no-npm-ci` rows, each dispositioned.
  - Check 6: the report entry ends DISPOSED by this amendment. The report's second entry (the research.md install-scripts bullet, corrected in the chunk's own window) is recorded in the report; no amendment owed.

## Escalations
None. No boundary widening, no two-rule collision, no cross-contradiction, no unjustified divergence from intent (the scope record holds no line).

## Result
2 amendments applied (security-plan 1 · test-plan 1) · 0 escalations · cascade: 0 further master sites, 0 leaves re-derived (`cascade-dispositions.md`) · sidecar entries appended and read back as each file's last lines.
