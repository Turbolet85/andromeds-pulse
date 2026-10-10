# Cascade step 2 — the sweep's dispositions

The listing: `cascade.py sweep --patterns-file cascade-patterns.toml` (cascade v1.2), run after every spec body of
this pass was applied and before any derived doc was touched. Baseline `8f655cae`. Its last lines, copied:

```
total (13 patterns) · 48 rows over 12 files
per class · new 7/4 · standing 14/5 · leaf 25/6 · curation 2/2 · base 0/0
```

## What was searched for

Thirteen patterns over the seven masters, every `.andromeda/registries/**` file, the three curation homes, the two
judgment bases and the leaf bodies; each pattern's control fired on the pre-pass masters.

| id | pattern | what it stands for |
|---|---|---|
| `wsl` | `wsl`, any case | the second operating system the check ran through |
| `pre-push` | `pre-push` or `pre_push`, any case | every site that names the check, whatever it says of it |
| `clone` | `distro clone`, `in the clone`, `the clone` | the copy of the repository the stages ran in |
| `windows-host` | `Windows host` | where the check could run |
| `apt-remedy` | `apt remediation`, the apt remediation line, `apt list` | the retired provisioning pin and its one install line |
| `eight-members` | `missing[], remediation` or `cache{bytes` | the verdict document's two retired members |
| `viola-node` | `viola-node` or `Viola repo` | the Node install of another project inside that system |
| `no-new-env` | `reads no new env var` | the row's retired claim |
| `cannot-run` | `cannot run on the dev host` | the key file's retired claim |
| `patch-sync` | `binary patch`, `sync-mismatch`, `40 GiB` | the sync, its tree-id check, the clone's cache cap |
| `stage-5` | `pre-push stage 5` or `stage 5` | the hand-run stage test-plan §4 named |
| `settle-line` | `reaches the settle verdict` | the boot job's `ci-gates` line claim and its neighbours |
| `node-22` | `Node 22` | the distro's own Node, the reason for the other project's install |

Not searched for: the mechanism's verbs alone (`syncs`, `caps`), which stand in many unrelated rows of these
documents; the three amended lines were read whole instead (below). Sections read whole: architecture §Occupied
Resources → xtask CLI surfaces and → Environment variables; the key file's `pre-push:linux` paragraph; test-plan §4's
corpus bullet and §10's frame row; obs-plan §10's frame row and CI gates; security-plan §Threat Model Summary's CLI
row, §Input Validation's CLI / env var row, §Security Anti-Patterns → Input.

## Zero-row patterns

`eight-members`, `no-new-env`, `cannot-run`: 0 rows, each control fired. A statement about those three wordings:
after the pass no master, key file, leaf, curation home or base holds them.

## Masters and key files — 21 rows (new 7, standing 14)

- `architecture.md:247`, `:248` (`pre-push`, new) — this pass's two Environment variables rows. Amended.
- `architecture.md:252` (`pre-push`, standing edited, ×12) — the xtask CLI surfaces line. Eleven matches are this
  pass's rewritten `pre-push:linux` row; the one at `@c6472` is the `check:english-sources` row's "and as
  `pre-push:linux`'s `source-lint` stage", read at that offset: true as it stands, no change.
- `security-plan.md:85`, `:138`, `:395` (`pre-push`, new) — this pass's three additions. Amended.
- `test-plan.md:227` (`pre-push` ×3 and `stage-5`, standing edited / new) — this pass's rewritten clean-skip clause.
  Amended; the line was re-read whole for a second statement of the hand-run stage: none.
- `registries/contracts/test-plan/per-chunk-gate-discipline.md:32` (`pre-push` ×9, `wsl` ×2, `windows-host`, standing
  edited) — this pass's rewritten paragraph. The two `wsl` matches and the `Windows host` match are its history
  sentence and its "Readings of the WSL form" label: true dated history, kept on purpose. Amended.
