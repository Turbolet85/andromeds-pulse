# Cascade dispositions — 2026-10-09-supply-chain-job-same-on-push-and-pull-request

The sweep: `cascade.py sweep --patterns-file cascade-patterns.toml` (cascade v1.2), run after every body of this pass
was applied; baseline `b3e58597` (the parent of the pre-CI commit). Its last two lines, copied:

```
total (10 patterns) · 49 rows over 15 files
per class · new 5/3 · standing 18/5 · leaf 21/8 · curation 4/1 · base 1/1
```

## What was looked for

| Pattern | The claim it follows |
|---|---|
| `stops-app` · `before-webview` · `only-witness` · `no-ci-job` · `no-webview-rec` | "the CI boot job stops the app before the webview issues any IPC", so the boot log holds no webview record and no CI job witnesses `ui.webgpu.adapter` — by its verbs and by its "only witness" form |
| `no-adapter-rec` | the frame arm's CI line stated as one fixed text |
| `audit-action` | the audit step named as an action (`actions-rust-lang/audit`, `audit-check`) |
| `reaches-main` | "a fix … reaches `main` with the version" |
| `keys-no-job` | the six cache entries "on keys no job writes" |
| `cargo-audit` | every statement about the `cargo audit` step (`cargo audit` not followed by `able`) |

`no-ci-job` printed 0 rows with its control fired: a statement about that pattern only. Not looked for by this
sweep: the boot verb's readiness wording ("TCP handshake") — two test-plan proposals on it were escalated to the
operator and their sweep ran after the answer (the second sweep, below).

## Master and key-file rows (23: new 5 · standing 18)

- `security-plan.md:447` (`stops-app`, `before-webview`, both `edited`) — amended; the swept text that stands is the
  new sentence saying the earlier statement was true of one run. Kept as written.
- `obs-plan.md:545` (`before-webview`, `no-webview-rec`, `no-adapter-rec` ×3, all `edited`) — amended; the standing
  matches are the one-run reading ("stopped the app before the webview issued any IPC", "0 webview-originated") and the
  three `frame_cause` texts, two of them now with the runs they were read on. Read at the offsets `@c1215`, `@c1647`,
  `@c1819`, `@c1885`, `@c2578`.
- `obs-plan.md:559` (`no-adapter-rec`, `edited`) — amended: the `ci#36765040464` reading scoped to its run, the two runs
  that read the other text named.
- `test-plan.md:100` (`no-adapter-rec`, `edited`) · `test-plan.md:142` (`only-witness`, `new`) — amended.
- `test-plan.md:135` (`no-adapter-rec` ×2, `standing`) — no change: the first match lists `frame_cause`'s three
  outputs, a true description; the second is "reads `… no adapter record in this log` on CI (`ci#36765040464`)", a
  dated reading of a named run that asserts no rule.
- `architecture.md:249` (`no-adapter-rec`, `standing`, read at `@c13665`) — no change: `frame_cause`'s three outputs.
- `security-plan.md:216` (`audit-action` `new`; `cargo-audit` ×5 `edited`) — amended; the action name that stands is
  the retired step, named as what was replaced.
- `security-plan.md:226` (`reaches-main` `edited`; `cargo-audit` `new`) — amended: the sentence stands with its one
  dated departure.
- `security-plan.md:259` (`cargo-audit` ×2, `edited`) — amended: `actions-rust-lang/audit` left the action list.
- `security-plan.md:218`, `:220` (×6), `:224`, `:232`, `:236` (`cargo-audit`, `standing`) — no change: the build-fail
  condition (still true: exit 1 on a vulnerability), the ended deferral's history, the vendored channel, `cargo
  auditable` pairings. None names an action or a token.
- `test-plan.md:494` (`cargo-audit` ×4, `new`) — amended: the row gains the step.
- `registries/contracts/architecture/ci-cd-approach.md:3` (`keys-no-job` `edited`, read at `@c3191`; `cargo-audit`
  `new`, `@c813`) — amended: the audit step, the trigger pin, the fourth cache reading, which also qualifies the third
  reading's "keys no job writes".

## Leaf rows (21 over 8 files)

- `.claude/rules/observability.md:64`, `:99` (`stops-app`, `before-webview` ×2, `only-witness` ×2, `no-adapter-rec`
  ×2) — re-derived from obs-plan §8 / §10 as amended. Both lines stand above `## Session Additions` (`:123`).
