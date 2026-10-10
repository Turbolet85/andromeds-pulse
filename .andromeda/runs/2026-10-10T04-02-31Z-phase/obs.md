# obs extract

## Relevance
partial — the chunk is harness and CI evidence work (the series verdict, the kept boot artifact, the process-end reading); it adds no span, metric or product log by intent, and touches the product's log surface only if research names the cause in the app's own start.

## Constraints
- obs-plan §9 Telemetry artifact handling (Log file row) requires `boot-series.json` to hold closed labels of bounded printable-ASCII length and counts, with no environment value and no path. The label a pre-ready self-end receives must stay inside that shape. Whether the series' present label vocabulary already has a member for a boot that took no settle verdict but holds a witness line is research's question.
- obs-plan §9 Telemetry artifact handling (Log file row) enumerates the boot artifact's entries and states that a series boot without a settle verdict keeps four files, its `exit-witness.jsonl` among them. The labelling repair reads that kept file; a new kept entry, a new member of `harness-settled.json` or a new witness line kind is outside that row (and halts under the scope's own boundary).
- obs-plan §7 Error classes captured (Process-end cause) classes the measured self-end as an unloggable end: such a boot is required to hold no `app.exit` and no `app.panic.fatal`, so their absence is the expected reading of a self-end, never evidence of a different cause. The same bullet records the end as named and not closed, with two items not measured (why the connection's read failed; the runner's own library build) and this route entry as owner.
- obs-plan §3 Logging stack (Sink) requires the app to have one sink, the JSON file, with no stderr `tracing` layer. A native GLib/GDK message is therefore not a `tracing` record and is not expected in `agent-latest.jsonl*`; obs-plan §7 Error classes captured places it at "stderr / `boot.log` at most". Whether the message is in `boot.log` of the ended boots is research's question (scope question d).
- obs-plan §3 Tracing init (Init order) names `WorkerGuard::drop` as the non-blocking file sink's only drain, run on every exit path that can log. An `_exit` end runs no such path, so records still queued at that moment are not required to reach the file. Whether any record was lost on the ended boots is research's question; a log-family record count of an ended boot is a lower bound, not a complete reading.
- obs-plan §9 CI Integration (CI-specific default fields) requires `git.commit.sha` on a `pull_request` run to be the merge commit, not the pushed tip. "Equal source" across runs is read against that value: two runs of one pushed tip are equal source only where the merge they built is the same.
- obs-plan §10 CI gates requires the boot job to skip its `ci-gates` step on a run whose smoke or series step fails, and to print the frame line the section states on a run where both pass. A green claimed for the close is a run that reaches `ci-gates`, not only one whose series step exits 0.

## Patterns to follow
- A verdict names its cause from what the same run kept, never from inference: obs-plan §10 Performance budgets (WebGPU canvas frame row) sets this for the 0-frame reading, and obs-plan §9 Telemetry artifact handling applies it to the settle verdict's `exit_witness` label. The series' per-boot label follows the same shape.
- Harness-written evidence sits under the data dir's `logs/` beside the product log, is never read by the product, and is uploaded `if: always()` under a distinct artifact name (obs-plan §9 Telemetry artifact handling).
- A measured reading carries its run id, its n and its limit ("one run measured"), and a reading under one step order is not written as a rule (obs-plan §10 Performance budgets, WebGPU canvas frame row). The close's evidence states boots, runs and the rate an unclosed job would pass at in the same form.
- If the close lands in the app's start as a posture or lever: obs-plan §6 Log levels mapping (`warn` row, `app.boot.render.posture`) is the precedent — one record per boot, emitted after the subscriber exists with the decision carried as a value, closed labels, the lever's NAME and never its value.
- Agent access to the evidence is `gh run download` plus `jq` over the artifact (obs-plan §9 CI failure → artifact triage workflow).

## Anti-patterns to avoid
- No retry-once and no soft budget: obs-plan §11 Obs Anti-Patterns → SLO bans a retry that masks a real failure and any gate without enforcement; a self-end stays red.
- No lost or unstructured CI evidence: obs-plan §11 Obs Anti-Patterns → CI bans dropping the log artifact on failure and parsing terminal text; the per-boot verdicts stay structured files in the artifact.
- No native message text in a product record: obs-plan §8 PII Scrubbing (`app.exit` allowlist bullet) requires that record to carry closed labels only, never a native message, a path or an error `Display`; obs-plan §11 Obs Anti-Patterns → Logs bans unstructured stderr text as an agent surface. Capturing GDK's message by widening a `tracing` record is out.

## Contract bindings
- obs ↔ tests: obs-plan §9 Telemetry artifact handling (the artifact's entries and label sets) binds to test-plan §9 Boot smoke and its failure conditions (which verdicts fail the job). A label added or re-read in the series changes both sides.
- obs ↔ security: obs-plan §7 Error classes captured and §9 Telemetry artifact handling describe the exit-witness record that security-plan §Security Anti-Patterns → Input holds PROVISIONAL; the shape named in the obs sections does not grow in this chunk.
- obs ↔ architecture: obs-plan §9 CI Integration (CI-specific default fields) cites architecture §Infrastructure Patterns → CI/CD approach for the merge-commit reading; the `harness:boot-series` verb is architecture §Occupied Resources' row.
- Wrap-time amendment due if the chunk lands: obs-plan §7 Error classes captured ("named, not closed", the two not-measured items) and the measured readings in obs-plan §9 Telemetry artifact handling and §10 CI gates describe the state before this chunk. The plan is not edited during the phase loop.

## Acceptance criteria contributions
- (obs) After the labelling repair every per-boot entry of `boot-series.json` is a closed label within the stated bound plus counts, with no environment value and no path, and the boot artifact holds no entry kind it did not hold before (per obs-plan §9 Telemetry artifact handling).
- (obs) A series that contains a self-end, including one that ends before ready, fails the job with the log artifact still uploaded; no retry and no soft-fail key appears on the boot or series step (per obs-plan §11 Obs Anti-Patterns → SLO; obs-plan §11 Obs Anti-Patterns → CI).
- (obs) Each run counted toward the close reaches `ci-gates`, reads zero `app.panic.fatal`, and is stated with its run id, its boot count and the merge `git.commit.sha` its log carries (per obs-plan §10 CI gates; obs-plan §9 CI Integration, CI-specific default fields).
- (obs) If the close adds or changes a product log record: it fires at most once per boot off any hot path, carries closed labels only, has an exact allowlist leaf so no field renders `<redacted>`, and holds no path and no native message (per obs-plan §6 Log levels mapping; obs-plan §8 PII Scrubbing).
