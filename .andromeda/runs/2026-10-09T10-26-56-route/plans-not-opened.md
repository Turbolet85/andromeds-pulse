# Plans — what Phase 1 opened and what it did not

_Route run `2026-10-09T10-26-56-route` (andromeda-pulse 0.4.0). Written in the second pass after the reads, before
its `1a-tree.md`; kept current in the third pass (the intent's third assembly): the masters were not re-read — HEAD
is still `7f99c38` and no master changed — and the one clipped read was completed. A section is cited in `1a-tree.md` only if it stands in the "opened" column here. Every read of a master
went through a line-ranged print of the file with no per-line cut, unless the row says "clipped"._

| Plan | Lines / sections opened | Not opened | Where a read was clipped |
|---|---|---|---|
| `architecture.md` (310 lines) | 1–310, whole: §Design Philosophy · §Stack and Technologies · §Established Decisions · §Conventions · §Standard Contracts · §Occupied Resources · §Cross-cutting Patterns · §Project Intent · §Inherited Defaults · §Existing Scopes. §Infrastructure Patterns is a stub line; its four key files opened whole: `ci-cd-approach` · `deployment-model` · `build-system` · `project-directory-structure` | nothing in the body. The sidecars `architecture-amendments.md` and its archive | none |
| `input.md` (375 lines) | 1–375, whole | nothing | none |
| `security-plan.md` (475 lines) | 1–475, whole: §Threat Model Summary · §Authentication & Authorization · §Input Validation · §Data Protection · §API Security · §Dependency Security (with §Supply chain integrity) · §Bootstrap phases · §Secret Management · §Error Handling · §Logging & Monitoring · §Compliance Controls · §Security Anti-Patterns (Authentication · Input · Data Protection · API · Secrets · Logging · Code Patterns · Universal) | nothing in the body. The sidecars `security-plan-amendments.md` and its archive | none |
| `test-plan.md` (677 lines) | 1–677, whole: §1 Test Scope Summary (with its Pending coverage triggers) · §2 · §4 · §5 · §6 · §7 · §8 · §9 · §10 (with Load profiles) · §11. §3 Test Harness Contract is a stub line; all seven of its key files opened whole: `5-command-implementation` · `bootstrap-phases` · `test-data-bootstrap` · `status-endpoint-shape` · `pid-file` · `log-format` · `per-chunk-gate-discipline` (44 lines: 1–21 in the second pass, 22–33 and 34–44 in the third, in two ranges) | nothing in the body or the keys. The sidecars `test-plan-amendments.md` and its archive | none now. In the second pass `per-chunk-gate-discipline` came back clipped at its first 2 KB; the file has 44 lines, not the 45 that pass recorded |
| `obs-plan.md` (662 lines) | 1–662, whole: §1 Obs Scope Summary · §2 · §4 · §5 · §6 · §7 · §8 PII Scrubbing & Compliance · §9 · §10 (with DuckDB Connection Isolation) · §11. §3 Observability Harness Contract preamble in the body; its eight key files opened whole: `tracing-init` · `service-identity` · `logging-stack` · `log-file-location` · `log-format-json-schema` · `heartbeat-ticks` · `trace-context-propagation` · `snapshot-paste-to-ai-integration` | nothing in the body. The sidecars `obs-plan-amendments.md` and its archive | none |
| `a11y-plan.md` (579 lines) | the heading list of the whole file; lines 1–44 (the preamble and §1 A11y Scope Summary up to the harness specification heading); the key file `bootstrap-phases-derive-for-route-setup-project`, whole | lines 45–579: §1's harness sub-sections · §2 A11y Strategy · §3 stub · §4 ARIA Patterns & Roles · §5 Keyboard Navigation · §6 Visual Design Verification · §7 Screen Reader Support · §8 Cognitive Accessibility · §9 CI Integration · §10 SLO Invariants · §11 Anti-Patterns · §12 Decisions Log; nine of the ten key files | lines 1–44 were cut at 520 characters a line (the three surface rows of the reach table lost their tails) |
| `design-system.md` (426 lines) | the heading list of the whole file; lines 1–27 (§Brand Identity) | lines 28–426: §Color Palette · §Typography · §Spacing · §Depth Strategy · §Border Radius · §Motion · §Iconography · §Surface: desktop-webview · §Surface: desktop-native · §Anti-Patterns · §Self-Validation Protocol | lines 1–27 were cut at 520 characters a line |
| `layout-templates.md` (399 lines) | the heading list of the whole file; lines 1–24 (the preamble and the head of §Surface: desktop-webview) | lines 25–399: both surfaces' wireframes, components and IA notes | lines 1–24 were cut at 520 characters a line |

**Why the last three were not read whole.** Their whole subject is the window P-083 retires (the directive for this
run: cite them only where a chunk retires or replaces what they describe). `1a-tree.md` cites them by plan and by the
opened part only — `a11y-plan §1`, `design-system §Brand Identity`, `layout-templates` preamble — never by an
unopened section.

**Read outside the plans, for the same phase:** `.andromeda/residuals.md` (whole), `andromeda-pulse-0.3.0/verification-matrix.json`
(ids, titles, methods, requirement heads), the superseded first derivation's `requirements.md`, `vision.md` and
`working-route.md` (to know what is superseded; nothing copied), and the source coordinates the intent names (a
read-only probe at HEAD `7f99c38`; its differing readings are in `requirements.md` "Notes for the reader").
