// WebGPU adapter-request outcome reporter. Each request's result crosses to the
// backend as one closed outcome + a sanitized window label through
// `telemetry.frontend.record_webgpu_adapter`, so a run that drew no frame can
// name why from the app's own log. Only the closed outcome crosses — never a
// browser error string (security plan §Anti-Patterns Logging row 4).

import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { createTauRPCProxy } from "../bindings/index";
import { sanitizeWindowLabel, type WindowLabel } from "../hooks/use-window-label";
import type { AdapterUnavailableReason } from "./webgpu-adapter";

export type WebgpuAdapterOutcome =
  | "obtained"
  | "no_navigator_gpu"
  | "adapter_null"
  | "adapter_request_rejected"
  | "device_request_failed";

const OUTCOME_BY_REASON: Record<AdapterUnavailableReason, WebgpuAdapterOutcome> = {
  "navigator.gpu undefined": "no_navigator_gpu",
  "requestAdapter returned null": "adapter_null",
  "requestAdapter rejected": "adapter_request_rejected",
  "requestDevice failed": "device_request_failed",
};

export function outcomeForReason(reason: AdapterUnavailableReason): WebgpuAdapterOutcome {
  return OUTCOME_BY_REASON[reason];
}

export function currentWindowLabel(): WindowLabel {
  try {
    return sanitizeWindowLabel(getCurrentWebviewWindow().label);
  } catch {
    return "unknown";
  }
}

export async function reportAdapterOutcome(outcome: WebgpuAdapterOutcome): Promise<void> {
  try {
    const proxyRoot = (await createTauRPCProxy()) as unknown as {
      telemetry?: {
        frontend?: {
          record_webgpu_adapter?: (input: {
            outcome: WebgpuAdapterOutcome;
            window_label: string;
          }) => Promise<void>;
        };
      };
    };
    const resolver = proxyRoot.telemetry?.frontend?.record_webgpu_adapter;
    if (typeof resolver !== "function") {
      return;
    }
    await resolver({ outcome, window_label: currentWindowLabel() });
  } catch {
    // Fire-and-forget: a failed report must never reach the canvas mount path.
  }
}
