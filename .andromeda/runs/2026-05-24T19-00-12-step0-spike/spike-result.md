# Step 0 Spike Result — chunk #82/#83

**Date:** 2026-05-24
**Spike test:** `pulse-app/tests/spike_mistralrs_strict_schema.rs`
**Model:** Llama-3.2-3B-Instruct-Q4_K_M.gguf
**Model path:** `D:/dev/projects/andromeda-pulse/AI-Model/Llama-3.2-3B-Instruct-Q4_K_M.gguf`
**Model SHA256:** `6c1a2b41161032677be168d354123594c0e6e67d2b9227c84f296ad037c728ff`
**mistralrs version:** 0.8.0 (pin-exact per arch §Established Decisions)
**Host:** Windows 11 Pro / D: workspace; CPU inference (forced via `.with_force_cpu()`)

## Verdict

**MIXED — load PASSES; inference latency MASSIVELY violates the L4 SLO.**

The spike demonstrates that the mistralrs 0.8.0 API binding works end-to-end through Phase 1 (model load) — the runtime accepts our `GgufModelBuilder::new(parent_dir, [filename])` invocation, parses the GGUF file, initializes weights into ~4.56 GB RAM, and exposes the loaded `Model` to subsequent inference calls. The chunk #82 `LlmInferenceRunner` trait surface + chunk #83 `MistralRsInference::generate_constrained` body are real-API-compatible.

But the spike ALSO demonstrates that primary-tier inference on а 3B Q4 GGUF model under strict-schema-mode constrained generation, with our 6275-char (~1500-token) primary-tier prompt, fails the dist-arch v3 §L4 hardware profile matrix SLO budget by а factor of >120×. The inference did NOT complete after 60 minutes of active CPU-saturated generation; we terminated it before it produced а parseable JSON output, so the JSON round-trip + schema validation portion of the spike did not run.

## Phase-by-phase findings

### Phase 1 — Model load

```
[spike] model path: "D:/dev/projects/andromeda-pulse/AI-Model/Llama-3.2-3B-Instruct-Q4_K_M.gguf"
[spike] phase 1: load starting
[spike] phase 1: load complete (58.92s) — result: Ok("Ok")
[spike] phase 1: model identity = Some(ModelIdentity { semantic_name: "Llama-3.2-3B-Instruct-Q4_K_M" })
```

**PASS.** Load wall time: **58.92s**. RAM working set after load: **~4.56 GB**.

`GgufModelBuilder::new(parent_dir, [filename]).with_force_cpu().build().await` succeeded. The semantic identity derived from the filename ("Llama-3.2-3B-Instruct-Q4_K_M") was attached к the runner via `ModelIdentity`; lifecycle transitioned to `ModelStatus::Loaded`; `ModelLoadEvent` emitted on the `pulse://stream/model-status` broadcast.

Notable detail: the load surfaces NO file path / no checkpoint URL / no mistralrs internal type names in the model identity (semantic name only) — the chunk #82/#83 security discipline holds at the real-API boundary.

### Phase 2 — Strict-schema inference

```
[spike] phase 2: prompt assembled (6275 chars; fixture digest + project context + schema embed + output reminder)
[spike] phase 2: inference starting
test spike_mistralrs_strict_schema_round_trips_against_real_model has been running for over 60 seconds
[... 60 minutes later — manual termination ...]
```

**FAIL by SLO.** Inference wall time when terminated: **~60 minutes** (>3600s).

Per dist-arch v3 §L4 hardware profile matrix:

| Profile | Primary-tier inference latency target | Tier-1 SLO |
|---|---|---|
| `gpu-primary` | 1-3s | < 5s |
| `gpu-fallback` | (fallback model) | < 3s |
| `cpu-primary` | 10-30s | < 30s (degraded) |
| `cpu-fallback` | 3-8s | < 15s |

The host's hardware profile classifies as `cpu-primary` (no GPU; primary-tier 3B model). The arch's degraded SLO budget is **30 seconds**. Actual observed: **inference did not complete after 60 minutes**, а delta of **≥120× over the SLO**. The model was actively burning CPU the whole time (CPU time accumulated к ~67,000s = ~18.7 CPU-hours across multi-core threads; not stalled, just slow).

The JSON round-trip + `interpretation::schema::parse_bounded` validation portion of the spike DID NOT EXECUTE because inference never completed.

### Phase 3 — JSON round-trip + schema validation

**NOT REACHED.** The inference phase was terminated before producing parseable output.

## Design-vs-reality API deltas

The mistralrs 0.8.0 strict-schema-mode API surface differs from the stub's optimistic assumptions in three small-but-load-bearing ways. All three are addressable WITHOUT trait surface changes:

### Delta 1 — Schema source

- **Stub assumed:** the trait accepts а JSON-schema STRING and the runtime handles parsing internally.
- **Reality:** mistralrs 0.8.0 wants а `serde_json::Value` for `Constraint::JsonSchema(value)`. The convenience method `Model::generate_structured<T>` derives the schema от а Rust TYPE via `schemars::JsonSchema` — we DON'T use this path because our schema is а static document (`L4_OUTPUT_JSON_SCHEMA: &str`), not parameterized over types.
- **Resolution:** parse the schema string к `serde_json::Value` inside the concrete impl; use the lower-level `RequestBuilder::set_constraint(Constraint::JsonSchema(value))` + `send_chat_request` path that `generate_structured` itself uses internally. Trait surface unchanged.

