# The other-account arm on this host — what was and was not read

Arm: `another_account_is_refused_at_each_of_three_layers` (`pulse-app/tests/integration_door.rs`). Read at
/implement on 2026-10-10 on the Linux dev host.

## A second account cannot be driven here

`sudo -n true` fails on this host (the arm's own probe). Every run of the arm in the gate block therefore printed
`door other-account arm: skipped, no second account` and passed. The refusal of another account is not read on this
host by any entry; it is read on the pull-request run, by the `lint / test` job's step `Door refuses another account`.

## A known-positive control of the arm's body, run once

So that the arm's body ran at least once before the hosted runner runs it, it was run with a stand-in named `sudo`
first on `PATH` (a three-line shell script in the session's temp dir, never in the tree) that drops the `-n` and
`-u {account}` words and runs the command as the caller. Under it every "other account" is the engine's owner, so a
correct door ANSWERS every client and a correct arm reads red.

- Run: `PATH="{stand-in dir}:$PATH" cargo test -p pulse-app --test integration_door another_account -- --nocapture`.
- Reading: **red, as it must be.** The arm printed
  `door other-account arm: readings as [nobody, root]: directory 0700 [Answered, Answered], socket 0600 [Answered,
  Answered], both opened [Answered, Answered]` and panicked at `pulse-app/tests/integration_door.rs:757:5`:
  `no client of another account got a byte: [Answered, Answered, Answered, Answered, Answered, Answered]`.
  `test result: FAILED. 0 passed; 1 failed`.
- What that run exercised: the engine's spawn and readiness, the three mode changes, six runs of the client and the
  parse of its line, and the owner's own call (a log record sent over OTLP and returned by `query_logs`), which
  passed its assertion before the panic.
- What it did not exercise: a client of another account, the kernel's refusal at the directory and at the socket's
  mode, the door's account check refusing, the refusal records' count, and the arm's two closing sentences.

## The client script's four outcomes, each run once against scratch sockets

The script the arm hands to `python3 -c`, cut out of the test source and run as the owner against sockets in the
session's temp dir: a missing path printed `connect-error 2`; a listener that closes at accept printed `closed`; a
listener that replies printed `answered`; a socket inside a directory of mode 000 printed `connect-error 13`; and a
socket whose own mode is 000 printed `connect-error 13`. The last is a reading of the mode rule for the owner on this
kernel (Linux 7.2.5): a connect needs write permission on the socket file. It says nothing of a second account.

## Limits

- One run of the control; not a gate entry and not repeated.
- Root's readings at the two outer layers are not predicted here. The plan's step 9 expects the kernel to refuse
  them; a root client is not held by a file mode, so the arm records each reading as measured, asserts that no
  client got a byte, asserts the kernel's refusal for `nobody` at the closed directory alone, and counts one
  `door.refused` record per client that reached the door.
