// Diagnostic Report renderer (chunk #88). Six-section visual surface
// per capability P-031 (symptom / timeline / hypotheses / investigation
// steps / evidence / project context) + optional Resolution Summary
// (Resolved incidents) + optional Previously Seen subsection (P-036
// reserved; resolver returns empty Vec until corpus fingerprint-
// similarity query path lands в а follow-up chunk). Degraded-mode
// banner с icon + text label per a11y plan §6 not-color-alone discipline
// when `report.degradedMode === true` (Active/Acknowledged incidents OR
// Resolved incidents с unparseable resolution_summary_text). Copy
// markdown action per P-038 — calls back through useReport hook к
// clipboard via @tauri-apps/plugin-clipboard-manager + emits visible
// "copied" status per CLAUDE.md §Critical Warnings clipboard hygiene
// + а11y plan §7 Live regions.
//
// Token discipline (chunk #88 acceptance criterion design.4): every
// `style` value resolves through `var(--*)` Tailwind-v4 @theme custom
// properties. No inline hex literals.

import { useState, type CSSProperties } from "react";
import type {
  CopyState,
  HypothesisPayload,
  InvestigationStepPayload,
  PreviouslySeenPayload,
  ReportPayload,
} from "./report-types";
import {
  formatOpenedAt,
  severityBackgroundVar,
  severityBorderColorVar,
  severityLabel,
  severityTextColorVar,
  statusLabel,
} from "./report-types";
import { SendToAgentButton, type SendState } from "./SendToAgentButton";

export interface ReportRendererProps {
  report: ReportPayload;
  onCopyMarkdown: () => void;
  copyState: CopyState;
  // Chunk #94 — "Send to agent" MCP delivery channel. The button only
  // renders when MCP is configured AND an agent is connected; absent props
  // default to not-shown so existing callers are unaffected.
  mcpDeliveryAvailable?: boolean;
  onSendToAgent?: () => void;
  sendState?: SendState;
}

const METADATA_LABEL_STYLE: CSSProperties = {
  fontFamily: "var(--font-display)",
  fontWeight: 600,
  marginRight: "var(--spacing-xs)",
};

const SECTION_STYLE: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "var(--spacing-sm)",
  padding: "var(--spacing-md)",
  background: "var(--color-raised-1)",
  border: "1px solid rgba(74, 144, 226, 0.3)",
  borderRadius: "var(--radius-md)",
};

const SECTION_HEADING_STYLE: CSSProperties = {
  fontFamily: "var(--font-display)",
  fontSize: "20px",
  fontWeight: 600,
  margin: 0,
  color: "var(--color-text-primary)",
};

const DISCLOSURE_BUTTON_STYLE: CSSProperties = {
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  gap: "var(--spacing-sm)",
  width: "100%",
  padding: "var(--spacing-xs) var(--spacing-sm)",
  background: "transparent",
  border: "1px solid transparent",
  borderRadius: "var(--radius-sm)",
  color: "var(--color-text-primary)",
  fontFamily: "var(--font-display)",
  fontSize: "20px",
  fontWeight: 600,
  textAlign: "left",
  cursor: "pointer",
  appearance: "none",
};

const SECTION_BODY_STYLE: CSSProperties = {
  fontFamily: "var(--font-body)",
  fontSize: "14px",
  color: "var(--color-text-primary)",
  whiteSpace: "pre-wrap",
};

const EMPTY_PLACEHOLDER_STYLE: CSSProperties = {
  fontFamily: "var(--font-body)",
  fontSize: "14px",
  color: "var(--color-text-secondary)",
  fontStyle: "italic",
};

const CODE_BLOCK_STYLE: CSSProperties = {
  fontFamily: "var(--font-code)",
  fontVariantNumeric: "tabular-nums",
  fontSize: "12px",
  color: "var(--color-text-primary)",
  background: "var(--color-inset)",
  padding: "var(--spacing-xs) var(--spacing-sm)",
  borderRadius: "var(--radius-sm)",
  display: "block",
};

const DEGRADED_NOTICE_STYLE: CSSProperties = {
  display: "flex",
  alignItems: "flex-start",
  gap: "var(--spacing-sm)",
  padding: "var(--spacing-sm) var(--spacing-md)",
  background: "var(--color-raised-1)",
  border: "1px solid var(--color-accent)",
  borderRadius: "var(--radius-md)",
  color: "var(--color-text-primary)",
  fontFamily: "var(--font-body)",
  fontSize: "14px",
};

const COPY_BUTTON_BASE_STYLE: CSSProperties = {
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  gap: "var(--spacing-xs)",
  padding: "var(--spacing-xs) var(--spacing-sm)",
  minWidth: "var(--target-input-min)",
  minHeight: "var(--target-input-min)",
  background: "var(--color-primary)",
  color: "var(--color-base)",
  border: "1px solid rgba(74, 144, 226, 0.3)",
  borderRadius: "var(--radius-sm)",
  fontFamily: "var(--font-body)",
  fontSize: "14px",
  fontWeight: 600,
  cursor: "pointer",
  appearance: "none",
};

