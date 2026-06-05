// Settings → Diagnostics view (chunk #97 — capability P-058). A read-only
// L6 self-observability surface: five sections (Connection / Model / Pipeline
// / Hardware / Templates) over the live point-in-time pipeline state.
//
// Hybrid-render scope: snapshot() aggregates what already exists; sub-fields
// with no production producer yet (inference success rate, queue depth,
// per-layer L0-L5 numerics, 30-day metric history) render an explicit
// "not yet recorded" notice — NEVER fabricated values. Connection reuses
// connection.current_state; Templates reuses the existing TemplateDistribution
// panel (diagnostics.template_distribution). The snapshot content area is a
// role="status" aria-live region so the Refresh action announces (SC 4.1.3).
//
// Design tokens only (design-tokens.md): NASA Deep Space palette; JetBrains
// Mono tabular-nums for numeric values; IBM Plex Sans for chrome; borders-only
// cards. State indicators pair color with text (SC 1.4.1 not-color-alone).

import type { ReactNode } from "react";
import { useEffect, useState } from "react";
import { useNavigate } from "@tanstack/react-router";
import {
  createTauRPCProxy,
  type AppError,
  type ConnectionStatePayload,
  type DiagnosticsHistoryPayload,
  type DiagnosticsSnapshotPayload,
} from "../../../bindings";
import {
  connectionColorToken,
  connectionStateLabel,
  formatLastSpanAgo,
} from "../../../components/ConnectionDot";
import { TemplateDistribution } from "./TemplateDistribution";

// 1-day default window for the (currently stubbed) metric-history accessor.
const HISTORY_WINDOW_SECONDS = 86_400;
const HISTORY_METRIC = "drain_template_count";

let cachedClient: ReturnType<typeof createTauRPCProxy> | null = null;

function getClient(): ReturnType<typeof createTauRPCProxy> {
  if (cachedClient === null) {
    cachedClient = createTauRPCProxy();
  }
  return cachedClient;
}

// Test-only seam mirroring TemplateDistribution.tsx / SettingsModalForm.tsx.
export function __setProxyForTest(
  proxy: ReturnType<typeof createTauRPCProxy> | null,
): void {
  cachedClient = proxy;
}

function appErrorMessage(err: AppError): string {
  switch (err.kind) {
    case "validation":
      return err.reason;
    case "not_found":
      return `not found: ${err.resource}`;
    case "internal":
    case "plugin":
    case "storage":
    case "ingest":
      return err.message;
  }
}

function toAppError(err: unknown): AppError {
  if (
    err !== null &&
    typeof err === "object" &&
    "kind" in err &&
    typeof (err as { kind: unknown }).kind === "string"
  ) {
    return err as AppError;
  }
  return { kind: "internal", message: "diagnostics.snapshot failed" };
}

interface DiagnosticsState {
  connection: ConnectionStatePayload | null;
  snapshot: DiagnosticsSnapshotPayload | null;
  history: DiagnosticsHistoryPayload | null;
  loading: boolean;
  error: string | null;
}

const INITIAL_STATE: DiagnosticsState = {
  connection: null,
  snapshot: null,
  history: null,
  loading: true,
  error: null,
};

function Section({ id, title, children }: { id: string; title: string; children: ReactNode }) {
  return (
    <section
      aria-labelledby={`diag-section-${id}`}
      data-testid={`diag-section-${id}`}
      style={{
        background: "var(--color-raised-1)",
        border: "1px solid rgba(74, 144, 226, 0.3)",
        borderRadius: "var(--radius-md)",
        padding: "var(--spacing-md)",
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-sm)",
      }}
    >
      <h2
        id={`diag-section-${id}`}
        style={{
          fontFamily: "var(--font-body)",
          fontSize: "14px",
          fontWeight: 600,
          color: "var(--color-text-primary)",
          margin: 0,
        }}
      >
        {title}
      </h2>
      {children}
    </section>
  );
}

function MetricRow({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div
      style={{
        display: "flex",
        justifyContent: "space-between",
        gap: "var(--spacing-sm)",
        fontSize: "12px",
      }}
    >
      <span style={{ fontFamily: "var(--font-body)", color: "var(--color-text-secondary)" }}>
        {label}
      </span>
      <span
        style={{
          fontFamily: "var(--font-code)",
          fontVariantNumeric: "tabular-nums",
          color: "var(--color-text-primary)",
          textAlign: "right",
        }}
      >
        {children}
      </span>
    </div>
  );
}

