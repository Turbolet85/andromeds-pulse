# Gotchas

Component-specific known issues. Grows over time via `/wrap-session`. Read on demand.

## Open reconciliations (carried from arch + security plan)

### `tonic 0.14` vs `opentelemetry-otlp 0.31`
`opentelemetry-otlp 0.31` still pins `tonic 0.13` in some feature combinations. The workspace targets `tonic 0.14.x` for the OTLP receiver server. **Resolution required before tagging v0.1.0** — `cargo deny check bans` (`multiple-versions = "deny"`) is the enforcement mechanism. If the duplicate cannot be resolved at lock time, the documented fallback is to downgrade Stack to `tonic 0.13.x` (matching `opentelemetry-otlp`) rather than fork or wait for upstream; the Stack/Decisions/Defaults rows are then re-pinned in the same iteration. Note: the receiver path doesn't actually use `opentelemetry-otlp` (we parse OTLP via `prost` + `opentelemetry-proto` directly), so the duplicate is purely transitive.

### `rmcp` "1.5.0" reference vs published 0.3.x line
The architecture references "rmcp 1.5.0" but the published line is `0.3.x`. **Reconcile before locking versions** — verify whether the reference is forward-looking, an internal spec name, or refers to the unrelated `4t145/rmcp` fork. Fall back to latest published `0.3.x` if 1.5.0 cannot be sourced; re-pin Stack accordingly.

### `rust-toolchain.toml` 1.84 → 1.85 bump
Architecture's Stack table specified `rustc 1.84+` initially, but Edition 2024 cannot parse below `1.85.0`. **Bump `rust-toolchain.toml` to `1.85.0` minimum** — the security-positive defaults (`unsafe_op_in_unsafe_fn`, `unsafe extern`, `static mut` reference denial, tightened `if let` temporary scopes) require this. CLAUDE.md universal warnings list the bump as a hard requirement.

## DuckDB encryption
- Do NOT enable DuckDB encryption-at-rest on the in-memory ring buffer — CVE-2025-64429 is documented against the encryption feature. The in-memory `:memory:` connection avoids the entire surface; an unnecessary feature flip would re-introduce it.

## `wasmtime` Cranelift on x86_64 (April 2026 advisory)
- Cranelift backend on x86_64 was the **unaffected configuration** for the two Critical sandbox escapes (CVE-2026-34941, CVE-2026-35195) per Bytecode Alliance advisory.
- A future "use the experimental Winch backend" change would re-introduce the exposure. NEVER add a non-Cranelift wasmtime feature flag on x86_64 builds without re-checking the April 2026 advisory cluster.
- `Config::max_wasm_http_fields_size` MUST be set per April 2026 CVE-2026-27572 (wasi-http header explosion).

## OTLP HTTP `:4318` "localhost is not a security boundary"
- Loopback-only binding does NOT defend against DNS rebinding from arbitrary websites (browser `fetch('http://localhost:4318/v1/traces', ...)`).
- Three-layer mitigation: Host-header allowlist middleware (`{127.0.0.1, localhost, [::1]}:configured-port`) + CORS deny-by-default (no `CorsLayer::permissive()`) + body-size cap (`DefaultBodyLimit::max(8 * 1024 * 1024)`).
- Anchors: Coder Agent API CVE-2025-09-19; MCP TypeScript SDK CVE-2025-66414 (Dec 2025).

## Self-observation recursion
- Product IS the local OTLP observer. Pointing the product's own OTel exporter at `:4317`/`:4318` creates an infinite loop AND bypasses the loopback authorization model.
- The chosen architecture eliminates this concern by construction: NO OTel SDK is linked into the self-observation runtime. `tracing` ecosystem only — the JSON file at `~/.andromeda-pulse/logs/agent-latest.jsonl` IS the agent surface.
- Any future `ANDROMEDA_OBSERVER_URL`-shaped variable MUST explicitly distinguish "outbound observer" (we ARE the observer — do not dial) from "inbound receivers" (the OTLP ports we listen on).

