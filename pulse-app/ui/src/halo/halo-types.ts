// Type definitions for the Halo State Pulse signature element. Shared across
// HaloCanvas + halo-pipeline + downstream consumers (chunk #32 compact-widget
// infographics + future tray-icon chunk).

export interface HaloInput {
  throughputHz: number;
  errorRate: number;
}

export type MotionMode = "animated" | "static_glow";
