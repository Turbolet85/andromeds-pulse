# The operator pass (plan entries 23 to 37), 2026-10-10

Run by the agent on the operator's word, given in this session after the implement report: "implement report read
against the tree (8 files: three new, ci.yml 16 lines added in the boot job alone). Deviations accepted as recorded.
Run the operator pass, entries 23 to 37 in the plan order: hygiene, the pre-CI commit, the pre-push check on the
committed tree, the push, the attempt-1 CI read and its three reads; the boot job alone is re-run only if attempt 1
holds no self-end, at most three times. At the first witnessed self-end stop and report that boot ordinal, its label
and its witness lines whole; read every kept witness file whole and stop on anything beyond its shape. Fix nothing
on top. Start no skill - the wrap is mine to call." — the operator, 2026-10-10.

Each operator entry was driven once, by hand, in the spelling the plan lists. Raw outputs are not kept; the lines
quoted are the tools' own verdict lines.

## Entry 23 — hygiene, 2026-10-10T03:00:07Z

- Run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- Exit 0. `gate v1.13 · 3718c868` · `base HEAD (no --marker)`.
- Summary line: `hygiene: clean — read 47 (runs 40 · evidence 2 · inputs 5) · trails 15 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1`
- Atoms: `exit 0` held; `contains hygiene: clean` held. **Green.**
- This record was written after that read. The verb was read once more after it, as a check of this file and not
  as the entry; that reading is the first line of the next section.

Everything below this line was written after the build-branch push, so it is in neither the pre-CI commit's tree nor
the pushed tip's.

## The second hygiene read, and the tree before the commit

- The verb read once more after this record was written: `hygiene: clean — read 48 (runs 40 · evidence 3 · inputs
  5) · trails 15 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1`, the one file
  more being this record.
- `git diff --quiet 8936f976b129cdce00a3fc916f3846254e2dce1c -- pulse-app/ui/src/bindings/index.ts`: exit 0, read
  before the add.

## The pre-CI commit

- `git add -A`, then one commit: `925be35f` (`925be35f77727f999ca3c7d62a91ce42bb686c70`),
  `chore(2026-10-10-boot-smoke-s-self-end-named-from-a-run): operator pre-CI commit`, parent `8936f976`.
- 64 files, 8542 insertions, 6 deletions: the eight source files, the phase and implement run dirs, the chunk
  folder, and the pipeline's own ledgers that stood uncommitted since the phase.
- The tree read clean after it (`git status --short`: 0 rows). `scripts/agent-run.sh` is mode 100755 in the index,
  `scripts/exit-witness.c` 100644. No respell was needed, so the body names none.

## Entry 22 once more — the pre-push check on the committed tree, 03:00:31Z to 03:01:58Z

- Run: `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux`
- Exit 0. Verdict `green`, reason `all-stages-ok`, `head` `925be35f77727f999ca3c7d62a91ce42bb686c70`, `tree`
  `ab72c0bc153ca8951cbed82fbc5951567dee236d` (equal to `git rev-parse 'HEAD^{tree}'`), six stages each `ok`
  (`script-modes`, `source-lint`, `npm`, `clippy`, `test`, `ci-gates`), `missing` empty. 87 s, `load1` 1.2.
- Atoms: `exit 0`, `contains "verdict": "green"`, `contains "reason": "all-stages-ok"`, `lacks "ok": false` held.
  **Green.** The tree read clean after it (0 rows): the verb put the bindings back as found.

## Entry 24 — the build-branch push, 2026-10-10T03:02:02Z

- Run: `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0`
- Exit 0. `8936f976..925be35f  build/andromeda-pulse-0.4.0 -> build/andromeda-pulse-0.4.0`; after it `HEAD` and
  `origin/build/andromeda-pulse-0.4.0` both read `925be35f77727f999ca3c7d62a91ce42bb686c70`. **Green** (default
  atom `exit 0`).
- The push started two pull-request runs on that sha, both created `2026-10-10T03:02:10Z`: `ci#38019133294` and
  `secret-scan#38019133241` (the second read `completed · success` at 03:02:50Z).

## Entry 25 — the CI read of the pushed tip, attempt 1, 03:02:10Z to 03:17:42Z

