# Slot 2 - GREEN re-measure and the end-to-end storm (plan Step 15)

Granted by the overseer 2026-10-01 ("slot 2 and the 4317 grant are given"; ports measured free,
conductor-builder idle). Shipped tree: the A5 schema order (prompt v2.3 / v1.2-fallback /
v1.2-reflection) plus the Step 13 OVERALL line. A1 was declined by the founder's live word
(2026-10-01, relayed by the pc overseer), so the pre-registered order fell through A3 (23/30) to
A5 (30/30).

## (a) `l4_decision_probe --arms shipped --n 10 --min 27` (plan gate 19, driven by hand)
Same binary and model as Slot 1 (`llama-cli.exe` b9305 cuda, `Llama-3.2-3B-Instruct-Q4_K_M.gguf`),
exit 0, wall 131 s, 0 failed generations.
```
arm shipped: would_create 29/30 · decision 19/0/11 · severity_none 1 · resolution_summary 0
  arm shipped: per shape would_create S1 10 S2 9 S3 10 · failed 0 · first_keys schema_version>prompt_version>title=30 · distinct outputs S1 10 S2 10 S3 10 · wall 131s
l4-decision-probe: verdict: PASS · would_create 29/30
```
- Combined change: **29/30** (PASS at the pre-registered 27). Attribution, per the overseer's rule:
  A2's own Slot 1 delta r(A2) - r(A0) = 28 - 24 = **+4**; A5 alone measured 30/30 in Slot 1.
- Claim scope: synthetic storm digests, n = 30 under the default sampler. This bounds nothing
  broader than that count.

## (b) The end-to-end storm (gates 20-24)
- Warm debug `target/debug/pulse-app.exe` (gate 17) booted by path on one fresh exported data dir
  (`pulse-slot2-20261001T161843Z` under the OS temp dir), the CUDA and model L4 vars set,
  `ANDROMEDA_PULSE_L4_DETERMINISTIC` unset (measured: absent from the launch shell). :4317 accepted
  within ~1 s; pidfile pid 19732 = `pulse-app` (Get-Process).
- Gate 18 / 20 (liveness): green.
- Gate 21 (`inject_demo`, arg-less, by path): exit 0, 16200 spans / 3540 identical-fingerprint
  exceptions, 16:19:10Z -> 16:24:17Z.
- Gate 22 (created-incident poll): **green on the first poll, count 2**.
- Gate 23 (report-only created-or-skipped count): 7.
- Gate 24 (smoke: `app.panic.fatal` + `"level":"ERROR"`): **exit 1, last line 0** (green).

Outcome records in the log family (bounded labels only):
- `interpretation.json.parse`: 8 generations, all `parse_outcome ok`; `interpretation.prompt.assemble`
  8x `prompt_version v2.3`; requests by digest kind: cadence_tier1 2 · cadence_tier2 1 · cadence_tier3 5.
- `interpretation.incident.created`: created 2 (`severity warn`, `priority_tier suggested`) ·
  deduped 1 (`severity error`, `priority_tier autonomous`).
- `interpretation.incident.skipped`: 5, all `skip_reason no_cue` on `digest_kind cadence_tier3`
  (decision surface; severity autonomous 4 / suggested 1) - baseline-cadence digests carrying no
  attention cue, which create nothing by the decided semantics. 0 fields rendered `<redacted>`:
  the new leaf reaches the wire whole.
- Every generation that reached the model is counted as created, deduped or skipped (2 + 1 + 5 = 8).
- WARN records: 1, `interpretation.model.allow_root` (the once-per-boot unconfined-posture record;
  `ANDROMEDA_PULSE_L4_ALLOW_ROOT` unset). ERROR: 0. Panics: 0.

Teardown: `Stop-Process -Id 19732 -Force`; pid 19732 not alive afterwards; :4317 and :4318 refuse
connections (released); no `pulse-app`, `inject_demo`, `llama-cli` or `l4_decision_probe` process left.
