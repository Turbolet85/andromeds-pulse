import { useEffect, useState, type RefObject } from "react";
import { useReducedMotion } from "motion/react";
import { Modal } from "../components/Modal";
import {
  createTauRPCProxy,
  type AppError,
  type SnapshotPreset,
} from "../bindings";

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
  const reducedMotion = useReducedMotion();

  useEffect(() => {
    if (!open) {
      setPhase("idle");
      setStatusMessage("");
      setErrorMessage("");
      return;
    }
    setPhase("capturing");
    setStatusMessage("Investigation snapshot capturing");
    setErrorMessage("");

    let cancelled = false;
    let timer: ReturnType<typeof setTimeout> | null = null;
    const supportingDelay = reducedMotion ? 0 : SUPPORTING_MOMENT_MS;

    const start = async () => {
      try {
        await getClient().snapshot.generate(preset);
        if (cancelled) return;
        setPhase("result");
        setStatusMessage("Investigation snapshot ready");
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

  const busy = phase === "capturing";
  const liveLevel: "polite" | "assertive" =
    phase === "error" || phase === "result" ? "assertive" : "polite";

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
        {phase === "result" ? (
          <p
            data-testid="investigation-ready-text"
            style={{
              fontFamily: "var(--font-body)",
              fontSize: "14px",
              color: "var(--color-text-primary)",
              margin: 0,
            }}
          >
            Snapshot ready.
          </p>
        ) : null}
      </div>
    </Modal>
  );
}
