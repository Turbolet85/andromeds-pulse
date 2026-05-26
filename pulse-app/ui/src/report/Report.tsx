// Diagnostic Report container (chunk #88). Composes the existing Modal
// primitive (`pulse-app/ui/src/components/Modal.tsx`, chunk #41/#44
// precedent) с the chunk #88 ReportRenderer. Fetches the report via
// useReport hook when `isOpen` flips к true + `incidentId` is non-null.
// Modal handles focus trap + Esc dismissal + focus restoration к
// `triggerRef` per a11y plan §5 + Modal primitive's built-in shape.

import { useMemo, type RefObject } from "react";
import { Modal } from "../components/Modal";
import { ReportRenderer } from "./ReportRenderer";
import { useReport } from "./use-report";
import { COPY_LIVE_REGION_MESSAGES } from "./report-types";

export interface ReportProps {
  isOpen: boolean;
  onClose: () => void;
  incidentId: number | null;
  triggerRef: RefObject<HTMLElement | null>;
}

export function Report({ isOpen, onClose, incidentId, triggerRef }: ReportProps) {
  const { report, loading, error, copyState, copyMarkdown } = useReport(
    isOpen ? incidentId : null,
  );

  const liveMessage = useMemo(() => {
    return COPY_LIVE_REGION_MESSAGES[copyState];
  }, [copyState]);

  const title = useMemo(() => {
    if (loading) {
      return "Loading diagnostic report…";
    }
    if (error !== null) {
      return "Diagnostic report unavailable";
    }
    if (report !== null) {
      return `Diagnostic Report: ${report.title}`;
    }
    return "Diagnostic Report";
  }, [loading, error, report]);

  const handleCopy = () => {
    void copyMarkdown();
  };

  return (
    <Modal
      open={isOpen}
      onClose={onClose}
      triggerRef={triggerRef}
      titleId="diagnostic-report-title"
      title={title}
      busy={loading}
      liveRegionLevel="polite"
      liveMessage={liveMessage}
    >
      {loading ? <LoadingSkeleton /> : null}
      {error !== null ? <ErrorState message={error} /> : null}
      {report !== null && !loading ? (
        <ReportRenderer
          report={report}
          onCopyMarkdown={handleCopy}
          copyState={copyState}
        />
      ) : null}
    </Modal>
  );
}

function LoadingSkeleton() {
  return (
    <div
      data-testid="report-loading"
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        padding: "var(--spacing-lg)",
        color: "var(--color-text-secondary)",
        fontFamily: "var(--font-body)",
        fontSize: "14px",
        fontStyle: "italic",
      }}
    >
      Loading diagnostic report…
    </div>
  );
}

function ErrorState({ message }: { message: string }) {
  return (
    <div
      data-testid="report-error"
      role="alert"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-sm)",
        padding: "var(--spacing-md)",
        background: "var(--color-raised-1)",
        border: "1px solid var(--color-accent)",
        borderRadius: "var(--radius-md)",
        color: "var(--color-accent)",
        fontFamily: "var(--font-body)",
        fontSize: "14px",
      }}
    >
      <strong>Could not load diagnostic report.</strong>
      <span style={{ color: "var(--color-text-secondary)" }}>{message}</span>
    </div>
  );
}
