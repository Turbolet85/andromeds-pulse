# design extract

## Relevance
partial — the chunk renders nothing, so no token, type, spacing or motion value binds its record file or its gate; the plan bears only on the classification of ids whose text concerns the two surfaces it specifies.

## Constraints
- No design token applies to the chunk's outputs: tokens are scoped to the webview's CSS custom properties and to rendered UI (per design-system §Surface: desktop-webview → Tokens (platform-specific)). A record file and an xtask gate are outside that scope; the plan adds no requirement on their form.
- The plan names exactly two surfaces and specifies each separately (per design-system §Surface: desktop-webview; per design-system §Surface: desktop-native). The plan does not say how its two surfaces map onto P-117's three retirement surfaces (the window, the model, the desktop): in particular whether the tray, its menu and the OS notifications of §Surface: desktop-native are "the window" or "the desktop" is research's question, and a doubtful mapping is P4's.
- The plan describes the window and the tray as TARGET state and holds no word on their removal in this version (per design-system §Brand Identity → Design direction). It is therefore evidence of what an id's surface IS, never evidence that the surface stays; whether the code still carries each surface is research's question.
- The signature glow layer is recorded as deferred to the next version, neither built nor deleted from the spec, its render half riding a cross-version residual targeted at 0.4.0 (per design-system §Brand Identity → Signature element). "Deferred" is the plan's status and not one of the record's two dispositions: an id whose text is that layer must still be read as claimed or retired, and the plan does not decide which — that turns on what the product is.
- Two capability ids are named in the plan by id, both inside the deferred layer's spec: P-026 (the modulation rule) and P-004 (health-versus-severity orthogonality) (per design-system §Motion → High-impact moments). Whether either id's text reaches beyond the glow layer — P-004 in particular, whose orthogonality could be read as a property of data and not of a rendering — is research's question against the 82 texts.
- While the glow layer is deferred, no output may claim that it renders (per design-system §Self-Validation Protocol → 3. Signature Test). A "claimed" disposition on a glow-layer id must not read as that claim.
- The severity signature the plan records as the one in force is the constellation dot hue on the dashboard map and the widget canvas (per design-system §Self-Validation Protocol → 3. Signature Test). An id whose text is the severity hue is tied to those two window surfaces by the plan; whether an engine-side form of it exists is research's question.

## Patterns to follow
- One status owner per fact: the plan keeps the glow layer's status in a single section and has every other section point to it (per design-system §Brand Identity → Signature element). The record's one-disposition-per-id form fits that pattern — a glow-layer id cites one owner, it does not restate the status.
- A disposition is derived per surface, not inherited: the plan gives the tray layer its own ground for deferral, separate from the webview measurement (per design-system §Surface: desktop-native → Component Patterns → Tray Icon). The per-id reading of P-001…P-082 follows the same rule for ids that span both surfaces.
- Surface-conditional reading: the plan splits its rules by surface wherever they differ (per design-system §Typography → Surface-conditional guidance; per design-system §Spacing → Surface-conditional; per design-system §Border Radius → Surface-conditional). An id's named surface can be read off the same split.
- Specified versus measured: the plan words a spec as "SPECIFIED" and a reading as dated and measured (per design-system §Brand Identity → Signature element). The record's reader outside the project is served by the same separation between what an id claims and what was read.

## Anti-patterns to avoid
- Reading a `TBD` item as a capability the product has: several navigation items are left to a later phase in the plan (per design-system §Surface: desktop-webview → Navigation Pattern). A plan sentence is not evidence of a production render site.
- Letting the webview's measurement stand for the tray: the plan states the tray layer was never probed and does not inherit that reading (per design-system §Surface: desktop-native → Component Patterns → Tray Icon).
- Citing the plan's universal bans against the record: they bind rendered chrome only (per design-system §Anti-Patterns (NEVER do these) → Universal Bans) and none applies to a file or a gate.

## Contract bindings
- design ↔ a11y: the reduced-motion mandate is app-wide and token-bound, and the glow layer's degrade rule ships with the layer (per design-system §Motion → Accessibility, which points at a11y-plan §6). Ids about reduced motion, contrast or not-colour-alone are carried by the window's tokens in both plans; their classification is one reading shared with the a11y extract, not two.
- design ↔ residuals: the glow layer's render half is said to ride `.andromeda/residuals.md` with target 0.4.0 (per design-system §Brand Identity → Signature element). The record's disposition of the glow-layer ids and that residual line speak of the same thing; which entry owns reconciling them is the route's, not this extract's.
- design ↔ architecture / security: the tray menu is specified to invoke `snapshot.generate` and `mcp.start` / `mcp.stop` (per design-system §Surface: desktop-native → Component Patterns → Tray Menu). An id whose text is a tray action names a procedure another plan owns; retiring the surface does not by itself retire the procedure — a disposition is not inherited.
- design ↔ layouts: per-screen placement of the compact widget and the full dashboard is the layouts distiller's (per design-system §Surface: desktop-webview → Navigation Pattern); ids about window geometry are read there.
- design ↔ wrap: the plan's body holds current truth only and its history lives in the amendments sidecar (per design-system, closing note). Nothing in the plan is written at phase; what the chunk changes in it is the wrap's, and what describes 0.2–0.3 is owned by the working entry `Records say what the product is`.

## Acceptance criteria contributions
- (design) P-026 and P-004 each appear exactly once in the record with one of the two dispositions, and a retired one names its surface (per design-system §Motion → High-impact moments).
- (design) No id whose text is the signature glow layer is recorded as claimed in words that say the layer renders (per design-system §Self-Validation Protocol → 3. Signature Test).
- (design) No id of the record carries "deferred", "next version" or an equivalent as its disposition; the plan's deferral status is cited, if at all, as the reason behind a claimed-or-retired reading (per design-system §Brand Identity → Signature element).
- (design) An id whose text spans the webview and the tray has its surface read on each ground, and the plan or the P4 dialog shows which reading decided it (per design-system §Surface: desktop-native → Component Patterns → Tray Icon).
