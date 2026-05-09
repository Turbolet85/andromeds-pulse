// Synthetic Halo input simulator extracted from chunk #31 App.tsx body. Drives
// `{ throughputHz, errorRate }` for the HaloCanvas signature element on a fast
// cadence (250ms) so the LCH hue interpolation + pulse rhythm look smooth.
// Real binding via `streams.subscribe_metrics` Arrow IPC bytes parser is
// deferred to chunks #34/#35 — the data-testid="halo-input-simulator" anchor
// in the consumer markup grep-marks the replacement point.

import { useEffect, useState } from "react";
import type { HaloInput } from "../halo/halo-types";

const SYNTHETIC_INPUT_INTERVAL_MS = 250;
const THROUGHPUT_BASE_HZ = 1000;
const THROUGHPUT_AMPLITUDE_HZ = 800;
const THROUGHPUT_FREQ = 0.0003;
const ERROR_RATE_FREQ = 0.0005;

export function useSyntheticHaloInput(): HaloInput {
  const [haloInput, setHaloInput] = useState<HaloInput>({
    throughputHz: THROUGHPUT_BASE_HZ,
    errorRate: 0,
  });

  useEffect(() => {
    const startedAt = performance.now();
    const interval = setInterval(() => {
      const elapsed = performance.now() - startedAt;
      const throughputHz =
        THROUGHPUT_BASE_HZ + THROUGHPUT_AMPLITUDE_HZ * Math.sin(elapsed * THROUGHPUT_FREQ);
      const errorRate = 0.5 + 0.5 * Math.sin(elapsed * ERROR_RATE_FREQ);
      setHaloInput({ throughputHz, errorRate });
    }, SYNTHETIC_INPUT_INTERVAL_MS);
    return () => clearInterval(interval);
  }, []);

  return haloInput;
}
