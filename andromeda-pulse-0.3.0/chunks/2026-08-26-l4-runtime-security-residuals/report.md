# Report — 2026-08-26-l4-runtime-security-residuals

**Chunk:** L4 runtime security residuals — the three product-consumed L4 path env vars and the digest-derived
`-p` argv prompt carry an enforced, stated guard instead of a convention, and the five artifacts that name
this entry as owner end consistent with what lands.
**Date:** 2026-08-26T18:45Z
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)

- **Files:**
  - `pulse-app/src/llamacli_inference.rs` (+251) — the path guard, the opt-in confinement root, the prompt
    bound, two emit sites
  - `pulse-app/src/observability.rs` (+6) — one allowlist leaf COMPLETED, one NEW exact leaf
  - `pulse-app/tests/unit_llamacli_inference.rs` (+20 tests) — extended
  - `pulse-app/tests/unit_observability_allowlist_l4_path_guard.rs` (**new**, +5 tests)

- **Symbols / APIs:**
  - NEW `pub const ENV_L4_ALLOW_ROOT` · `MAX_PATH_INPUT_BYTES` (4096) · `MAX_PROMPT_BYTES` (16384)
  - NEW `pub enum PathRejection` (6 bounded variants + `label()`) · `pub enum AllowRoot`
    (`NotConfigured` / `Enforced(PathBuf)` / `Unresolvable`) · `pub enum PromptRejection` (2 + `label()`)
  - NEW `pub fn resolve_allow_root()` · `validate_path_input()` · `resolve_guarded_path()` ·
    `validate_prompt_bounded()`; private `path_contains_traversal` · `path_basename` ·
    `emit_allow_root_posture` · `is_forbidden_control`
  - **CHANGED `pub fn canonicalize_path`** — signature unchanged; body now delegates to
    `validate_path_input(.., AllowRoot::NotConfigured)`, making its behaviour a strict SUPERSET (adds
    traversal + length pre-checks). **It KEEPS both production callers** (`LlamaCliInference::new` at
    `:116` and `:117`) and all 5 test references — not a sole-caller change.
  - **CHANGED `LlamaCliInference::new`** — now resolves the allow-root, announces the posture once, and
    routes both path reads through the guard. `build_llama_cli_args` is unchanged in signature; its ONE
    production caller (`generate_constrained` @ `:530`) now validates the prompt before calling it.
  - NO new TauRPC procedure, NO new `pulse://stream/*` topic, NO `pulse-app/capabilities/` edit,
    NO new crate. `EXPECTED_PROCEDURES` untouched; `capability-drift` clean.

- **Crates / modules:** none added · none removed · `pulse-app` changed only.

- **Dependencies:** **none added, none bumped.** `Cargo.toml` untouched. (`strict-path` was evaluated as the
  confinement primitive and DECLINED at the P4 review — see *Decisions*.)

- **Schema / config:** no migrations, no `config.toml` keys, no `Settings` field.
  **NEW product-consumed env var: `ANDROMEDA_PULSE_L4_ALLOW_ROOT`** — opt-in confinement root for the three
  L4 path inputs. Bounded parse: trimmed; unset/empty ⇒ `NotConfigured` (unconfined, announced);
  set-but-traversal-bearing / over-long / non-canonicalizable / not-a-directory ⇒ `Unresolvable`, which
  **fails closed** (every candidate rejected) rather than degrading to unconfined. Never panics, never
  blocks boot. Env-layer only.

- **Spec-master edits:** **none applied in P1** — this chunk's expected amendments are P2's to apply. The
  plan's `Expected amendments (wrap)` list names: `security-plan.md` §Security Anti-Patterns → Input ·
  `security-plan.md` §Input Validation · `architecture.md` §Occupied Resources → Environment variables ·
  `obs-plan.md` §6 + §8 · `test-plan.md` §1. Operator directive at this wrap adds an `architecture.md`
  item — see *Decisions*.

