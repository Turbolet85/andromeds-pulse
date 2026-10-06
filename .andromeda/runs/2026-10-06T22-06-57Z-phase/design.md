# design extract

## Relevance
partial — the chunk renders nothing and adds no token, component or motion; the design plan binds it only through the webview toolkit it names and the token pipeline that toolkit carries, both of which the chunk's dependency moves (`@tanstack/react-router`, `@tailwindcss/cli` / `@tailwindcss/node`, `vite`, `postcss`) can touch.

## Constraints
- design-system §Surface: desktop-webview (Toolkit / Framework row) requires the webview stack to stay on the major lines it names for React, Vite + TanStack Router, Tailwind CSS and the a11y-primitive layer. A fix at the source (an upgrade or an `overrides` entry) for `seroval` or `source-map-js` must leave each holder package inside its named major line. Whether a patched release is reachable without crossing one of those majors is research's question; a fix that needs a major crossing is a fork to raise, not a silent upgrade.
- design-system §Surface: desktop-webview → Tokens (platform-specific) requires the token set to be delivered through the Tailwind `@theme` configuration and to stay reachable from React via `var()`. A move in the Tailwind / PostCSS / Vite toolchain must leave that delivery intact. Whether the build on the changed lockfile still emits the full token set is research's and the gates' question.
- The same Toolkit / Framework row records the radix family as lockfile-absent and denylisted by `pulse-app/ui/npm-policy.json`. An exception added to that policy file must not alter the package-name denylist, and no upgrade or override may bring a radix-family package into the lockfile. Whether the denylist and the lockfile read so at HEAD is research's question.
- design-system §Typography (Loading) and §Surface: desktop-webview → Platform-Specific Notes (CSP policy) require self-hosted fonts and a `'self'`-only script origin with no CDN. A dependency move must not introduce a remote font, script or style source into the built webview.
- The chunk has no rendered surface, so design-system §Color Palette, §Typography (the type table), §Spacing, §Depth Strategy, §Border Radius, §Motion and §Iconography contribute no token to it. Any visible difference in the webview after the lockfile change is a regression of the plan's target state, never a design change this chunk may make.

## Patterns to follow
- Treat the toolkit row of design-system §Surface: desktop-webview as the compatibility envelope for every holder-package move: read each proposed version against the major line the row names before applying it.
- Use the token pipeline of design-system §Surface: desktop-webview → Tokens (platform-specific) as the design-side smoke for a toolchain move: the `--color-*` / `--spacing-*` / `--radius-*` / `--font-*` / `--duration-*` / `--easing-*` families must all survive the build unchanged.
- Keep the policy file's existing denylist as it stands when adding or pruning an exception, per the denylist the toolkit row of design-system §Surface: desktop-webview cites.

## Anti-patterns to avoid
- Do not resolve an advisory by swapping a holder package for a different UI library or component stack: design-system §Surface: desktop-webview (Toolkit / Framework row) names the shipped stack, and design-system §Anti-Patterns → Per-Surface Bans (desktop-webview) binds the dialog primitive to the first-party `Modal` on the a11y-primitive layer.
- Do not accept a toolchain move that makes Tailwind's default palette or a generic font the effective rendering in place of the plan's tokens (design-system §Anti-Patterns → Universal Bans).

## Contract bindings
- design ↔ security: the radix-family denylist cited in design-system §Surface: desktop-webview (Toolkit / Framework row) lives in `pulse-app/ui/npm-policy.json`, the same file the chunk may edit for an exception; the security plan owns that file's exception form and the gate, the design plan owns only the reason the denylist exists.
- design ↔ security: the no-CDN, `'self'`-only origin rule of design-system §Surface: desktop-webview → Platform-Specific Notes (CSP policy) is a design-plan statement of a boundary the security plan enforces.
- design ↔ tests: the UI typecheck, lint, unit tests and build that the scope re-runs on the changed lockfile are the only mechanical evidence that the token pipeline of design-system §Surface: desktop-webview → Tokens (platform-specific) survived; the design plan defines no gate of its own for it.
- design ↔ a11y: the a11y-primitive layer named in the toolkit row of design-system §Surface: desktop-webview is the a11y plan's substrate; if a move touches that package, the a11y extract owns the consequence.

## Acceptance criteria contributions
- (design) After the change, every holder package the chunk moved resolves inside the major line the toolkit row names — React, Vite, TanStack Router, Tailwind CSS (per design-system §Surface: desktop-webview, Toolkit / Framework row).
- (design) The UI build on the changed lockfile succeeds and the `@theme` token families remain present in the built stylesheet, with no token added, removed or revalued by this chunk (per design-system §Surface: desktop-webview → Tokens (platform-specific); per design-system §Self-Validation Protocol → 4. Token Test).
- (design) The changed lockfile holds no radix-family package and the policy file's package-name denylist is unchanged (per design-system §Surface: desktop-webview, Toolkit / Framework row).
- (design) The built webview references no remote font, script or style origin after the change (per design-system §Typography, Loading; per design-system §Surface: desktop-webview → Platform-Specific Notes, CSP policy).
