// Settings modal form (chunk #38) — composes the chunk #37 Modal primitive
// with form content covering all persisted Settings fields. Persistence flows
// through existing `update_settings` TauRPC procedure; no new IPC namespace
// per .claude/rules/security.md Session Additions 2026-05-09 (Settings-extension
// pattern avoids the security ↔ tests/CI ↔ arch capability-drift triple binding).
//
// Plugin-manager section is a static placeholder: the `plugins.list` /
// `plugins.reload` IPC surface is part of epoch 7 (chunk #43+) and not yet
// exposed in the TauRPC bindings. Rendering a heading + "available in epoch 7"
// caption preserves the plan's section structure without requiring a new IPC
// namespace this chunk.

import { useEffect, useRef, useState, type RefObject } from "react";
import {
  Button,
  Label,
  Radio,
  RadioGroup,
  Switch,
} from "react-aria-components";
import { Modal } from "../../components/Modal";
import {
  createTauRPCProxy,
  type AppError,
  type Settings,
  type SnapshotFormat,
  type SnapshotPreset,
  type Theme,
  type WidgetPosition,
} from "../../bindings";
import { TemplateDistribution } from "./diagnostics/TemplateDistribution";
import { ExportForTraining } from "./settings/ExportForTraining";

type SettingsResolved = Required<Settings>;

const DEFAULT_SETTINGS: SettingsResolved = {
  theme: "dark",
  widget_position: "top-right",
  retention_seconds: 600,
  mcp_server_enabled: false,
  notifications_enabled: true,
  always_on_top: true,
  snapshot_preset: "balanced",
  snapshot_format: "markdown",
  lifecycle_dormant_after_secs: 3_600,
  lifecycle_archived_after_secs: 86_400,
  drain_depth: 4,
  drain_similarity_x100: 50,
  drain_max_clusters: 1000,
  // Chunk #80 — Cadence Coordinator defaults. Surfaced through Settings
  // bindings but not yet form-editable; chunk #86 carries forward the
  // backend-side defaults к unblock the typecheck gate (Required<Settings>
  // shape now includes these). Form controls land in а follow-up UI chunk.
  cadence_baseline_seconds: 60,
  cadence_accelerated_seconds: 20,
  cadence_reflection_seconds: 1800,
  cadence_tier2_acceleration_enabled: true,
};

const RETENTION_SECONDS_MIN = 60;
const RETENTION_SECONDS_MAX = 86_400;
// Chunk #69 Phase B Session 5 — Drain knob bounds mirror the Rust-side
// constants in `crates/ui-bridge/src/contract.rs` (DRAIN_DEPTH_MIN/MAX /
// DRAIN_SIMILARITY_X100_MIN/MAX / DRAIN_MAX_CLUSTERS_MIN/MAX). Kept in
// sync with the backend bounds discipline; validation surfaces via the
// `update_settings` AppError::Validation { field, reason } path.
const DRAIN_SIMILARITY_X100_MIN = 30;
const DRAIN_SIMILARITY_X100_MAX = 70;
const DRAIN_MAX_CLUSTERS_MIN = 100;
const DRAIN_MAX_CLUSTERS_MAX = 10_000;

let cachedClient: ReturnType<typeof createTauRPCProxy> | null = null;

function getClient(): ReturnType<typeof createTauRPCProxy> {
  if (cachedClient === null) {
    cachedClient = createTauRPCProxy();
  }
  return cachedClient;
}

// Test-only seam mirroring use-traces.ts __setProxyForTest pattern.
export function __setProxyForTest(
  proxy: ReturnType<typeof createTauRPCProxy> | null,
): void {
  cachedClient = proxy;
}

interface SettingsModalFormProps {
  open: boolean;
  onClose: () => void;
  triggerRef: RefObject<HTMLElement | null>;
  // Chunk #97 — navigate to the full Settings → Diagnostics route. Optional
  // so the modal stays router-agnostic for unit tests (rendered outside a
  // RouterProvider); SettingsRoute supplies the navigate callback.
  onOpenDiagnostics?: () => void;
}

