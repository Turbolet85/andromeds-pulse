# Advisory record — `cargo xtask check:npm-supply-chain`

The gate's verdict before and after the lockfile move. Advisories are counted as DISTINCT GHSA ids.

## Before — chunk base `b1fcba5`
Source: research.md §Measured (2026-10-06, 22:15Z–22:21Z, this host), the same reading as CI job 112517747768.

- exit 1 · arm `findings-red` · `advisory.distinct` 5
- excepted (2): `extract-zip` GHSA-7pqw-9j4j-h8q3 (high) · `extract-zip` GHSA-jmr9-qjv8-65gv (high)
- unexcepted (3): `seroval` GHSA-p6vx-979v-rg4c (critical) · `seroval` GHSA-jp82-f5mq-hwhp (high) ·
  `source-map-js` GHSA-68fv-2mgg-jv7q (high)
- license arm: `checked` 883 · `excepted` 2 · `runtime_packages` 37 · `violations []`
- ban arm: `hits []`

## After — the changed lockfile
Source: gate entry 7 of the /implement run `2026-10-06T22-32-37Z-implement` (fired 2026-10-06 ~22:34Z, after a
fresh `PUPPETEER_SKIP_DOWNLOAD=1 npm ci` of the changed lockfile, entry 1 of the same run).

- exit 0 · arm `green-with-dispositions` · `verdict` `green` · `advisory.distinct` 2
- excepted (2): `extract-zip` GHSA-7pqw-9j4j-h8q3 (high) · `extract-zip` GHSA-jmr9-qjv8-65gv (high)
- unexcepted (0): `unexcepted []`
- GHSA-p6vx-979v-rg4c, GHSA-jp82-f5mq-hwhp and GHSA-68fv-2mgg-jv7q: absent from the output (the entry's three
  `lacks` atoms held)
- license arm: `checked` 883 · `excepted` 2 · `runtime_packages` 37 · `violations []` — identical to before
- ban arm: `hits []` — identical to before

## Reading
- All three unexcepted findings are closed at their source, by an in-range lockfile bump. No exception was added
  and no `overrides` entry was taken; `npm-policy.json` and `package.json` are byte-identical to the chunk base
  (the scope guard, entry 6: exit 0, no output).
- The two `extract-zip` exceptions stand unchanged: the gate still reports both, so neither closing condition
  fired.
- The exit was 0, not 2: the registry was reachable and the advisory arm was evaluated.
- Measured on this host only. The CI `supply-chain` job on this chunk's pushed commit is the operator pass's
  read (plan entries 26–28) and is not part of this record.
