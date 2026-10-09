
## 2026-10-06-l4-first-hypothesis-names-the-triggering-service — L4 argv prompt: the observed maximum re-based to 7,824 B; the other two tiers first measured
**Section:** §Input Validation → row "L4 inference argv prompt"; §Security Anti-Patterns → Code Patterns (the `tokio::process::Command` argv ban, its measured-ceiling sentence)
**Change:**
- A dated v2.6 measurement note joins both sites: the synthetic A6 composition at prompt v2.6 measured 7,824 B (2026-10-07, `--dry-run`, primary builder only). The framing instruction's added scope sentence adds 249 B to every composition (the 248 B sentence and its joining space).
- The observed maximum was 7,575 B (A6 at v2.5); now 7,824 B. The ceiling `MAX_PROMPT_BYTES` 16,384 B is unchanged; it sat ~2.16× above the maximum with 8,809 B of headroom, now ~2.09× with 8,560 B (≈ 8.4 KiB). 7,575 B stays as the prior note.
- The clause that the fallback and reflection compositions "are unmeasured" is dated to v2.5. Their first readings are stated, on an EMPTY digest only, at v2.6: primary 7,011 B, fallback 7,200 B (its sanity pin asserts under 8,000 B), reflection 7,549 B.
- The row also records the real-model reading's largest product composition, 7,693 B (the synthetic S8, a retry storm with two corpus matches).
- All of it stays a measurement note, never a bound.
**Why:** The chunk appended one static sentence to the framing instruction, so every composed prompt grew and the recorded maximum moved; the plan's own entry re-bases both sites when it does. The added text is first-party and static, so the row's non-widening argument is untouched: `-p` stays the one OTLP-derived operand, and the validator and the ceiling did not change.
**Kept:** The before-chain of earlier measurements (7,405 B, 7,185 B, 6,932 B, the v2.2 range) as dated. The fallback and reflection tiers remain unmeasured on a LOADED digest; only the empty-digest figures are new.
**Ref:** .andromeda/runs/2026-10-07T06-47-12Z-wrap/
