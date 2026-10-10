
## 2026-10-09-pre-push-check-native-on-linux — HOME, PATH and PUPPETEER_CACHE_DIR registered as harness-only
**Section:** §Occupied Resources → Environment variables (two new rows)
**Change:** Two rows are new.
- `HOME` · `PATH`: SYSTEM variables, not `ANDROMEDA_PULSE_*` inputs, read harness-only and by value by `cargo xtask pre-push:linux` to build the environment its stage children receive. `HOME` unset or empty reads `cannot-evaluate` / `home-unset`. `PATH` is searched, absolute entries only, for the first directory holding a `node` file, and that directory alone is carried into the stage PATH between `{HOME}/.cargo/bin` and `/usr/local/bin:/usr/bin:/bin`. No value of either is printed or written.
- `PUPPETEER_CACHE_DIR`: harness-only, SET by the verb into its `npm` stage's children only, as the absolute `target/pre-push/run/puppeteer`, so the stage's browser download never touches `~/.cache/puppeteer`; read by no product code. The same row records that the verb SETs `GIT_INDEX_FILE` (`target/pre-push/run/index`) for the three git calls that compute `head` and `tree`, and `ANDROMEDA_PULSE_DATA_DIR` (`target/pre-push/run/data`) for every probe and stage child.
**Why:** The native verb reads two system variables and sets one variable no row held; the registry lists every harness-only variable beside the product's own.
**Ref:** .andromeda/runs/2026-10-10T01-19-03Z-wrap/
