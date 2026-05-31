// Type definitions for the Halo State Pulse signature element. Shared across
// HaloCanvas + halo-pipeline + downstream consumers (chunk #32 compact-widget
// infographics + future tray-icon chunk).

export type MotionMode = "animated" | "static_glow";

// Telemetry-flow tier driving the Halo breathing pace (chunk #90). Derived
// webview-side from the ingest throughput signal; independent of connection
// health (a separate orthogonal axis) per P-004.
export type ActivityState = "active" | "quiet" | "idle";