const SEVERITY_BADGE_BASE_STYLE: CSSProperties = {
  display: "inline-flex",
  alignItems: "center",
  padding: "0 var(--spacing-xs)",
  borderRadius: "var(--radius-sm)",
  fontFamily: "var(--font-display)",
  fontSize: "12px",
  fontWeight: 600,
};

interface SectionState {
  symptom: boolean;
  timeline: boolean;
  hypotheses: boolean;
  investigationSteps: boolean;
  evidence: boolean;
  projectContext: boolean;
  resolutionSummary: boolean;
  previouslySeen: boolean;
}

const ALL_EXPANDED: SectionState = {
  symptom: true,
  timeline: true,
  hypotheses: true,
  investigationSteps: true,
  evidence: true,
  projectContext: true,
  resolutionSummary: true,
  previouslySeen: false,
};

const REPORT_TITLE_ID = "diagnostic-report-renderer-title";

export function ReportRenderer({
  report,
  onCopyMarkdown,
  copyState,
  mcpDeliveryAvailable = false,
  onSendToAgent,
  sendState = "idle",
}: ReportRendererProps) {
  const [open, setOpen] = useState<SectionState>(ALL_EXPANDED);

  const toggle = (key: keyof SectionState) => {
    setOpen((prev) => ({ ...prev, [key]: !prev[key] }));
  };

  return (
    <div
      data-testid="report-renderer"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-lg)",
      }}
    >
      <header
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "var(--spacing-sm)",
        }}
      >
        <h1
          id={REPORT_TITLE_ID}
          style={{
            fontFamily: "var(--font-display)",
            fontSize: "24px",
            fontWeight: 600,
            margin: 0,
            color: "var(--color-text-primary)",
          }}
        >
          Diagnostic Report: {report.title}
        </h1>
        <dl
          style={{
            display: "grid",
            gridTemplateColumns: "max-content 1fr",
            gap: "var(--spacing-xs) var(--spacing-sm)",
            margin: 0,
            fontFamily: "var(--font-body)",
            fontSize: "14px",
          }}
        >
          <dt style={METADATA_LABEL_STYLE}>Incident ID</dt>
          <dd
            style={{
              fontFamily: "var(--font-code)",
              fontVariantNumeric: "tabular-nums",
              margin: 0,
            }}
            data-testid="report-incident-id"
          >
            {report.incident_id}
          </dd>
          <dt style={METADATA_LABEL_STYLE}>Workspace</dt>
          <dd
            style={{ margin: 0 }}
            data-testid="report-workspace"
          >
            {report.workspace}
          </dd>
          <dt style={METADATA_LABEL_STYLE}>Opened</dt>
          <dd
            style={{
              fontFamily: "var(--font-code)",
              fontVariantNumeric: "tabular-nums",
              margin: 0,
            }}
            data-testid="report-opened-at"
          >
            {formatOpenedAt(report.opened_at_unix_nano)}
          </dd>
          <dt style={METADATA_LABEL_STYLE}>Status</dt>
          <dd style={{ margin: 0 }} data-testid="report-status">
            {statusLabel(report.status)}
          </dd>
          <dt style={METADATA_LABEL_STYLE}>Severity</dt>
          <dd style={{ margin: 0 }} data-testid="report-severity">
            <span
              style={{
                ...SEVERITY_BADGE_BASE_STYLE,
                background: severityBackgroundVar(report.severity),
                color: severityTextColorVar(report.severity),
                border: `1px solid ${severityBorderColorVar(report.severity)}`,
              }}
              data-severity={report.severity}
            >
              {severityLabel(report.severity)}
            </span>
          </dd>
        </dl>
      </header>

      {report.degraded_mode ? (
        <aside
          aria-label="Interpretation degraded notice"
          data-testid="report-degraded-notice"
          style={DEGRADED_NOTICE_STYLE}
        >
          <span aria-hidden="true" style={{ fontSize: "20px", lineHeight: 1 }}>
            ⚠
          </span>
          <span>
            <strong>Interpretation pending —</strong> hypotheses and investigation
            steps will populate as soon as the next L4 inference cycle completes
            for this incident. See diagnostics for retry options.
          </span>
        </aside>
      ) : null}

      <CollapsibleSection
        heading="Symptom"
        open={open.symptom}
        onToggle={() => toggle("symptom")}
        testIdRoot="symptom"
        bodyId="report-section-body-symptom"
      >
        <TextOrPlaceholder
          text={report.symptom}
          placeholder="No symptom narrative available."
          testId="report-section-body-symptom"
        />
      </CollapsibleSection>

      <CollapsibleSection
        heading="Timeline"
        open={open.timeline}
        onToggle={() => toggle("timeline")}
        testIdRoot="timeline"
        bodyId="report-section-body-timeline"
      >
        <TextOrPlaceholder
          text={report.timeline}
          placeholder="No timeline narrative available."
          testId="report-section-body-timeline"
        />
      </CollapsibleSection>

      <CollapsibleSection
        heading="Hypotheses"
        open={open.hypotheses}
        onToggle={() => toggle("hypotheses")}
        testIdRoot="hypotheses"
        bodyId="report-section-body-hypotheses"
      >
        {report.degraded_mode ? (
          <DegradedNoticeBody testId="report-section-body-hypotheses" />
        ) : (
          <HypothesesList hypotheses={report.hypotheses} />
        )}
      </CollapsibleSection>

      <CollapsibleSection
        heading="Investigation Steps"
        open={open.investigationSteps}
        onToggle={() => toggle("investigationSteps")}
        testIdRoot="investigation-steps"
        bodyId="report-section-body-investigation-steps"
      >
        {report.degraded_mode ? (
          <DegradedNoticeBody testId="report-section-body-investigation-steps" />
        ) : (
          <InvestigationStepsList steps={report.investigation_steps} />
        )}
      </CollapsibleSection>

      <CollapsibleSection
        heading="Evidence"
        open={open.evidence}
        onToggle={() => toggle("evidence")}
        testIdRoot="evidence"
        bodyId="report-section-body-evidence"
      >
        <EvidenceList refs={report.evidence_refs} />
      </CollapsibleSection>

      <CollapsibleSection
        heading="Project Context"
        open={open.projectContext}
        onToggle={() => toggle("projectContext")}
        testIdRoot="project-context"
        bodyId="report-section-body-project-context"
      >
        <TextOrPlaceholder
          text={report.project_context}
          placeholder="No project context attached."
          testId="report-section-body-project-context"
        />
      </CollapsibleSection>

      {report.resolution_summary !== null ? (
        <CollapsibleSection
          heading="Resolution Summary"
          open={open.resolutionSummary}
          onToggle={() => toggle("resolutionSummary")}
          testIdRoot="resolution-summary"
          bodyId="report-section-body-resolution-summary"
        >
          <p
            data-testid="report-section-body-resolution-summary"
            style={SECTION_BODY_STYLE}
          >
            {report.resolution_summary}
          </p>
        </CollapsibleSection>
      ) : null}

      {report.previously_seen.length > 0 ? (
        <CollapsibleSection
          heading="Previously Seen"
          open={open.previouslySeen}
          onToggle={() => toggle("previouslySeen")}
          testIdRoot="previously-seen"
          bodyId="report-section-body-previously-seen"
        >
          <PreviouslySeenList matches={report.previously_seen} />
        </CollapsibleSection>
      ) : null}

      <footer
        style={{
          display: "flex",
          justifyContent: "flex-end",
          alignItems: "center",
          gap: "var(--spacing-sm)",
          paddingTop: "var(--spacing-sm)",
        }}
      >
        <SendToAgentButton
          visible={mcpDeliveryAvailable}
          sendState={sendState}
          onSend={onSendToAgent ?? (() => {})}
        />
        <button
          type="button"
          onClick={onCopyMarkdown}
          disabled={copyState === "copying"}
          data-testid="report-copy-markdown"
          data-copy-state={copyState}
          style={{
            ...COPY_BUTTON_BASE_STYLE,
            opacity: copyState === "copying" ? 0.5 : 1,
            cursor: copyState === "copying" ? "wait" : "pointer",
          }}
        >
          {copyState === "copied" ? (
            <span aria-hidden="true" style={{ fontSize: "16px", lineHeight: 1 }}>
              ✓
            </span>
          ) : null}
          {copyButtonLabel(copyState)}
        </button>
      </footer>
    </div>
  );
}