- Run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700`
- Exit 0. `ci v1.2 · 881cd498` · polled 31× over 932 s.
- Verdict line: `925be35f7772 verdict: red · checks 7/7 · first-fail +915 s boot smoke (ubuntu-22.04) · runs ci#38019133294 in_progress/…`
- `failed 1: boot smoke (ubuntu-22.04) (failure)`; the tool returned at the first failure with the run still open
  (`running 1: oldest coverage gate … 929 s`).
- Report-only (`expect = []`). **Recorded: red. This is the chunk's CI verdict.** It was not re-run.
- The run closed at 03:24:13Z: `completed · failure · attempt 1`. The three reads below were made after that, in the
  plan's order.

## Entry 26 — every job of attempt 1, 2026-10-10T03:24:31Z

- Run: `gh api "repos/Turbolet85/andromeds-pulse/actions/runs/38019133294/attempts/1/jobs?per_page=100" --jq '.jobs[] | "\(.name) · \(.conclusion) · \(.started_at) · \(.completed_at) · \(.id)"'`
- Exit 0. Report-only. The six rows as read:

| job | conclusion | started | completed | id |
|---|---|---|---|---|
| boot smoke (ubuntu-22.04) | **failure** | 03:02:14Z | 03:17:28Z | 114115991438 |
| a11y (ubuntu-22.04) | success | 03:02:13Z | 03:06:43Z | 114115991596 |
| lint / test (ubuntu-22.04) | success | 03:02:14Z | 03:08:59Z | 114115991572 |
| mcp-server tests (ubuntu-22.04) | success | 03:02:14Z | 03:08:09Z | 114115991601 |
| supply-chain (audit + deny + auditable) | success | 03:02:14Z | 03:10:12Z | 114115991602 |
| coverage gate (line ≥75% / branch ≥70% / function ≥85%) | success | 03:02:13Z | 03:24:12Z | 114115991526 |

- `lint / test` ran the workspace suite on the runner, the library's seven controls among them: the first reading
  of the four line patterns and the child boundary on the runner's compiler and C library, and it is green.

## Entry 27 — the boot job's verdict lines, 2026-10-10T03:24:36Z

- Run: the plan's entry, `<id>` = `38019133294` (the job id it resolves: `114115991438`).
- Exit 0; the fetched log was not empty. Report-only. What the lines read, in the job's order (the readiness lines
  carry the runner's own temp path, written here as `{runner temp}`):
  - **The smoke step** (03:08:56Z to 03:12:56Z): `boot: ready (PID=7051, data_dir={runner temp}/andromeda-pulse-ci-data)`
    at 03:12:54.8Z; settle verdict `ended`, `ended` `exit 1`, `app_exit_record` `absent`, `windows_settled` 0,
    `display` `reachable`, `session_bus` `reachable`, **`exit_witness` `exit-call`**; status `not-running`;
    `cleanup: clean`; `Process completed with exit code 1`.
  - **The series step** (03:12:56Z to 03:17:23Z, run under `if: always()`), one cycle marker per boot:

| ordinal | boot verb | settle verdict | marker |
|---|---|---|---|
| 2 | `boot: ready` at 03:16:24.0Z | `ended`, `exit 1`, `exit-call` | `series-cycle boot=0 settled=1 status=1 cleanup=0` |
| 3 | `boot: failed to reach ready state within 10s`, `app ended: exit 1` | not run | `series-cycle boot=1 settled=- status=- cleanup=0` |
| 4 | the same two lines | not run | `series-cycle boot=1 settled=- status=- cleanup=0` |
| 5 | the same two lines | not run | `series-cycle boot=1 settled=- status=- cleanup=0` |
| 6 | `boot: ready` | `settled`, 4, `loaded`; status `running-healthy` | `series-cycle boot=0 settled=0 status=0 cleanup=0` |
| 7 | `boot: failed to reach ready state within 10s`, `app ended: exit 1` | not run | `series-cycle boot=1 settled=- status=- cleanup=0` |
| 8 | `boot: ready` | `ended`, `exit 1`, `exit-call` | `series-cycle boot=0 settled=1 status=1 cleanup=0` |

  - Every cycle printed `cleanup: clean`. The series verdict: `self-ended`; `Process completed with exit code 1`.
    `ci-gates` did not run (the step before it failed); the upload ran.
