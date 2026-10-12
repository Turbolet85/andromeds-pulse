# Node 24 on the dev host (plan step 1)

Run by /andromeda-implement on the operator's recorded word (plan.md step 1; the operator (the pc overseer),
2026-10-09). Read at 2026-10-10T00:40:09Z.

- Command: `mise install node@24` — exit 0. A user-level install under the home directory; nothing system-wide, no
  `sudo`, the user's `~/.config/mise/config.toml` not edited (it still names Node 26.8.2 as the default).
- mise's own lines: `node@24.21.0 v24.21.0` · `node@24.21.0 11.19.0` · `installed 1 tool in 7.0s`
  (`node-v24.21.0-linux-x64.tar.gz`, 58.1 MB).
- `mise where node@24` — exit 0, prints `~/.local/share/mise/installs/node/24.21.0`.
- `{that directory}/bin/node --version` → `v24.21.0`
- `{that directory}/bin/npm --version` → `11.19.0`
- `mise ls node` after the install: `24.21.0` (no source) and `26.8.2` (`~/.config/mise/config.toml`).
- `command -v node` in the session shell is unchanged: it still resolves the 26.8.2 install. Node 24 is reached only
  by a caller that puts `{that directory}/bin` first on PATH, as the plan's green live entry does.

Before the install (read at this run's Setup): `mise where node@24` exited 1 with `node@24.21.0 not installed`, and
`mise ls node` listed 26.8.2 alone.
