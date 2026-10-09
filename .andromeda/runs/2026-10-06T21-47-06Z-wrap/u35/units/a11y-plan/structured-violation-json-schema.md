### Structured violation JSON schema

Binding contract from upstream-context Section 6 Obs Plan Excerpt. Required fields per a11y-scope Section 3:
- `timestamp` (RFC 3339)
- `level` ("ERROR" for WCAG violations)
- `target` ("a11y::assertion")
- `message` (human-readable violation summary)
- `fields`: { `wcag_criterion`, `violation_type`, `severity`, `surface`, `selector`, `remediation`, `tool`, `tool_result_id` }

**Extensions (optional):**
- `token_name` — design token binding (e.g., "--color-text-primary")
- `measured_value` / `required_value` — numeric comparison (contrast ratio)
- `affected_component` — React component or layout type