- `.claude/docs/obs-summary.md:80` (`before-webview`, `only-witness`, `no-adapter-rec`) — re-derived.
- `.claude/rules/verification-harness.md:89` (`no-adapter-rec`) — no change: `frame_cause`'s outputs.
- `.claude/rules/security.md:78`, `:79`, `:80` (`cargo-audit`) — the three lines stand as true; the body gained one
  bullet above `:79` for the plain step (re-derived from security-plan §Dependency Security → CI integration).
- `.claude/docs/security-summary.md:37` (`cargo-audit`) — re-derived: the gate named as a plain `run:` step.
- `CLAUDE.md:91` · `.claude/rules/verification-harness.md:86` · `.claude/docs/commands.md:96`, `:98`, `:112` ·
  `.claude/docs/workflow.md:45`, `:65` (`cargo-audit`) — no change: the local command and its xtask wrapper.
- Leaves checked by provenance and not in the listing: `.claude/docs/tests-summary.md` names no audit step and no
  boot-job witness claim (`grep -n -i -E 'supply.chain|cargo audit|witness'`: its release-build line and two
  process-end lines); it is left as it is. CLAUDE.md's `GENERATED:setup` blocks name the `supply-chain` job only in the
  six-job list, which stands.

## Curation homes (4 rows, 1 file) and judgment bases (1 row)

- `.claude/docs/session-learnings.md:1678`, `:1680`, `:1685`, `:1688` (`cargo-audit`) — no change: an entry about
  running `cargo audit` after adding a dependency. Preserve-verbatim, and not stale.
- `.andromeda/playbook.md:64` (`cargo-audit`) — no change: a rule's own history of the advisory-database load failure.

## The second sweep — the readiness clause, after the operator's word at the route card

`cascade.py sweep --patterns-file cascade-patterns-2.toml`, run after the two escalated test-plan proposals were
resolved (inputs#I7) and their bodies applied. Patterns: `handshake` (`TCP handshake|handshake succeeds`) and
`ready-signal` (`Readiness signal|readiness poll|reports? ready`). Its last two lines, copied:

```
total (2 patterns) · 14 rows over 5 files
per class · new 1/1 · standing 10/4 · leaf 3/1 · curation 0/0 · base 0/0
```

- `test-plan.md:66` (`handshake` `edited`; `ready-signal` `new`) · `registries/contracts/test-plan/5-command-implementation.md:5`
  (`handshake` `edited`; `ready-signal` ×2 `edited`) — amended: the clause kept as the contract, the measured gap and
  its owner beside it.
- `registries/contracts/test-plan/5-command-implementation.md:34` (`handshake`) — no change: `cleanup`'s verification
  probes, a different claim.
- `architecture.md:217` (read at `@c1896`), `:249` (read at `@c7516`) (`handshake`) — no change: the pid-file row's
  liveness probes and `cleanup`'s verdict probes; neither says `boot` confirms a handshake.
- `architecture.md:249` (`ready-signal` ×3), `test-plan.md:134` (×2),
  `registries/contracts/test-plan/5-command-implementation.md:4` (×2) — no change: the pre-build split and the failed
  readiness poll's end-naming, true of the script as read.
- `registries/test-plan-contracts.toml:9` (`ready-signal`) — no change: the row's label list; no label was renamed.
- `.claude/rules/verification-harness.md:22` (`handshake`, `ready-signal`) — re-derived: the gap and its owner added
  to the boot line. `:25` (`handshake`) — no change: `cleanup`'s probes.
- `.claude/docs/tests-summary.md`: 0 hits for `handshake` or `readiness`; left as it is.

## Lateral binds

- test-plan §3 ↔ obs-plan §3 (harness commands): neither §3 was edited in this pass. The readiness wording in
  test-plan §3 is the escalated pair above.
- a11y-plan schema ↔ obs-plan schema: untouched.
