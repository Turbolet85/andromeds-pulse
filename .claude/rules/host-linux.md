# Host Recipes

Shell mechanics of the Bash tool on this host: the recurring ways a recipe breaks. Every rule is measured on live
work, not hypothetical.

## Paths & the working directory
- The working directory MAY persist across calls (measured both ways on one host: persisted in one session,
  reset after every call in another) — rely on neither: anchor every cross-call path absolutely, or `cd` only
  inside a subshell `( cd X && … )` (setup's Bash guard denies a `cd` that would move the cwd); a
  `cd` inside a compound command can also trip a permission prompt.

## Probes & pattern tools
- A zero-is-healthy count probe (`grep -c` / `grep -q`) exits non-zero on no matches and aborts a
  `&&` chain — suffix `|| true`, or chain with `;`.

## Transports (JSON & documents)
- Never build JSON — or any multi-KB document — through shell quoting: `printf` collapses escapes
  content-dependently.
- Documents: the Write tool, whole-content (setup's Bash guard refuses a `cat`/`tee` heredoc with a file
  target). Ledger appends: a VALIDATED python append (`json.dumps(json.loads(...))` — a mangled payload fails
  loudly instead of landing).
- Inline `python -c` is fine for a SHORT, quote-free, single-expression probe; anything longer,
  quote-bearing, or document-carrying goes through a scratchpad file run by path (the measured failure
  modes are size and unquoted expansion, not inlining or quoted heredocs).

## Compound commands & permissions
- `rm -rf` + `mkdir` + launch compounds get denied by permission layers and abort mid-chain —
  granular steps, fresh unique dirs, no `rm` in a launch path.
- `;` over `&&` for optional probes: an optional probe's failure must not abort the chain.

## Processes
- Never `pgrep -f` / `pkill -f` on a pattern the invoking shell's own command line contains — it matches, and
  kills, that shell. Select by `ps -eo pid,comm,args` and act on the pid.

## Exit codes
- Read `$?` from the BARE command — `| tail` / `| head` report the pipe's LAST command's exit
  (measured green-masking red gates). Redirect output to a file and inspect the file.
- When a tool prints its own verdict, the printed text outranks an unisolated `$?`.

## Encoding & heredocs
- Heredoc terminators must be column-0 and whitespace-exact — a padded terminator silently
  swallows the rest of the script.
- The transport collapses a BACKSLASH PAIR `\\` to `\` before bash sees it, inside a quoted heredoc too (measured
  2026-09-23 on three payloads) — a backslash-bearing payload goes through a scratchpad file written by the
  Write tool.
- The repo pins LF through `.gitattributes` (setup appends the lines): `git ls-files --eol` on a pipeline file
  reads `i/lf w/lf`. Never write CRLF into a pipeline file, and never fix a terminator reading with a
  terminator-agnostic script.

## Session Additions
- 2026-10-05: On the Linux host the Bash tool's working directory persists across calls and the cwd guard blocks only a LEADING `cd`, so a `cd` inside a loop or a compound command silently moves the session cwd for every later call — use absolute paths or a `( cd DIR && … )` subshell there.
