// Vendored OTLP .proto compilation per obs-plan §12 user-pivot ("NO OTel SDK in
// self-observation runtime"). See proto/opentelemetry/proto/common/v1/common.proto
// header for full rationale. Generated stubs land in OUT_DIR and are included via
// `tonic::include_proto!` from src/grpc.rs.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Vendored protoc removes the system-dep on a `protoc` binary in PATH —
    // matches arch §Cross-cutting Patterns "deterministic harness" + agent-driven
    // CI hygiene (Linux/macOS/Windows runners get the same protoc bytes).
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    // SAFETY: build scripts run single-threaded before any other crate code; env
    // mutation here is safe per Rust 2024 unsafe set_var contract.
    unsafe {
        std::env::set_var("PROTOC", protoc);
    }

    let proto_root = "proto";
    let services = [
        "proto/opentelemetry/proto/collector/trace/v1/trace_service.proto",
        "proto/opentelemetry/proto/collector/metrics/v1/metrics_service.proto",
        "proto/opentelemetry/proto/collector/logs/v1/logs_service.proto",
    ];

    // Client stubs are needed by integration tests in crates/ingest/tests/ to
    // drive the server through real HTTP/2 + gRPC framing (size-cap enforcement,
    // bind behavior). Production binaries link only the server path; client
    // types are dead code in release builds.
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&services, &[proto_root])?;

    for proto in services.iter() {
        println!("cargo:rerun-if-changed={}", proto);
    }
    println!("cargo:rerun-if-changed=proto/opentelemetry/proto/common/v1/common.proto");
    println!("cargo:rerun-if-changed=proto/opentelemetry/proto/resource/v1/resource.proto");
    println!("cargo:rerun-if-changed=proto/opentelemetry/proto/trace/v1/trace.proto");
    println!("cargo:rerun-if-changed=proto/opentelemetry/proto/metrics/v1/metrics.proto");
    println!("cargo:rerun-if-changed=proto/opentelemetry/proto/logs/v1/logs.proto");

    Ok(())
}
