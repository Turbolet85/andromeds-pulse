export interface PresetPrompt {
  readonly id: string;
  readonly label: string;
  readonly template: string;
}

export const PRESET_PROMPTS: ReadonlyArray<PresetPrompt> = [
  {
    id: "diagnose-latency-outlier",
    label: "Diagnose latency outlier",
    template:
      "Diagnose the latency outlier shown in the snapshot above. Identify the slowest service on the critical path and the contributing spans.",
  },
  {
    id: "find-error-correlation",
    label: "Find error correlation",
    template:
      "Examine the error-correlated spans in the snapshot above. Group by service and identify likely upstream causes.",
  },
  {
    id: "trace-failed-request",
    label: "Trace failed request",
    template:
      "Walk the trace of the failed request in the snapshot above from entry span through downstream calls; surface where the error first appears.",
  },
  {
    id: "summarize-service-health",
    label: "Summarize service health",
    template:
      "Summarize the per-service health in the snapshot above using p50/p95/p99 latencies and error rates; flag any service exceeding expected bounds.",
  },
];