function NotRecorded() {
  return (
    <span
      data-testid="diag-not-recorded"
      style={{ fontFamily: "var(--font-body)", color: "var(--color-text-secondary)" }}
    >
      not yet recorded
    </span>
  );
}

// Color + text pairing (SC 1.4.1) for a status word.
function StateBadge({ label, colorVar }: { label: string; colorVar: string }) {
  return (
    <span style={{ display: "inline-flex", alignItems: "center", gap: "var(--spacing-xs)" }}>
      <span
        aria-hidden="true"
        style={{
          display: "inline-block",
          width: "8px",
          height: "8px",
          borderRadius: "var(--radius-full)",
          background: colorVar,
        }}
      />
      <span style={{ fontFamily: "var(--font-body)", color: "var(--color-text-primary)" }}>
        {label}
      </span>
    </span>
  );
}

function loadStatusColor(loadStatus: string): string {
  switch (loadStatus) {
    case "loaded":
      return "var(--color-feedback-success)";
    case "error":
      return "var(--color-accent)";
    default:
      return "var(--color-primary)";
  }
}

function backoffColor(stateLabel: string): string {
  return stateLabel === "degraded" ? "var(--color-accent)" : "var(--color-feedback-success)";
}

export function Diagnostics() {
  const navigate = useNavigate();
  const [state, setState] = useState<DiagnosticsState>(INITIAL_STATE);
  const [reloadKey, setReloadKey] = useState(0);

  useEffect(() => {
    let cancelled = false;
    setState((prev) => ({ ...prev, loading: true, error: null }));
    const client = getClient();
    void Promise.all([
      client.connection.current_state(),
      client.diagnostics.snapshot(),
      client.diagnostics.history(HISTORY_METRIC, HISTORY_WINDOW_SECONDS),
    ])
      .then(([connection, snapshot, history]) => {
        if (cancelled) return;
        setState({ connection, snapshot, history, loading: false, error: null });
      })
      .catch((err: unknown) => {
        if (cancelled) return;
        setState((prev) => ({
          ...prev,
          loading: false,
          error: appErrorMessage(toAppError(err)),
        }));
      });
    return () => {
      cancelled = true;
    };
  }, [reloadKey]);

  return (
    <section
      id="tabpanel-diagnostics"
      aria-labelledby="route-heading-diagnostics"
      data-testid="route-diagnostics"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-lg)",
        padding: "var(--spacing-md)",
      }}
    >
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          gap: "var(--spacing-sm)",
        }}
      >
        <h1
          id="route-heading-diagnostics"
          style={{
            fontFamily: "var(--font-display)",
            fontSize: "20px",
            fontWeight: 600,
            color: "var(--color-text-primary)",
            margin: 0,
          }}
        >
          Diagnostics
        </h1>
        <div style={{ display: "flex", gap: "var(--spacing-sm)" }}>
          <button
            type="button"
            data-testid="diagnostics-refresh"
            onClick={() => setReloadKey((k) => k + 1)}
            style={{
              fontFamily: "var(--font-body)",
              fontSize: "12px",
              color: "var(--color-text-primary)",
              background: "var(--color-raised-2)",
              border: "1px solid rgba(74, 144, 226, 0.3)",
              borderRadius: "var(--radius-sm)",
              padding: "var(--spacing-xs) var(--spacing-sm)",
              cursor: "pointer",
            }}
          >
            Refresh
          </button>
          <button
            type="button"
            data-testid="diagnostics-back-to-settings"
            onClick={() => {
              void navigate({ to: "/settings" });
            }}
            style={{
              fontFamily: "var(--font-body)",
              fontSize: "12px",
              color: "var(--color-text-primary)",
              background: "var(--color-raised-2)",
              border: "1px solid rgba(74, 144, 226, 0.3)",
              borderRadius: "var(--radius-sm)",
              padding: "var(--spacing-xs) var(--spacing-sm)",
              cursor: "pointer",
            }}
          >
            Back to Settings
          </button>
        </div>
      </div>

      <div
        role="status"
        aria-live="polite"
        data-testid="diagnostics-content"
        style={{ display: "flex", flexDirection: "column", gap: "var(--spacing-lg)" }}
      >
        {state.loading ? (
          <div
            data-testid="diagnostics-loading"
            aria-busy="true"
            style={{
              fontFamily: "var(--font-body)",
              fontSize: "12px",
              color: "var(--color-text-secondary)",
            }}
          >
            Loading diagnostics…
          </div>
        ) : state.error !== null ? (
          <div
            role="alert"
            data-testid="diagnostics-error"
            style={{
              border: "1px solid var(--color-accent)",
              background: "var(--color-inset)",
              borderRadius: "var(--radius-sm)",
              padding: "var(--spacing-sm)",
              color: "var(--color-text-primary)",
              fontFamily: "var(--font-body)",
              fontSize: "12px",
            }}
          >
            Failed to load diagnostics: {state.error}
          </div>
        ) : (
          <>
            <Section id="connection" title="Connection">
              {state.connection ? (
                <>
                  <MetricRow label="State">
                    <StateBadge
                      label={connectionStateLabel(state.connection)}
                      colorVar={connectionColorToken(state.connection.state)}
                    />
                  </MetricRow>
                  <MetricRow label="Last span">
                    {formatLastSpanAgo(state.connection.last_span_ago_ms)}
                  </MetricRow>
                </>
              ) : (
                <MetricRow label="State">
                  <NotRecorded />
                </MetricRow>
              )}
            </Section>

            <Section id="model" title="Model">
              {state.snapshot ? (
                <>
                  <MetricRow label="Tier">{state.snapshot.model.tier_label}</MetricRow>
                  <MetricRow label="Load status">
                    <StateBadge
                      label={state.snapshot.model.load_status}
                      colorVar={loadStatusColor(state.snapshot.model.load_status)}
                    />
                  </MetricRow>
                  <MetricRow label="Model">
                    {state.snapshot.model.model_identity_name ?? <NotRecorded />}
                  </MetricRow>
                  <MetricRow label="Backoff">
                    <StateBadge
                      label={state.snapshot.model.backoff_state_label}
                      colorVar={backoffColor(state.snapshot.model.backoff_state_label)}
                    />
                  </MetricRow>
                  <MetricRow label="Backoff remaining">
                    {state.snapshot.model.backoff_remaining_seconds}s
                  </MetricRow>
                  <MetricRow label="Consecutive failures">
                    {state.snapshot.model.consecutive_failures}
                  </MetricRow>
                  <MetricRow label="Inference success rate">
                    {state.snapshot.model.inference_success_rate_basis_points === null ? (
                      <NotRecorded />
                    ) : (
                      `${(state.snapshot.model.inference_success_rate_basis_points / 100).toFixed(2)}%`
                    )}
                  </MetricRow>
                  <MetricRow label="Queue depth">
                    {state.snapshot.model.queue_depth ?? <NotRecorded />}
                  </MetricRow>
                </>
              ) : (
                <NotRecorded />
              )}
            </Section>

            <Section id="pipeline" title="Pipeline">
              {state.snapshot ? (
                <>
                  <MetricRow label="Drain templates">
                    {state.snapshot.pipeline.drain_template_count}
                  </MetricRow>
                  <MetricRow label="Per-layer metrics (L0–L5)">
                    {state.snapshot.pipeline.per_layer_recorded ? "recorded" : <NotRecorded />}
                  </MetricRow>
                  <MetricRow label="Metric history">
                    {state.history && state.history.recorded ? (
                      `${state.history.points.length} points`
                    ) : (
                      <span
                        data-testid="diagnostics-history-notice"
                        style={{
                          fontFamily: "var(--font-body)",
                          color: "var(--color-text-secondary)",
                        }}
                      >
                        {state.history?.notice ?? "not yet recorded"}
                      </span>
                    )}
                  </MetricRow>
                </>
              ) : (
                <NotRecorded />
              )}
            </Section>

            <Section id="hardware" title="Hardware">
              {state.snapshot ? (
                <>
                  <MetricRow label="Profile">
                    {state.snapshot.hardware.profile_label}
                  </MetricRow>
                  <MetricRow label="Detection detail">
                    {state.snapshot.hardware.detection_detail ?? <NotRecorded />}
                  </MetricRow>
                </>
              ) : (
                <NotRecorded />
              )}
            </Section>
          </>
        )}
      </div>

      <Section id="templates" title="Templates">
        <TemplateDistribution />
      </Section>
    </section>
  );
}
