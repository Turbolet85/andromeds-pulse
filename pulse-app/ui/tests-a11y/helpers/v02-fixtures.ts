// Chunk #99 — shared v0.2.0 mock payloads for the extended-surface axe
// specs (p8-p12) + keyboard-focus specs. Field shapes mirror the TauRPC
// bindings types verbatim (IncidentRecord / IncidentsListPayload /
// ConnectionStatePayload / ServiceListPayload / ReportPayload /
// ExportPreviewPayload). Timestamps are computed (not literal) so relative
// rendering ("3m ago") stays meaningful and the no-loss-of-precision lint
// never trips on ns-scale literals.

const NOW_NS = Date.now() * 1_000_000;
const MINUTE_NS = 60_000_000_000;

export const incidentRecords = [
  {
    id: 1,
    workspace: "demo-workspace",
    kind: "retry_storm",
    scope: "service",
    status: "active",
    severity: "critical",
    priority_tier: "autonomous",
    title: "payment-service retry storm (fingerprint 9f3a)",
    detail: "Same exception fingerprint observed 14 times in 30s window.",
    opened_at_unix_nano: NOW_NS - 3 * MINUTE_NS,
    updated_at_unix_nano: NOW_NS - MINUTE_NS,
    acknowledged_at_unix_nano: null,
    resolved_at_unix_nano: null,
    read_at_unix_nano: null,
    evidence_count: 4,
  },
  {
    id: 2,
    workspace: "demo-workspace",
    kind: "error_rate_spike",
    scope: "service",
    status: "active",
    severity: "warn",
    priority_tier: "suggested",
    title: "checkout-api error rate 3.4x baseline",
    detail: "Short-window error rate exceeded baseline multiplier.",
    opened_at_unix_nano: NOW_NS - 8 * MINUTE_NS,
    updated_at_unix_nano: NOW_NS - 2 * MINUTE_NS,
    acknowledged_at_unix_nano: null,
    resolved_at_unix_nano: null,
    read_at_unix_nano: null,
    evidence_count: 2,
  },
  {
    id: 3,
    workspace: "demo-workspace",
    kind: "service_went_silent",
    scope: "service",
    status: "active",
    severity: "info",
    priority_tier: "curious",
    title: "inventory-worker quiet beyond learned floor",
    detail: "Quiet duration exceeded p95 historical quiet window.",
    opened_at_unix_nano: NOW_NS - 15 * MINUTE_NS,
    updated_at_unix_nano: NOW_NS - 12 * MINUTE_NS,
    acknowledged_at_unix_nano: null,
    resolved_at_unix_nano: null,
    read_at_unix_nano: null,
    evidence_count: 1,
  },
];

export const incidentsListPayload = {
  items: incidentRecords,
  total: incidentRecords.length,
  next_cursor: null,
};

export const connectionReceivingPayload = {
  state: { state: "Receiving" },
  last_span_ago_ms: 1_200,
  severity: "info",
  message: null,
  reason: null,
};

export const serviceListPayload = {
  items: [
    {
      service: "payment-service",
      state: "active",
      last_seen_unix_nano: NOW_NS - MINUTE_NS,
      manual_override: null,
      priority_tier: "autonomous",
    },
    {
      service: "checkout-api",
      state: "active",
      last_seen_unix_nano: NOW_NS - 2 * MINUTE_NS,
      manual_override: null,
      priority_tier: "suggested",
    },
    {
      service: "inventory-worker",
      state: "quiet",
      last_seen_unix_nano: NOW_NS - 15 * MINUTE_NS,
      manual_override: null,
      priority_tier: null,
    },
  ],
  total: 3,
  next_cursor: null,
};