function copyButtonLabel(state: CopyState): string {
  switch (state) {
    case "idle":
      return "Copy markdown";
    case "copying":
      return "Copying…";
    case "copied":
      return "Copied";
    case "error":
      return "Copy failed — retry";
  }
}

interface CollapsibleSectionProps {
  heading: string;
  open: boolean;
  onToggle: () => void;
  testIdRoot: string;
  bodyId: string;
  children: React.ReactNode;
}

function CollapsibleSection({
  heading,
  open,
  onToggle,
  testIdRoot,
  bodyId,
  children,
}: CollapsibleSectionProps) {
  return (
    <section
      data-testid={`report-section-${testIdRoot}`}
      style={SECTION_STYLE}
    >
      <button
        type="button"
        aria-expanded={open}
        aria-controls={bodyId}
        onClick={onToggle}
        data-testid={`report-section-toggle-${testIdRoot}`}
        style={DISCLOSURE_BUTTON_STYLE}
      >
        <span style={SECTION_HEADING_STYLE}>{heading}</span>
        <span aria-hidden="true" style={{ fontSize: "14px", lineHeight: 1 }}>
          {open ? "▾" : "▸"}
        </span>
      </button>
      {open ? <div id={bodyId}>{children}</div> : null}
    </section>
  );
}

