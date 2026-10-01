
## 2026-10-01-real-model-incident-surfacing — the English-only source gate registered; pre-push gains its sixth stage
**Section:** §Occupied Resources → xtask CLI surfaces (dev/CI gates) — the `check:english-sources` entry (new) and the `pre-push:linux` stage list
**Change:**
- Registered `cargo xtask check:english-sources` (`xtask/src/source_lint.rs`), a formalized CLI contract:
  - scans `crates` · `pulse-app/src` · `pulse-app/tests` · `pulse-app/ui/src` · `xtask/src` (`.rs`/`.ts`/`.tsx`) for U+0400–U+04FF;
  - one `::error file=…,line=…::` annotation per hit, every non-ASCII character rendered `\u{XXXX}`, so the whole stdout is ASCII;
  - then a `{verdict, hits, files_scanned}` JSON verdict with a twin `target/english-sources/report.json`;
  - exit 0 `clean` · 1 `findings` · 2 `cannot-evaluate` (never a pass);
  - wired as ci.yml `lint-test`'s "No Cyrillic in sources" step on all three OSes and as `pre-push:linux`'s `source-lint` stage.
- `pre-push:linux` stage list: was five (`script-modes` · `npm` · `clippy` · `test` · `ci-gates`); now six, with `source-lint` second after `script-modes`.
**Why:** the CI check had been an inline `shell: python` step that crashed with `UnicodeEncodeError` under the Windows runner's ANSI code-page pipe whenever it had a hit to print, and the local pre-push gate never ran it — so a Cyrillic literal passed pre-push green and failed CI on every `lint / test` job. One xtask verb now serves both, ASCII by construction.
**Ref:** .andromeda/runs/2026-10-01T18-16-18Z-wrap/