- Read a second time, to a scratch file, for the step boundaries only: 3 min 27 s passed between the series step's
  start (03:12:57.4Z) and boot 2's readiness line (03:16:24.0Z), against 12 s, 12 s, 12 s, 8 s, 12 s and 3 s for the
  six boots after it. The boot verb's own pre-build is the one thing in that span. Its build log is not among the
  kept files, so a rebuild inside boot 2 is not shown; the seven `end` lines carry equal offsets in the app
  binary's own frames (below).

## Entry 28 — what attempt 1 kept, 2026-10-10T03:24:44Z

- Run: the plan's entry, `<id>` = `38019133294`; the download landed in `target/boot-smoke/ci-38019133294-attempt-1/`.
- Exit 0. Report-only. Artifact `logs-boot-Linux` `11657059728`, 34178 B, expires 2026-10-24. 38 files.
- `find … -name xvfb.log -size +0c` printed nothing: all eight `xvfb.log` are 0 B.
- `boot-series.json`: `verdict` `self-ended`, `boots` 7, `settled` 1, `ended` 2, `other` 4; eight `per_boot`
  entries, ordinal 1 with `cycle` `smoke`.
- The smoke's `harness-settled.json`: `verdict` `ended`, `ended` `exit 1`, `app_exit_record` `absent`, `pid` 7051,
  `windows_settled` 0, `display` `reachable`, `session_bus` `reachable`, `exit_witness` `exit-call`.

### The eight boots of attempt 1

Seconds are after each log's first record. Every log's `git.commit.sha` reads
`ab6a1ae6ef0dbf72574c0931885e42fcc7c94885` (the merge commit the job built; not the pushed tip).

| ordinal | cycle | settle verdict | exit record | `exit_witness` in the verdict | witness file's lines | records | http bind | `ui-bridge.ready` | second adapter record | first navigation | last record | `app.exit` |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | smoke | `ended` | `exit 1` | `exit-call` | `loaded`, `end` | 57 | 0.445 | 1.042 | 1.109 | none | 1.447 `triage.cue.tick` | 0 |
| 2 | status-not-healthy | `ended` | `exit 1` | `exit-call` | `loaded`, `end` | 53 | 0.239 | 0.934 | 0.942 | none | 0.968 `tonic::transport::server` | 0 |
| 3 | boot-failed | none kept | `app ended: exit 1` (job log) | null | `loaded`, `end` | 51 | 0.216 | 0.850 | 0.854 | none | 0.856 `viz.query.traces` | 0 |
| 4 | boot-failed | none kept | `app ended: exit 1` (job log) | null | `loaded`, `end` | 51 | 0.240 | 0.772 | 0.779 | none | 0.779 `ui.webgpu.adapter` | 0 |
| 5 | boot-failed | none kept | `app ended: exit 1` (job log) | null | `loaded`, `end` | 41 | 0.216 | none | none (0 adapter records) | none | 0.218 `plugins.tick` | 0 |
| 6 | complete | `settled`, 4 windows | none | `loaded` | `loaded` | 112 | 0.209 | 0.893 | 0.899 | 5.201 | 5.815 `app.exit` | 1 |
| 7 | boot-failed | none kept | `app ended: exit 1` (job log) | null | `loaded`, `end` | 51 | 0.224 | 0.830 | 0.835 | none | 0.835 `ui.webgpu.adapter` | 0 |
| 8 | status-not-healthy | `ended` | `exit 1` | `exit-call` | `loaded`, `end` | 53 | 0.201 | 0.747 | 0.790 | none | 0.866 `tonic::transport::server` | 0 |

- No log holds `app.panic.fatal`. Every `boot.log` holds the one accessibility-bus warning and nothing else.
- **Seven of the eight boots ended by themselves; one settled.** Three of the seven ended after `boot: ready`
  (ordinals 1, 2, 8) and read `ended` in a settle verdict. Four ended before the boot verb's readiness poll could
  read ready (ordinals 3, 4, 5, 7): the cycle then runs no `harness:settled`, so these four carry no settle verdict
  and no `exit_witness` label, and the series verdict counts them `other`, not `ended`. Their witness files hold
  the same `end` line as the other three.
- The first boot (ordinal 1) ended. Of the seven later boots six ended and one settled.

### The witness files, each read whole

