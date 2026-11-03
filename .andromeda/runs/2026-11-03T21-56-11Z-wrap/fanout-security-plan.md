proposals:
  - detector: D-security-deps
    severity: escalate
    section: "§Security Anti-Patterns → Input"
    change: "In the NARROWED EXCEPTION paragraph's CVE-anchor sentence, retire «involve crates not in this project's dependency graph» for zip: RustFS CVE-2025-68705 involves a crate not in the graph, while the zip crate IS in it — zip 9.3.0 in Cargo.lock (no zip below 9) since chunk 2026-11-03-advisories-since-2026-10-12-closed, with zip 4.6.1 (tauri-plugin-updater 2.10.1) and 6.0.0 (libduckdb-sys build dependency) standing before it; both anchors still document the same symlink-chain / TOCTOU traversal class."
    sidecar: "2026-11-03 (chunk 2026-11-03-advisories-since-2026-10-12-closed): the claim that the zip crate is not in the dependency graph was false by measurement (zip 4.6.1 and 6.0.0 stood at the base; zip 9.3.0 stands now, no zip below 9); the CVE-anchor sentence corrected."
    rationale: "The report's Spec claims disproved bullet names security-plan fct-tg5z4vbe's zip sentence FALSE (grep of Cargo.lock; gate reads no zip below 9); Dependencies lists zip 6.0.0 → 9.3.0 and zip 4.6.1 removed; the plan's expected amendment."
    basis: "security-plan fct-tg5z4vbe; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-owqcejo2; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-w7imi2kp; Cargo.lock"
  - detector: D-security-deps
    severity: escalate
    section: "§Dependency Security"
    change: "In the npm channel bullet, replace the «Current state (chunk 2026-09-29-…): 2 advisory + 2 license exceptions; both advisory exceptions are extract-zip» sentence with the current state at chunk 2026-11-03-advisories-since-2026-10-12-closed: exit 0 green-with-dispositions, unexcepted [], 5 advisory + 2 license exceptions — the two extract-zip exceptions (GHSA-jmr9-qjv8-65gv, GHSA-7pqw-9j4j-h8q3) plus GHSA-g3hv-75wg-m249 (minimatch), GHSA-3g3m-2fjr-6pw4 (glob) and GHSA-mrhx-c7xc-m3w5 (zod), each with reason, owner that chunk, and closing condition ending «prune when the gate runs green without this entry», added 2026-11-03 on the founder's word; the other advisories closed by in-range lockfile updates (package.json unchanged); the earlier readings stay as history."
    sidecar: "2026-11-03 (chunk 2026-11-03-advisories-since-2026-10-12-closed): npm-policy.json advisory_exceptions 2 → 5 (minimatch, glob, zod GHSA-scoped, founder's word); the npm channel's current-state sentence updated; license exceptions unchanged at 2."
    rationale: "The report's Schema / config bullet records pulse-app/ui/npm-policy.json:19-42 advisory_exceptions 2 → 5 with provenance; Counts names security-plan fct-3ey2oony's current-state sentence stale (pattern npm-exc); Outcome reads the npm gate exit 0, green-with-dispositions, unexcepted []."
    basis: "security-plan fct-3ey2oony; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-vk5yisc4; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-szuihdoq; pulse-app/ui/npm-policy.json:19-42"
  - detector: D-security-auth
    severity: warning
    section: "§Threat Model Summary → Attack surface"
    change: "In the console engine's door vector, replace the «PROVISIONAL — a boundary widening, one item … the founder's own word is owed on the surface itself, and nothing of it is ratified … the founder's own word is still owed» status with: the door's one item (the endpoint, the owner check, the reach of the nine tools and of the one write, the write-switch variable, the second-account workflow step, and since chunk 2026-10-12-stdio-agent-reaches-the-door the console program's word door and the step's second arm) is founder-confirmed — the founder's word of 2026-11-02, «Подтвердить как есть», relayed by overseer1; the operator's earlier answers kept as provenance; no longer PROVISIONAL."
    sidecar: "2026-11-03 (chunk 2026-11-03-advisories-since-2026-10-12-closed, wrap addendum inputs#I5): the founder confirmed the door's own surface (item 27) on 2026-11-02; the door's one PROVISIONAL item is ratified and its marks across the plan drop."
    rationale: "The report's Addendum 1 (inputs#I5) supersedes the Counts line: the founder confirmed the door's own surface (item 27) 2026-11-02, so the door's PROVISIONAL mark drops; security-plan fct-kmsyqgta is the item every other door statement rests on."
    basis: "security-plan fct-kmsyqgta; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-zevs52kv; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-szuihdoq"
  - detector: D-security-auth
    severity: warning
    section: "§Input Validation"
    change: "In the MCP tool inputs row's undeclared-argument clause, drop «PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector» from the parenthesis, leaving «(the next row)»."
    sidecar: "2026-11-03: MCP tool inputs row — the door's PROVISIONAL mark dropped on the founder's confirmation of 2026-11-02."
    rationale: "Addendum 1 names this mark explicitly: the door's own item is confirmed, so the mark at security-plan fct-6geshnvr drops."
    basis: "security-plan fct-6geshnvr; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-zevs52kv"
    dependent-of: D-security-auth
  - detector: D-security-auth
    severity: warning
    section: "§Input Validation"
    change: "In the console engine's door row, drop the «PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector» mark after the boundary's description."
    sidecar: "2026-11-03: the door row of §Input Validation — PROVISIONAL mark dropped (founder confirmation 2026-11-02)."
    rationale: "The mark points to the door's one item, confirmed per the report's Addendum 1."
    basis: "security-plan fct-gn744k3z; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-zevs52kv"
    dependent-of: D-security-auth
  - detector: D-security-auth
    severity: warning
    section: "§Input Validation"
    change: "In the console program command-line row, replace «`door` is not covered by that reading: it is a boundary widening, PROVISIONAL with the door's one item (the operator's answers …)» with: `door` is not covered by that reading: it is a boundary widening, part of the door's one item, founder-confirmed 2026-11-02."
    sidecar: "2026-11-03: console command-line row — the door word's PROVISIONAL status replaced by the founder's confirmation of 2026-11-02."
    rationale: "The door word is part of the door's one item, confirmed per the report's Addendum 1."
    basis: "security-plan fct-lwwyrpa6; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-zevs52kv"
    dependent-of: D-security-auth
  - detector: D-security-auth
    severity: warning
    section: "§Input Validation"
    change: "In the CLI / env var inputs row's ANDROMEDA_PULSE_DOOR_RESOLVE clause, drop the closing «PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector»; the row's pre-push sixth-stage and exit-witness PROVISIONAL marks stay."
    sidecar: "2026-11-03: CLI / env var row — the door switch's PROVISIONAL mark dropped; the harness items' marks unchanged."
    rationale: "The write switch is part of the door's one item, confirmed per the report's Addendum 1."
    basis: "security-plan fct-ty7icrgc; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-zevs52kv"
    dependent-of: D-security-auth
  - detector: D-security-auth
    severity: warning
    section: "§Threat Model Summary → Attack surface"
    change: "In the CLI input vector, drop the door's PROVISIONAL marks — «PROVISIONAL — the door's vector» on the `door` word, «PROVISIONAL with the door's item» in the trust boundary, and «PROVISIONAL — the door's vector above» on ANDROMEDA_PULSE_DOOR_RESOLVE; the sixth-stage and exit-witness PROVISIONAL classifications stay."
    sidecar: "2026-11-03: CLI input vector — the door's three PROVISIONAL marks dropped; the harness items' marks unchanged."
    rationale: "These marks point to the door's one item, confirmed per the report's Addendum 1."
    basis: "security-plan fct-j4oj6ftt; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-zevs52kv"
    dependent-of: D-security-auth
  - detector: D-security-auth
    severity: warning
    section: "§Threat Model Summary → Auth model"
    change: "In the Scope line, drop «PROVISIONAL — Attack surface, the door's vector» from the door's owner-check parenthesis, leaving the §Authentication & Authorization pointer."
    sidecar: "2026-11-03: Auth model scope — the door's PROVISIONAL mark dropped."
    rationale: "The owner check is part of the door's one item, confirmed per the report's Addendum 1."
    basis: "security-plan fct-inw2zo7r; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-zevs52kv"
    dependent-of: D-security-auth
  - detector: D-security-auth
    severity: warning
    section: "§Authentication & Authorization"
    change: "Drop the closing «PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector» from the owner-check paragraph."
    sidecar: "2026-11-03: §Authentication & Authorization — the owner check's PROVISIONAL mark dropped."
    rationale: "The owner check is part of the door's one item, confirmed per the report's Addendum 1."
    basis: "security-plan fct-ashdoovw; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-zevs52kv"
    dependent-of: D-security-auth
  - detector: D-security-auth
    severity: warning
    section: "§Security Anti-Patterns → Authentication"
    change: "In the lead-in of the door's four bans, drop «PROVISIONAL with the door — §Threat Model Summary → Attack surface, the door's vector», keeping the chunk reference."
    sidecar: "2026-11-03: the door's four bans — PROVISIONAL mark dropped."
    rationale: "The bans belong to the door's one item, confirmed per the report's Addendum 1."
    basis: "security-plan fct-xsadt7mu; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-zevs52kv"
    dependent-of: D-security-auth
  - detector: D-security-auth
    severity: warning
    section: "§Security Anti-Patterns → Input"
    change: "In the second-account step clause, replace «classification PROVISIONAL, part of the door's one item» with «part of the door's one item, founder-confirmed 2026-11-02»; the sixth-stage and exit-witness PROVISIONAL classifications in the same bullet stay."
    sidecar: "2026-11-03: the second-account step — PROVISIONAL classification replaced by the founder's confirmation of the door's item."
    rationale: "The step is named part of the door's one item, confirmed per the report's Addendum 1."
    basis: "security-plan fct-ixrmtmoe; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-zevs52kv"
    dependent-of: D-security-auth
  - detector: D-security-auth
    severity: warning
    section: "§Logging & Monitoring"
    change: "In the MCP tool-response hygiene bullet, drop the closing «(PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector)»."
    sidecar: "2026-11-03: MCP tool-response hygiene — the door's PROVISIONAL mark dropped."
    rationale: "The mark points to the door's one item, confirmed per the report's Addendum 1."
    basis: "security-plan fct-cvhwrox6; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-zevs52kv"
    dependent-of: D-security-auth
  - detector: D-security-auth
    severity: warning
    section: "§Security Anti-Patterns → Logging"
    change: "In the incident_events NO-SCRUB paragraph, drop «PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector» from the mark_incident_resolved parenthesis."
    sidecar: "2026-11-03: incident_events no-scrub boundary — the door's PROVISIONAL mark dropped."
    rationale: "The door's write path is part of the door's one item, confirmed per the report's Addendum 1."
    basis: "security-plan fct-y6svlsyp; doc:2026-11-03-advisories-since-2026-10-12-closed/report fct-zevs52kv"
    dependent-of: D-security-auth
note: "Two readings to check. (1) The zip and npm proposals keep D-security-deps' escalate severity, but no banned dependency was added: cargo deny bans/licenses/sources is green and the founder ruled in inputs#I2. (2) The door proposals read the founder's item-27 confirmation in inputs#I5 as covering the door's whole one item, so they leave the sixth-stage (pre-push), exit-witness and test-port PROVISIONAL items alone. fct-hhbgn5ow (the skip carve-outs) and fct-wnflmema (the cargo audit history) were left as the report reads them, true or history. D-security-input, D-security-logging and D-retired found no drift."
