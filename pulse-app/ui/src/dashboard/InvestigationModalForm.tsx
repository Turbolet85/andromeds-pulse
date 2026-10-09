import { useEffect, useState, type CSSProperties, type RefObject } from "react";
import { useReducedMotion } from "motion/react";
import { Modal } from "../components/Modal";
import { PresetPromptList } from "../components/PresetPromptList";
import { Icon } from "../components/icons";
import {
  createTauRPCProxy,
  type AppError,
  type InvestigateResultDto,
  type SnapshotPreset,
  type SnapshotResultDto,
} from "../bindings";
import { PRESET_PROMPTS, type PresetPrompt } from "./preset-prompts";

type Phase = "idle" | "capturing" | "result" | "error";

const SUPPORTING_MOMENT_MS = 350;

let cachedClient: ReturnType<typeof createTauRPCProxy> | null = null;

function getClient(): ReturnType<typeof createTauRPCProxy> {
  if (cachedClient === null) {
    cachedClient = createTauRPCProxy();
  }
  return cachedClient;
}

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
  return { kind: "internal", message: "snapshot.generate failed" };
}

interface InvestigationModalFormProps {
  open: boolean;
  onClose: () => void;
  triggerRef: RefObject<HTMLElement | null>;
  preset?: SnapshotPreset;
}

function InvestigateResultPanel({ result }: { result: InvestigateResultDto }) {
  const sectionLabel: CSSProperties = {
    fontFamily: "var(--font-body)",
    fontSize: "12px",
    fontWeight: 600,
    color: "var(--color-text-secondary)",
    margin: 0,
  };
  const sectionBody: CSSProperties = {
    fontFamily: "var(--font-body)",
    fontSize: "14px",
    color: "var(--color-text-primary)",
    margin: 0,
  };
  const listStyle: CSSProperties = {
    ...sectionBody,
    paddingLeft: "var(--spacing-md)",
  };
  return (
    <section
      data-testid="investigation-action-result"
      aria-label="Investigation analysis result"
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "var(--spacing-sm)",
        border: "1px solid #17B3A3",
        background: "var(--color-inset)",
        borderRadius: "var(--radius-sm)",
        padding: "var(--spacing-sm)",
      }}
    >
      <h3 style={{ ...sectionBody, fontWeight: 600 }}>{result.title}</h3>
      {result.symptom !== "" ? (
        <div>
          <p style={sectionLabel}>Symptom</p>
          <p style={sectionBody}>{result.symptom}</p>
        </div>
      ) : null}
      {result.timeline !== "" ? (
        <div>
          <p style={sectionLabel}>Timeline</p>
          <p style={sectionBody}>{result.timeline}</p>
        </div>
      ) : null}
      {result.hypotheses.length > 0 ? (
        <div>
          <p style={sectionLabel}>Hypotheses</p>
          <ul style={listStyle}>
            {result.hypotheses.map((h) => (
              <li key={h.statement}>
                {h.statement}
                {h.justification !== "" ? ` — ${h.justification}` : ""}
              </li>
            ))}
          </ul>
        </div>
      ) : null}
      {result.investigation_steps.length > 0 ? (
        <div>
          <p style={sectionLabel}>Suggested steps</p>
          <ul style={listStyle}>
            {result.investigation_steps.map((s) => (
              <li key={s.step}>
                {s.step}
                {s.expected_yield !== "" ? ` — ${s.expected_yield}` : ""}
              </li>
            ))}
          </ul>
        </div>
      ) : null}
    </section>
  );
}

