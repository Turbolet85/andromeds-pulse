# arch extract

## Relevance
partial — a probe-only measurement chunk in `pulse-app/examples/` with no product surface change. Arch matters because §Established Decisions [LLM Inference Runtime] and [Fault Identity] both name THIS route entry as the owner of the L4 model choice and fix the runtime and framing it must measure without altering.

## Constraints
- The runtime stays as decided. §Established Decisions [LLM Inference Runtime — L4 interpretation layer] requires:
  - llama.cpp b9305 prebuilt binaries, invoked as D1 spawn-per-generation;
  - bounded invocation: `-n {max_tokens}`, `-st` and an outer wall-clock timeout, with `kill_on_drop`;
  - the two first-party constants `-c 8192` (`LLAMA_CLI_CTX_SIZE`) and `-rea off` (`LLAMA_CLI_REASONING`).

  Pin discipline: a build-tag bump is a deliberate chunk-scoped event, so this chunk takes none (the scope agrees). The probe's argv should mirror these bounds. Whether the probe's existing argv path already carries them is research's question.
- The shipped model is not this chunk's output. The same decision (§[LLM Inference Runtime] → **Model**) requires:
  - the shipped GGUF stays Llama-3.2-3B-Instruct-Q4_K_M;
  - "the model that ships is chosen by the pattern-discrimination route entry and shipped by the entry after it". This entry produces the choice input, the founder decides, and the next entry ships.

  §Inherited Defaults → LLM inference runtime restates the runtime posture.
- The b9305 grammar-prefill trap binds the candidate argv. §[LLM Inference Runtime] records it:
  - under `--json-schema-file`, the qwen35 and gemma4 GGUFs print `Failed to initialize samplers`, exit 0 and emit no JSON;
  - `-rea off` does not prevent it;
  - a `--grammar-file` user grammar is never prefilled.

  Every candidate therefore runs through `--grammar-file`. Whether the trap still reproduces at HEAD is P3's re-verification (scope `[inferred]`).
- The prompt framing is fixed context, not a lever. §Established Decisions [Fault Identity] makes these the framing layer:
  - `triage::digest::render_payload` renders `TRIGGER: {cue_cause_label(kind)}` keyed on `cues.first()`, and NONE without a cue;
  - `interpretation::prompt::TRIGGER_FRAMING_INSTRUCTION` is conditional on a TRIGGER line;
  - the CORPUS MATCHES note frames matches as context only.

  The same decision records that "the remainder is owned by the pattern-discrimination route entry". The chunk changes none of it (the scope agrees). Implication for the shape design: the A1–A6 and B shapes are cueless, so they render no TRIGGER line and get no framing obligation, while A7's wrong-service cue does. Whether the probe's render path reproduces that conditional exactly is research's question.
- Module boundaries hold. §Cross-cutting Patterns → Module dependency direction requires dependencies to flow toward `pulse-app`, with no reverse edge. §Conventions → Module visibility discipline allows a crate's public surface only in its contract module. So a probe-side enriched render that needs a non-`pub` `triage` item, a widened visibility or a new seam is a PRODUCT change, to be surfaced at P4, never slipped in (consistent with the scope's `[inferred]` on the enriched render).
- Workspace naming is locked. §Occupied Resources → Cargo workspace crate names and §Inherited Defaults reserve the 16 member names. The work lands as an example of the existing `pulse-app` crate, never as a new crate.
- Env-var namespace. §Occupied Resources → Environment variables registers the product-consumed `ANDROMEDA_PULSE_MODEL_PATH`, `_LLAMA_CUDA_BIN_PATH` and `_L4_ALLOW_ROOT`.
  - A probe that reuses them must not change their product semantics.
  - Any NEW probe-only variable follows the harness-class precedent: it is NOT prefixed `ANDROMEDA_PULSE_` and does not cross into the product (the `PULSE_*` relay-set entry). It is registered only if it becomes a claimed namespace.
  - Whether the probe introduces any variable is research's question.