Eight files, 15 lines: eight `loaded` lines and seven `end` lines. Each line was read against its closed shape (its
member set, integers, a process name, one of the five calls, frames of module basename, symbol and hex offset):
**0 lines hold anything beyond it** — no environment value, no path beyond a basename, no secret-class value, no
telemetry of a watched service. The pass did not stop on this check.

The first witnessed self-end, ordinal 1, its file whole (two lines):

```json
{"kind":"loaded","pid":7051,"comm":"pulse-app"}
{"kind":"end","pid":7051,"tid":7051,"comm":"pulse-app","call":"_exit","code":1,"errno":11,"frames":[{"m":"exit-witness.so","s":"_exit","o":"0x195e"},{"m":"libgdk-3.so.0","s":"","o":"0x76ccc"},{"m":"libX11.so.6","s":"_XIOError","o":"0x41393"},{"m":"libX11.so.6","s":"_XReply","o":"0x4660f"},{"m":"libX11.so.6","s":"XGetWindowProperty","o":"0x255a8"},{"m":"libgdk-3.so.0","s":"","o":"0x781f9"},{"m":"libgdk-3.so.0","s":"","o":"0x7830e"},{"m":"libgdk-3.so.0","s":"gdk_x11_screen_supports_net_wm_hint","o":"0x794dc"},{"m":"libgdk-3.so.0","s":"","o":"0x7fb4f"},{"m":"libgobject-2.0.so.0","s":"g_signal_emit_valist","o":"0x32700"},{"m":"libgobject-2.0.so.0","s":"g_signal_emit","o":"0x32863"},{"m":"libgdk-3.so.0","s":"","o":"0x46966"},{"m":"libgdk-3.so.0","s":"","o":"0x332ad"},{"m":"libglib-2.0.so.0","s":"","o":"0x56318"},{"m":"libglib-2.0.so.0","s":"g_main_context_dispatch","o":"0x55c94"},{"m":"libglib-2.0.so.0","s":"","o":"0xab5f8"},{"m":"libglib-2.0.so.0","s":"g_main_context_iteration","o":"0x53433"},{"m":"libgtk-3.so.0","s":"gtk_main_iteration_do","o":"0x248e15"},{"m":"pulse-app","s":"","o":"0x2a46fa6"},{"m":"pulse-app","s":"","o":"0x2d23c1d"},{"m":"pulse-app","s":"","o":"0x2958813"},{"m":"pulse-app","s":"","o":"0x2958489"},{"m":"pulse-app","s":"","o":"0x3034774"}]}
```

- The seven `end` lines are equal in every member but `pid` and `tid` (7051, 7950, 8238, 8608, 8972, 9656, 10020;
  in each `tid` equals `pid`, the main thread): `call` `_exit`, `code` 1, `errno` 11, the same 23 frames with the
  same offsets, the app binary's own five frames included.
- What the lines name: the process was ended by `_exit(1)`, called from a static function of `libgdk-3.so.0`
  (+0x76ccc), entered from Xlib's `_XIOError`, reached from `_XReply` under `XGetWindowProperty`, called by
  `gdk_x11_screen_supports_net_wm_hint` out of a GObject signal emission in GDK, dispatched from GLib's main loop
  under `gtk_main_iteration_do`, on the app's main thread.
- Ordinal 6, the one boot that settled: one line, `{"kind":"loaded","pid":9338,"comm":"pulse-app"}`.

## The stop

Attempt 1 holds a witnessed self-end in its first boot, so the pass stopped here on the operator's word and the
plan's step 5. **Entries 29 to 37 were not fired:** no re-run of the `boot` job, no attempt 2. Nothing was fixed on
top. No CI run was re-run.

- The draft pull request #40 read `MERGEABLE · UNSTABLE` on `925be35f` at 03:25:50Z (the red check).
- What is not shown by this attempt, each a limit of the reading and not a finding:
  - **The instrument's reading on green boots on the runner is one boot** (ordinal 6, `loaded`). The plan's step 6
    asked for it on eight; attempt 1 holds seven self-ends.
  - **The rate under the instrument differs from the rate without it**: seven of eight boots here, against four of
    twelve first boots in the runs before it. Whether the library, the series (seven boots in a row on one runner),
    or this one runner accounts for the difference is not measured. The first boot, which runs as the smoke always
    did but for the library, ended too.
  - The lines name the call and its callers. They do not say why the X connection read failed while the display
    server still accepted connections after each end (`display` `reachable` in the three settle verdicts kept).
