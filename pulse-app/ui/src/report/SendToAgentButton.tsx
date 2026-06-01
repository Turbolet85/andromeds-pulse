// "Send to agent" button (chunk #94) — the third equal-tier output channel
// alongside the in-app Diagnostic Report and the clipboard markdown action.
// Conditionally rendered in the Report toolbar: it returns null (removed from
// the DOM + accessibility tree, NOT aria-hidden) unless MCP is both
// configured (Settings.mcp_server_enabled) AND an agent is connected
// (mcp.status().connected_agent.is_some()). Secondary-button token set —
// distinct from the primary Earth-Blue Copy-markdown fill (one primary per
// toolbar). Send result conveyed with text + icon, never color alone
// (a11y SC 1.4.1).
//
// Token discipline (design.4): every `style` value resolves through
// `var(--*)` Tailwind-v4 @theme custom properties; no inline hex literals
// beyond the structural border rgba already used across the report toolbar.

import type { CSSProperties } from "react";

export type SendState = "idle" | "sending" | "sent" | "error";

export interface SendToAgentButtonProps {
  /** MCP configured (Settings.mcp_server_enabled) AND an agent connected. */
  visible: boolean;
  sendState: SendState;
  onSend: () => void;
}

const SEND_BUTTON_STYLE: CSSProperties = {
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  gap: "var(--spacing-xs)",
  padding: "var(--spacing-xs) var(--spacing-sm)",
  minWidth: "var(--target-input-min)",
  minHeight: "var(--target-input-min)",
  background: "var(--color-raised-1)",
  color: "var(--color-text-primary)",
  border: "1px solid rgba(74, 144, 226, 0.3)",
  borderRadius: "var(--radius-sm)",
  fontFamily: "var(--font-body)",
  fontSize: "14px",
  fontWeight: 600,
  cursor: "pointer",
  appearance: "none",
  transitionProperty: "border-color",
  transitionDuration: "var(--duration-fast)",
  transitionTimingFunction: "var(--easing-out)",
};

export function SendToAgentButton({
  visible,
  sendState,
  onSend,
}: SendToAgentButtonProps) {
  if (!visible) {
    return null;
  }
  return (
    <button
      type="button"
      onClick={onSend}
      disabled={sendState === "sending"}
      aria-busy={sendState === "sending"}
      data-testid="report-send-to-agent"
      data-send-state={sendState}
      className="motion-reduce:transition-none motion-reduce:duration-0"
      style={{
        ...SEND_BUTTON_STYLE,
        cursor: sendState === "sending" ? "wait" : "pointer",
        opacity: sendState === "sending" ? 0.5 : 1,
      }}
    >
      {sendState === "sent" ? (
        <span aria-hidden="true" style={{ fontSize: "16px", lineHeight: 1 }}>
          ✓
        </span>
      ) : null}
      {sendButtonLabel(sendState)}
    </button>
  );
}

function sendButtonLabel(state: SendState): string {
  switch (state) {
    case "idle":
      return "Send to agent";
    case "sending":
      return "Sending…";
    case "sent":
      return "Sent to agent";
    case "error":
      return "Send failed — retry";
  }
}
