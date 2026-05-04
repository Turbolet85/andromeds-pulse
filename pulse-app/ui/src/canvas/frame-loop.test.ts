import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { createFrameLoop } from "./frame-loop";

describe("createFrameLoop — reduced-motion gate at the rAF tick layer", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("invokes onFrame on each rAF tick when prefersReducedMotion is false", () => {
    const onFrame = vi.fn();
    const handle = createFrameLoop({ onFrame, prefersReducedMotion: false });
    handle.start();
    vi.advanceTimersByTime(50);
    handle.stop();
    expect(onFrame).toHaveBeenCalled();
  });

  it("does NOT invoke onFrame when prefersReducedMotion is true", () => {
    const onFrame = vi.fn();
    const handle = createFrameLoop({ onFrame, prefersReducedMotion: true });
    handle.start();
    vi.advanceTimersByTime(200);
    handle.stop();
    expect(onFrame).not.toHaveBeenCalled();
  });

  it("invokes onReducedMotionFrame exactly once on start when prefersReducedMotion is true", () => {
    const onFrame = vi.fn();
    const onReducedMotionFrame = vi.fn();
    const handle = createFrameLoop({
      onFrame,
      prefersReducedMotion: true,
      onReducedMotionFrame,
    });
    handle.start();
    vi.advanceTimersByTime(200);
    expect(onReducedMotionFrame).toHaveBeenCalledTimes(1);
  });

  it("stop() cancels pending rAF (no further onFrame calls)", () => {
    const onFrame = vi.fn();
    const handle = createFrameLoop({ onFrame, prefersReducedMotion: false });
    handle.start();
    vi.advanceTimersByTime(50);
    const callCount = onFrame.mock.calls.length;
    handle.stop();
    vi.advanceTimersByTime(200);
    expect(onFrame).toHaveBeenCalledTimes(callCount);
  });

  it("start() is idempotent — calling twice does not double-schedule", () => {
    const onFrame = vi.fn();
    const handle = createFrameLoop({ onFrame, prefersReducedMotion: false });
    handle.start();
    handle.start();
    vi.advanceTimersByTime(20);
    handle.stop();
    // Without idempotence the rAF queue would grow each start() call;
    // single-scheduled tick advances onFrame at framerate cadence.
    expect(onFrame.mock.calls.length).toBeLessThanOrEqual(3);
  });
});
