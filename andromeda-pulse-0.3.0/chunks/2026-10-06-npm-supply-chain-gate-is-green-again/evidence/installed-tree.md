# Installed tree — `pulse-app/ui/node_modules` after the change

Measured 2026-10-06 ~22:34Z on the dev host (Node v26.8.2 / npm 11.19.1), on the tree gate entry 1 of the
/implement run `2026-10-06T22-32-37Z-implement` installed fresh from the CHANGED lockfile
(`PUPPETEER_SKIP_DOWNLOAD=1 npm ci --prefix pulse-app/ui`, exit 0).

## The installed-tree probe (gate entry 3)
Prints `{copies below the first patched release} {copies found}` over every `seroval` and `source-map-js`
`package.json` under `node_modules`: **`0 2`** at exit 0. At the chunk base the same probe printed `2 2`.

The lockfile probe (gate entry 2), the same two numbers over the lockfile's holder paths: **`0 2`** at exit 0.

## Every copy on disk, by its own `package.json`
One copy of each package, no nested copy:

| path | version |
|---|---|
| `pulse-app/ui/node_modules/seroval/package.json` | 1.6.8 |
| `pulse-app/ui/node_modules/seroval-plugins/package.json` | 1.6.8 |
| `pulse-app/ui/node_modules/source-map-js/package.json` | 1.2.2 |

First patched releases: `seroval` 1.6.2 (GHSA-p6vx-979v-rg4c) and 1.6.3 (GHSA-jp82-f5mq-hwhp); `source-map-js`
1.2.2 (GHSA-68fv-2mgg-jv7q). Every installed copy is at or above them.

## `npm ls seroval seroval-plugins source-map-js` (run in `pulse-app/ui`, exit 0)
The host path is replaced by `<repo>`.

```
andromeda-pulse-ui@0.1.0 <repo>/pulse-app/ui
├─┬ @tailwindcss/cli@4.2.4
│ └─┬ @tailwindcss/node@4.2.4
│   └── source-map-js@1.2.2
├─┬ @tanstack/react-router@1.169.2
│ └─┬ @tanstack/router-core@1.169.2
│   ├─┬ seroval-plugins@1.6.8
│   │ └── seroval@1.6.8 deduped
│   └── seroval@1.6.8
└─┬ vite@7.3.6
  └─┬ postcss@8.5.26
    └── source-map-js@1.2.2 deduped
```

## The built bundle on this tree (gate entries 8–10)
`npm run build` exit 0; `dist/tokens.css` carries `--color-accent:#c7556a` once. The three hashes are identical to
the chunk-base hashes the plan records, as forecast:

- `assets/index-DmVOp_0t.js` — `3e20c97243c3193930499bbe96c799f0a4c3f3420f27ab773228b2be7b1da8e9`
- `tokens.css` — `a67dadbe0bee547a82842ab7f17c21ab5341d406ff5874797995eb779ada3870`
- `index.html` — `48b692e7647b58017b42ef777d8fcd56f44fec734827dfab786a3c00979669be`

## Limits
- This host installs with `PUPPETEER_SKIP_DOWNLOAD=1`; the plain `npm ci` is not measured here (the standing
  shared-browser-cache defect, owned on the route). CI's runners install the plain form, on Node 24.
- An identical bundle hash shows the shipped JS did not change with the move; it does not by itself prove
  `seroval` is absent from the bundle (research.md §Measured, indirect).
