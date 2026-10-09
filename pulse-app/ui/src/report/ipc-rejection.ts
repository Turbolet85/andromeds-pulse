// Capability-rejected IPC classifier + fire-and-forget reporter. A denied
// webview invoke surfaces ONLY here — Tauri's runtime rejects it back to the
// webview with no Rust-side log, event, or hook — so this is the one place
// the security-plan "log capability-rejected IPC calls" record can originate.
// Exported as a shared helper so future ACL-rejectable catch sites (plugin +
// core:window calls; TauRPC procedures ride one core:default-admitted handler
// and are not in the class) can join cheaply.

import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { createTauRPCProxy } from "../bindings/index";
import { sanitizeWindowLabel, type WindowLabel } from "../hooks/use-window-label";

export type IpcRejectionCategory = "acl_rejected" | "other";

const PAYLOAD_BYTES_MAX = 100_000_000;

// Tauri 2.11 rejects a capability-denied invoke with a message containing
// "not allowed" — release builds: `Command {cmd} not allowed by ACL`
// (tauri src/webview/mod.rs); debug builds: the resolve_access_message text
// (tauri src/ipc/authority.rs). The substring is an implementation detail,
// not a contract: a non-matching rejection degrades to "other" rather than
// misclassifying, and a Tauri rewording fails the named classifier pins
// instead of silently dropping the class.
export function classifyIpcRejection(err: unknown): IpcRejectionCategory {
  const text = err instanceof Error ? err.message : String(err ?? "");
  return text.includes("not allowed") ? "acl_rejected" : "other";
}

function currentWindowLabel(): WindowLabel {
  try {
    return sanitizeWindowLabel(getCurrentWebviewWindow().label);
  } catch {
    return "unknown";
  }
}

export function clampPayloadBytes(value: number): number {
  if (!Number.isFinite(value) || value < 0) {
    return 0;
  }
  return Math.min(Math.floor(value), PAYLOAD_BYTES_MAX);
}

export async function reportIpcRejection(
  category: IpcRejectionCategory,
  payloadBytes: number,
): Promise<void> {
  try {
    const proxyRoot = (await createTauRPCProxy()) as unknown as {
      telemetry?: {
        frontend?: {
          record_ipc_rejection?: (input: {
            error_category: IpcRejectionCategory;
            window_label: string;
            payload_bytes: number;
          }) => Promise<void>;
        };
      };
    };
    const resolver = proxyRoot.telemetry?.frontend?.record_ipc_rejection;
    if (typeof resolver !== "function") {
      return;
    }
    await resolver({
      error_category: category,
      window_label: currentWindowLabel(),
      payload_bytes: clampPayloadBytes(payloadBytes),
    });
  } catch {
    // Fire-and-forget by design: a failed report of a failure must never
    // throw into the caller's control flow or recurse into itself.
  }
}
