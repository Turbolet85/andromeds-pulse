// Diagnostics — Drain template distribution panel (chunk #69 Phase B
// Session 6). Fetches `diagnostics.template_distribution()` via the
// typed taurpc proxy and renders the top-50 templates as a semantic
// <table>. Mounted as a collapsible section inside the SettingsModalForm
// per Open Question Q5 option (i) — inline disclosure rather than
// dedicated route.
//
// Per a11y rule §Semantic HTML + §Forms + SC 1.3.1: semantic <table>
// with <thead> + <tbody> + <th scope="col"> column headers. Drift
// indicator paired with text label (SC 1.4.1 not-color-alone). Top-50
// row enforcement at render layer per layouts AC-L5 (server-side
// already caps at TEMPLATE_DISTRIBUTION_TOP_N=50, this is defensive).
//
// Design tokens only (per design-tokens.md): NASA Deep Space palette
// custom properties; JetBrains Mono for data cells; IBM Plex Sans for
// chrome. No hardcoded hex.

import { useEffect, useState } from "react";
import {
  createTauRPCProxy,
  type AppError,
  type DriftIndicatorPayload,
  type TemplateDistributionPayload,
} from "../../../bindings";

const TOP_N_RENDER_CAP = 50;

let cachedClient: ReturnType<typeof createTauRPCProxy> | null = null;

function getClient(): ReturnType<typeof createTauRPCProxy> {
  if (cachedClient === null) {
    cachedClient = createTauRPCProxy();
  }
  return cachedClient;
}

// Test-only seam mirroring SettingsModalForm.tsx / use-traces.ts pattern.
export function __setProxyForTest(
  proxy: ReturnType<typeof createTauRPCProxy> | null,
): void {
  cachedClient = proxy;
}

interface TemplateDistributionState {
  templates: TemplateDistributionPayload["templates"];
  totalTemplateCount: number;
  lastUpdatedUnixNano: number;
  loading: boolean;
  error: string | null;
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
  return {
    kind: "internal",
    message: "diagnostics.template_distribution failed",
  };
}

function driftLabel(indicator: DriftIndicatorPayload): string {
  switch (indicator) {
    case "Healthy":
      return "Healthy";
    case "OverGeneralized":
      return "Over-generalized";
    case "UnderClustered":
      return "Under-clustered";
  }
}

function driftColorVar(indicator: DriftIndicatorPayload): string {
  switch (indicator) {
    case "Healthy":
      return "var(--color-feedback-success)";
    case "OverGeneralized":
      return "var(--color-text-secondary)";
    case "UnderClustered":
      return "var(--color-accent)";
  }
}

const INITIAL_STATE: TemplateDistributionState = {
  templates: [],
  totalTemplateCount: 0,
  lastUpdatedUnixNano: 0,
  loading: true,
  error: null,
};

