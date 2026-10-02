// install-launch-ingest-query smoke harness (chunk #51).
//
// Per route#51 + arch §Cross-cutting Patterns "Test-time telemetry
// injection": the smoke launches an installed `pulse-app` bundle as a
// subprocess, polls loopback OTLP ports, injects a synthetic OTLP span
// via the same boundary that external SDKs use (HTTP `/v1/traces` on
// `127.0.0.1:4318` per arch §Occupied Resources Network ports), then
// tails `agent-latest.jsonl` to assert platform-specific boot spans +
// zero panics + PII canary scrubbed.
//
// The harness lives in `xtask/` per arch §Established Decisions
// [CI Task Runner] — `cargo xtask smoke --bundle <path> --format <fmt>`
// is the canonical invocation. Mirrors the chunk #50 e2e_p1 pattern
// (`pulse-app/tests/e2e_p1_otlp_grpc_to_traces_query.rs`) but targets a
// bundled subprocess instead of in-process boot.
//
// OTLP injection uses HTTP (`:4318`) rather than gRPC (`:4317`) to
// avoid pulling gRPC client framing into xtask. Per combined.md
// §Acceptance arch row 3 "127.0.0.1:4317 and/or 127.0.0.1:4318"; both
// are spec-compliant test surfaces.

use std::path::{Path, PathBuf};
use std::process::{ExitCode, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use prost::Message;
use serde_json::Value;
use tempfile::TempDir;
use tokio::net::TcpStream;
use tokio::process::{Child, Command};
use tokio::time::{Instant, sleep};

use crate::bundle_format::BundleFormat;

// Per security plan §Security Anti-Patterns § API row 7: synthetic OTLP
// fixtures bind/target 127.0.0.1 only. Loopback IS the de-facto authorization
// boundary; non-loopback fixtures would invalidate the Minimal-tier risk
// model in tests.
const LOOPBACK: &str = "127.0.0.1";

// PII negative canary injected as an OTLP attribute value. Post-smoke
// jsonl tail MUST NOT contain this literal — verifies obs-plan §8
// default-deny scrubber holds on the installed bundle (per security plan
// §Logging row 1 / obs Vector 1).
const CANARY_VALUE: &str = "secret-canary-bundle-smoke-12345";

const DEFAULT_HTTP_PORT: u16 = 4318;
const DEFAULT_GRPC_PORT: u16 = 4317;
const READINESS_TIMEOUT: Duration = Duration::from_secs(10);
const POLL_INTERVAL: Duration = Duration::from_millis(500);
// After OTLP injection, give the receiver enough time to decode + scrub +
// flush spans to the JSON log file. 2s is generous on local dev.
const POST_INJECT_FLUSH: Duration = Duration::from_secs(2);
const SHUTDOWN_GRACE: Duration = Duration::from_secs(5);

pub async fn run_smoke(bundle: &Path, format: BundleFormat) -> Result<ExitCode> {
    if !bundle.exists() {
        bail!("bundle artifact not found: {}", bundle.display());
    }

    if let Some(detected) = BundleFormat::from_path(bundle) {
        if detected != format {
            bail!(
                "bundle path extension implies format {:?} but --format {:?} was passed; verify --bundle and --format are consistent",
                detected,
                format,
            );
        }
    }

    if !format.matches_host() {
        // Cross-platform invocation (e.g., `cargo xtask smoke --format msi`
        // on macOS) — install recipe cannot run; surface clearly rather
        // than crash with a cryptic per-tool error.
        bail!(
            "smoke for format {:?} requires host platform '{}'; current host is '{}'",
            format,
            format.host_platform(),
            current_host(),
        );
    }

    // Per arch §Occupied Resources Filesystem locations: bundled-app smoke
    // tests set ANDROMEDA_PULSE_DATA_DIR to a temp dir per-job to keep
    // cross-platform `%APPDATA%` / `~/Library/Application Support/` /
    // `~/.andromeda-pulse/` resolution from leaking state between matrix jobs.
    let tempdir = TempDir::new().context("create per-smoke tempdir")?;
    let data_dir = tempdir.path().to_path_buf();
    let log_dir = data_dir.join("logs");
    let log_file = log_dir.join("agent-latest.jsonl");
    println!(
        "smoke: tempdir={} log_file={}",
        data_dir.display(),
        log_file.display()
    );

    let binary_path = install_bundle(bundle, format, &data_dir)
        .await
        .context("install bundle")?;
    println!("smoke: installed binary at {}", binary_path.display());

    let mut child = launch_bundle(&binary_path, &data_dir)
        .await
        .context("launch bundle")?;
    let pid = child.id().context("captured PID")?;
    println!("smoke: launched PID={pid}");

    let smoke_result = run_smoke_inner(&mut child, &log_file, format).await;
    let cleanup_result = cleanup_bundle(&mut child, &data_dir).await;

    if let Err(e) = cleanup_result {
        eprintln!("smoke: cleanup warning — {e:#}");
    }

    smoke_result?;

    println!("smoke: PASS");
    Ok(ExitCode::SUCCESS)
}

async fn run_smoke_inner(child: &mut Child, log_file: &Path, format: BundleFormat) -> Result<()> {
    let pid = child.id().unwrap_or(0);
    poll_readiness().await.with_context(|| {
        format!("loopback OTLP ports not ready within {READINESS_TIMEOUT:?} for PID={pid}")
    })?;

    inject_synthetic_span().await.context("inject OTLP span")?;
    sleep(POST_INJECT_FLUSH).await;

    assert_log_invariants(log_file, format).context("verify post-smoke log invariants")?;
    Ok(())
}

// ===== install / launch / cleanup =====

async fn install_bundle(bundle: &Path, format: BundleFormat, data_dir: &Path) -> Result<PathBuf> {
    match format {
        BundleFormat::Msi => install_msi(bundle, data_dir).await,
        BundleFormat::Dmg => install_dmg(bundle, data_dir).await,
        BundleFormat::AppImage => install_appimage(bundle).await,
        BundleFormat::Deb => install_deb().await,
    }
}

async fn install_msi(bundle: &Path, data_dir: &Path) -> Result<PathBuf> {
    // Windows MSI: install to a custom dir under data_dir to keep state
    // isolated per matrix job. `msiexec /qn` is silent (no UI). INSTALLDIR
    // override + ALLUSERS=2 means current-user install (no admin needed).
    let install_dir = data_dir.join("install");
    tokio::fs::create_dir_all(&install_dir).await?;
    let status = Command::new("msiexec")
        .args(["/qn", "/i"])
        .arg(bundle)
        .arg(format!("INSTALLDIR={}", install_dir.display()))
        .arg("ALLUSERS=2")
        .arg("MSIINSTALLPERUSER=1")
        .status()
        .await
        .context("spawn msiexec")?;
    if !status.success() {
        bail!("msiexec install failed: exit={status}");
    }
    Ok(install_dir.join("andromeda-pulse.exe"))
}

async fn install_dmg(bundle: &Path, data_dir: &Path) -> Result<PathBuf> {
    // macOS DMG: hdiutil attach mounts the disk image; binary lives in the
    // mounted .app bundle. Mount point is captured from hdiutil's plist
    // output if possible, else falls back to /Volumes/<bundle-name>.
    let output = Command::new("hdiutil")
        .args(["attach", "-nobrowse", "-noverify"])
        .arg(bundle)
        .output()
        .await
        .context("spawn hdiutil attach")?;
    if !output.status.success() {
        bail!(
            "hdiutil attach failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    // Track mount point in data_dir so cleanup can detach (best-effort).
    let mount_point = parse_hdiutil_mount(&output.stdout).unwrap_or_else(|| {
        PathBuf::from("/Volumes").join(
            bundle
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "andromeda-pulse".to_string()),
        )
    });
    let marker = data_dir.join("dmg-mount.txt");
    let _ = tokio::fs::write(&marker, mount_point.to_string_lossy().as_bytes()).await;
    Ok(mount_point
        .join("andromeda-pulse.app")
        .join("Contents")
        .join("MacOS")
        .join("andromeda-pulse"))
}

fn parse_hdiutil_mount(stdout: &[u8]) -> Option<PathBuf> {
    // hdiutil attach default output (text format) emits tab-separated rows
    // ending with `<mount-point>` on the row whose device is a disk image.
    // Take the last `/Volumes/...` token observed.
    let text = std::str::from_utf8(stdout).ok()?;
    text.lines()
        .filter_map(|line| {
            line.split_whitespace()
                .find(|tok| tok.starts_with("/Volumes/"))
        })
        .next_back()
        .map(PathBuf::from)
}

async fn install_appimage(bundle: &Path) -> Result<PathBuf> {
    // AppImage is a self-contained executable; install = chmod +x.
    // Returns the bundle path itself as the launchable binary.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = tokio::fs::metadata(bundle).await?.permissions();
        perms.set_mode(perms.mode() | 0o111);
        tokio::fs::set_permissions(bundle, perms).await?;
        Ok(bundle.to_path_buf())
    }
    #[cfg(not(unix))]
    {
        let _ = bundle;
        bail!("AppImage install requires Unix host; current build target is non-Unix");
    }
}

async fn install_deb() -> Result<PathBuf> {
    // .deb requires sudo dpkg -i, which generally fails in CI without admin. Per
    // plan Implementation notes, .deb smoke may be deferred to follow-on if
    // CI sudo cost is prohibitive. Surface a clear "not supported here"
    // message rather than crash; CI matrix can opt into .AppImage instead.
    bail!(
        "deb smoke requires `sudo dpkg -i` (privileged install); deferred per chunk #51 \
         plan Implementation notes. Use --format AppImage for Linux smoke instead."
    );
}

async fn launch_bundle(binary: &Path, data_dir: &Path) -> Result<Child> {
    if !binary.exists() {
        bail!("installed binary not found at {}", binary.display());
    }
    let mut cmd = Command::new(binary);
    cmd.env("ANDROMEDA_PULSE_DATA_DIR", data_dir)
        .env("ANDROMEDA_PULSE_LOG_LEVEL", "debug")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    let child = cmd
        .spawn()
        .with_context(|| format!("spawn bundle binary at {}", binary.display()))?;
    Ok(child)
}

async fn cleanup_bundle(child: &mut Child, data_dir: &Path) -> Result<()> {
    // tokio::process::Child::start_kill sends SIGKILL on Unix, TerminateProcess
    // on Windows. For a smoke harness, terminate-without-graceful is acceptable
    // — we're not testing graceful shutdown; we need the process gone. Pair
    // with the `kill_on_drop(true)` set at spawn time as a belt-and-suspenders
    // guard against a panic mid-cleanup.
    let _ = child.start_kill();
    match tokio::time::timeout(SHUTDOWN_GRACE, child.wait()).await {
        Ok(Ok(_status)) => {}
        Ok(Err(e)) => eprintln!("smoke: child wait error: {e}"),
        Err(_) => {
            eprintln!("smoke: kill grace expired; child may be lingering");
        }
    }

    // hdiutil detach if a .dmg was mounted earlier (macOS-only).
    if let Ok(mp) = tokio::fs::read_to_string(data_dir.join("dmg-mount.txt")).await {
        let _ = Command::new("hdiutil")
            .arg("detach")
            .arg(mp.trim())
            .status()
            .await;
    }

    // Verify ports released (best-effort; warn but don't fail cleanup).
    for port in [DEFAULT_HTTP_PORT, DEFAULT_GRPC_PORT] {
        let addr = format!("{LOOPBACK}:{port}");
        if tokio::time::timeout(Duration::from_secs(1), TcpStream::connect(&addr))
            .await
            .is_ok()
        {
            eprintln!("smoke: warning — port {port} still accepting connections post-cleanup");
        }
    }
    Ok(())
}

// ===== readiness + injection =====

async fn poll_readiness() -> Result<()> {
    let deadline = Instant::now() + READINESS_TIMEOUT;
    let mut last_err: Option<String> = None;
    while Instant::now() < deadline {
        let http_ok = tcp_handshake(LOOPBACK, DEFAULT_HTTP_PORT).await;
        let grpc_ok = tcp_handshake(LOOPBACK, DEFAULT_GRPC_PORT).await;
        if http_ok.is_ok() && grpc_ok.is_ok() {
            println!("smoke: receivers ready (HTTP :4318 + gRPC :4317)");
            return Ok(());
        }
        last_err = Some(format!(
            "http={http_ok:?} grpc={grpc_ok:?}",
            http_ok = http_ok.as_ref().err(),
            grpc_ok = grpc_ok.as_ref().err()
        ));
        sleep(POLL_INTERVAL).await;
    }
    bail!(
        "readiness probe timed out after {READINESS_TIMEOUT:?}; last_err={}",
        last_err.unwrap_or_default()
    );
}

async fn tcp_handshake(host: &str, port: u16) -> Result<()> {
    let addr = format!("{host}:{port}");
    let stream =
        tokio::time::timeout(Duration::from_millis(200), TcpStream::connect(&addr)).await??;
    drop(stream);
    Ok(())
}

async fn inject_synthetic_span() -> Result<()> {
    let request = build_export_request();
    let body = request.encode_to_vec();
    // Per security plan §Input Validation OTLP rows: body must be ≤8 MB.
    // The minimal smoke request is far under (~200 bytes); this guard is
    // belt-and-suspenders for any future request shape extension.
    if body.len() > 8 * 1024 * 1024 {
        bail!(
            "synthetic OTLP body exceeds 8 MB cap (got {} bytes)",
            body.len()
        );
    }

    let url = format!("http://{LOOPBACK}:{DEFAULT_HTTP_PORT}/v1/traces");
    let response = reqwest::Client::new()
        .post(&url)
        .header("Content-Type", "application/x-protobuf")
        .body(body)
        .send()
        .await
        .with_context(|| format!("POST {url}"))?;
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        bail!("OTLP HTTP POST failed: status={status} body={text}");
    }
    println!("smoke: injected synthetic OTLP span via :4318 (canary embedded)");
    Ok(())
}

