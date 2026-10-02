# RED-at-base probe — 2026-10-01-conductor-return

Measured 2026-10-02T00:00Z on the Windows dev host (rustc 1.95.0), operator slot #1, against the base product
(`a2addb3` + only `libc = "0.2.186"` added to `pulse-app/Cargo.toml`; no exit-cause code in the tree). Run:
`cargo nextest run -p pulse-app --profile ci --test integration_exit_red_probe --no-capture` (exit 0, 3 tests run),
after `cargo clean --profile dev` (exit 0, 69 321 files / 66.5 GiB removed). The probe was a TEMPORARY test target in the
re-exec form (testing.md 2026-08-15): each child called `observability::init` on its own TempDir data dir and ended one
way; the parent read the child's `agent-latest.jsonl*` family after the child had ended. Deleted after the reading; it is
not part of the chunk.

## Arms (a)-(c) at base — the cause record is ABSENT (block-shaped RED)

| child end | child status | records | `app.boot.tracing.init` (child ran) | `app.exit` |
|---|---|---|---|---|
| `std::process::exit(3)` (the event-loop end tao performs today) | 3 | 1 | 1 | **0** |
| `std::process::exit(4)` | 4 | 1 | 1 | **0** |
| `libc::exit(1)` (a native C exit) | 1 | 1 | 1 | **0** |

## Flush-race tally (research.md Mechanism 2) — 20 children per arm

| child body | children | `app.exit` landed | init landed |
|---|---|---|---|
| `tracing::error!(target: "app.exit", …)` then `std::process::exit(3)` | 20 | **0** | 20 |
| the same, with the `WorkerGuard` dropped before `exit` (control) | 20 | **20** | 20 |

A record emitted immediately before `process::exit` never reached the file on this host (0/20); draining the guard first
lands it every time (20/20). The equality the fix rests on (emit → drop the guard → exit) is measured, not assumed.

## The decider — does an `atexit` handler run? (overseer: "record the Windows atexit reading as the decider")

| child end (after `libc::atexit(marker)`) | status | handler ran |
|---|---|---|
| `std::process::exit(4)` | 4 | **false** |
| `libc::exit(1)` | 1 | **true** |

On Windows, Rust's `std::process::exit` is `ExitProcess` (`library/std/src/sys/exit.rs`, the `target_os = "windows"`
arm), which does not run C `atexit` handlers and terminates every other thread first. So a Rust `process::exit` on Windows
is UNLOGGABLE by construction, in the same class as `TerminateProcess`. On Unix the same call is `libc::exit`, which runs
the handlers. Consequence for the witness: plan Step 8 arm (b) is Unix-only (it runs on the Linux/macOS lint-test jobs and
in `pre-push:linux`); arm (c), the native `exit()` class the 69f0b93 death falls into, runs on every OS. Research
Mechanism 3's sentence "Rust's `process::exit` calls the platform `exit`, which runs `atexit` handlers" holds on Unix and
is FALSE on Windows.
