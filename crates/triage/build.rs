//! Windows linker workaround — libduckdb-sys 1.10502.x relies on Windows
//! Restart Manager API (`RmStartSession` / `RmEndSession` /
//! `RmRegisterResources` / `RmGetList`) but does not emit
//! `cargo:rustc-link-lib=rstrtmgr` directive in its own build.rs. Existing
//! crates (`buffer`, `viz`, `corpus`) link successfully because their test
//! binaries were cached from а prior libduckdb-sys version that didn't
//! need rstrtmgr; new triage test binary (chunk #79) builds against
//! 1.10502.x cleanly and surfaces the missing-symbol link error.
//!
//! Mitigation: emit the link directive at triage crate level.
//! Cross-crate fix: workspace Cargo.toml could pin duckdb-sys < 1.10502 OR
//! add same directive к other DuckDB-consuming crates. Pinning chosen here
//! because workspace dep pin requires lockfile churn + downstream impact;
//! local triage workaround is minimal-blast-radius.

fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=rstrtmgr");
    }
}
