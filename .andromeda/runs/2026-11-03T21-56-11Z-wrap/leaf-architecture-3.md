group: architecture-3
read: .claude/docs/stack.md · 99 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 102 · «- For Tauri webview debugging in dev: `cargo tauri dev` open»
read: .claude/rules/frontend.md · 140 · «- 2026-09-29: UNDER vitest 4, `vi.restoreAllMocks()` NO LONG»

- leaf: .claude/docs/stack.md:6   class: stale
  leaf-text: «(wasmtime 48.0.5 and its internal crates, cranelift 0.135.5, pulley 48.0.5)»
  source: fct-moirbwp3 «on wasmtime 48.0.5 / cranelift 0.135.5 / pulley 48.0.5, not re-read on the resolved wasmtime 48.0.8 / cranelift 0.135.8 / pulley 48.0.8, which the pinned 1.95.0 toolchain builds»
  replace-with: «(wasmtime and its internal crates, cranelift, pulley — as measured on wasmtime 48.0.5 / cranelift 0.135.5 / pulley 48.0.5, not re-read on the resolved wasmtime 48.0.8 / cranelift 0.135.8 / pulley 48.0.8, which the pinned 1.95.0 toolchain builds)»

- leaf: .claude/docs/stack.md:12   class: retired
  leaf-text: «Still no Cargo or npm dependency, `Cargo.lock` byte-identical (PROVISIONAL with the door).»
  source: fct-rkbubyxp «and the `lint-test` step reads both arms' sentences and fails on either skip. No Cargo or npm dependency and `Cargo.lock` byte-identical at that chunk.»
  replace-with: «Still no Cargo or npm dependency, `Cargo.lock` byte-identical.»

- leaf: .claude/docs/stack.md:13   class: stale
  leaf-text: «(GTK's main thread and `tao` 0.35.0's device-event thread)»
  source: fct-s36jrtky «No Cargo dependency, no manifest or lockfile change of its own; `tao` resolves at 0.37.2»
  replace-with: «(GTK's main thread and the device-event thread `tao` starts on X11 — `tao` resolves at 0.37.2; its one `XInitThreads` call read off the start path on 0.35.0, 0.35.3 and 0.37.1, not read on 0.37.2)»

- leaf: .claude/docs/stack.md:20   class: stale
  leaf-text: «(Cargo.toml requirement `"48.0.4"`, lockfile-resolved 48.0.5 as of 2026-10-04; 49.x out of reach while the toolchain is pinned at 1.95)»
  source: fct-wn2ucoyt «Cargo.toml requirement `"48.0.8"`, lockfile-resolved **48.0.8** (Cranelift 0.135.8)»
  replace-with: «(Cargo.toml requirement `"48.0.8"`, lockfile-resolved 48.0.8, Cranelift 0.135.8; 49.x out of reach while the toolchain is pinned at 1.95)»

- leaf: .claude/docs/stack.md:21   class: retired
  leaf-text: «(a relay to the door) — PROVISIONAL (arch §Occupied Resources → Network ports, the door).»
  source: fct-bka6rc62 «a stdio agent reaches them through the console program's word `andromeda-pulse-engine door` (a relay to the door) |»
  replace-with: «(a relay to the door).»

- leaf: .claude/docs/stack.md:41   class: stale
  leaf-text: «lockfile-resolved 1.10505.0 as of 2026-08-28»
  source: fct-6y2lp7rz «Cargo.toml requirement `1.10500` (caret, untouched), lockfile-resolved **1.10506.1**»
  replace-with: «lockfile-resolved 1.10506.1»

- leaf: .claude/docs/stack.md:50   class: retired
  leaf-text: «host no engine part and join nothing (PROVISIONAL — arch §Occupied Resources → Network ports, the door).»
  source: fct-wdpv3pq2 «the console program's `door` word also runs as short-lived relay processes that host no engine part and join nothing |»
  replace-with: «host no engine part and join nothing.»

- leaf: .claude/docs/stack.md:72   class: retired
  leaf-text: «`door`, since chunk 2026-10-12-stdio-agent-reaches-the-door and PROVISIONAL, a stdio relay»
  source: fct-r6qmu7wo «`door` (since chunk 2026-10-12-stdio-agent-reaches-the-door) a stdio relay to the running engine's door that hosts no engine part»
  replace-with: «`door`, since chunk 2026-10-12-stdio-agent-reaches-the-door, a stdio relay»