interface FieldErrors {
  retention_seconds?: string;
  snapshot_preset?: string;
  snapshot_format?: string;
  theme?: string;
  widget_position?: string;
  drain_depth?: string;
  drain_similarity_x100?: string;
  drain_max_clusters?: string;
  generic?: string;
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

function toFieldErrors(err: AppError): FieldErrors {
  if (err.kind === "validation") {
    return { [err.field]: err.reason };
  }
  return { generic: appErrorMessage(err) };
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
  return { kind: "internal", message: "update_settings failed" };
}

export function SettingsModalForm({
  open,
  onClose,
  triggerRef,
  onOpenDiagnostics,
}: SettingsModalFormProps) {
  const [formState, setFormState] = useState<SettingsResolved>(DEFAULT_SETTINGS);
  const [loading, setLoading] = useState<boolean>(true);
  const [saving, setSaving] = useState<boolean>(false);
  const [errors, setErrors] = useState<FieldErrors>({});
  const [statusMessage, setStatusMessage] = useState<string>("");
  // Chunk #69 Phase B Session 6 — Diagnostics disclosure section (plan
  // Step 21). Inline collapsible per Open Question Q5 option (i);
  // component-local state OK (no persistence).
  const [diagnosticsOpen, setDiagnosticsOpen] = useState<boolean>(false);
  // Chunk #86 — manual retry override for L4 interpretation degraded-mode
  // FSM. State carries the in-flight flag + post-invocation status message
  // for the aria-live region. Component-local; no persistence.
  const [retryInflight, setRetryInflight] = useState<boolean>(false);
  const [retryStatusMessage, setRetryStatusMessage] = useState<string>("");
  const initialFocusRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!open) {
      return;
    }
    let cancelled = false;
    setLoading(true);
    setErrors({});
    void getClient()
      .get_settings()
      .then((settings) => {
        if (cancelled) return;
        setFormState({ ...DEFAULT_SETTINGS, ...settings });
        setLoading(false);
      })
      .catch((err: unknown) => {
        if (cancelled) return;
        setErrors(toFieldErrors(toAppError(err)));
        setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [open]);

  const onSave = () => {
    setSaving(true);
    setErrors({});
    setStatusMessage("");
    void getClient()
      .update_settings(formState)
      .then(() => {
        setSaving(false);
        setStatusMessage("Settings saved");
        window.setTimeout(() => onClose(), 600);
      })
      .catch((err: unknown) => {
        setSaving(false);
        setErrors(toFieldErrors(toAppError(err)));
        setStatusMessage("");
      });
  };

  const onRetentionChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const raw = event.target.value;
    const parsed = Number(raw);
    if (Number.isFinite(parsed)) {
      setFormState((prev) => ({ ...prev, retention_seconds: parsed }));
    }
  };

  const onDrainSimilarityChange = (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    const raw = event.target.value;
    const parsed = Number(raw);
    if (Number.isFinite(parsed)) {
      setFormState((prev) => ({ ...prev, drain_similarity_x100: parsed }));
    }
  };

  const onDrainMaxClustersChange = (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    const raw = event.target.value;
    const parsed = Number(raw);
    if (Number.isFinite(parsed)) {
      setFormState((prev) => ({ ...prev, drain_max_clusters: parsed }));
    }
  };

