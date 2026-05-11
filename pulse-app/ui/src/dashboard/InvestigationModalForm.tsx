import { useEffect, useState, type RefObject } from "react";
import { useReducedMotion } from "motion/react";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { Modal } from "../components/Modal";
import { PresetPromptList } from "../components/PresetPromptList";
import { Icon } from "../components/icons";
import {
  createTauRPCProxy,
  type AppError,
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
  const reducedMotion = useReducedMotion();

  useEffect(() => {
    if (!open) {
      setPhase("idle");
      setStatusMessage("");
      setErrorMessage("");
      setResult(null);
      return;
    }
    setPhase("capturing");
    setStatusMessage("Investigation snapshot capturing");
    setErrorMessage("");
    setResult(null);

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

  const handlePresetPick = async (prompt: PresetPrompt) => {
    try {
      await writeText(prompt.template);
      setStatusMessage(`Prompt '${prompt.label}' copied to clipboard`);
    } catch {
      setStatusMessage(`Failed to copy '${prompt.label}' to clipboard`);
    }
  };

  const busy = phase === "capturing";
  const liveLevel: "polite" | "assertive" =
    phase === "error" ? "assertive" : "polite";

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
              onPick={handlePresetPick}
            />
          </>
        ) : null}
      </div>
    </Modal>
  );
}