export function TemplateDistribution() {
  const [state, setState] = useState<TemplateDistributionState>(INITIAL_STATE);

  useEffect(() => {
    let cancelled = false;
    setState((prev) => ({ ...prev, loading: true, error: null }));
    void getClient()
      .diagnostics.template_distribution()
      .then((payload) => {
        if (cancelled) return;
        setState({
          templates: payload.templates.slice(0, TOP_N_RENDER_CAP),
          totalTemplateCount: payload.total_template_count,
          lastUpdatedUnixNano: payload.last_updated_unix_nano,
          loading: false,
          error: null,
        });
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
  }, []);

  if (state.loading) {
    return (
      <div
        data-testid="template-distribution-loading"
        aria-busy="true"
        style={{
          fontFamily: "var(--font-body)",
          fontSize: "12px",
          color: "var(--color-text-secondary)",
          padding: "var(--spacing-sm)",
        }}
      >
        Loading template distribution…
      </div>
    );
  }

  if (state.error !== null) {
    return (
      <div
        role="alert"
        data-testid="template-distribution-error"
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
        Failed to load template distribution: {state.error}
      </div>
    );
  }

  if (state.templates.length === 0) {
    return (
      <div
        data-testid="template-distribution-empty"
        style={{
          fontFamily: "var(--font-body)",
          fontSize: "12px",
          // text-secondary, not tertiary: 12px body text on raised-1 needs
          // 4.5:1 (tertiary measured 3.92:1 — chunk #99 axe finding)
          color: "var(--color-text-secondary)",
          padding: "var(--spacing-md)",
          textAlign: "center",
        }}
      >
        No templates yet
      </div>
    );
  }

  return (
    <div
      data-testid="template-distribution-data"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-xs)",
      }}
    >
      <div
        style={{
          fontFamily: "var(--font-body)",
          fontSize: "12px",
          color: "var(--color-text-secondary)",
        }}
      >
        Showing top {state.templates.length} of {state.totalTemplateCount}{" "}
        template{state.totalTemplateCount === 1 ? "" : "s"}
      </div>
      <table
        data-testid="template-distribution-table"
        style={{
          width: "100%",
          borderCollapse: "collapse",
          fontFamily: "var(--font-code)",
          fontSize: "12px",
          fontVariantNumeric: "tabular-nums",
        }}
      >
        <thead>
          <tr>
            <th
              scope="col"
              style={{
                textAlign: "left",
                fontFamily: "var(--font-body)",
                fontWeight: 600,
                fontSize: "12px",
                color: "var(--color-text-primary)",
                padding: "var(--spacing-sm)",
                borderBottom: "1px solid rgba(74, 144, 226, 0.3)",
                width: "100px",
              }}
            >
              Template ID
            </th>
            <th
              scope="col"
              style={{
                textAlign: "right",
                fontFamily: "var(--font-body)",
                fontWeight: 600,
                fontSize: "12px",
                color: "var(--color-text-primary)",
                padding: "var(--spacing-sm)",
                borderBottom: "1px solid rgba(74, 144, 226, 0.3)",
                width: "100px",
              }}
            >
              Count
            </th>
            <th
              scope="col"
              style={{
                textAlign: "left",
                fontFamily: "var(--font-body)",
                fontWeight: 600,
                fontSize: "12px",
                color: "var(--color-text-primary)",
                padding: "var(--spacing-sm)",
                borderBottom: "1px solid rgba(74, 144, 226, 0.3)",
                width: "150px",
              }}
            >
              Drift
            </th>
            <th
              scope="col"
              style={{
                textAlign: "left",
                fontFamily: "var(--font-body)",
                fontWeight: 600,
                fontSize: "12px",
                color: "var(--color-text-primary)",
                padding: "var(--spacing-sm)",
                borderBottom: "1px solid rgba(74, 144, 226, 0.3)",
              }}
            >
              Sample message
            </th>
          </tr>
        </thead>
        <tbody>
          {state.templates.map((t) => (
            <tr key={String(t.id)}>
              <td
                style={{
                  padding: "var(--spacing-sm)",
                  color: "var(--color-text-secondary)",
                  borderBottom: "1px solid rgba(74, 144, 226, 0.1)",
                }}
              >
                {t.id}
              </td>
              <td
                style={{
                  padding: "var(--spacing-sm)",
                  color: "var(--color-text-primary)",
                  borderBottom: "1px solid rgba(74, 144, 226, 0.1)",
                  textAlign: "right",
                }}
              >
                {t.occurrence_count}
              </td>
              <td
                style={{
                  padding: "var(--spacing-sm)",
                  borderBottom: "1px solid rgba(74, 144, 226, 0.1)",
                }}
              >
                <span
                  data-drift-indicator={t.drift_indicator}
                  style={{
                    display: "inline-flex",
                    alignItems: "center",
                    gap: "var(--spacing-xs)",
                    color: driftColorVar(t.drift_indicator),
                    fontFamily: "var(--font-body)",
                    fontSize: "12px",
                  }}
                >
                  <span
                    aria-hidden="true"
                    style={{
                      display: "inline-block",
                      width: "8px",
                      height: "8px",
                      borderRadius: "var(--radius-full)",
                      background: driftColorVar(t.drift_indicator),
                    }}
                  />
                  {driftLabel(t.drift_indicator)}
                </span>
              </td>
              <td
                title={t.content}
                style={{
                  padding: "var(--spacing-sm)",
                  color: "var(--color-text-primary)",
                  borderBottom: "1px solid rgba(74, 144, 226, 0.1)",
                  whiteSpace: "nowrap",
                  overflow: "hidden",
                  textOverflow: "ellipsis",
                  maxWidth: "1px",
                }}
              >
                {t.content}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