## Tauri capability silent rejections
- Adding a TauRPC procedure to a router crate without a matching entry in `pulse-app/capabilities/` JSON produces a silent runtime rejection — hard-to-diagnose UX bug.
- The `xtask capability-drift` check is the enforcement mechanism. Wire into CI on every PR.
- `pulse:default` enumerates exactly the procedures listed in arch §Occupied Resources Tauri IPC routes.

## Snapshot / clipboard / MCP tool response — OTLP attribute leakage
- Telemetry can incidentally contain secrets, IDs, URLs, error messages, SQL fragments from the host application being instrumented. OTLP attributes are user-controlled content.
- The plan does NOT mandate sanitization of OTLP attribute values themselves (data is what the user instrumented), but:
  - Snapshot files at `~/.andromeda-pulse/snapshots/` MUST surface a user-facing warning (README + first-run UI hint).
  - `snapshot.copy_to_clipboard` MUST emit a `pulse://stream/snapshot-progress` event at clipboard-write time so the UI can show a non-suppressible "X bytes copied" toast.
  - MCP `query_*` and `generate_snapshot` tool responses are the indirect-prompt-injection surface that the calling LLM client must defend against — receiver app cannot fully sanitize OTLP-derived attribute values. Documented behavior, not a defect.

## rmcp stdio reservation
- When running as the rmcp stdio sidecar, stdout is reserved for JSON-RPC 2.0 framing.
- ANY accidental `println!` / `dbg!` / library stdout write corrupts the protocol and silently disconnects the MCP client.
- ALL `tracing` output forced to stderr (JSON formatter); TTY check disabled for the sidecar.

## Tauri updater Minisign keypair rotation
- No hot key rotation path without a transitional release.
- If the private key is lost or leaked, every existing installed instance is locked out from updates.
- Runbook required before v0.1.0 ships: emergency new-key generation, transitional release with both old and new public keys (Tauri updater accepts multiple via repeated `pubkey` entries — verify against current docs at rotation time), advisory communication, final rotation release dropping the old key.

## Plugin signature verification deferred post-v1
- Third-party plugins run unverified in v1.
- Capability-scoped WIT + `wasmtime::ResourceLimiter` mitigate impact, NOT provenance.
- Document the limitation in user-facing plugin install README.

## Webview WebGPU vs native `wgpu`
- v1 uses Webview WebGPU (`<canvas>` + `navigator.gpu`, WGSL) — WGSL shaders run unchanged across WebView2 (Windows) and WKWebView (macOS/Linux); Arrow data pushed into canvas via WebGPU compute shader, side-stepping VDOM diffing.
- Native `wgpu` 25+ surface was rejected for v1 because Tauri 2 native-overlay flickering is documented (issue #9220).
- **Documented upgrade path:** native `wgpu` 25+ surface for the compact widget if browser GC jank becomes a measured problem (hybrid Option C).

## Tray icon Halo State Pulse fallback
- Tray icon Halo implementation is mandatory per Brand Identity Coherence (signature presence in 3 places: full dashboard / compact widget / tray icon).
- WebGPU canvas in a small click-through overlay window is the preferred path; SVG-filter fallback substitutes when WebGPU is unavailable in the tray context (visual equivalence ≥95% — blur radius envelope, LCH fidelity, opacity envelope).
- If Phase 8 implementation cannot materialize Halo on tray, design provides a static halo color fallback (steady glow without rhythm) to satisfy signature presence.

## CSP `'unsafe-inline'` on style-src
- The `'unsafe-inline'` on `style-src` in the Tauri 2 CSP is a conscious risk acceptance (local-first first-party UI; no third-party style sources reachable) NOT a planned nonce/hash mitigation.
- If a future scope adds dynamically generated styles from untrusted input, this MUST be replaced with `'nonce-{NONCE}'` + Tauri's CSP nonce-injection mechanism BEFORE that scope ships.
- NEVER add `'unsafe-inline'` to `script-src`.

---

_Wrap-session appends new entries below as they emerge from implementation work._