### Delta 2 — Request shape

- **Stub assumed:** the trait passes а raw `&str` prompt.
- **Reality:** mistralrs's `send_chat_request<R: RequestLike>(req)` takes а structured request. The conversion path is `TextMessages::new().add_message(TextMessageRole::User, prompt).into() -> RequestBuilder`, then `.set_constraint(...)` к layer the schema constraint on top.
- **Resolution:** wrap the prompt в а one-message-deep `TextMessages` inside the concrete impl. Trait surface unchanged.

### Delta 3 — Response shape

- **Stub assumed:** the trait returns а raw JSON String, downstream parses via `serde_json`.
- **Reality:** mistralrs returns а `ChatCompletionResponse` с а `choices: Vec<...>` field; the strict-schema-mode JSON output lands at `response.choices[0].message.content` as а String.
- **Resolution:** extract the first choice's content string in the concrete impl; expose к the trait as the raw String the trait already promised. Downstream `interpretation::schema::parse_bounded` consumes it unchanged. Trait surface unchanged.

**Overall delta assessment:** the chunk #82/#83 substrate trait design (`generate_constrained(&str, &str) -> Future<String>`) is well-chosen — it abstracts over the runtime в а way that LET us cleanly translate к mistralrs's actual API without forcing а trait rewrite. Bus-factor mitigation per arch §Established Decisions [LLM Inference Runtime] is intact: the swap к candle would land а sibling `CandleInference` impl с the same trait surface and а different concrete body.

## L4 SLO matrix implication

The arch §Established Decisions [LLM Inference Runtime] §Hardware Profile Matrix gives concrete latency budgets per hardware profile. The matrix appears to be derived от mistralrs's published benchmarks for plain (unconstrained) generation, NOT strict-schema-mode generation. Strict-schema-mode adds per-token grammar enforcement overhead which is invisible to the matrix.

For а 3B Q4 GGUF model on the spike's CPU host с our 6275-char prompt:

- **Plain chat (estimated)**: ~30-90s per inference (per general 3B-Q4-on-CPU rules of thumb с llama.cpp / mistralrs)
- **Strict-schema-mode (observed)**: ≥60min, did not complete

The strict-schema mode appears to add а large multiplier on top of plain inference, likely because the L4 schema (с deeply-nested object / array constraints + multiple enum unions) forces token-by-token grammar tree walking. The arch's 10-30s budget for cpu-primary is unrealistic for this combination.

## What the spike validated (positive findings)

1. **mistralrs 0.8.0 API surface is real + bound correctly.** Imports resolve, types match, GGUF load works, strict-schema mode call succeeds в build-time + reach-runtime sense.
2. **Chunk #82/#83 trait surface translates к mistralrs without redesign.** No `LlmInferenceRunner` trait modifications needed.
3. **Boot-time graceful-degraded mode is preserved.** The fire-and-forget load в `main.rs` works (model loads cleanly + does not block boot).
4. **Security discipline holds.** No paths / library types / GPU device strings leak from the API boundary.
5. **Cargo deny gate clean.** mistralrs's transitive deps (~50 crates) pass `cargo deny check bans licenses sources` — no new advisory ignores needed.

## What the spike INVALIDATED (the hard finding)

**The arch §Established Decisions [LLM Inference Runtime] §Hardware Profile Matrix `cpu-primary` SLO budget of 10-30s does NOT hold для strict-schema-mode inference of the L4 prompt against а 3B Q4 model on this host CPU.** Reality is ≥120× slower (>60 min observed). Production deployment on CPU-only hosts is not viable с the current prompt / schema / model combination.

## Recommended next steps (FYI — not part of chunk #82/#83 scope)

1. **Re-validate the SLO matrix.** Either:
   - (a) Restrict L4 inference to GPU-equipped hosts (gpu-primary/gpu-fallback profiles ONLY; cpu-* profiles route to graceful-degraded mode + skip L4 entirely)
   - (b) Adjust the cpu-primary SLO budget upward к match measured reality (60+ minutes is too slow even for а degraded SLO; consider 5-10 minutes max if we want CPU inference at all)
   - (c) Use а smaller fallback model on cpu-* profiles (chunk #84 fallback tier territory) AND а tighter prompt AND а simpler schema
2. **Profile the strict-schema-mode overhead specifically.** Run а pair-test: same prompt + same model + same host, once с strict-schema mode and once unconstrained. The delta IS the schema-grammar-enforcement cost. This data point would inform whether the schema needs simplification or whether strict-schema-mode is fundamentally not GPU-free-able.
3. **Consider the digest size budget.** Our prompt is 6275 chars (~1500 tokens). Per dist-arch v3 §L4 input composition the target is 6-8K tokens TOTAL (prompt + project context + L3 digest + corpus retrieval). The fixture digest here was small; а real digest could be 4-8K tokens larger, making inference proportionally slower. Reducing the digest token budget (currently 500-2000 soft per chunk #81) could help if CPU is the deployment target.

## Spike artifacts committed

- `pulse-app/tests/spike_mistralrs_strict_schema.rs` (the spike test itself, `#[ignore]`-gated)
- `.andromeda/runs/2026-05-24T19-00-12-step0-spike/spike-result.md` (this file)

The spike test will sit `#[ignore]`-gated in the codebase as а future-regression catcher. When the underlying mistralrs binding evolves OR а GPU runner lands, re-running с `cargo test ... -- --ignored --nocapture` retests the same surfaces.
