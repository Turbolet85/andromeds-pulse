// Export for community training (chunk #95) — Settings → Storage action.
// Reuses the chunk #37 Modal primitive for the pre-write preview/confirm
// dialog. Calls storage.export_for_training(null, false) for the preview
// (no file written) then (null, true) on confirm (writes the JSONL). No
// auto-submission — the user manually shares the written file (P-046).

import { useRef, useState } from "react";
import { Button } from "react-aria-components";
import { Modal } from "../../../components/Modal";
import {
  createTauRPCProxy,
  type AppError,
  type ExportPreviewPayload,
} from "../../../bindings";

let cachedClient: ReturnType<typeof createTauRPCProxy> | null = null;

function getClient(): ReturnType<typeof createTauRPCProxy> {
  if (cachedClient === null) {
    cachedClient = createTauRPCProxy();
  }
  return cachedClient;
}

// Test-only seam mirroring the SettingsModalForm / use-traces pattern.
export function __setProxyForTest(
  proxy: ReturnType<typeof createTauRPCProxy> | null,
): void {
  cachedClient = proxy;
}

function appErrorMessage(err: unknown): string {
  if (
    err !== null &&
    typeof err === "object" &&
    "kind" in err &&
    typeof (err as { kind: unknown }).kind === "string"
  ) {
    const e = err as AppError;
    switch (e.kind) {
      case "validation":
        return e.reason;
      case "not_found":
        return `not found: ${e.resource}`;
      case "internal":
      case "plugin":
      case "storage":
      case "ingest":
        return e.message;
    }
  }
  return "export failed";
}

function formatRange(payload: ExportPreviewPayload): string | null {
  const { date_range_start_unix_nano: start, date_range_end_unix_nano: end } =
    payload;
  if (start === null || end === null) {
    return null;
  }
  const startIso = new Date(start / 1_000_000).toISOString().slice(0, 10);
  const endIso = new Date(end / 1_000_000).toISOString().slice(0, 10);
  return startIso === endIso ? startIso : `${startIso} – ${endIso}`;
}