- **Counts / qualifiers moved:**
  - Workspace test count **1963 → 1988** (+25: 20 in `unit_llamacli_inference.rs`, 5 in the new allowlist
    guard). Stated in the handoff and prior chunk reports.
  - `interpretation.model.load.error` allowlist leaf **4 → 6 fields** (`env_var`, `path_basename` added).
    obs-plan §8 states this leaf's field set.
  - obs-plan §8 / `rules/observability.md` enumerate the `interpretation.*` leaf family — a **new** member
    (`interpretation.model.allow_root`) joins it.
  - `interpretation.model.load.error` moved from **zero producers to one** — obs-plan's muted/partial
    diagnostics narrative characterises `interpretation.*` targets by producer state.

- **Dev-tool versions:** none.

- **Reverted / negative API facts:** none shipped-then-reverted. Three guard neutralisations were applied
  and reverted as **mutation checks** (methodology, not surface); restoration verified byte-exact at all
  three sites.

- **Spec claims disproved by measurement:**
  1. **This chunk's own `plan.md` `## Test Commands` named `./target/release/andromeda-pulse`, a path that
     exists on no host.** Cargo's `[[bin]] name = "pulse-app"` means every build emits
     `target/{profile}/pulse-app.exe`; `andromeda-pulse.exe` is the **bundled** artifact the Tauri bundler
     produces, which arch §Occupied Resources records as "Binary name". A CHUNK-ARTIFACT claim, so its
     home is this report bullet — no amendment owed, and both scope.md and plan.md are closed to
     amendment by now. **Disposition: recorded here; no further action owed.**
  2. **The operator's wrap directive stated this confusion "has now cost two consecutive plans (P-077's
     deviation 1)". Re-derived at HEAD: FALSE.** `…/2026-08-25-demo-injector-formalized-api-surface-retire/plan.md:117`
     names `./target/debug/pulse-app.exe` — correctly — and that chunk's report deviation 1 is about flat
     config (option (b) vs (a)), unrelated to binary naming. A repo-wide sweep for
     `target/{debug,release}/andromeda-pulse` returns hits ONLY for `andromeda-pulse-mcp`, which is
     genuinely that crate's `[[bin]]` name and therefore correct. **One measured instance, not two, and the
     predecessor got it right.** The directive's CONCLUSION (record the distinction in arch) stands on its
     own merit and is applied at P2; only its stated basis is corrected. **Disposition: coordinate
     corrected here; the amendment proceeds on the corrected basis.**
  3. **Live-confirmed, pre-existing, NOT introduced here:** `interpretation.model.load` emits
     `inference_mode` while its exact leaf names only four other fields, so the field rendered
     `"<redacted>"` in all three smoke legs. obs-plan §8 already records this as a PARTIAL-redaction
     backlog item owned by "Diagnostics un-muting + harness-truth sweep". **Disposition: owner already
     named; this is corroborating evidence, not new drift.**

- **Coverage of new surfaces:**
  - `ANDROMEDA_PULSE_L4_ALLOW_ROOT` (env boundary) → validation ✓ (bounded parse, fail-closed) ·
    instrumentation ✓ (`interpretation.model.allow_root`, own exact leaf, once per boot) · PII ✓
    (`root_basename` only; full root path 0× at the wire) · tests ✓ (4 unit branches: unset / real dir /
    missing dir / file-not-dir) · a11y n/a · tokens n/a
  - `validate_path_input` (path guard, product-binary input) → validation ✓ (traversal → length →
    canonicalize → regular-file → confinement) · instrumentation ✓ (`interpretation.model.load.error`,
    exact leaf completed) · PII ✓ (`path_basename` + `env_var` only; the model path's containing directory
    appears **0×** across 11,251 log lines) · tests ✓ (9 unit + mutation-checked) · a11y n/a · tokens n/a
  - `validate_prompt_bounded` (argv boundary) → validation ✓ (byte ceiling + NUL/C0/C1 with `\n\r\t`
    whitelist) · instrumentation ✓ (existing `interpretation.inference.error` leaf, no leaf change needed) ·
    PII ✓ (bounded category only — no prompt text, excerpt, or offending substring) · tests ✓ (6 unit +
    mutation-checked) · a11y n/a · tokens n/a