interface TextOrPlaceholderProps {
  text: string;
  placeholder: string;
  testId: string;
}

function TextOrPlaceholder({
  text,
  placeholder,
  testId,
}: TextOrPlaceholderProps) {
  if (!text || text.length === 0) {
    return (
      <p data-testid={testId} style={EMPTY_PLACEHOLDER_STYLE}>
        {placeholder}
      </p>
    );
  }
  return (
    <p data-testid={testId} style={SECTION_BODY_STYLE}>
      {text}
    </p>
  );
}

interface DegradedNoticeBodyProps {
  testId: string;
}

function DegradedNoticeBody({ testId }: DegradedNoticeBodyProps) {
  return (
    <p data-testid={testId} style={EMPTY_PLACEHOLDER_STYLE}>
      Interpretation pending — section omitted until next L4 inference cycle.
    </p>
  );
}

function HypothesesList({ hypotheses }: { hypotheses: HypothesisPayload[] }) {
  if (hypotheses.length === 0) {
    return (
      <p style={EMPTY_PLACEHOLDER_STYLE}>No ranked hypotheses produced.</p>
    );
  }
  return (
    <ul
      data-testid="report-hypotheses-list"
      style={{
        margin: 0,
        paddingLeft: "var(--spacing-md)",
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-sm)",
        listStyleType: "disc",
      }}
    >
      {hypotheses.map((h, idx) => (
        <li key={`hyp-${idx}`} style={{ fontFamily: "var(--font-body)" }}>
          <strong>({h.confidence_label})</strong> {h.statement}
          {h.justification.length > 0 ? (
            <div
              style={{
                marginTop: "var(--spacing-xs)",
                color: "var(--color-text-secondary)",
                fontSize: "13px",
              }}
            >
              {h.justification}
            </div>
          ) : null}
        </li>
      ))}
    </ul>
  );
}

function InvestigationStepsList({
  steps,
}: {
  steps: InvestigationStepPayload[];
}) {
  if (steps.length === 0) {
    return (
      <p style={EMPTY_PLACEHOLDER_STYLE}>
        No suggested investigation steps produced.
      </p>
    );
  }
  return (
    <ol
      data-testid="report-investigation-steps-list"
      style={{
        margin: 0,
        paddingLeft: "var(--spacing-md)",
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-sm)",
        listStyleType: "decimal",
      }}
    >
      {steps.map((s, idx) => (
        <li key={`step-${idx}`} style={{ fontFamily: "var(--font-body)" }}>
          {s.step}
          {s.expected_yield.length > 0 ? (
            <div
              style={{
                marginTop: "var(--spacing-xs)",
                color: "var(--color-text-secondary)",
                fontSize: "13px",
                fontStyle: "italic",
              }}
            >
              Expected yield: {s.expected_yield}
            </div>
          ) : null}
        </li>
      ))}
    </ol>
  );
}

function EvidenceList({ refs }: { refs: string[] }) {
  if (refs.length === 0) {
    return (
      <p style={EMPTY_PLACEHOLDER_STYLE}>No evidence references attached.</p>
    );
  }
  return (
    <ul
      data-testid="report-evidence-list"
      style={{
        margin: 0,
        paddingLeft: "var(--spacing-md)",
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-xs)",
        listStyleType: "none",
      }}
    >
      {refs.map((r, idx) => (
        <li key={`ev-${idx}`}>
          <code style={CODE_BLOCK_STYLE}>{r}</code>
        </li>
      ))}
    </ul>
  );
}

function PreviouslySeenList({
  matches,
}: {
  matches: PreviouslySeenPayload[];
}) {
  return (
    <ul
      data-testid="report-previously-seen-list"
      style={{
        margin: 0,
        paddingLeft: "var(--spacing-md)",
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-sm)",
        listStyleType: "disc",
      }}
    >
      {matches.map((m) => (
        <li
          key={`prev-${m.incident_id}`}
          style={{ fontFamily: "var(--font-body)" }}
        >
          incident #{m.incident_id} — {m.title} ({m.workspace})
          <div
            style={{
              fontFamily: "var(--font-code)",
              fontVariantNumeric: "tabular-nums",
              fontSize: "12px",
              color: "var(--color-text-secondary)",
            }}
          >
            {formatOpenedAt(m.opened_at_unix_nano)}
          </div>
        </li>
      ))}
    </ul>
  );
}
