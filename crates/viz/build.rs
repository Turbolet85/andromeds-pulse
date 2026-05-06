fn main() {
    // libduckdb-sys 1.10502 (DuckDB 1.5 bundled C++) references Windows
    // Restart Manager APIs (RmStartSession / RmEndSession / RmRegisterResources
    // / RmGetList) but does not emit a link directive for `rstrtmgr.lib` from
    // its own build.rs in test binary contexts. Without this hint the linker
    // fails with LNK2019 unresolved-external errors when building the viz
    // crate's test binaries on the x86_64-pc-windows-msvc target. Mirrors
    // the chunk #20 buffer crate workaround.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        println!("cargo:rustc-link-lib=dylib=rstrtmgr");
    }
}