- `test-plan.md:134` (`wsl`, standing) — "30 WSL trials", the dated evidence of the `agent-run` wrapper's teardown.
  Another subject sharing the token; no change.
- `test-plan.md:223` (`clone`, standing) — "asserts the clone is a DISTINCT `Arc`", a database connection clone.
  Another subject; no change.
- `per-chunk-gate-discipline.md:42` (`windows-host`, standing) — the process-end witness form's "on a Windows host
  they are unrunnable", read at its offset. Another subject; no change.
- `settle-line`, seven rows:
  - `test-plan.md:100` (standing edited) and `obs-plan.md:545` (new) — amended by this pass: the `ci-gates` LINE is
    printed on a run whose smoke step passes and not on one whose smoke step fails. Each line still says the smoke
    reads past the adapter request, or that the log is a witness, "on a run that reaches the settle verdict"; that
    half holds on `ci#38010977166` (2 records) and stays.
  - `architecture.md:180`, `security-plan.md:447`, `test-plan.md:142`, `obs-plan.md:132`, `obs-plan.md:445`
    (standing) — each says the boot job's LOG holds the `ui.webgpu.adapter` record, or that the smoke reads past the
    adapter request, on every run that reaches the settle verdict. Read; true on the red run as well. No change.
  - `obs-plan.md:559` does not print: its retired wording was "on a run that reaches the settle verdict" and the
    amended line no longer carries it.

## Leaves — 25 rows over 6 files, each file re-derived from the amended masters

- `CLAUDE.md:14`, `:36` (`wsl`, `pre-push`) — the overview's and the modules block's one-clause description of the
  verb. Re-derived: native on the dev host. The warnings block's harness-only list gained the verb's by-value read of
  `HOME` / `PATH` (it carried no swept token; found by reading the block against security-plan's amended list).
- `.claude/rules/verification-harness.md:96` (`wsl` ×3, `pre-push` ×3, `clone`, `windows-host`, `apt-remedy`,
  `viola-node` ×2, `patch-sync`, `node-22`) — the `pre-push:linux` row. Re-derived whole from the architecture row
  and the key file. `:85` (`pre-push`) — the `check:english-sources` row's "local: `pre-push:linux`'s `source-lint`
  stage": true, no change.
- `.claude/docs/commands.md:103` (`wsl`, `pre-push`, `windows-host`) — the command's comment. Re-derived.
- `.claude/docs/tests-summary.md:90` (`wsl` ×2, `pre-push` ×3, `clone`, `windows-host`, `viola-node` ×2, `node-22`) —
  the summary's paragraph. Re-derived from the key file.
- `.claude/rules/observability.md:99` (`settle-line`) — the line claim. Re-derived from obs-plan §10. `:64`
  (`settle-line`) — the log-holds-the-record claim: true, no change.
- `.claude/docs/obs-summary.md:80` (`settle-line`) — the line claim. Re-derived from obs-plan §10.
- No swept token, re-derived by provenance: `.claude/rules/security.md` (the body's harness-only sentence, from
  security-plan §Security Anti-Patterns → Input) and `.claude/docs/security-summary.md:43` (the same class).

## Curation homes — 2 rows, neither edited (preserve-verbatim)

- `.claude/rules/verification-harness.md:157` (`pre-push`) — the 2026-10-04 Session Addition, "the pre-push `npm`
  stage" failing on a partial browser cache. Still true of the native stage, whose cache is per-run. No change.
- `.claude/docs/session-learnings.md:1422` (`windows-host`) — "180s for Windows hosts", a boot timeout. Another
  subject; no change.

## Judgment bases — 0 rows

Neither `playbook.md` nor `drift-base.md` holds a swept wording.

## What was seen and left

- The `agent-run` scripts, the `.ps1` files and the other systems' branches in source are outside this pass by the
  chunk's own boundary.
- `.claude/rules/verification-harness.md`'s 2026-08-30 Session Addition on MSYS path conversion and its 2026-05-19
  one on a Windows orphan process name another system; they are dated host learnings, not the check, and are
  curation's.
