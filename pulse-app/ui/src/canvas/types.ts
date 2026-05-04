// Frame-loop scaffold types consumed by chunks #25 (WebGPU canvas + WGSL
// pipeline) and #28 (Halo State Pulse signature element). Reduced-motion gate
// lives at the rAF tick layer per design-system.md §Motion Accessibility:
// hue interpolation is NOT gated (data-driven Halo color updates per error
// rate continue under reduced-motion); only the pulse-rhythm envelope is.
export interface FrameLoopOptions {
  onFrame: (timestamp: DOMHighResTimeStamp) => void;
  prefersReducedMotion: boolean;
  onReducedMotionFrame?: (timestamp: DOMHighResTimeStamp) => void;
}

export interface FrameLoopHandle {
  start: () => void;
  stop: () => void;
}
