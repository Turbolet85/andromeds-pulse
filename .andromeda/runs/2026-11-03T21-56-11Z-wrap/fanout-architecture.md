proposals:
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies (Plugin runtime row)"
    change: "Plugin runtime row: `wasmtime` 25+ family — Cargo.toml requirement `48.0.8`, lockfile-resolved **48.0.8** as of 2026-11-03 (Cranelift 0.135.8, pulley 48.0.8; chunk 2026-11-03-advisories-since-2026-10-12-closed, `cargo update -p wasmtime`, 28 packages); that bump left `cargo audit` at 0 vulnerabilities with no advisory ignore naming wasmtime; the earlier bumps (48.0.3 closing RUSTSEC-2026-0325 / -0326 / -0327, 46.0.3 closing RUSTSEC-2026-0316) and the 49.x-out-of-reach-at-1.95 clause stand as written"
    sidecar: "2026-11-03 chunk 2026-11-03-advisories-since-2026-10-12-closed: Stack Plugin runtime row re-pinned to wasmtime requirement/lockfile 48.0.8, Cranelift 0.135.8"
    rationale: "Report Dependencies restates `wasmtime = \"48.0.4\"` -> `\"48.0.8\"` (Cargo.toml:136) and bumps wasmtime/pulley 48.0.5 -> 48.0.8, cranelift 0.135.5 -> 0.135.8; the row still states requirement 48.0.4 / lockfile 48.0.5 / Cranelift 0.135.5 (report Counts `wasmtime-ver`, `cranelift-ver`: stale; Expected amendments name this site)"
    basis: "doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-np6ugxqp · architecture fct-wn2ucoyt · Cargo.toml:136"
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies (Primary language / runtime row)"
    change: "Primary language row: the resolved graph now carries wasmtime and its internal crates 48.0.8, cranelift 0.135.8, pulley 48.0.8 (chunk 2026-11-03-advisories-since-2026-10-12-closed); the count of 28 packages declaring `rust-version = \"1.95.0\"` stands as measured at chunk `2026-10-04-declared-rust-floor-matches-the-code` on wasmtime 48.0.5 / cranelift 0.135.5 and was not re-read at 48.0.8, while the pinned 1.95.0 toolchain builds 48.0.8 and `declared_floor_equals_the_pinned_channel` passes on it; the rest of the row unchanged"
    sidecar: "2026-11-03 chunk 2026-11-03-advisories-since-2026-10-12-closed: Primary language row names the 48.0.8 / 0.135.8 graph and dates the 28-package rust-version count to its 48.0.5 measurement"
    rationale: "Report Dependencies (wasmtime/pulley/cranelift bumped, 28 packages) and Outcome (arch acceptance: toolchain 1.95.0, `declared_floor_equals_the_pinned_channel` passes on 48.0.8); report Counts `wasmtime-ver` / `cranelift-ver` name this site (architecture:14) stale"
    basis: "doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-aklotj3w · architecture fct-moirbwp3"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions ([Plugin Runtime])"
    change: "[Plugin Runtime] heading: WASM Component Model via `wasmtime` 25+ (requirement `48.0.8`, lockfile-resolved 48.0.8 as of 2026-11-03) with WIT interfaces; the rationale unchanged"
    sidecar: "2026-11-03 chunk 2026-11-03-advisories-since-2026-10-12-closed: [Plugin Runtime] decision re-pinned to wasmtime 48.0.8"
    rationale: "Report Dependencies restates the wasmtime requirement 48.0.4 -> 48.0.8 and lockfile 48.0.5 -> 48.0.8; the decision still reads 48.0.4 / 48.0.5 (report Counts `wasmtime-ver`, architecture:57)"
    basis: "doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · architecture fct-xsj444ab"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Inherited Defaults (Plugin runtime)"
    change: "Plugin runtime default: `wasmtime` 25+ Component Model with WIT interfaces (requirement `48.0.8`, lockfile-resolved 48.0.8 as of 2026-11-03), capability-scoped; the `pulse:plugin-fs` sentence unchanged"
    sidecar: "2026-11-03 chunk 2026-11-03-advisories-since-2026-10-12-closed: Inherited Defaults plugin runtime re-pinned to wasmtime 48.0.8"
    rationale: "Report Dependencies (wasmtime 48.0.4/48.0.5 -> 48.0.8); report Counts `wasmtime-ver` names architecture:303 stale"
    basis: "doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · architecture fct-jcbi3j36"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies (Storage engine row)"
    change: "Storage engine row: DuckDB 1.5.x via the `duckdb` crate — Cargo.toml requirement `1.10500` (caret, untouched), lockfile-resolved **1.10506.1** as of 2026-11-03 (chunk 2026-11-03-advisories-since-2026-10-12-closed, on the founder's word)"
    sidecar: "2026-11-03 chunk 2026-11-03-advisories-since-2026-10-12-closed: Storage engine row lockfile-resolved duckdb 1.10505.0 -> 1.10506.1, requirement unchanged"
    rationale: "Report Dependencies: duckdb / libduckdb-sys 1.10505.0 -> 1.10506.1, requirement `1.10500` unchanged, DuckDB 1.5.x line kept (`inputs#I2` item 1); report Counts `duckdb-ver` names architecture:20 stale"
    basis: "doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · architecture fct-6y2lp7rz"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions ([Database])"
    change: "[Database] heading: DuckDB embedded via the `duckdb` crate — requirement `1.10500` (caret) in Cargo.toml, lockfile-resolved 1.10506.1 as of 2026-11-03 (DuckDB 1.5.x bundled), in-memory ring buffer; the rationale unchanged"
    sidecar: "2026-11-03 chunk 2026-11-03-advisories-since-2026-10-12-closed: [Database] decision lockfile-resolved duckdb 1.10506.1"
    rationale: "Report Dependencies (duckdb 1.10505.0 -> 1.10506.1); report Counts `duckdb-ver` names architecture:54 stale"
    basis: "doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · architecture fct-rj5pbaue"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Inherited Defaults (Database)"
    change: "Database default: DuckDB 1.5.x via the `duckdb` crate — requirement `1.10500` (caret), lockfile-resolved 1.10506.1 as of 2026-11-03 — in-memory ring buffer (5–10 min retention, configurable)"
    sidecar: "2026-11-03 chunk 2026-11-03-advisories-since-2026-10-12-closed: Inherited Defaults database lockfile-resolved duckdb 1.10506.1"
    rationale: "Report Dependencies (duckdb 1.10505.0 -> 1.10506.1); report Counts `duckdb-ver` names architecture:296 stale"
    basis: "doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · architecture fct-a62vsmwv"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies (Xlib thread initialisation row)"
    change: "Xlib row: replace «`tao` stays 0.35.0» with «no Cargo dependency or manifest change of its own; `tao` is 0.37.2 since chunk 2026-11-03-advisories-since-2026-10-12-closed (the Tauri 2.12.5 move)», and in the Why clause name the device-event thread as measured on `tao` 0.35.x and 0.37.1 (as measured at chunk 2026-10-10-boot-smoke-s-self-end-closed), not measured on 0.37.2, while the boot series (2 boots settled) and CI's boot smoke on `ubuntu-22.04` read green on 0.37.2; the rest of the row unchanged"
    sidecar: "2026-11-03 chunk 2026-11-03-advisories-since-2026-10-12-closed: Xlib row's tao qualifier moved 0.35.0 -> 0.37.2, the mechanism dated to its 0.35.x / 0.37.1 measurement"
    rationale: "Report Dependencies: tao 0.35.0 -> 0.37.2 under tauri-plugin-updater 2.14.1 / Tauri 2.12.5 (`inputs#I2` item 2); report Counts `tauri-ver` names architecture:35 a stale qualifier, the mechanism measured on 0.35.x and 0.37.1, not 0.37.2, boot series and CI boot smoke green on 0.37.2"
    basis: "doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-szuihdoq · architecture fct-s36jrtky"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions ([License])"
    change: "[License]: `tauri.conf.json` sets no `bundle.license`, which defaults to the Cargo value (as read at tauri-utils 2.9.0; tauri-utils is 2.10.2 since chunk 2026-11-03-advisories-since-2026-10-12-closed, the default not re-read there — no bundle was built); the rest unchanged"
    sidecar: "2026-11-03 chunk 2026-11-03-advisories-since-2026-10-12-closed: [License] bundle-license default dated to tauri-utils 2.9.0, now 2.10.2 unmeasured"
    rationale: "Report Dependencies: tauri-utils 2.9.0 -> 2.10.2; report Counts `tauri-ver` names architecture:79 a stale version qualifier, the default's behaviour at 2.10.2 not measured (no bundle built this chunk)"
    basis: "doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-szuihdoq · architecture fct-s765xh2i"
  - detector: D-arch-decisions
    severity: warning
    section: "§Occupied Resources (Tauri IPC routes — telemetry.frontend.record_ipc_rejection)"
    change: "record_ipc_rejection entry: the webview-side classification rests on Tauri 2.11 rejecting a denied command with a message containing «not allowed» and no runtime-side hook at that version, as read at Tauri 2.11; Tauri is 2.12.5 since chunk 2026-11-03-advisories-since-2026-10-12-closed and that behaviour was not re-probed at 2.12.5; the rest unchanged"
    sidecar: "2026-11-03 chunk 2026-11-03-advisories-since-2026-10-12-closed: IPC-rejection classification dated to Tauri 2.11, Tauri now 2.12.5 not re-probed"
    rationale: "Report Dependencies: tauri 2.11.0 -> 2.12.5; report Counts `tauri-ver` names architecture:174 a version qualifier whose behaviour at 2.12.5 was not measured by a dedicated probe this chunk"
    basis: "doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp · doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-szuihdoq · architecture fct-qiuibtut"
note: "D-arch-resources clean — report Symbols/APIs, Crates/modules and Harness surface are none (no IPC method, port, socket, env var or workspace crate); crate names new to the lockfile are registry dependencies, not workspace members. Retired is none, so no D-retired. Stack rows naming Tauri 2.x, tauri-plugin-notification 2.x and tauri-plugin-updater 2.x stay true."