pub(crate) fn build_export_request() -> ExportTraceServiceRequest {
    let now_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64;
    // Per security plan §Input Validation: span_id is 8 bytes, trace_id is
    // 16 bytes. Hard-coded fixture values respect the invariants.
    let span = Span {
        trace_id: vec![0x10; 16],
        span_id: vec![0x20; 8],
        name: "smoke-span".to_string(),
        kind: 0,
        start_time_unix_nano: now_ns,
        end_time_unix_nano: now_ns + 1_000_000,
        attributes: vec![KeyValue {
            key: "canary".to_string(),
            value: Some(AnyValue {
                value: Some(any_value::Value::StringValue(CANARY_VALUE.to_string())),
            }),
        }],
        ..Default::default()
    };
    let resource = Resource {
        attributes: vec![KeyValue {
            key: "service.name".to_string(),
            value: Some(AnyValue {
                value: Some(any_value::Value::StringValue(
                    "andromeda-pulse-smoke".to_string(),
                )),
            }),
        }],
        dropped_attributes_count: 0,
    };
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(resource),
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: vec![span],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

// ===== assertions =====

fn assert_log_invariants(log_file: &Path, format: BundleFormat) -> Result<()> {
    let lines = read_jsonl_lines(log_file)?;
    if lines.is_empty() {
        bail!("log file empty or missing at {}", log_file.display());
    }

    let mut webview_value: Option<String> = None;
    let mut tray_value: Option<String> = None;
    let mut gpu_value: Option<String> = None;
    let mut found_panic: Option<String> = None;
    let mut canary_leaks: Vec<String> = Vec::new();

    for raw in &lines {
        if raw.contains(CANARY_VALUE) {
            canary_leaks.push(raw.clone());
        }
        let v: Value = match serde_json::from_str(raw) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let target = v.get("target").and_then(Value::as_str).unwrap_or("");
        let level = v.get("level").and_then(Value::as_str).unwrap_or("");
        let fields = v.get("fields");
        match target {
            "app.boot.webview.init" => {
                webview_value = extract_field(fields, "webview_backend").or(webview_value);
            }
            "app.boot.gpu.check" => {
                gpu_value = extract_field(fields, "wgpu_backend").or(gpu_value);
            }
            "app.boot.tray.init" => {
                tray_value = extract_field(fields, "tray_api").or(tray_value);
            }
            "app.panic.fatal" if level == "ERROR" => {
                let preview: String = raw.chars().take(240).collect();
                if found_panic.is_none() {
                    found_panic = Some(preview);
                }
            }
            _ => {}
        }
    }

    if let Some(panic) = found_panic {
        bail!("zero-unlogged-panics SLO violated: {panic}");
    }
    if !canary_leaks.is_empty() {
        bail!(
            "PII canary leaked into log file ({} occurrences); first: {}",
            canary_leaks.len(),
            canary_leaks[0].chars().take(240).collect::<String>()
        );
    }

    expect_field(
        "app.boot.webview.init",
        "webview_backend",
        webview_value.as_deref(),
        format.expected_webview_backend(),
    )?;
    expect_field(
        "app.boot.gpu.check",
        "wgpu_backend",
        gpu_value.as_deref(),
        format.expected_wgpu_backend(),
    )?;
    expect_field(
        "app.boot.tray.init",
        "tray_api",
        tray_value.as_deref(),
        format.expected_tray_api(),
    )?;

    println!(
        "smoke: log invariants verified ({} log lines; boot trio present with expected platform values; zero panics; canary scrubbed)",
        lines.len()
    );
    Ok(())
}

fn extract_field(fields: Option<&Value>, name: &str) -> Option<String> {
    fields?
        .get(name)
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn expect_field(
    span_target: &str,
    field_name: &str,
    actual: Option<&str>,
    expected: &str,
) -> Result<()> {
    match actual {
        None => bail!("missing {span_target} span with `{field_name}` field in log file"),
        Some(got) if got == expected => Ok(()),
        Some(got) => bail!(
            "{span_target}.{field_name} mismatch: expected `{expected}` for current bundle format, got `{got}`"
        ),
    }
}

/// Reads the whole rotated log family, not one file. `tracing_appender`'s
/// daily roller date-suffixes the sink (`agent-latest.jsonl.YYYY-MM-DD`), so a
/// bare-name read silently returns nothing and the caller reports an empty log
/// on a perfectly healthy boot (obs-plan §3 Log file location).
pub(crate) fn read_jsonl_lines(path: &Path) -> Result<Vec<String>> {
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    let stem = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();

    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("read log dir at {}", dir.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().starts_with(&stem))
                .unwrap_or(false)
        })
        .collect();
    paths.sort();

    let mut lines = Vec::new();
    for p in paths {
        let Ok(content) = std::fs::read_to_string(&p) else {
            continue;
        };
        lines.extend(
            content
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(str::to_string),
        );
    }
    Ok(lines)
}

