group: a11y-plan
read: .claude/docs/a11y-summary.md · 109 · «Full plan: `.andromeda/a11y-plan.md`.»
read: .claude/rules/a11y.md · 125 · «- 2026-08-26: ESLINT'S `--rule` CLI FLAG CANNOT CARRY A PLUG»
read: CLAUDE.md · 156 · «<!-- USER:session-learnings end -->»

- leaf: .claude/rules/a11y.md:81   class: stale
  leaf-text: «`lighthouse` 12.x CLI»
  source: .andromeda/a11y-plan.md:46 «**Lighthouse** 13.x CLI for CI gate + **pa11y** 10.x for parallel rule matrix (majors moved at `2026-08-30-npm-advisory-coverage`)»
  replace-with: «`lighthouse` 13.x CLI»

- leaf: .claude/rules/a11y.md:82   class: stale
  leaf-text: «`pa11y` 9.x / `pa11y-ci` 4.x»
  source: .andromeda/a11y-plan.md:46 «**Lighthouse** 13.x CLI for CI gate + **pa11y** 10.x for parallel rule matrix (majors moved at `2026-08-30-npm-advisory-coverage`)»
  replace-with: «`pa11y` 10.x / `pa11y-ci` 4.x»

- leaf: .claude/rules/a11y.md:12   class: stale
  leaf-text: «§3 (Harness Contract) + §3.5 (Bootstrap phases)»
  source: .andromeda/a11y-plan.md:279 «Contracts: .andromeda/registries/a11y-plan-contracts.toml — ask registry.py contracts; read one contracts/a11y-plan/{key}.md; never whole.»
  replace-with: «§3 (Harness Contract; keyed: `.andromeda/registries/a11y-plan-contracts.toml`, the Bootstrap phases key among them)»

- leaf: .claude/rules/a11y.md:74   class: omission
  leaf-text: «live updates do NOT steal focus.»
  source: .andromeda/a11y-plan.md:228 «**The re-poll preserves the focused row** (roving `tabindex` state is clamped, never reset — P-081's verified acceptance)»
  replace-with: «live updates do NOT steal focus, and the re-poll preserves the focused row (roving `tabindex` state is clamped, never reset).»

- leaf: .claude/docs/a11y-summary.md:29   class: omission
  leaf-text: «Live updates do NOT steal focus |»
  source: .andromeda/a11y-plan.md:228 «**The re-poll preserves the focused row** (roving `tabindex` state is clamped, never reset — P-081's verified acceptance)»
  replace-with: «Live updates do NOT steal focus; the re-poll preserves the focused row (roving `tabindex` clamped, never reset) |»

- leaf: .claude/docs/a11y-summary.md:18   class: stale
  leaf-text: «12 with documented reasons»
  source: .andromeda/a11y-plan.md:496 «~17 SCs explicitly marked non-applicable per Section 3 applicability list with documented reasons»
  replace-with: «17 with documented reasons»

- leaf: .claude/docs/a11y-summary.md:44   class: stale
  leaf-text: «## Bootstrap phases (a11y-plan §3.5)»
  source: .andromeda/a11y-plan.md:279 «Contracts: .andromeda/registries/a11y-plan-contracts.toml — ask registry.py contracts; read one contracts/a11y-plan/{key}.md; never whole.»
  replace-with: «## Bootstrap phases (a11y-plan §3, keyed contract `Bootstrap phases (derive for route / setup-project)`)»