## Deviations from intent

1. **One new file, against the plan's `New files: (none)`.** `pulse-app/tests/unit_observability_allowlist_l4_path_guard.rs`.
   Justification: the plan's own obs acceptance criterion requires an EXACT-leaf guard living under
   `pulse-app/tests/` (where `[lib] test = false` lets it actually run), and the project convention is one
   `unit_observability_allowlist_*.rs` per leaf family. It is the test for `observability.rs`, which IS in
   the modify list, and the chunk scope names "the guard + the bound + **their tests**". In-scope by the
   discipline's gray-area rule; reported rather than absorbed.
2. **Smoke ran `target/debug/pulse-app.exe`, not the plan's `./target/release/andromeda-pulse`.**
   Justification: the plan's path exists on no host (see *Spec claims disproved* #1); the profile switch is
   separately sanctioned — test-plan §3's direct-binary smoke variant was broadened to either profile at the
   prior wrap.
3. **No RED-before-GREEN leg.** Justification: the guards are NEW functions, so there was no pre-existing
   surface to measure red — a cold run would be a compile error, not a meaningful RED. The three mutation
   checks are the discriminator proof in its place, and all three fired as designed.
4. **A third smoke leg was added at this wrap** (not in the plan's two-leg list), on operator directive —
   see *Outcome*. Justification: the plan's two legs proved rejection and unconfined-boot but left the
   positive arm ("a legitimate GGUF loads when the root contains it") unit-proven only; that arm is the
   does-not-brick-the-real-host proof.

## Decisions & corrections

- **P4 review, guard shape (operator-selected):** opt-in allow-root + always-on structural hardening, over
  (a) hardening-only and (b) data-dir-default-with-opt-out. Ground: the chunk-#84 omission had a real
  reason — the GGUF and `llama-cli.exe` live outside the data dir **by design** — so a data-dir default
  would break every host that has run L4. The exception is therefore NARROWED with a truthful ground, not
  deleted.
- **P4 review, primitive (operator-selected):** `std` both-sides-canonicalize on the
  `workspace_detector::publish_workspace_key` precedent, over `strict-path`. Ground: `strict-path` is the
  primitive security-plan NAMES, and is already declared by `pulse-app` (so adoption needed no manifest
  change) — but it has **zero `.rs` users repo-wide** and its Windows `\\?\` behaviour is unmeasured here,
  and a security guard is the worst place for a library's first use. **Consequence: the specs' "via
  `strict-path`" wording owes a narrowing amendment** (P2).
- **Fail-closed on an unresolvable root** (implementation decision, recorded in the plan's rejected list):
  a set-but-unresolvable `ANDROMEDA_PULSE_L4_ALLOW_ROOT` rejects every candidate rather than degrading to
  unconfined, because a typo must not silently disable the guard the operator explicitly asked for.
- **Operator directive 1 (this wrap):** the positive arm must be live, not implied. **Run**, not stated as
  unit-only — see *Outcome*.
- **Operator directive 2 (this wrap):** record the dev-vs-bundled binary-name distinction in arch so a
  third plan cannot trip. **Applied at P2 — with its stated basis corrected** (one instance, not two; the
  predecessor was correct). See *Spec claims disproved* #2.
- **Operator directive 3 (this wrap):** the two full-path boot records (`app.boot.tracing.init` `log_dir` ·
  `app.boot.pid` `path`) get a **named owner as a CARRY at route-resolve**, beside the sweep's
  bare-`interpretation` item — not a floating note.
- **Operator verification (this wrap):** the prompt-ceiling derivation population is real — 154
  `interpretation.prompt.assemble` records in the durable evidence log, confirmed first-hand by the
  operator. The ceiling (16 KiB) sits ~2.6× above the observed max (6,297 B) and below the Windows
  `CreateProcess` 32,767 limit, so the guard fires before the OS does.
- **Surfaced, not fixed:** two PRE-EXISTING boot records log full paths (`app.boot.tracing.init` `log_dir`,
  `app.boot.pid` `path`), in tension with the basename-only discipline this chunk applies to its own
  records. Predates this chunk; owner assigned at P5 per directive 3.
- **Surfaced, not fixed:** `strict-path` 0.2.2 is declared in `Cargo.toml` and five crate manifests with
  zero `.rs` usage anywhere. Worth its own cleanup entry — either adopt it or drop the declarations.

## Outcome

**Acceptance criteria: met.**

Gates (the plan's `## Test Commands`, all green):
`cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` ·
`cargo nextest run --workspace --profile ci` (**1988/1988 + 1 skip**) · `cargo xtask capability-widening-check`
(clean, 0 violations / 3 inspected) · `cargo xtask check:ingest-progress` (PASS, 128 ticks, longest
zero-delta run 0) · `cargo xtask capability-drift` (clean, after the documented bindings regen).
One fix-loop iteration (`clippy::assertions_on_constants`).

**Three mutation checks, all discriminating:**
- Dropping the `\n\r\t` whitelist reddened ONLY `prompt_bound_accepts_a_realistic_multi_line_prompt` — the
  conditional-property asymmetry, where the POSITIVE pin carries the guard. A naive `char::is_control()`
  would have rejected every legitimate prompt and silently turned L4 off.
- Neutralising confinement reddened exactly the 2 confinement pins; both accept-arms correctly stayed green.
- Deleting the new exact leaf left `allow_root_resolves_to_an_exact_leaf` **passing vacuously** via the bare
  `interpretation` fallback while only the field-set pin caught it — a first-hand reproduction of the
  documented most-deceptive fallback shape, and the reason a resolver-only probe cannot guard this.

**Smoke — direct-binary variant, THREE legs, fresh data dir each, all green, 0 orphans, ports released:**
| leg | configuration | result |
|---|---|---|
| A | root enforced · model deliberately OUTSIDE it | `confinement=enforced` + `root_basename`; rejection with `env_var` / `error_category=outside_allow_root` / `path_basename` / `recovery_action` — **the first live emit for a leaf that had zero producers**. Model dir path 0× across 11,251 lines; basename 1×. 0 panic / 0 ERROR |
| B | root UNSET | `confinement=unconfined` at WARN, exactly once; 0 rejections. 0 panic / 0 ERROR, 8,140 lines |
| C | root enforced · legitimate model INSIDE it | **`confinement=enforced`, 0 rejections, 0 `outside_allow_root`** — the positive arm, live. 0 panic / 0 ERROR, 9,676 lines, 0 path leak |

**Leg C exists because of operator directive 1** and answers it directly: the acceptance's positive arm —
"a legitimate out-of-tree GGUF still loads when the operator names its directory as the root" — is now
proven at the wire on a real boot, not inferred from unit tests. It is the does-not-brick-the-real-host
evidence, and without it the guard's safety rested on unit coverage alone.

**`cargo audit` PREREQ — probe skipped per ratified interval (next: 49).** This wrap is interval point 47
(point 46 was discharged in full form at the predecessor). Never a silent skip: basis + overlap were
re-verified first-hand at this wrap — `cargo deny check bans licenses sources` exit 0, and
`cargo deny check advisories` exit 1 at the SAME eight owned upgradeable IDs
(0189/0190/0194/0195/0204/0222/0253/0258), read as DISTINCT `RUSTSEC-` ids rather than error blocks, and
unchanged for a FIFTH consecutive observation. The pin re-homes to the next markerless entry with its
origin preserved.

**Capability matrix:** this chunk claims **no** capability. All 22 entries in
`andromeda-pulse-0.3.0/verification-matrix.json` and all 60 in `docs/v0_2_0/capability-verification-matrix.json`
were reviewed at phase P5; none covers path confinement, argv bounding, or env-var validation. The matrix is
byte-unchanged and the P7 coverage gate is a no-op for this chunk.