fn current_host() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn build_export_request_has_canary_in_attribute_value() {
        let req = build_export_request();
        let body = req.encode_to_vec();
        assert!(!body.is_empty(), "serialized OTLP body must be non-empty");
        assert!(
            body.len() <= 8 * 1024 * 1024,
            "serialized body must respect 8 MB cap"
        );

        let span = &req.resource_spans[0].scope_spans[0].spans[0];
        assert_eq!(span.trace_id.len(), 16, "trace_id must be 16 bytes");
        assert_eq!(span.span_id.len(), 8, "span_id must be 8 bytes");

        let canary_attr = span
            .attributes
            .iter()
            .find(|kv| kv.key == "canary")
            .expect("canary attribute present");
        let value = canary_attr
            .value
            .as_ref()
            .and_then(|v| v.value.as_ref())
            .map(|v| match v {
                any_value::Value::StringValue(s) => s.clone(),
                _ => String::new(),
            })
            .unwrap_or_default();
        assert_eq!(value, CANARY_VALUE);
    }

    #[test]
    fn parse_hdiutil_mount_extracts_last_volumes_path() {
        let stdout = b"/dev/disk5          \tApple_partition_scheme\t\n\
                       /dev/disk5s1        \tApple_partition_map   \t\n\
                       /dev/disk5s2        \tApple_HFS             \t/Volumes/andromeda-pulse\n";
        let mp = parse_hdiutil_mount(stdout);
        assert_eq!(mp, Some(PathBuf::from("/Volumes/andromeda-pulse")));
    }

    #[test]
    fn parse_hdiutil_mount_returns_none_on_no_volumes_token() {
        let stdout = b"some other text\nno mount points here\n";
        assert_eq!(parse_hdiutil_mount(stdout), None);
    }

    #[test]
    fn read_jsonl_lines_resolves_date_suffixed_family_with_no_bare_file() {
        // The family branch: ONLY rotated members exist (tracing_appender's
        // daily roller date-suffixes every file), no bare-name file at all.
        // The 6 pre-existing tests never enter this branch — a bare name
        // still matches the prefix — which is why this pin was owed
        // (test-plan §1 `harness-log-family-resolution-coverage`).
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().join("agent-latest.jsonl");
        std::fs::write(
            dir.path().join("agent-latest.jsonl.2026-08-29"),
            "{\"day\":1}\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("agent-latest.jsonl.2026-08-30"),
            "{\"day\":2}\n",
        )
        .unwrap();
        // A non-family neighbour must NOT be swept in.
        std::fs::write(dir.path().join("boot.log"), "{\"noise\":true}\n").unwrap();

        let lines = read_jsonl_lines(&base).expect("family resolves with no bare file");
        assert_eq!(lines.len(), 2, "both rotated members concatenate");
        assert!(lines[0].contains("\"day\":1") && lines[1].contains("\"day\":2"));
        assert!(
            lines.iter().all(|l| !l.contains("noise")),
            "non-family files stay out"
        );
    }

    #[test]
    fn assert_log_invariants_passes_on_complete_boot_trio_and_clean_log() {
        let dir = tempfile::tempdir().unwrap();
        let log = dir.path().join("agent-latest.jsonl");
        let mut f = std::fs::File::create(&log).unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:00Z","level":"INFO","target":"app.boot.webview.init","message":"boot","fields":{{"webview_backend":"WebView2"}}}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:01Z","level":"INFO","target":"app.boot.gpu.check","message":"boot","fields":{{"wgpu_backend":"dx12"}}}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:02Z","level":"INFO","target":"app.boot.tray.init","message":"boot","fields":{{"tray_api":"NotifyIcon"}}}}"#
        )
        .unwrap();
        drop(f);
        assert_log_invariants(&log, BundleFormat::Msi).expect("clean log + boot trio passes");
    }

    #[test]
    fn assert_log_invariants_fails_on_missing_boot_webview_span() {
        let dir = tempfile::tempdir().unwrap();
        let log = dir.path().join("agent-latest.jsonl");
        let mut f = std::fs::File::create(&log).unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:01Z","level":"INFO","target":"app.boot.gpu.check","message":"boot","fields":{{}}}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:02Z","level":"INFO","target":"app.boot.tray.init","message":"boot","fields":{{}}}}"#
        )
        .unwrap();
        drop(f);
        let err = assert_log_invariants(&log, BundleFormat::Msi).unwrap_err();
        assert!(err.to_string().contains("app.boot.webview.init"));
    }

    #[test]
    fn assert_log_invariants_fails_on_panic_record() {
        let dir = tempfile::tempdir().unwrap();
        let log = dir.path().join("agent-latest.jsonl");
        let mut f = std::fs::File::create(&log).unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:00Z","level":"INFO","target":"app.boot.webview.init","message":"boot","fields":{{}}}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:01Z","level":"INFO","target":"app.boot.gpu.check","message":"boot","fields":{{}}}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:02Z","level":"INFO","target":"app.boot.tray.init","message":"boot","fields":{{}}}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:03Z","level":"ERROR","target":"app.panic.fatal","message":"boom","fields":{{}}}}"#
        )
        .unwrap();
        drop(f);
        let err = assert_log_invariants(&log, BundleFormat::Msi).unwrap_err();
        assert!(err.to_string().contains("zero-unlogged-panics"));
    }

    #[test]
    fn assert_log_invariants_fails_when_canary_string_leaks_into_log() {
        let dir = tempfile::tempdir().unwrap();
        let log = dir.path().join("agent-latest.jsonl");
        let mut f = std::fs::File::create(&log).unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:00Z","level":"INFO","target":"app.boot.webview.init","message":"boot","fields":{{}}}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:01Z","level":"INFO","target":"app.boot.gpu.check","message":"boot","fields":{{}}}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:02Z","level":"INFO","target":"app.boot.tray.init","message":"boot","fields":{{}}}}"#
        )
        .unwrap();
        // Simulate a scrubber regression where the canary value reaches the log.
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:03Z","level":"INFO","target":"ingest.grpc.export.request","message":"leak","fields":{{"value":"secret-canary-bundle-smoke-12345"}}}}"#
        )
        .unwrap();
        drop(f);
        let err = assert_log_invariants(&log, BundleFormat::Msi).unwrap_err();
        assert!(err.to_string().contains("canary leaked"));
    }

    #[test]
    fn assert_log_invariants_fails_on_empty_log_file() {
        let dir = tempfile::tempdir().unwrap();
        let log = dir.path().join("agent-latest.jsonl");
        std::fs::File::create(&log).unwrap();
        let err = assert_log_invariants(&log, BundleFormat::Msi).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn current_host_returns_one_of_known_values() {
        assert!(matches!(
            current_host(),
            "windows" | "macos" | "linux" | "unknown"
        ));
    }

    #[test]
    fn assert_log_invariants_fails_on_field_value_mismatch_with_expected_format() {
        // Emit Windows-style boot trio values but assert against Dmg (macOS)
        // expectations — the format-bound check MUST surface the mismatch.
        let dir = tempfile::tempdir().unwrap();
        let log = dir.path().join("agent-latest.jsonl");
        let mut f = std::fs::File::create(&log).unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:00Z","level":"INFO","target":"app.boot.webview.init","message":"boot","fields":{{"webview_backend":"WebView2"}}}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:01Z","level":"INFO","target":"app.boot.gpu.check","message":"boot","fields":{{"wgpu_backend":"dx12"}}}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-01-01T00:00:02Z","level":"INFO","target":"app.boot.tray.init","message":"boot","fields":{{"tray_api":"NotifyIcon"}}}}"#
        )
        .unwrap();
        drop(f);
        let err = assert_log_invariants(&log, BundleFormat::Dmg).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("mismatch"),
            "expected mismatch message, got: {msg}"
        );
        assert!(
            msg.contains("WKWebView") || msg.contains("WebView2"),
            "expected platform-value detail in error, got: {msg}"
        );
    }
}