export function ExportForTraining() {
  const triggerRef = useRef<HTMLButtonElement>(null);
  const [previewOpen, setPreviewOpen] = useState(false);
  const [preview, setPreview] = useState<ExportPreviewPayload | null>(null);
  const [loadingPreview, setLoadingPreview] = useState(false);
  const [confirming, setConfirming] = useState(false);
  const [statusMessage, setStatusMessage] = useState("");
  const [errorMessage, setErrorMessage] = useState("");

  const onOpenPreview = () => {
    setLoadingPreview(true);
    setErrorMessage("");
    setStatusMessage("");
    void getClient()
      .storage.export_for_training(null, false)
      .then((payload) => {
        setPreview(payload);
        setLoadingPreview(false);
        setPreviewOpen(true);
      })
      .catch((err: unknown) => {
        setLoadingPreview(false);
        setErrorMessage(appErrorMessage(err));
      });
  };

  const onConfirmExport = () => {
    setConfirming(true);
    setErrorMessage("");
    void getClient()
      .storage.export_for_training(null, true)
      .then((payload) => {
        setConfirming(false);
        setPreviewOpen(false);
        const where = payload.written_path_basename ?? "the export file";
        setStatusMessage(
          `Exported ${payload.total_records} record(s) to ${where}`,
        );
      })
      .catch((err: unknown) => {
        setConfirming(false);
        setErrorMessage(appErrorMessage(err));
      });
  };

  const onClosePreview = () => {
    setPreviewOpen(false);
  };

  const range = preview ? formatRange(preview) : null;

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-sm)",
      }}
    >
      <Button
        ref={triggerRef}
        type="button"
        onPress={onOpenPreview}
        isDisabled={loadingPreview}
        data-testid="export-training-trigger"
        style={{
          alignSelf: "flex-start",
          background: "var(--color-raised-1)",
          color: "var(--color-text-primary)",
          border: "1px solid rgba(74, 144, 226, 0.3)",
          borderRadius: "var(--radius-sm)",
          padding: "var(--spacing-sm) var(--spacing-md)",
          fontFamily: "var(--font-body)",
          fontSize: "14px",
          cursor: loadingPreview ? "not-allowed" : "pointer",
        }}
      >
        {loadingPreview
          ? "Preparing preview…"
          : "Export for community training"}
      </Button>
      <p
        style={{
          fontSize: "11px",
          // text-secondary, not tertiary: 11px body text needs 4.5:1
          // (chunk #99 axe finding: 3.11:1 on raised-3)
          color: "var(--color-text-secondary)",
          margin: 0,
        }}
      >
        Produces an anonymized JSONL dump of the incident corpus. PII is
        scrubbed; nothing is submitted automatically — you share the file.
      </p>
      <div
        role="status"
        aria-live="polite"
        data-testid="export-training-status"
        style={{
          fontSize: "12px",
          color: "var(--color-text-secondary)",
          minHeight: "1.5em",
        }}
      >
        {statusMessage}
      </div>
      {errorMessage && !previewOpen ? (
        <div
          role="alert"
          data-testid="export-training-error"
          style={{
            fontSize: "12px",
            color: "var(--color-text-primary)",
            border: "1px solid rgba(199, 85, 106, 0.5)",
            borderRadius: "var(--radius-sm)",
            padding: "var(--spacing-sm)",
          }}
        >
          {errorMessage}
        </div>
      ) : null}

      <Modal
        open={previewOpen}
        onClose={onClosePreview}
        triggerRef={triggerRef}
        titleId="export-training-modal-title"
        title="Export for community training"
        busy={confirming}
        liveRegionLevel="polite"
        liveMessage={confirming ? "Exporting…" : ""}
      >
        {preview ? (
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              gap: "var(--spacing-md)",
              paddingTop: "var(--spacing-sm)",
            }}
          >
            <p
              style={{
                fontSize: "14px",
                color: "var(--color-text-primary)",
                margin: 0,
              }}
            >
              <span
                style={{
                  fontFamily: "var(--font-code)",
                  fontVariantNumeric: "tabular-nums",
                }}
              >
                {preview.total_records}
              </span>{" "}
              incident record(s) will be exported.
            </p>

            <div
              data-testid="export-anonymization-confirmed"
              style={{
                display: "flex",
                alignItems: "center",
                gap: "var(--spacing-xs)",
              }}
            >
              <span aria-hidden="true" style={{ color: "var(--color-feedback-success)" }}>
                ✓
              </span>
              <span
                style={{
                  fontSize: "12px",
                  // state word stays in text-primary: feedback-cyan as 12px
                  // TEXT measured 4.36:1 on raised-3 (chunk #99 axe finding);
                  // the cyan glyph above conveys the state hue (non-text)
                  color: "var(--color-text-primary)",
                }}
              >
                Anonymized (PII scrubbed)
              </span>
            </div>

            {range ? (
              <p
                style={{
                  fontSize: "12px",
                  color: "var(--color-text-secondary)",
                  margin: 0,
                }}
              >
                Date range:{" "}
                <span
                  style={{
                    fontFamily: "var(--font-code)",
                    fontVariantNumeric: "tabular-nums",
                  }}
                >
                  {range}
                </span>
              </p>
            ) : null}

            <div
              data-testid="export-category-breakdown"
              style={{
                display: "flex",
                flexDirection: "column",
                gap: "var(--spacing-xs)",
              }}
            >
              {preview.categories.map((c) => (
                <div
                  key={`${c.dimension}:${c.label}`}
                  style={{
                    display: "flex",
                    justifyContent: "space-between",
                    gap: "var(--spacing-sm)",
                  }}
                >
                  <span
                    style={{
                      fontSize: "12px",
                      color: "var(--color-text-secondary)",
                    }}
                  >
                    {c.dimension} / {c.label}
                  </span>
                  <span
                    style={{
                      fontFamily: "var(--font-code)",
                      fontSize: "12px",
                      fontVariantNumeric: "tabular-nums",
                      color: "var(--color-text-primary)",
                    }}
                  >
                    {c.count}
                  </span>
                </div>
              ))}
            </div>

            {errorMessage ? (
              <div
                role="alert"
                data-testid="export-training-modal-error"
                style={{
                  fontSize: "12px",
                  color: "var(--color-text-primary)",
                  border: "1px solid rgba(199, 85, 106, 0.5)",
                  borderRadius: "var(--radius-sm)",
                  padding: "var(--spacing-sm)",
                }}
              >
                {errorMessage}
              </div>
            ) : null}

            <div
              style={{
                display: "flex",
                justifyContent: "flex-end",
                gap: "var(--spacing-sm)",
              }}
            >
              <Button
                type="button"
                onPress={onClosePreview}
                isDisabled={confirming}
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
                type="button"
                onPress={onConfirmExport}
                isDisabled={confirming}
                data-testid="export-training-confirm"
                style={{
                  background: "var(--color-primary)",
                  color: "var(--color-base)",
                  border: "1px solid var(--color-primary)",
                  borderRadius: "var(--radius-sm)",
                  padding: "var(--spacing-sm) var(--spacing-md)",
                  fontFamily: "var(--font-body)",
                  fontSize: "14px",
                  fontWeight: 500,
                  cursor: confirming ? "not-allowed" : "pointer",
                }}
              >
                {confirming ? "Exporting…" : "Confirm export"}
              </Button>
            </div>
          </div>
        ) : null}
      </Modal>
    </div>
  );
}