export function InvestigationModalForm({
  open,
  onClose,
  triggerRef,
  preset = "balanced",
}: InvestigationModalFormProps) {
  const [phase, setPhase] = useState<Phase>("idle");
  const [statusMessage, setStatusMessage] = useState<string>("");
  const [errorMessage, setErrorMessage] = useState<string>("");
  const [result, setResult] = useState<SnapshotResultDto | null>(null);
  const [actionPhase, setActionPhase] = useState<
    "idle" | "running" | "result" | "error"
  >("idle");
  const [actionResult, setActionResult] = useState<InvestigateResultDto | null>(
    null,
  );
  const [actionError, setActionError] = useState<string>("");
  const [runningActionId, setRunningActionId] = useState<string | null>(null);
  const reducedMotion = useReducedMotion();

  useEffect(() => {
    if (!open) {
      setPhase("idle");
      setStatusMessage("");
      setErrorMessage("");
      setResult(null);
      setActionPhase("idle");
      setActionResult(null);
      setActionError("");
      setRunningActionId(null);
      return;
    }
    setPhase("capturing");
    setStatusMessage("Investigation snapshot capturing");
    setErrorMessage("");
    setResult(null);
    setActionPhase("idle");
    setActionResult(null);
    setActionError("");
    setRunningActionId(null);

    let cancelled = false;
    let timer: ReturnType<typeof setTimeout> | null = null;
    const supportingDelay = reducedMotion ? 0 : SUPPORTING_MOMENT_MS;

    const start = async () => {
      try {
        const dto = await getClient().snapshot.generate(preset, null);
        if (cancelled) return;
        setResult(dto);
        setPhase("result");
        setStatusMessage(
          `Snapshot copied to clipboard (${dto.byte_count} characters)`,
        );
      } catch (rawErr: unknown) {
        if (cancelled) return;
        const sanitized = appErrorMessage(toAppError(rawErr));
        setPhase("error");
        setStatusMessage(sanitized);
        setErrorMessage(sanitized);
      }
    };

    timer = setTimeout(() => {
      if (cancelled) return;
      void start();
    }, supportingDelay);

    return () => {
      cancelled = true;
      if (timer !== null) {
        clearTimeout(timer);
      }
    };
  }, [open, preset, reducedMotion]);

  const handleActionRun = async (prompt: PresetPrompt) => {
    setRunningActionId(prompt.id);
    setActionPhase("running");
    setActionError("");
    setActionResult(null);
    setStatusMessage(`Running investigation: ${prompt.label}…`);
    try {
      const dto = await getClient().investigate.run_action(prompt.id);
      setActionResult(dto);
      setActionPhase("result");
      setStatusMessage(`${prompt.label}: analysis ready`);
    } catch (rawErr: unknown) {
      const sanitized = appErrorMessage(toAppError(rawErr));
      setActionPhase("error");
      setActionError(sanitized);
      setStatusMessage(sanitized);
    } finally {
      setRunningActionId(null);
    }
  };

  const busy = phase === "capturing" || actionPhase === "running";
  const liveLevel: "polite" | "assertive" =
    phase === "error" || actionPhase === "error" ? "assertive" : "polite";

  return (
    <Modal
      open={open}
      onClose={onClose}
      triggerRef={triggerRef}
      titleId="investigation-modal-title"
      title="Investigate"
      busy={busy}
      liveRegionLevel={liveLevel}
      liveMessage={statusMessage}
    >
      <div
        data-testid="investigation-modal-content"
        data-phase={phase}
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "var(--spacing-md)",
          paddingTop: "var(--spacing-sm)",
          minWidth: "320px",
        }}
      >
        {phase === "capturing" ? (
          <p
            data-testid="investigation-capturing-text"
            style={{
              fontFamily: "var(--font-body)",
              fontSize: "14px",
              color: "var(--color-text-secondary)",
              margin: 0,
            }}
          >
            Capturing snapshot…
          </p>
        ) : null}
        {phase === "error" && errorMessage !== "" ? (
          <div
            role="alert"
            data-testid="investigation-error"
            style={{
              border: "1px solid rgba(199, 85, 106, 0.5)",
              background: "var(--color-inset)",
              borderRadius: "var(--radius-sm)",
              padding: "var(--spacing-sm)",
              color: "var(--color-text-primary)",
              fontFamily: "var(--font-body)",
              fontSize: "14px",
            }}
          >
            {errorMessage}
          </div>
        ) : null}
        {phase === "result" && result !== null ? (
          <>
            <div
              data-testid="investigation-success-badge"
              style={{
                display: "flex",
                alignItems: "center",
                gap: "var(--spacing-xs)",
                border: "1px solid #17B3A3",
                background: "var(--color-inset)",
                borderRadius: "var(--radius-sm)",
                padding: "var(--spacing-sm)",
                color: "var(--color-text-primary)",
                fontFamily: "var(--font-body)",
                fontSize: "14px",
                fontWeight: 500,
              }}
            >
              <Icon glyph="telescope" size={20} aria-hidden="true" />
              <span>Snapshot ready</span>
            </div>
            <dl
              data-testid="investigation-file-paths"
              style={{
                display: "grid",
                gridTemplateColumns: "auto 1fr",
                columnGap: "var(--spacing-sm)",
                rowGap: "var(--spacing-xs)",
                margin: 0,
                fontFamily: "var(--font-body)",
                fontSize: "12px",
                color: "var(--color-text-secondary)",
              }}
            >
              <dt>markdown</dt>
              <dd
                style={{
                  margin: 0,
                  fontFamily: "var(--font-code)",
                  fontVariantNumeric: "tabular-nums",
                }}
              >
                {result.markdown_path_basename}
              </dd>
              <dt>json</dt>
              <dd
                style={{
                  margin: 0,
                  fontFamily: "var(--font-code)",
                  fontVariantNumeric: "tabular-nums",
                }}
              >
                {result.json_path_basename}
              </dd>
              <dt>tokens</dt>
              <dd
                style={{
                  margin: 0,
                  fontFamily: "var(--font-code)",
                  fontVariantNumeric: "tabular-nums",
                  color: "var(--color-text-primary)",
                }}
              >
                {result.token_count}
              </dd>
            </dl>
            <PresetPromptList
              prompts={PRESET_PROMPTS}
              onPick={handleActionRun}
              busyId={runningActionId}
            />
            {actionPhase === "running" ? (
              <p
                data-testid="investigation-action-running"
                style={{
                  fontFamily: "var(--font-body)",
                  fontSize: "14px",
                  color: "var(--color-text-secondary)",
                  margin: 0,
                }}
              >
                Running analysis…
              </p>
            ) : null}
            {actionPhase === "error" && actionError !== "" ? (
              <div
                role="alert"
                data-testid="investigation-action-error"
                style={{
                  border: "1px solid rgba(199, 85, 106, 0.5)",
                  background: "var(--color-inset)",
                  borderRadius: "var(--radius-sm)",
                  padding: "var(--spacing-sm)",
                  color: "var(--color-text-primary)",
                  fontFamily: "var(--font-body)",
                  fontSize: "14px",
                }}
              >
                {actionError}
              </div>
            ) : null}
            {actionPhase === "result" && actionResult !== null ? (
              <InvestigateResultPanel result={actionResult} />
            ) : null}
          </>
        ) : null}
      </div>
    </Modal>
  );
}
