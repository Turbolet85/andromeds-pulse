# Windows/MSYS Host Recipes

The Bash tool on this host is Git Bash (MSYS) over Windows; the primary shell is PowerShell.
POSIX-shaped recipes break here in the recurring ways below. Every rule is measured on live work,
not hypothetical. (Rendered only on Windows-host projects; inert elsewhere.)

## Paths & argument conversion
- MSYS auto-converts leading-`/` arguments into Windows paths (the `/X`-mangling class) — pass
  `//X` for a literal slash-arg, or scope `MSYS2_ARG_CONV_EXCL` for the call.
- Bash's `/tmp` is NOT the Windows temp dir and is invisible to native tools (a Windows-native
  python cannot open `/tmp/x`) — use explicit full paths across every bash↔native boundary.
- The working directory MAY persist across calls (measured both ways on one host: persisted in one session,
  reset after every call in another) — rely on neither: anchor every cross-call path absolutely, or `cd` at
  the head of the call that needs it; a `cd` inside a compound command can also trip a permission prompt.

## Probes & pattern tools
- `grep -P` dies on the host locale ("supports only unibyte and UTF-8 locales") — use `grep -E`
  or a python char-class instead.
- A zero-is-healthy count probe (`grep -c` / `grep -q`) exits non-zero on no matches and aborts a
  `&&` chain — suffix `|| true`, or chain with `;`.

## Transports (JSON & documents)
- Never build JSON — or any multi-KB document — through shell quoting: `printf` collapses escapes
  content-dependently, and a PowerShell redirect writes UTF-16/BOM.
- Documents: the Write tool, whole-content. Ledger appends: a VALIDATED python append
  (`json.dumps(json.loads(...))` — a mangled payload fails loudly instead of landing).
- Inline `python -c` is fine for a SHORT, quote-free, single-expression probe; anything longer,
  quote-bearing, or document-carrying goes through a scratchpad file run by path (the measured failure
  modes are size and unquoted expansion, not inlining or quoted heredocs).

## Compound commands & permissions
- `rm -rf` + `mkdir` + launch compounds get denied by permission layers and abort mid-chain —
  granular steps, fresh unique dirs, no `rm` in a launch path.
- `;` over `&&` for optional probes: an optional probe's failure must not abort the chain.

## Processes & ports
- MSYS and Windows pids are DIFFERENT spaces — bash's `$!` is the app's Windows pid only when the
  binary was spawned by path (no shell wrapper in between).
- When an msys probe is ambiguous, verify liveness/kill via PowerShell (`Get-Process -Id` /
  `Stop-Process -Id`); judge teardown by loopback port probes plus pid-liveness, never pid
  inference alone.

## Exit codes
- Read `$?` from the BARE command — `| tail` / `| head` report the pipe's LAST command's exit
  (measured green-masking red gates). Redirect output to a file and inspect the file.
- When a tool prints its own verdict, the printed text outranks an unisolated `$?`.

## Encoding & heredocs
- Force UTF-8 on python invocations (`-X utf8`; the project settings.json env block sets
  PYTHONUTF8 for hook-mediated runs).
- Heredoc terminators must be column-0 and whitespace-exact — a padded terminator silently
  swallows the rest of the script.
- The Bash tool corrupts a command past a fixed offset near 7.5 KB (measured 2026-09-02: a 100-line
  and a 60-line quoted heredoc both died at script line 57 with `unexpected EOF while looking for
  matching`; 6.0 KB passed; all 22 logged heredoc failures were larger calls) — keep every command
  under 6500 bytes: a document goes through the Write tool, a script to a scratchpad file run by
  path, a ledger payload splits across calls. The quoted heredoc itself is sound below the cut.
- The repo pins LF through `.gitattributes` (setup appends the lines): `git ls-files --eol` on a pipeline file
  reads `i/lf w/lf`. A `w/crlf` or `w/mixed` reading is a checkout-time artifact (`core.autocrlf=true` in this
  host's system gitconfig) — after the `.gitattributes` commit, `git rm --cached -r . && git reset --hard` on a
  clean tree re-checks every file out under the attributes (`git add --renormalize` refreshes the index's stat
  cache and rewrites no file — measured); never fix it with a terminator-agnostic script, and never write CRLF
  into a pipeline file.

## Long single-line files
- A multi-KB single line does NOT defeat anchored Edit on an LF-pinned file — measured: an 11 225-char
  entry, a 3 066-char route line and a 92 KB master all took a first-try Edit on a SHORT unique anchor.
  Probe the anchor's uniqueness first (`grep -c` = 1), then read the line through an OFFSET-BOUNDED
  Read-tool window: that read also satisfies Read-before-Edit, which a Bash or python read does not, so
  a shell extraction costs a forced duplicate read of content already in the window (measured 4× in one
  step). A `matched 0` on such a line is first a TERMINATOR question, not a length one — an anchor
  written with `\n` against a CRLF file matches nothing.
- Read such a file as a structural extraction, never in full: `grep -n` the headers AND per-entry
  introducers for an index → offset-bounded reads covering every indexed span; a `head`- or
  `cut`-limited view never answers a membership question and has stated a false coordinate finding to
  the operator — the clause sat at the END of a 500+ char line.
- Where the write genuinely cannot be one anchored substring — a whole-line move or reorder, an append
  whose position would have to be located, a line addressed by number — and no pipeline tool is
  reachable, a python write by path carries ALL FOUR assertions or it is not written: (1) binary mode,
  or `newline=''` — never a terminator-agnostic read (measured: a text-mode append took a 2 003-line
  all-LF ledger to `w/mixed` on its first write after the repo's LF pin); (2) the target is a UNIQUE
  anchor asserted to match exactly once, NEVER a computed character offset (measured: a computed offset
  severed an entry's last line mid-token and parked the tail after the following entry, in two files,
  read by two downstream agents before the repair); (3) each line's own terminator preserved and
  re-emitted — a normalizing write showed 119 insertions / 119 deletions for a two-line intent; (4) a
  read-back of BOTH junctions — the bytes before and after every edit point — plus a diff-size check
  against the intended edit count; the diff size is what surfaced (3).

## Session Additions
