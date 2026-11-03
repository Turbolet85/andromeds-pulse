# Apply — architecture

## Verdicts
- fanout-architecture.md 1 · applied · E1
- fanout-architecture.md 2 · applied · E2
- fanout-architecture.md 3 · applied · E3
- fanout-architecture.md 4 · applied · E4
- fanout-architecture.md 5 · applied · E5
- fanout-architecture.md 6 · applied · E6
- fanout-architecture.md 7 · applied · E7
- fanout-architecture.md 8 · applied · E8
- fanout-architecture.md 9 · applied · E9
- fanout-architecture.md 10 · applied · E10
- fanout-architecture.md 11 · applied · E11
- fanout-architecture.md 12 · applied · E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28

## Edits
@fact id=fct-wn2ucoyt expect_rev=1
old: Cargo.toml requirement `"48.0.4"`, lockfile-resolved **48.0.5** as of 2026-10-04 (Cranelift 0.135.5, as measured
new: Cargo.toml requirement `"48.0.8"`, lockfile-resolved **48.0.8** as of 2026-11-03 (Cranelift 0.135.8, as measured
old: the bump from 48.0.3 closed RUSTSEC-2026-0325 / -0326 / -0327 with no advisory ignore (the earlier bump from 46.0.3 closed RUSTSEC-2026-0316)
new: the bump from 48.0.5 closed the wasmtime advisory ids open at 48.0.5 with no advisory ignore (the earlier bumps: from 48.0.3, closing RUSTSEC-2026-0325 / -0326 / -0327; from 46.0.3, closing RUSTSEC-2026-0316)

@fact id=fct-moirbwp3 expect_rev=1
old: (wasmtime 48.0.5 and its internal crates, cranelift 0.135.5, pulley 48.0.5; `cargo metadata --offline`, as measured at chunk `2026-10-04-declared-rust-floor-matches-the-code`), so no toolchain
new: (wasmtime and its internal crates, cranelift, pulley; `cargo metadata --offline`, as measured at chunk `2026-10-04-declared-rust-floor-matches-the-code` on wasmtime 48.0.5 / cranelift 0.135.5 / pulley 48.0.5, not re-read on the resolved wasmtime 48.0.8 / cranelift 0.135.8 / pulley 48.0.8, which the pinned 1.95.0 toolchain builds with `declared_floor_equals_the_pinned_channel` passing, as measured at `ci#41720832757`), so no toolchain

@fact id=fct-xsj444ab expect_rev=1
old: (requirement `"48.0.4"`, lockfile-resolved 48.0.5 as of 2026-10-04)
new: (requirement `"48.0.8"`, lockfile-resolved 48.0.8 as of 2026-11-03)

@fact id=fct-jcbi3j36 expect_rev=1
old: (requirement `"48.0.4"`, lockfile-resolved 48.0.5 as of 2026-10-04)
new: (requirement `"48.0.8"`, lockfile-resolved 48.0.8 as of 2026-11-03)

@fact id=fct-6y2lp7rz expect_rev=1
old: lockfile-resolved **1.10505.0** as of 2026-08-28
new: lockfile-resolved **1.10506.1** as of 2026-11-03

@fact id=fct-rj5pbaue expect_rev=1
old: lockfile-resolved 1.10505.0 as of 2026-08-28
new: lockfile-resolved 1.10506.1 as of 2026-11-03

@fact id=fct-a62vsmwv expect_rev=1
old: lockfile-resolved 1.10505.0 as of 2026-08-28
new: lockfile-resolved 1.10506.1 as of 2026-11-03

@fact id=fct-s36jrtky expect_rev=1
old: No Cargo dependency, no manifest or lockfile change; `tao` stays 0.35.0 |
new: No Cargo dependency, no manifest or lockfile change of its own; `tao` resolves at 0.37.2 |
old: the device-event thread `tao` 0.35.0 starts on X11, whose one `XInitThreads` call is not on the start path (0.35.3 and 0.37.1 the same)
new: the device-event thread `tao` starts on X11, whose one `XInitThreads` call is not on the start path (as read on `tao` 0.35.0, 0.35.3 and 0.37.1; not read on the resolved 0.37.2, on which the boot series of 2 boots reads `all-settled` and CI's boot smoke on `ubuntu-22.04` reads green, as measured at `ci#41720832757`)

@fact id=fct-s765xh2i expect_rev=1
old: which defaults to the Cargo value (tauri-utils 2.9.0).
new: which defaults to the Cargo value (as read at tauri-utils 2.9.0; not re-read at the resolved tauri-utils 2.10.2, no bundle built on it).

@fact id=fct-qiuibtut expect_rev=2
old: Tauri 2.11 rejects a denied command back to the webview with a message containing "not allowed", and NO runtime-side hook exists at that version)
new: as read at Tauri 2.11, a denied command is rejected back to the webview with a message containing "not allowed", and NO runtime-side hook exists at that version; not re-probed at the resolved Tauri 2.12.5)

@fact id=fct-twrv2qiu expect_rev=1
old: **PROVISIONAL — a boundary widening, one item:** the endpoint, the owner check, the reach of the nine tools and of the one write, the write-switch variable and the workflow step that runs a client as a second account are the operator's answers (the pc overseer: at that chunk's P4 dialog, 2026-10-10, and the operator's reading of 2026-10-11 at its wrap); the founder's own word is owed on the surface itself, and nothing of it is ratified. Since chunk 2026-10-12-stdio-agent-reaches-the-door the same item also covers the console program's third word `door` — the stdio relay to this socket (one connection per request; no file, listener, pid file, variable or binary of its own; an unreachable door is exit 1 with one fixed standard-error line) — and the second arm of the workflow step that runs a client as a second account: the operator's answers at that chunk's P4 (the pc overseer, 2026-10-12) and at its operator pass, and the operator's answer as the founder's delegate at its wrap; the founder's own word is still owed and nothing of it is ratified. Every other statement
new: **A boundary widening, one item, ratified:** the endpoint, the owner check, the reach of the nine tools and of the one write, the write-switch variable and the workflow step that runs a client as a second account, and (since chunk 2026-10-12-stdio-agent-reaches-the-door) the console program's third word `door` — the stdio relay to this socket (one connection per request; no file, listener, pid file, variable or binary of its own; an unreachable door is exit 1 with one fixed standard-error line) — and the second arm of the workflow step that runs a client as a second account. Every other statement

@fact id=fct-rjyoe4ax expect_rev=1
old: (§Infrastructure Patterns → Deployment model; PROVISIONAL — §Occupied Resources → Network ports, the door)
new: (§Infrastructure Patterns → Deployment model)

@fact id=fct-bka6rc62 expect_rev=2
old: `pulse-app/src/door.rs`, Unix only; PROVISIONAL — §Occupied Resources → Network ports)
new: `pulse-app/src/door.rs`, Unix only)
old: (a relay to the door) (PROVISIONAL — §Occupied Resources → Network ports, the door) |
new: (a relay to the door) |

