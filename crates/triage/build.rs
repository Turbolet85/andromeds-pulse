//! Triage crate build script.
//!
//! ## Phase 1 — Windows linker workaround (chunk #79)
//!
//! libduckdb-sys 1.10502.x relies on Windows Restart Manager API
//! (`RmStartSession` / `RmEndSession` / `RmRegisterResources` /
//! `RmGetList`) but does not emit `cargo:rustc-link-lib=rstrtmgr`
//! directive in its own build.rs. Mitigation: emit the link directive at
//! triage crate level (local workaround; minimal blast radius).
//!
//! ## Phase 2 — Llama-3 tokenizer fixture download (chunk #81)
//!
//! L3 digest assembler requires a deterministic tokenizer for token-count
//! enforcement against the 500-2000 soft / 3000 hard cap budget per
//! dist-arch v3 §L3. Forward-binds to chunk #82+ LLM runtime choice
//! (Llama-3 family per dist-arch v3 §L4 Hardware Profile Matrix).
//!
//! Download source: `Xenova/llama-3-tokenizer` public HuggingFace mirror
//! (no auth required; mirrors Meta's Llama-3-8B-Instruct tokenizer.json).
//! Downloaded once per build to `$OUT_DIR/tokenizer.json`; subsequent
//! invocations reuse the cached file.
//!
//! Optional SHA-256 pin via `ANDROMEDA_LLAMA3_TOKENIZER_SHA256` env var
//! (CI sets pinned value; local dev gets warning if env var unset and
//! verifies the downloaded SHA-256 matches expected).
//!
//! Network is required at first build only. CI environments without
//! outbound HTTPS must pre-seed `$OUT_DIR/tokenizer.json` or set
//! `ANDROMEDA_LLAMA3_TOKENIZER_PATH` to point at a local pre-downloaded
//! copy.

use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

const TOKENIZER_URL: &str =
    "https://huggingface.co/Xenova/llama-3-tokenizer/resolve/main/tokenizer.json";
const SHA_ENV_VAR: &str = "ANDROMEDA_LLAMA3_TOKENIZER_SHA256";
const LOCAL_PATH_ENV_VAR: &str = "ANDROMEDA_LLAMA3_TOKENIZER_PATH";

fn main() {
    // Phase 1: Windows linker workaround.
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=rstrtmgr");
    }

    // Phase 2: tokenizer fixture acquisition.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={SHA_ENV_VAR}");
    println!("cargo:rerun-if-env-changed={LOCAL_PATH_ENV_VAR}");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR set by cargo"));
    let target = out_dir.join("tokenizer.json");

    if let Ok(local_path) = env::var(LOCAL_PATH_ENV_VAR) {
        // Operator override: skip download; copy local file into OUT_DIR.
        let src = PathBuf::from(&local_path);
        if !src.exists() {
            panic!("{LOCAL_PATH_ENV_VAR}={local_path} does not exist");
        }
        fs::copy(&src, &target)
            .unwrap_or_else(|e| panic!("copy {local_path} → tokenizer.json: {e}"));
        println!("cargo:warning=triage: using local tokenizer override from {local_path}");
        return;
    }

    if target.exists() {
        // Cached: reuse.
        return;
    }

    let bytes = match download(TOKENIZER_URL) {
        Ok(b) => b,
        Err(e) => {
            panic!(
                "tokenizer download failed: {e}\n\
                 To use a pre-downloaded tokenizer in offline / air-gapped builds, \
                 set {LOCAL_PATH_ENV_VAR} to a local tokenizer.json path."
            );
        }
    };

    let actual_sha = sha256_hex(&bytes);
    if let Ok(expected_sha) = env::var(SHA_ENV_VAR) {
        if !expected_sha.eq_ignore_ascii_case(&actual_sha) {
            panic!(
                "tokenizer SHA-256 mismatch:\n  expected ({SHA_ENV_VAR}): {expected_sha}\n  actual:   {actual_sha}\n"
            );
        }
    } else {
        println!(
            "cargo:warning=triage: tokenizer downloaded (sha256={actual_sha}); pin via {SHA_ENV_VAR} env var for reproducible builds"
        );
    }

    fs::write(&target, &bytes).unwrap_or_else(|e| panic!("write tokenizer.json: {e}"));
}

fn download(url: &str) -> Result<Vec<u8>, String> {
    let resp = ureq::get(url)
        .timeout(std::time::Duration::from_secs(60))
        .call()
        .map_err(|e| format!("ureq.call({url}): {e}"))?;
    let mut bytes = Vec::with_capacity(2 * 1024 * 1024);
    resp.into_reader()
        .take(8 * 1024 * 1024)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("read body: {e}"))?;
    if bytes.len() < 1024 {
        return Err(format!("response body too small ({} bytes)", bytes.len()));
    }
    Ok(bytes)
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    digest.iter().map(|b| format!("{b:02x}")).collect()
}
