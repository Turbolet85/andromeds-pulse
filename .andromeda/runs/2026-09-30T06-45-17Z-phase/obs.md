# obs extract

## No domain coverage
The chunk adds license texts and changes the SPDX `license` fields in the Cargo, npm and lockfile manifests, plus a cargo-deny/manifest witness. It adds no span, metric, log, error envelope, PII path, SLO or CI telemetry artifact. It also leaves the §3 Service identity inputs alone: `CARGO_PKG_NAME`/`CARGO_PKG_VERSION` are not the `license` key, so the `[workspace.package]` edit must leave `version` untouched.
