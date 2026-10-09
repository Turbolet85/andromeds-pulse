
## 2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape — L4 argv prompt observed maximum re-based to 7,405 B
**Section:** §Input Validation (L4 inference argv prompt row) · §Security Anti-Patterns → Code Patterns (the L4 `-p` bullet), in lockstep
**Change:** The observed maximum was 7,185 B, ~2.28× under the 16384 ceiling with 9,199 B ≈ 9.0 KiB headroom (per "2026-10-04-l4-interpretation-names-its-triggering-cue — L4 argv prompt observed maximum re-based to 7,185 B", kept as the prior note); now 7,405 B — the v2.5 dev-probe composition of the same synthetic S4, `--dry-run`, primary builder only — ~2.21× under the ceiling with 8,979 B ≈ 8.8 KiB headroom. The fallback and reflection compositions carry the same +220 B and are unmeasured. `MAX_PROMPT_BYTES` 16384 and `validate_prompt_bounded` are unchanged.
**Why:** The framing instruction grew 318 → 538 B, so the measurement note the bound's derivation cites moved; it stays a measurement note at both sites, never a bound.
**Ref:** .andromeda/runs/2026-10-05T06-01-47Z-wrap/
