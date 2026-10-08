# Host leaf re-seed — the proposed sort (nothing is moved until the card's word)

Old leaf: `.claude/rules/host-win32.md` · rendered for win32 · this host is linux · 7638 B · md5
`4be4626b14363aec1fc1b9dbd80f9005` · 2 items below `## Session Additions`. The machine form is `host-reseed.json`
beside this file; a `host {n} …` word on the card re-sorts one item there.

## The items

1. **lines 91-91 · 873 B → `learnings`** (mixed) — Tier-3 heading
   `## 2026-10-08 — Moved from the host leaf: stop the rust-analyzer flycheck cargo tree before a cargo clean or a long build`
   (the heading carries the day of the move: the sort was proposed 2026-10-07 and applied 2026-10-08, on the word).
   The entry opens "2026-10-01: The session's own rust-analyzer LSP runs a `cargo check --workspace --all-targets`
   flycheck into the shared `target/` and respawns after source edits…". It is a live lesson carrying an old-host
   clause (its 2026-10-01 extension identifies the flycheck by its `rust-analyzer.exe` parent through
   `Get-CimInstance Win32_Process`), so rule 3 sends it to `.claude/docs/session-learnings.md` whole and it is never
   kept. What that costs: the lesson leaves the every-turn tier. Its Linux extension's ban (never `pgrep -f` /
   `pkill -f` on a pattern the invoking shell's own command line contains) stays loaded, because the new leaf's own
   `## Processes` section carries it; the instruction to stop the flycheck before a `cargo clean` does not.
2. **lines 92-92 · 295 B → `keep`** — the entry opens "2026-10-05: On the Linux host the Bash tool's working
   directory persists across calls and the cwd guard blocks only a LEADING `cd`…". Rule 4: host mechanics of this
   host, no old-host clause, within ~600 B, and live as measured at this run against the stored Bash guard — a `cd`
   inside `for … do … done`, `if … then … fi` or a piped `while` exits 0, so it still moves the session cwd. One
   clause is behind the guard as re-rendered 2026-10-06 (U02): the entry says the guard blocks only a leading `cd`,
   and the guard now also refuses a `cd` after `&&`, `||`, `;` or a newline (`ls && cd DIR` exits 2). The entry moves
   byte for byte; its wording is a wrap's to correct, never setup's.

No item is proposed `drop` and none goes to another rule file (both are host/shell mechanics; no rule file with
`paths:` owns that class).

## `USER:session-learnings` bullets that name the old host

None. CLAUDE.md lines 132-154 (18 bullets) name neither win32, Windows, MSYS, Git Bash nor PowerShell.

## Above the cut — not sorted, the template's (stated so the card can show it)

The sort covers only the items below `## Session Additions`. The body above it is replaced by the template's render
for linux (89 lines · 6467 B → 50 lines · 2857 B). What the old body holds and the new one does not:

- **win32-tagged in the template, so not rendered for linux:** MSYS leading-`/` argument conversion · Bash's `/tmp`
  is not the Windows temp dir · `grep -P` dies on the host locale · a PowerShell redirect writes UTF-16/BOM · MSYS
  and Windows pids are different spaces, liveness through PowerShell · force UTF-8 on python (`-X utf8`) · the Bash
  tool corrupts a command near 7.5 KB (keep every command under 6500 B) · the `core.autocrlf` re-checkout recipe.
- **in no host section of the template:** the whole `## Long single-line files` section (3 bullets, 2156 B: an
  anchored Edit on a multi-KB line, the structural-extraction read, the four assertions of a python write by path).
  It entered this leaf through a setup run (`8b86529`) and the template has since dropped it; the same rules stand in
  the pipeline's `line-write-contract.md` (the phase and wrap skills' references). In this project it stands nowhere
  else, so after the re-seed it is loaded only where a skill reads that contract.

What the new body holds and the old one does not: a `cd` only inside a subshell (the Bash guard denies one that would
move the cwd) · the Bash guard refuses a `cat` / `tee` heredoc with a file target · `## Processes`: never `pgrep -f`
/ `pkill -f` on a pattern the invoking shell's own command line contains · the transport collapses a backslash pair.

The old leaf is backed up before anything moves (`.claude/backup/host-win32.md.pre-setup-2026-10-07T20-59-36Z-setup-project`,
gitignored) and stays in git history.