  // Chunk #86 — manual L4 retry override. Invokes
  // `diagnostics.retry_interpretation()` TauRPC procedure; resets the
  // degraded-mode FSM regardless of current state. Focus stays on the
  // button across async completion per a11y-plan §5 focus restoration
  // (matches Critical Path P6 snapshot-generation pattern). aria-live
  // region announces the post-invocation state — `polite` (not assertive)
  // for routine success per WCAG SC 4.1.3.
  const onRetryInterpretation = () => {
    setRetryInflight(true);
    setRetryStatusMessage("");
    void getClient()
      .diagnostics.retry_interpretation()
      .then((payload) => {
        setRetryInflight(false);
        if (payload.triggered) {
          setRetryStatusMessage(
            `Retry triggered — interpretation state: ${payload.current_state}`,
          );
        } else {
          setRetryStatusMessage(
            `No retry needed — interpretation already ${payload.current_state}`,
          );
        }
      })
      .catch((err: unknown) => {
        setRetryInflight(false);
        setRetryStatusMessage(`Retry failed: ${appErrorMessage(toAppError(err))}`);
      });
  };

  return (
    <Modal
      open={open}
      onClose={onClose}
      triggerRef={triggerRef}
      titleId="settings-modal-title"
      title="Settings"
      busy={saving || loading}
      liveRegionLevel="polite"
      liveMessage={statusMessage}
      initialFocus={initialFocusRef}
    >
      <form
        onSubmit={(e) => {
          e.preventDefault();
          onSave();
        }}
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "var(--spacing-lg)",
          paddingTop: "var(--spacing-sm)",
        }}
      >
        {errors.generic ? (
          <div
            role="alert"
            data-testid="settings-form-error"
            style={{
              border: "1px solid rgba(199, 85, 106, 0.5)",
              background: "var(--color-inset)",
              borderRadius: "var(--radius-sm)",
              padding: "var(--spacing-sm)",
              color: "var(--color-text-primary)",
              fontSize: "14px",
            }}
          >
            {errors.generic}
          </div>
        ) : null}

        <RadioGroup
          value={formState.theme}
          onChange={(value) =>
            setFormState((prev) => ({ ...prev, theme: value as Theme }))
          }
          isDisabled={loading || saving}
          aria-labelledby="theme-label"
          style={fieldGroupStyle}
        >
          <Label id="theme-label" style={labelStyle}>
            Theme
          </Label>
          <div ref={initialFocusRef} style={radioRowStyle}>
            <Radio value="dark" style={radioStyle}>
              Dark
            </Radio>
            <Radio value="light" style={radioStyle}>
              Light
            </Radio>
            <Radio value="auto" style={radioStyle}>
              Auto
            </Radio>
          </div>
        </RadioGroup>

        <RadioGroup
          value={formState.widget_position}
          onChange={(value) =>
            setFormState((prev) => ({
              ...prev,
              widget_position: value as WidgetPosition,
            }))
          }
          isDisabled={loading || saving}
          aria-labelledby="widget-position-label"
          style={fieldGroupStyle}
        >
          <Label id="widget-position-label" style={labelStyle}>
            Widget position
          </Label>
          <div style={radioRowStyle}>
            <Radio value="top-left" style={radioStyle}>
              Top left
            </Radio>
            <Radio value="top-right" style={radioStyle}>
              Top right
            </Radio>
            <Radio value="bottom-left" style={radioStyle}>
              Bottom left
            </Radio>
            <Radio value="bottom-right" style={radioStyle}>
              Bottom right
            </Radio>
          </div>
        </RadioGroup>

        <div style={fieldGroupStyle}>
          <label htmlFor="settings-retention-seconds" style={labelStyle}>
            Retention seconds
          </label>
          <input
            id="settings-retention-seconds"
            type="number"
            min={RETENTION_SECONDS_MIN}
            max={RETENTION_SECONDS_MAX}
            step={1}
            value={String(formState.retention_seconds)}
            onChange={onRetentionChange}
            disabled={loading || saving}
            aria-describedby={
              errors.retention_seconds
                ? "retention-error retention-help-text"
                : "retention-help-text"
            }
            aria-invalid={Boolean(errors.retention_seconds)}
            style={{
              background: "var(--color-inset)",
              border: errors.retention_seconds
                ? "1px solid rgba(199, 85, 106, 0.5)"
                : "1px solid rgba(74, 144, 226, 0.3)",
              borderRadius: "var(--radius-sm)",
              padding: "var(--spacing-sm)",
              color: "var(--color-text-primary)",
              fontFamily: "var(--font-code)",
              fontSize: "12px",
              maxWidth: "180px",
            }}
          />
          <span
            id="retention-help-text"
            style={{
              fontSize: "12px",
              color: "var(--color-text-secondary)",
            }}
          >
            Range {RETENTION_SECONDS_MIN}–{RETENTION_SECONDS_MAX} (1 minute to 24 hours)
          </span>
          {errors.retention_seconds ? (
            <span
              id="retention-error"
              role="alert"
              style={{
                fontSize: "12px",
                color: "var(--color-text-primary)",
              }}
            >
              {errors.retention_seconds}
            </span>
          ) : null}
        </div>

        <div style={fieldGroupStyle}>
          <Switch
            isSelected={formState.mcp_server_enabled}
            onChange={(value) =>
              setFormState((prev) => ({ ...prev, mcp_server_enabled: value }))
            }
            isDisabled={loading || saving}
            style={{
              display: "flex",
              alignItems: "center",
              gap: "var(--spacing-sm)",
            }}
          >
            <span style={labelStyle}>MCP server</span>
            <span
              data-testid="mcp-toggle-state-label"
              style={{
                fontSize: "12px",
                color: "var(--color-text-secondary)",
              }}
            >
              {formState.mcp_server_enabled ? "On" : "Off"}
            </span>
          </Switch>
        </div>

        <RadioGroup
          value={formState.snapshot_preset}
          onChange={(value) =>
            setFormState((prev) => ({
              ...prev,
              snapshot_preset: value as SnapshotPreset,
            }))
          }
          isDisabled={loading || saving}
          aria-labelledby="snapshot-preset-label"
          style={fieldGroupStyle}
        >
          <Label id="snapshot-preset-label" style={labelStyle}>
            Snapshot preset
          </Label>
          <div style={radioRowStyle}>
            <Radio value="conservative" style={radioStyle}>
              Conservative (10k tokens)
            </Radio>
            <Radio value="balanced" style={radioStyle}>
              Balanced (25k tokens)
            </Radio>
            <Radio value="detailed" style={radioStyle}>
              Detailed (50k tokens)
            </Radio>
          </div>
        </RadioGroup>

        <RadioGroup
          value={formState.snapshot_format}
          onChange={(value) =>
            setFormState((prev) => ({
              ...prev,
              snapshot_format: value as SnapshotFormat,
            }))
          }
          isDisabled={loading || saving}
          aria-labelledby="snapshot-format-label"
          style={fieldGroupStyle}
        >
          <Label id="snapshot-format-label" style={labelStyle}>
            Snapshot format
          </Label>
          <div style={radioRowStyle}>
            <Radio value="markdown" style={radioStyle}>
              Markdown
            </Radio>
            <Radio value="json" style={radioStyle}>
              JSON
            </Radio>
          </div>
        </RadioGroup>

        <section
          aria-labelledby="drain-config-label"
          style={fieldGroupStyle}
        >
          <h3
            id="drain-config-label"
            style={{
              ...labelStyle,
              fontFamily: "var(--font-display)",
              fontSize: "14px",
              fontWeight: 600,
              margin: 0,
            }}
          >
            Drain log-template mining
          </h3>

          <RadioGroup
            value={String(formState.drain_depth)}
            onChange={(value) =>
              setFormState((prev) => ({
                ...prev,
                drain_depth: Number(value),
              }))
            }
            isDisabled={loading || saving}
            aria-labelledby="drain-depth-label"
            style={fieldGroupStyle}
          >
            <Label id="drain-depth-label" style={labelStyle}>
              Tree depth
            </Label>
            <div style={radioRowStyle}>
              <Radio value="3" style={radioStyle}>
                3
              </Radio>
              <Radio value="4" style={radioStyle}>
                4 (default)
              </Radio>
              <Radio value="5" style={radioStyle}>
                5
              </Radio>
            </div>
            {errors.drain_depth ? (
              <span
                id="drain-depth-error"
                role="alert"
                style={{
                  fontSize: "12px",
                  color: "var(--color-text-primary)",
                }}
              >
                {errors.drain_depth}
              </span>
            ) : null}
          </RadioGroup>

          <div style={fieldGroupStyle}>
            <label
              htmlFor="settings-drain-similarity"
              style={labelStyle}
            >
              Similarity threshold (×100)
            </label>
            <input
              id="settings-drain-similarity"
              type="number"
              min={DRAIN_SIMILARITY_X100_MIN}
              max={DRAIN_SIMILARITY_X100_MAX}
              step={5}
              value={String(formState.drain_similarity_x100)}
              onChange={onDrainSimilarityChange}
              disabled={loading || saving}
              aria-describedby={
                errors.drain_similarity_x100
                  ? "drain-similarity-error drain-similarity-help-text"
                  : "drain-similarity-help-text"
              }
              aria-invalid={Boolean(errors.drain_similarity_x100)}
              style={{
                background: "var(--color-inset)",
                border: errors.drain_similarity_x100
                  ? "1px solid rgba(199, 85, 106, 0.5)"
                  : "1px solid rgba(74, 144, 226, 0.3)",
                borderRadius: "var(--radius-sm)",
                padding: "var(--spacing-sm)",
                color: "var(--color-text-primary)",
                fontFamily: "var(--font-code)",
                fontSize: "12px",
                maxWidth: "180px",
              }}
            />
            <span
              id="drain-similarity-help-text"
              style={{
                fontSize: "12px",
                color: "var(--color-text-secondary)",
              }}
            >
              Range {DRAIN_SIMILARITY_X100_MIN}–{DRAIN_SIMILARITY_X100_MAX}{" "}
              (represents 0.30–0.70; 50 ↔ 0.50)
            </span>
            {errors.drain_similarity_x100 ? (
              <span
                id="drain-similarity-error"
                role="alert"
                style={{
                  fontSize: "12px",
                  color: "var(--color-text-primary)",
                }}
              >
                {errors.drain_similarity_x100}
              </span>
            ) : null}
          </div>

          <div style={fieldGroupStyle}>
            <label
              htmlFor="settings-drain-max-clusters"
              style={labelStyle}
            >
              Max template clusters
            </label>
            <input
              id="settings-drain-max-clusters"
              type="number"
              min={DRAIN_MAX_CLUSTERS_MIN}
              max={DRAIN_MAX_CLUSTERS_MAX}
              step={100}
              value={String(formState.drain_max_clusters)}
              onChange={onDrainMaxClustersChange}
              disabled={loading || saving}
              aria-describedby={
                errors.drain_max_clusters
                  ? "drain-max-clusters-error drain-max-clusters-help-text"
                  : "drain-max-clusters-help-text"
              }
              aria-invalid={Boolean(errors.drain_max_clusters)}
              style={{
                background: "var(--color-inset)",
                border: errors.drain_max_clusters
                  ? "1px solid rgba(199, 85, 106, 0.5)"
                  : "1px solid rgba(74, 144, 226, 0.3)",
                borderRadius: "var(--radius-sm)",
                padding: "var(--spacing-sm)",
                color: "var(--color-text-primary)",
                fontFamily: "var(--font-code)",
                fontSize: "12px",
                maxWidth: "180px",
              }}
            />
            <span
              id="drain-max-clusters-help-text"
              style={{
                fontSize: "12px",
                color: "var(--color-text-secondary)",
              }}
            >
              Range {DRAIN_MAX_CLUSTERS_MIN}–{DRAIN_MAX_CLUSTERS_MAX} (LRU-bounded)
            </span>
            {errors.drain_max_clusters ? (
              <span
                id="drain-max-clusters-error"
                role="alert"
                style={{
                  fontSize: "12px",
                  color: "var(--color-text-primary)",
                }}
              >
                {errors.drain_max_clusters}
              </span>
            ) : null}
          </div>

          <div
            role="status"
            aria-live="polite"
            data-testid="drain-restart-required-notice"
            style={{
              border: "1px solid var(--color-accent)",
              background: "var(--color-base)",
              borderRadius: "var(--radius-sm)",
              padding: "var(--spacing-sm)",
              color: "var(--color-text-primary)",
              fontSize: "12px",
            }}
          >
            Drain config changes require restart to apply (template tree
            invalidation).
          </div>
        </section>

        <section
          aria-labelledby="diagnostics-section-label"
          style={fieldGroupStyle}
        >
          <h3
            id="diagnostics-section-label"
            style={{
              ...labelStyle,
              fontFamily: "var(--font-display)",
              fontSize: "14px",
              fontWeight: 600,
              margin: 0,
            }}
          >
            Diagnostics
          </h3>
          {onOpenDiagnostics ? (
            <button
              type="button"
              data-testid="open-diagnostics-view"
              onClick={onOpenDiagnostics}
              disabled={loading || saving}
              style={{
                alignSelf: "flex-start",
                background: "var(--color-primary)",
                color: "var(--color-base)",
                border: "1px solid var(--color-primary)",
                borderRadius: "var(--radius-sm)",
                padding: "var(--spacing-sm) var(--spacing-md)",
                fontFamily: "var(--font-body)",
                fontSize: "12px",
                fontWeight: 500,
                cursor: "pointer",
              }}
            >
              Open full Diagnostics view
            </button>
          ) : null}
          <button
            type="button"
            aria-expanded={diagnosticsOpen}
            aria-controls="diagnostics-panel"
            data-testid="diagnostics-disclosure-toggle"
            onClick={() => setDiagnosticsOpen((prev) => !prev)}
            disabled={loading || saving}
            style={{
              alignSelf: "flex-start",
              background: "var(--color-raised-1)",
              color: "var(--color-text-primary)",
              border: "1px solid rgba(74, 144, 226, 0.3)",
              borderRadius: "var(--radius-sm)",
              padding: "var(--spacing-sm) var(--spacing-md)",
              fontFamily: "var(--font-body)",
              fontSize: "12px",
              cursor: "pointer",
            }}
          >
            {diagnosticsOpen ? "Hide" : "Show"} template distribution
          </button>
          {diagnosticsOpen ? (
            <div
              id="diagnostics-panel"
              data-testid="diagnostics-panel"
              style={{
                background: "var(--color-raised-1)",
                border: "1px solid rgba(74, 144, 226, 0.1)",
                borderRadius: "var(--radius-md)",
                padding: "var(--spacing-md)",
              }}
            >
              <TemplateDistribution />
            </div>
          ) : null}
          <div
            data-testid="retry-interpretation-row"
            style={{
              display: "flex",
              flexDirection: "column",
              gap: "var(--spacing-xs)",
              marginTop: "var(--spacing-sm)",
            }}
          >
            <button
              type="button"
              aria-busy={retryInflight}
              data-testid="retry-interpretation-button"
              onClick={onRetryInterpretation}
              disabled={loading || saving || retryInflight}
              style={{
                alignSelf: "flex-start",
                background: "var(--color-raised-1)",
                color: "var(--color-text-primary)",
                border: "1px solid rgba(74, 144, 226, 0.3)",
                borderRadius: "var(--radius-sm)",
                padding: "var(--spacing-sm) var(--spacing-md)",
                fontFamily: "var(--font-body)",
                fontSize: "12px",
                cursor: retryInflight ? "not-allowed" : "pointer",
              }}
            >
              {retryInflight
                ? "Retrying interpretation..."
                : "Retry interpretation now"}
            </button>
            <p
              style={{
                fontSize: "11px",
                // text-secondary, not tertiary: small body text on raised-3
                // needs 4.5:1 (chunk #99 axe finding: 3.11:1)
                color: "var(--color-text-secondary)",
                margin: 0,
              }}
            >
              Manual override for L4 interpretation backoff. Resets failure
              counter and exits degraded mode immediately.
            </p>
            <div
              role="status"
              aria-live="polite"
              data-testid="retry-interpretation-status"
              style={{
                fontSize: "12px",
                color: "var(--color-text-secondary)",
                minHeight: "1.5em",
              }}
            >
              {retryStatusMessage}
            </div>
          </div>
        </section>

        <section
          aria-labelledby="storage-section-label"
          style={fieldGroupStyle}
        >
          <h3
            id="storage-section-label"
            style={{
              ...labelStyle,
              fontFamily: "var(--font-display)",
              fontSize: "14px",
              fontWeight: 600,
              margin: 0,
            }}
          >
            Storage
          </h3>
          <ExportForTraining />
        </section>

        <section
          aria-labelledby="plugin-manager-label"
          style={fieldGroupStyle}
        >
          <h3
            id="plugin-manager-label"
            style={{
              ...labelStyle,
              fontFamily: "var(--font-display)",
              fontSize: "14px",
              fontWeight: 600,
              margin: 0,
            }}
          >
            Plugin manager
          </h3>
          <p
            data-testid="plugin-manager-placeholder"
            style={{
              fontSize: "12px",
              // text-secondary, not tertiary (12px body text on raised-3;
              // chunk #99 axe finding)
              color: "var(--color-text-secondary)",
              margin: 0,
            }}
          >
            Plugin discovery + reload UI lands in epoch 7 alongside the
            <code style={{ fontFamily: "var(--font-code)" }}>
              {" plugins.list "}
            </code>
            IPC surface.
          </p>
        </section>

        <div
          style={{
            display: "flex",
            justifyContent: "flex-end",
            gap: "var(--spacing-sm)",
            paddingTop: "var(--spacing-md)",
            borderTop: "1px solid rgba(74, 144, 226, 0.1)",
          }}
        >
          <Button
            type="button"
            onPress={onClose}
            isDisabled={saving}
            style={{
              background: "var(--color-raised-1)",
              color: "var(--color-text-primary)",
              border: "1px solid rgba(74, 144, 226, 0.3)",
              borderRadius: "var(--radius-sm)",
              padding: "var(--spacing-sm) var(--spacing-md)",
              fontFamily: "var(--font-body)",
              fontSize: "14px",
              cursor: "pointer",
            }}
          >
            Cancel
          </Button>
          <Button
            type="submit"
            isDisabled={loading || saving}
            onPress={onSave}
            style={{
              background: "var(--color-primary)",
              color: "var(--color-base)",
              border: "1px solid var(--color-primary)",
              borderRadius: "var(--radius-sm)",
              padding: "var(--spacing-sm) var(--spacing-md)",
              fontFamily: "var(--font-body)",
              fontSize: "14px",
              fontWeight: 500,
              cursor: "pointer",
            }}
          >
            {saving ? "Saving…" : "Save"}
          </Button>
        </div>
      </form>
    </Modal>
  );
}

const fieldGroupStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "var(--spacing-sm)",
};

const labelStyle: React.CSSProperties = {
  fontFamily: "var(--font-body)",
  fontSize: "12px",
  fontWeight: 500,
  color: "var(--color-text-primary)",
};

const radioRowStyle: React.CSSProperties = {
  display: "flex",
  flexWrap: "wrap",
  gap: "var(--spacing-sm)",
};

const radioStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "center",
  gap: "var(--spacing-xs)",
  fontFamily: "var(--font-body)",
  fontSize: "14px",
  color: "var(--color-text-primary)",
  cursor: "pointer",
};
