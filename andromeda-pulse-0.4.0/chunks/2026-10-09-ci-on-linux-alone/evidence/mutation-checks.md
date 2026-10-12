# Mutation check — the release-build witness pin

Chunk `2026-10-09-ci-on-linux-alone`, plan Step 5. A one-shot at /implement on 2026-10-09 (UTC), after Step 3 and
before the gate block. `ci_workflow_keeps_the_linux_release_build_witnesses` passes before and after the workflow
edit, so this mutation is its only control.

The call, the same as Steps 2 and 4:

    cargo nextest run --workspace --profile ci -E 'binary(quality_gate_workflow)'

## The mutation

One token of one step in `.github/workflows/ci.yml`, the `supply-chain` job's build step (`:474` after Step 3),
applied with the Edit tool:

    -        run: cargo auditable build --workspace --release
    +        run: cargo auditable build --workspace

Confirmed present before the run: `grep -n 'cargo auditable build --workspace' .github/workflows/ci.yml` printed
`474:        run: cargo auditable build --workspace`, and the file's sha256 moved from `a81f7baabc512037…` (the Step 3
result) to `564b2b586a85f559…`.

## Reading 1 — under the mutation

Exit 100. `Summary: 19 tests run: 18 passed, 1 failed, 0 skipped`.

The one failure is the pin:

    FAIL pulse-app::quality_gate_workflow ci_workflow_keeps_the_linux_release_build_witnesses
    panicked at pulse-app/tests/quality_gate_workflow.rs:135:5:
    the supply-chain job MUST keep `cargo auditable build --workspace --release`
    (test-plan §9 Pipeline structure, Supply chain row); block: …

The other 18 tests, `ci_workflow_runs_on_linux_only` and `data_dir_export_precedes_every_consumer` among them, stayed
green: none of them reads the build step's profile.

## Reading 2 — restored

The line was restored with the Edit tool. `git diff --no-index --quiet {step-3 copy} .github/workflows/ci.yml` exited
0 against a copy of the file taken before the mutation, and the sha256 read `a81f7baabc512037…` again. The same call
then read exit 0, `Summary: 19 tests run: 19 passed, 0 skipped`.

## Limits

One arm was mutated, the `supply-chain` one, as the plan lists. The pin's second assertion (the `boot` job's
`cargo build --workspace --release --features mcp-server`) was not mutated; it is the same substring check through the
same helper, and nothing here measures it.