@fact id=fct-wdpv3pq2 expect_rev=1
old: host no engine part and join nothing (PROVISIONAL — §Occupied Resources → Network ports, the door) |
new: host no engine part and join nothing |

@fact id=fct-rkbubyxp expect_rev=2
old: at that chunk. PROVISIONAL — §Occupied Resources → Network ports, the door |
new: at that chunk. |

@fact id=fct-rsvfsams expect_rev=2
old: with no feature. PROVISIONAL — §Occupied Resources → Network ports, the door. The old
new: with no feature. The old

@fact id=fct-syp7tq5s expect_rev=2
old: (§Standard Contracts → Engine door; PROVISIONAL — §Occupied Resources → Network ports)
new: (§Standard Contracts → Engine door)

@fact id=fct-wzk2x6ka expect_rev=1
old: host no engine part; PROVISIONAL — §Occupied Resources → Network ports, the door)
new: host no engine part)

@fact id=fct-3k7t2g4l expect_rev=2
old: through the engine's own `CorpusWriter`; PROVISIONAL — §Occupied Resources → Network ports, the door)
new: through the engine's own `CorpusWriter`)

@fact id=fct-ob4ts4xm expect_rev=2
old: (§Standard Contracts → Engine door; PROVISIONAL — §Occupied Resources → Network ports)
new: (§Standard Contracts → Engine door)

@fact id=fct-qsvopkhj expect_rev=2
old: Unix only, in every build. PROVISIONAL — §Occupied Resources → Network ports, the door.
new: Unix only, in every build.

@fact id=fct-4nu3wh7b expect_rev=2
old: the one stdio surface. PROVISIONAL — §Network ports, the door.
new: the one stdio surface.

@fact id=fct-hgj34est expect_rev=2
old: the relay is the one stdio carrier. PROVISIONAL — §Network ports, the door.
new: the relay is the one stdio carrier.

@fact id=fct-wuyupllp expect_rev=2
old: neither entry point changed; PROVISIONAL — §Network ports, the door)
new: neither entry point changed)
old: §Standard Contracts → Engine door, Stdio; PROVISIONAL — §Network ports, the door)
new: §Standard Contracts → Engine door, Stdio)

@fact id=fct-ulzkjofc expect_rev=2
old: chunk 2026-10-10-door-inside-the-engine-s-process; PROVISIONAL — §Network ports, the door)
new: chunk 2026-10-10-door-inside-the-engine-s-process)

@fact id=fct-g7bqq44n expect_rev=2
old: no TauRPC contract. PROVISIONAL — §Network ports, the door.
new: no TauRPC contract.

@fact id=fct-r6qmu7wo expect_rev=1
old: 2026-10-12-stdio-agent-reaches-the-door; PROVISIONAL — §Occupied Resources → Network ports, the door)
new: 2026-10-12-stdio-agent-reaches-the-door)

@fact id=fct-x56az6wm expect_rev=2
old: `andromeda-pulse-engine door` (PROVISIONAL — §Occupied Resources → Network ports, the door); no feature gate
new: `andromeda-pulse-engine door`; no feature gate

## Patterns
[[pattern]]
id = "arch-01"
regex = 'requirement `"48\.0\.4"`|lockfile-resolved \*{0,2}48\.0\.5'

[[pattern]]
id = "arch-02"
fixed = 'Cranelift 0.135.5'
ignore_case = true

[[pattern]]
id = "arch-03"
fixed = 'wasmtime 48.0.5 and its internal crates'

[[pattern]]
id = "arch-04"
regex = 'lockfile-resolved \*{0,2}1\.10505\.0'

[[pattern]]
id = "arch-05"
fixed = '`tao` stays 0.35.0'

[[pattern]]
id = "arch-06"
fixed = 'defaults to the Cargo value (tauri-utils 2.9.0)'

[[pattern]]
id = "arch-07"
fixed = 'Tauri 2.11 rejects a denied command'

[[pattern]]
id = "arch-08"
fixed = 'nothing of it is ratified'

[[pattern]]
id = "arch-09"
regex = 'PROVISIONAL — §(Occupied Resources → )?Network ports'
