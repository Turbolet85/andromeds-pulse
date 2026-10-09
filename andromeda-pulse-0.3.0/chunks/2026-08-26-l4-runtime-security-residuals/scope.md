# Scope — 2026-08-26-l4-runtime-security-residuals

**Working-route entry (verbatim intent):** L4 runtime security residuals — the product's model/binary path
inputs and its inference prompt argument carry a stated guard, not a convention.

**Epoch:** Epoch 4 — Polish & ship: verification · **Version:** andromeda-pulse-0.3.0

---

## Coordinates re-verified at promotion (the entry's claims folded as HYPOTHESES)

Per `promotion.md` step 2, every artifact/line the working entry names was re-derived first-hand at HEAD
before it shaped this scope. All of them hold; one item the entry does NOT mention was found.

| claim (from the entry) | verdict at HEAD |
|---|---|
| three env vars read by the SHIPPED binary in `pulse-app/src/llamacli_inference.rs` | **EXACT** — `ENV_MODEL_PATH:45` · `ENV_LLAMA_CUDA_BIN_PATH:50` · `ENV_LLAMA_CPU_BIN_PATH:55`, consumed at the construction site `:118` |
| they trim + `canonicalize()` + assert regular-file | **EXACT** — `read_env_path:434` (trim + reject-empty) → `canonicalize_path:450` (`canonicalize():452`, `is_file():455` → `InvalidModelPath`); a second `is_file()` re-check at load, `:222` / `:226` |
| they deliberately OMIT the data-dir confinement assert | **EXACT**, and the rationale is stated in code, not only in the spec — the `canonicalize_path` doc comment (`:444-449`) reads "no bounded confinement root since binary paths are intentionally user-managed in dev mode per chunk #84 plan" |
| `build_llama_cli_args` passes prompt text as the `-p` argv value at `llamacli_inference.rs:409` | **EXACT to the line** — fn at `:381`; `"-p".to_string()` at `:408`, `prompt.to_string()` at `:409` |
| security-plan §Input + §Input Validation "and their three leaves" were amended as-measured and NAME this entry as owner | **EXACT, and the count is right** — body `security-plan.md:393` + record `security-plan-amendments.md:312`; the three leaves are `.claude/docs/security-summary.md:43`, `.claude/rules/security.md:17`, `CLAUDE.md:44`. All five name *"L4 runtime security residuals"* verbatim as owner |
| the argv shape is the one security-plan §Anti-Patterns → Code Patterns bans | **EXACT** — `security-plan.md:451`, "NEVER use `tokio::process::Command::new(...).arg(user_input)` against any string sourced from an OTLP attribute…" |

**Found at promotion, NOT named by the entry — `strict-path` is declared but never used.**
`strict-path = "0.2"` is a workspace dependency (`Cargo.toml:49`) pulled into FIVE crates —
`config-watcher` · `corpus` · `triage` · `workspace-detector` · `pulse-app`. A repo-wide sweep for
`strict_path` / `StrictPath` / `PathBoundary` / `VirtualPath` across every `.rs` file returns **zero hits**.
Every confinement that actually ships is hand-rolled `std`:

- `crates/workspace-detector/src/contract.rs:90-105` — `fs::canonicalize` on both sides + `starts_with` →
  `Error::PathTraversalRejected`. This is the repo's one true canonicalize-and-confine precedent.
- `crates/plugins/src/loader.rs:155-178` — reject `Component::ParentDir` + `canonicalize`, with a
  NotFound clean-skip.