export const reportPayload = {
  incident_id: 1,
  title: "payment-service retry storm (fingerprint 9f3a)",
  workspace: "demo-workspace",
  opened_at_unix_nano: NOW_NS - 3 * MINUTE_NS,
  status: "active",
  severity: "critical",
  symptom:
    "payment-service is failing repeatedly with the same exception fingerprint at roughly ten occurrences per thirty seconds.",
  timeline: "T-3m first occurrence; T-2m storm threshold crossed; T-1m severity escalated.",
  hypotheses: [
    {
      statement: "Downstream card-gateway timeout triggering blind client retries.",
      confidence_label: "high",
      justification: "All fingerprint occurrences share the gateway call frame.",
    },
    {
      statement: "Connection-pool exhaustion under burst checkout traffic.",
      confidence_label: "medium",
      justification: "Latency p99 rose before the first exception burst.",
    },
  ],
  investigation_steps: [
    { step: "Inspect card-gateway timeout config in payment-service.", expected_yield: "Confirms or rules out hypothesis 1." },
    { step: "Check connection-pool saturation metrics for the burst window.", expected_yield: "Distinguishes pool exhaustion from gateway latency." },
  ],
  evidence_refs: ["fingerprint:9f3a", "cue:retry_storm:payment-service"],
  project_context: "Workspace demo-workspace; primary language rust; recent commits touch payment retry logic.",
  degraded_mode: false,
  resolution_summary: null,
  previously_seen: [
    {
      incident_id: 7,
      opened_at_unix_nano: NOW_NS - 2_880 * MINUTE_NS,
      title: "payment-service retry storm (fingerprint 9f3a)",
      workspace: "demo-workspace",
    },
  ],
  markdown: "# payment-service retry storm\n\n(symptom, timeline, hypotheses, steps, evidence, context)",
};

export const exportPreviewPayload = {
  categories: [
    { dimension: "incidents", label: "Incident records", count: 12 },
    { dimension: "interpretations", label: "Model interpretations", count: 9 },
  ],
  total_records: 21,
  date_range_start_unix_nano: NOW_NS - 43_200 * MINUTE_NS,
  date_range_end_unix_nano: NOW_NS,
  anonymization_confirmed: true,
  written: false,
  written_path_basename: null,
};

// Diagnostics snapshot in its designed HYBRID state (chunk #97): resolver
// succeeds, but producer-less sub-fields are null/false so the view renders
// explicit "not yet recorded" notices — the honest fresh-install state.
export const diagnosticsSnapshotPayload = {
  model: {
    tier_label: "primary",
    profile_label: "gpu-primary",
    load_status: "not_configured",
    model_identity_name: null,
    backoff_state_label: "inactive",
    backoff_remaining_seconds: 0,
    consecutive_failures: 0,
    inference_success_rate_basis_points: null,
    queue_depth: null,
  },
  hardware: {
    profile_label: "gpu-primary",
    detection_detail: "nvcuda.dll probe succeeded",
  },
  pipeline: {
    drain_template_count: 0,
    per_layer_recorded: false,
  },
  captured_unix_nano: NOW_NS,
};

export const diagnosticsHistoryPayload = {
  metric_name: "inference_success_rate",
  points: [],
  recorded: false,
  notice: "not yet recorded — no numeric-metric-history producer",
};

export const storageInspectPayload = {
  record_counts: [
    { table: "incidents", count: 12 },
    { table: "incident_events", count: 31 },
    { table: "digest_archive", count: 9 },
  ],
  total_bytes_on_disk: 262_144,
  schema_version: 1,
};

export const templateDistributionPayload = {
  templates: [],
  total_template_count: 0,
  last_updated_unix_nano: NOW_NS,
};

export const configStatusPayload = {
  last_reload_unix_nano: NOW_NS,
  last_error_category: null,
  restart_required_pending: 0,
  reload_count: 1,
};

export const v02WidgetOverrides = {
  "incidents.list_active": incidentsListPayload,
  "connection.current_state": connectionReceivingPayload,
  "services.list_with_states": serviceListPayload,
  "incidents.get_report": reportPayload,
  "incidents.mark_all_read": null,
  "diagnostics.snapshot": diagnosticsSnapshotPayload,
  "diagnostics.history": diagnosticsHistoryPayload,
  "diagnostics.template_distribution": templateDistributionPayload,
  "storage.inspect": storageInspectPayload,
  "storage.path": "C:\\Users\\demo\\AppData\\Roaming\\andromeda-pulse",
  "config.status": configStatusPayload,
};
