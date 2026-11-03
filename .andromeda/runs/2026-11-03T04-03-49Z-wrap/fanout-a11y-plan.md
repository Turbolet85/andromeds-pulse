proposals:
  - detector: D-retired
    severity: warning
    section: '§1 A11y Scope Summary (critical paths table, P3 row)'
    change: 'P3 row: the Settings MCP switch and its state-change notification are retired (chunk 2026-11-03-stdio-sidecar-retired); no settings-modal switch, no switch[aria-checked] MCP toggle and no OS notification on an MCP state change exist, and the mcp-server sidecar service identity is gone. P3 now reaches the engine only through the door (socket and stdio relay), which has no UI surface and is not-assertable; the settings-modal focus order stays covered by P7.'
    sidecar: '2026-11-03-stdio-sidecar-retired: §1 P3 row corrected — the Settings MCP switch, its state-change notification and the sidecar identity retired; P3 carries no a11y surface.'
    rationale: 'The report''s Retired bullet names the stdio sidecar, the Settings MCP switch and the tray MCP toggle, and states the MCP server status change notification never had an emitter; Schema / config removes Settings.mcp_server_enabled; Coverage says the Settings modal loses the switch; Expected amendments name a11y-plan §1 (P3 row).'
    basis: 'a11y-plan fct-ggne23uy; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx; doc:2026-11-03-stdio-sidecar-retired/report fct-vdoco5gy; doc:2026-11-03-stdio-sidecar-retired/report fct-vikzxbdd'
  - detector: D-retired
    severity: warning
    section: '§1 A11y Scope Summary (critical paths table, P7 row)'
    change: 'P7 row: drop the MCP toggle from the title (theme, widget position, retention, snapshot preset), drop `switch` (MCP toggle) from the required roles, and read the Tab order as theme radio → widget position radio → retention input → snapshot preset → Save button → Cancel button.'
    sidecar: '2026-11-03-stdio-sidecar-retired: §1 P7 row — the retired MCP toggle switch removed from the title, the roles and the settings Tab order.'
    rationale: 'The report''s Retired bullet names the Settings MCP switch; Schema / config removes Settings.mcp_server_enabled; Coverage records the a11y focus order without the switch, a11y list and CI a11y green.'
    basis: 'a11y-plan fct-v344ogev; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-vdoco5gy; pulse-app/ui/src/dashboard/routes/SettingsModalForm.tsx'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§5 Keyboard Navigation (settings-modal focus order)'
    change: 'settings-modal focus order reads modal title → theme radio group → widget position radio group → retention input → snapshot preset combobox → Save button → Cancel button; no MCP toggle switch.'
    sidecar: '2026-11-03-stdio-sidecar-retired: §5 settings-modal focus order — the retired MCP toggle switch removed.'
    rationale: 'The report''s Retired bullet names the Settings MCP switch; Coverage says the Settings modal loses a control, the a11y focus order now without the switch; Expected amendments name a11y-plan §5 (settings focus order).'
    basis: 'a11y-plan fct-btetkki6; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-vdoco5gy; pulse-app/ui/tests-a11y/axe/p3-settings-modal.spec.ts:5'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§1 A11y Scope Summary (scope entities table, Settings modal form row)'
    change: 'Settings modal form row lists theme selector, widget snap position, retention input, snapshot preset, plugin manager; no MCP toggle.'
    sidecar: '2026-11-03-stdio-sidecar-retired: §1 Settings modal form entity — the retired MCP toggle removed from its control list.'
    rationale: 'The report''s Retired bullet names the Settings MCP switch; Schema / config removes Settings.mcp_server_enabled; Coverage says the Settings modal loses the switch.'
    basis: 'a11y-plan fct-mwm27cak; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx'
    dependent-of: D-retired
note: 'D-a11y-surface and D-a11y-obs-schema hold: the report adds no interactive UI element (Coverage, fct-vdoco5gy) and changes neither the a11y violation schema nor the obs log schema. The report names four a11y-plan sites (mcp-setting, a11y-plan:31, :227, :231, :319); this sweep matched four facts, one proposal each. No a11y-plan passage names the tray MCP toggle, Send to agent or the status-change notification, and no keyed contract names MCP.'
