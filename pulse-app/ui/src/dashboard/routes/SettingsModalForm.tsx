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
};

const RETENTION_SECONDS_MIN = 60;
const RETENTION_SECONDS_MAX = 86_400;

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
}

interface FieldErrors {
  retention_seconds?: string;
  snapshot_preset?: string;
  snapshot_format?: string;
  theme?: string;
  widget_position?: string;
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
}: SettingsModalFormProps) {
  const [formState, setFormState] = useState<SettingsResolved>(DEFAULT_SETTINGS);
  const [loading, setLoading] = useState<boolean>(true);
  const [saving, setSaving] = useState<boolean>(false);
  const [errors, setErrors] = useState<FieldErrors>({});
  const [statusMessage, setStatusMessage] = useState<string>("");
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
              color: "var(--color-text-tertiary)",
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
