import type { FrameLoopHandle, FrameLoopOptions } from "./types";

// Static-paint pattern when prefersReducedMotion is true: invoke
// onReducedMotionFrame exactly once on start() and do NOT schedule subsequent
// rAF — matches design-system.md §Motion Accessibility "frequency Hz → 0,
// blur radius → fixed midpoint". Consumers in chunks #25 / #28 may re-call
// start() to repaint after data updates (LCH hue change per error rate);
// the contract is "reduced-motion = no rhythm envelope, but consumer-driven
// repaints permitted for hue updates" per layout-templates.md §Decisions Log.
export function createFrameLoop(options: FrameLoopOptions): FrameLoopHandle {
  let rafId: number | null = null;

  const tick = (timestamp: DOMHighResTimeStamp): void => {
    options.onFrame(timestamp);
    // TODO(chunk #29): replace with TauRPC telemetry.frontend.record_frame_ms
    // call once the backend handler signature is defined per obs-plan §12
    // Open questions; chunk #15 ships the wiring substrate only, chunk #28
    // wires the canvas substrate, chunk #29 lands the per-frame metric resolver.
    rafId = requestAnimationFrame(tick);
  };

  return {
    start() {
      if (rafId !== null) return;
      if (options.prefersReducedMotion) {
        options.onReducedMotionFrame?.(performance.now());
        return;
      }
      rafId = requestAnimationFrame(tick);
    },
    stop() {
      if (rafId === null) return;
      cancelAnimationFrame(rafId);
      rafId = null;
    },
  };
}