## Patterns to follow
- Keep the runtime behind its trait. Per §[LLM Inference Runtime] → Bus-factor / swap paths, `pub trait LlmInferenceRunner` (`crates/interpretation/src/contract.rs`) is the swap boundary: only the concrete impl changes, at the binary boundary. The probe stays beside the product runner and never edits it. `build_llama_cli_args` keeps its signature.
- Use the product's own renderer for "today's" digest. Per §[Fault Identity], `triage::digest::render_payload` is the canonical prompt-payload renderer, so the C-today arm and the A/B shapes go through it. The enriched C render is a probe-side extension (scope Boundaries).
- Build outputs follow the binary-path registry. Per §Occupied Resources → Process / service identity, the build emits `target/{profile}/...` paths, and example binaries land under `target/{profile}/examples/` (the `inject_demo` precedent in the §Infrastructure / §Occupied Resources harness rows). Gitignored `target/` holds the audit texts.
- Agent-driven, machine-parseable outputs. Per §Cross-cutting Patterns → Development Style, the scorer emits schema-stable, bounded labels (detect · cause · valid · no-reading) and a deterministic aggregate table that a later wrap can read without re-interpretation.

## Anti-patterns to avoid
- Never change the shipped model, the product argv constants, sampling or the GBNF in product code, and never bump b9305 or swap the runtime. Each is reserved to the next entry or to a deliberate pin chunk (§[LLM Inference Runtime] → Model + Pin discipline).
- Never read a zero-output `--json-schema-file` run as a valid generation (§[LLM Inference Runtime] → b9305 grammar-prefill trap: exit 0 with no JSON). The scope's CARRY 1 classifies such a row as `no reading`, never `valid` and never a correct B dismissal.
- Never key a score or a grouping on a model-authored field as if it were identity. §[Fault Identity] REJECTED `L4Output.fingerprint` as model-authored. The `cause` label grounds on the pre-registered ground-truth service and cause words, never on model-chosen identifiers.

## Contract bindings
- arch ↔ tests: the predecessor moved the test-plan §1 probe pins 16 → 29 (handoff). The probe file is a pinned surface, so its extension binds to whatever test-plan §1 pins on `l4_decision_probe.rs`. Which pins exist and whether the new scorer needs its own is research's question.
- arch ↔ security: the three product-consumed L4 path variables carry a guard (§Occupied Resources → `ANDROMEDA_PULSE_MODEL_PATH` / `_L4_ALLOW_ROOT`; security-plan §Security Anti-Patterns → Input). A probe reading model and binary paths either reuses that guard or sits in the harness carve-out shape. It must not weaken the product guard.
- arch ↔ arch (wrap amendment): §[LLM Inference Runtime] → Model and §[Fault Identity] both name this entry as the owner of the model choice. At wrap, those two sites expect an amendment that records the measured table and the founder's decision as the next entry's input, and they keep "shipped GGUF = Llama-3.2-3B".

## Acceptance criteria contributions
- (arch) No diff to the product runtime: the b9305 pin, the `LLAMA_CLI_CTX_SIZE` and `LLAMA_CLI_REASONING` constants, the `build_llama_cli_args` signature and the shipped GGUF identity are byte-unchanged outside `pulse-app/examples/` (per architecture §Established Decisions [LLM Inference Runtime — L4 interpretation layer]).
- (arch) No diff to the framing layer: the `render_payload` TRIGGER and CORPUS MATCHES behaviour and `TRIGGER_FRAMING_INSTRUCTION` are unchanged. The enriched render exists only inside the probe (per architecture §Established Decisions [Fault Identity]).
- (arch) The work lives in `pulse-app/examples/l4_decision_probe.rs` plus probe-only assets. There is no new workspace crate, no new reverse dependency edge, and no contract-module visibility widened for the probe unless it was surfaced and ratified at P4 (per architecture §Occupied Resources → Cargo workspace crate names; §Cross-cutting Patterns → Module dependency direction; §Conventions → Module visibility discipline).
- (arch) No new `ANDROMEDA_PULSE_*` variable. Any probe-only variable is harness-class, unprefixed and never consumed by the product binary (per architecture §Occupied Resources → Environment variables).
