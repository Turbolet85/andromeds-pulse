# cargo audit at P5 — the untouched tree (f2bb258f), 2026-11-03

Read by `gate.py run --entry 20` (log `$TMPDIR/andromeda-gate/2026-11-03-stdio-sidecar-retired/phase-2026-11-03T02-29-07Z/20.log`):
`error: 42 vulnerabilities found!` · `warning: 29 allowed warnings found` · 62 distinct `RUSTSEC-` ids over both.
External decay since 2026-10-12; owner «Advisories since 2026-10-12 closed», minted by the wrap (`inputs#I3`).

| crate | version | id | dated | solution |
|---|---|---|---|---|
| bincode | 1.3.3 | RUSTSEC-2025-0141 | 2025-12-16 | none |
| event-listener | 5.4.1 | RUSTSEC-2026-0221 | 2026-07-13 | none |
| glib | 0.18.5 | RUSTSEC-2024-0429 | 2024-03-30 | none |
| paste | 1.0.15 | RUSTSEC-2024-0436 | 2024-10-07 | none |
| proc-macro-error | 1.0.4 | RUSTSEC-2024-0370 | 2024-09-01 | none |
| pulldown-cmark | 0.13.3 | RUSTSEC-2026-0414 | 2026-10-18 | >=0.13.5 |
| pulldown-cmark | 0.13.3 | RUSTSEC-2026-0433 | 2026-10-28 | >=0.13.6 |
| quinn-proto | 0.11.15 | RUSTSEC-2026-0338 | 2026-09-14 | >=0.11.18 |
| quinn-proto | 0.11.15 | RUSTSEC-2026-0339 | 2026-08-17 | >=0.11.17 |
| quinn-proto | 0.11.15 | RUSTSEC-2026-0340 | 2026-09-14 | >=0.11.18 |
| quinn-proto | 0.11.15 | RUSTSEC-2026-0341 | 2026-08-17 | >=0.11.17 |
| quinn-proto | 0.11.15 | RUSTSEC-2026-0342 | 2026-08-17 | >=0.11.17 |
| rustls | 0.23.45 | RUSTSEC-2026-0346 | 2026-10-14 | >=0.23.46 |
| rustls | 0.23.45 | RUSTSEC-2026-0347 | 2026-10-14 | >=0.23.46 |
| rustls | 0.23.45 | RUSTSEC-2026-0348 | 2026-10-14 | >=0.23.46 |
| rustls | 0.23.45 | RUSTSEC-2026-0349 | 2026-10-14 | >=0.23.46 |
| rustls-webpki | 0.103.15 | RUSTSEC-2026-0382 | 2026-10-20 | >=0.103.16, <0.104.0-alpha.1 OR >=0.104.0-alpha.8 |
| tokio | 1.52.1 | RUSTSEC-2026-0386 | 2026-10-13 | none |
| unic-char-property | 0.9.0 | RUSTSEC-2025-0081 | 2025-10-18 | none |
| unic-char-range | 0.9.0 | RUSTSEC-2025-0075 | 2025-10-18 | none |
| unic-common | 0.9.0 | RUSTSEC-2025-0080 | 2025-10-18 | none |
| unic-ucd-ident | 0.9.0 | RUSTSEC-2025-0100 | 2025-10-18 | none |
| unic-ucd-version | 0.9.0 | RUSTSEC-2025-0098 | 2025-10-18 | none |
| wasmtime | 48.0.5 | RUSTSEC-2026-0353 … -0359, -0361 … -0364 | 2026-10-14 | >=48.0.6, <49.0.0 OR >=49.0.3 |
| wasmtime | 48.0.5 | RUSTSEC-2026-0378, -0379 | 2026-10-19 | >=48.0.7, <49.0.0 OR >=49.0.4 |
| wasmtime | 48.0.5 | RUSTSEC-2026-0422, -0423, -0424 | 2026-10-29 | >=48.0.8, <49.0.0 OR >=49.0.5, <50.0.0 OR >=50.0.1 |
| webpki-root-certs | 1.0.7 | RUSTSEC-2026-0383 | 2026-10-19 | >=1.0.10 |
| webpki-roots | 1.0.7 | RUSTSEC-2026-0381 | 2026-10-19 | >=1.0.10 |
| zbus_names | 3.0.0 | RUSTSEC-2026-0371 | 2026-10-16 | none |
| zip | 4.6.1 | RUSTSEC-2026-0367, -0368, -0369, -0429 | 2026-10-09 … 10-22 | >=9.0.0 |
| zstd | 0.13.3 | RUSTSEC-2026-0391 | 2026-10-16 | ^0.13.4 OR >=0.14.3 |
| zstd-safe | 7.2.4 | RUSTSEC-2026-0395 … -0406 (12) | 2026-10-16 | -0398, -0403: ^7.4.0 OR >=8.2.0; the rest none |
| zstd-sys | 2.0.16+zstd.1.5.7 | RUSTSEC-2026-0410 | 2026-10-16 | >=2.1.2 |
| zvariant | 4.2.0 | RUSTSEC-2026-0370 | 2026-10-16 | none |
| zvariant_derive | 4.2.0 | RUSTSEC-2026-0372 | 2026-10-16 | none |