This matters because the invariant this chunk answers to names the library specifically — CLAUDE.md:44 and
`rules/security.md:17` both say "MUST canonicalize via `strict-path`", and `security-plan.md:393` closes with
"Use `strict-path` … for the canonicalize-and-confine primitive." So the *named mechanism* of the rule the
chunk is closing has, at HEAD, no user anywhere in the product. Whether this chunk adopts `strict-path` (making
the invariant's wording true for the first time) or confines with `std` on the `publish_workspace_key`
precedent (making the wording measurably aspirational, and owing an amendment) is a real fork — see
P-2 below, CLOSED at P3.

---

## Outcome

Three things must be true when this chunk is done:

1. **The three product-consumed path vars carry an enforced guard, and the guard is the one the specs
   describe.** Today the gap is not "no validation" — trim, canonicalize and regular-file all fire. The gap is
   that the *confinement* half is absent while five artifacts state it as a MUST, so what protects the product
   is a convention (nobody sets a hostile `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`) rather than a check.

2. **The digest-derived prompt reaching `-p` is bounded by something the code asserts.** The measured
   mitigating facts stay true and are not the deliverable — an argv vector is not shell-mediated, the target
   is a real `.exe` and not `.bat`/`.cmd`, and the value is consumed as `-p`'s operand. This is hardening of a
   banned SHAPE, not repair of a live exploit, and the acceptance must say so rather than implying a
   vulnerability was closed.

3. **The five artifacts stop being self-contradictory.** `security-plan.md:393` says in its own words "Until
   it lands, this is the current truth, not a target state", and `rules/security.md:17` says "Do NOT cite this
   rule as universal until that entry lands." Whatever this chunk enforces, those five sites must end the
   chunk describing the same reality — either the exception is gone, or it is narrowed and re-stated with the
   narrower ground.

**NOT an outcome of this chunk, stated plainly:** blanket data-dir confinement of the three vars. The
chunk-#84 decision omitted it for a REASON that still holds — the GGUF model and the prebuilt `llama-cli.exe`
live outside the data dir **by design**, and arch §Established Decisions [LLM Inference Runtime] still carries
their distribution as an explicitly OPEN caveat. Applying the categorical rule literally would break L4 on
every host that has ever run it, including the one that produced this project's only real-model chain proof.
The deliverable is a *stated, enforced* guard appropriate to a deliberately out-of-tree target — not the
data-dir assert copy-pasted from a rule written for in-tree files.

---

## Deliverable A — the path-input guard

The three vars gain an enforced bound that is written down and tested, replacing the current
convention. Constraints the design must respect:

- It must not break the shipped configuration: a user-managed GGUF and a user-managed `llama-cli.exe` living
  anywhere on the host must still load. P-1, VERIFIED at P3: satisfied by an operator-declared allow-root,
  since the ROOT is what the operator names — so an out-of-tree target stays loadable.
- It must fail the way the rest of this file fails: `InferenceError::InvalidModelPath`, graceful-degraded
  boot (`ModelStatus::Error` → `ModelNotConfigured`), never a panic and never a hard startup failure. That is
  the established contract at `:118` and `:210-230`.
- Whatever it asserts must be observable — a rejection that leaves no trace is the same dead-guard class this
  epoch has repeatedly caught. The existing `interpretation.model.load` target is the natural site.

## Deliverable B — the argv-prompt hardening

The prompt value reaching `:409` gains an asserted bound. Candidate shapes, to be decided at P4 from P3's
findings, not here:

- bound the value (length ceiling + reject control characters / embedded NUL / newline framing), the same
  shape `read_published_workspace_key` already applies to a cross-process value
  (`workspace-detector/src/contract.rs:115-126`);
- or move the prompt OFF argv entirely — llama.cpp's `-f/--file` prompt-file form would make the banned
  `.arg(user_input)` shape structurally absent rather than merely bounded. P-3, PREMISE-CORRECTED at P3:
  the flag could not be measured against the pinned b9305 build, so this route is DECLINED for this chunk and
  the bound is the deliverable.

The two are not exclusive; a file-based prompt still wants a bound.

## PREREQ — `cargo audit` interval

The entry carries the re-pinned `cargo audit` standing deferral (origin `2026-08-15-corpus-key-persistence`,
ratified 2026-08-16 pin #15, every-3rd-wrap INTERVAL). **Point 46 was DISCHARGED in full form** at
`2026-08-26-cadence-runaway-blocking-pool` — probe RAN, true exit 1 read directly under cargo-audit 0.22.2,
basis byte-identical (`parse error: duplicate advisory ID: RUSTSEC-2026-0244`), overlap re-enumerated
first-hand at the same **eight** owned IDs (0189/0190/0194/0195/0204/0222/0253/0258), unchanged for a fourth
consecutive probe. **Next interval point is 49.** `state.yaml` records `session_count: 46`, so this chunk's
wrap is **point 47 → a SKIP**: re-verify basis + overlap and record `probe skipped per ratified interval
(next: 49)` in the report. Never a silent skip. The pin rides forward to whichever entry is current after
this one.

---

## Boundaries

**In scope:** `pulse-app/src/llamacli_inference.rs` (the guard + the bound + their tests) · the five
security artifacts that name this entry as owner, insofar as they must end the chunk consistent with what
lands · a workspace `Cargo.toml` touch only if the `strict-path` fork resolves toward adoption.

**Out of scope:** the arch-level open caveat on how the CUDA/CPU binaries and the GGUF are DISTRIBUTED
(bundle vs xtask fetch vs env pair) — that is a separate decision the arch entry still owns, and this chunk
guards whatever path the user supplies rather than deciding where it should come from. Also out: the L4
runtime's behaviour, prompt CONTENT, model selection, and the sibling entries' subjects (the interpretation
brief's completeness, the ingest initiating freeze).

## Premise closure (P3, 2026-08-26) — all four resolved

Two verified, two premise-corrected. P4 consumes this corrected text, not the promotion-time wording.

- **P-1 VERIFIED** — a bound weaker than data-dir confinement is both expressible and meaningful. `strict-path`
  0.2.2 provides exactly the operator-declared-allow-root primitive: `PathBoundary::try_new` + `strict_join` +
  `StrictPath`, with `boundary_check` / `canonicalize` / `canonicalize_anchored` / `metadata` beneath them
  (`VirtualRoot` / `VirtualPath` are the sibling virtualized form). A root can therefore hold without breaking
  a user-managed out-of-tree GGUF, because the ROOT is what the operator declares.
- **P-2 VERIFIED, with one sub-fact left unmeasured.** `strict-path` is genuinely adoptable: resolved in
  `Cargo.lock` at **0.2.2** with a checksum, source present in the local registry, and its API expresses
  "canonicalize + assert regular file + confine to root R". What research also established — and the working
  entry never mentioned — is that it has **zero users in the repo today**: declared in `Cargo.toml:49` and in
  five crates' manifests, with no `strict_path` / `StrictPath` / `PathBoundary` / `VirtualPath` occurrence in
  any `.rs` file. Adoption would be its FIRST use. **Unmeasured, deliberately deferred to /implement:** its
  behaviour on Windows extended-length (`\\?\`) paths, which is the form `canonicalize()` already returns on
  this host.
- **P-3 `[premise-corrected: no llama-cli.exe is present on this host and the two bin-path env vars are unset
  in the research shell, so a prompt-file flag in the PINNED b9305 build could not be measured]`** — the
  premise assumed the flag's availability was checkable at plan time. It is not, and arch §Established
  Decisions [LLM Inference Runtime] pins `b9305` exact with "bumps are deliberate chunk-scoped events", so an
  unverified flag may not be assumed. **Consequence for the plan:** the prompt-file route is DECLINED for this
  chunk and Deliverable B is the BOUND, not the relocation. The relocation stays a documented future option
  whose precondition is a measurement against the pinned binary.
- **P-4 `[premise-corrected: the prompt is a first-party TEMPLATE with OTLP-derived content confined to two
  injected sections, and that content is ALREADY PII-scrubbed upstream]`** — the second reading the premise
  offered is the true one. `crates/interpretation/src/prompt.rs:143` builds the value from
  `ROLE_DEFINITION` + `CONVENTIONS_SNIPPET` + `L4_OUTPUT_JSON_SCHEMA` + `OUTPUT_REMINDER` + section headers,
  with variability entering ONLY through `digest_payload` (`digest.payload_summary`), `project_context`, and
  `corpus_retrieval` — the third passed as `""` at all three call sites (`inference_runtime.rs:310/316/322`).
  `payload_summary` is scrubbed by `Digest::scrubbed_clone` (`crates/triage/src/contract.rs:589`), so the
  OTLP-derived substring reaching argv is already redacted. **Consequence for the plan:** the security-plan
  §Code Patterns ban still binds — an OTLP-sourced substring does reach `Command` argv — but the residual is a
  SHAPE-and-BOUND problem (unbounded length, control characters, argv framing), NOT a PII leak. The acceptance
  must say that rather than implying a confidentiality fix.

**Also found at P3, not stated by the entry and load-bearing for the plan:**

- **No production bound on prompt length exists.** The only length assertion in the tree is a TEST
  (`crates/interpretation/src/prompt.rs:637`, `prompt.len() < 8000`); production only *logs* the length
  (`inference_runtime.rs:329`, `token_count = prompt.len()`). Deliverable B closes a real, currently-open gap.
- **`interpretation.model.load.error` is an allowlist leaf with NO PRODUCER.** It is declared at
  `pulse-app/src/observability.rs:1969` carrying `error_msg` / `model_identity` / `recovery_action` /
  `error_category`, and that declaration is its ONLY occurrence repo-wide — zero emit sites. The rejection
  observable can therefore adopt an EXISTING leaf rather than mint one, converting a dead declaration into a
  live target. The leaf still needs a `*_basename` field added to satisfy obs-plan §1 Vector 6, which is a leaf
  COMPLETION on the `triage.cue.tick` precedent, not a new leaf.
- **The blast radius is one file plus one test file.** The code-graph impact query (rust plane, `db_state:
  fresh`) returns: `canonicalize_path` 7 rows — 2 production callers, both inside `LlamaCliInference::new`
  (`:116`, `:117`) — `build_llama_cli_args` 10 rows with exactly ONE production caller
  (`generate_constrained`, `:530`), and `read_env_path` 3 rows, all in-file. Every other row is
  `pulse-app/tests/unit_llamacli_inference.rs`, which already pins both the canonicalize branches and the
  spawn-arg vector. No cross-crate edge, no new file needed.
