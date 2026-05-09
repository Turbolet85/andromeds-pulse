// Bounded enums for the full dashboard surface — kept in their own module so
// router.tsx, TabNav.tsx, CommandPalette.tsx, and route components agree on
// the canonical 5-tab vocabulary. Values match TanStack Router route paths
// (without leading slash) so URL ↔ tab mapping is direct + cardinality
// discipline holds (per obs-plan §5 Metric label cardinality).

import type { GlyphName } from "../components/icons";

export const TAB_IDS = ["traces", "metrics", "logs", "snapshots", "settings"] as const;

export type TabId = (typeof TAB_IDS)[number];

export interface TabDescriptor {
  id: TabId;
  label: string;
  path: string;
  glyph: GlyphName;
}

export const TABS: readonly TabDescriptor[] = [
  { id: "traces", label: "Traces", path: "/traces", glyph: "telescope" },
  { id: "metrics", label: "Metrics", path: "/metrics", glyph: "circular-pulse" },
  { id: "logs", label: "Logs", path: "/logs", glyph: "star" },
  { id: "snapshots", label: "Snapshots", path: "/snapshots", glyph: "constellation-grid" },
  { id: "settings", label: "Settings", path: "/settings", glyph: "aperture" },
] as const;

export type PaletteActionKind = "open-tab";

export interface PaletteItem {
  kind: PaletteActionKind;
  id: string;
  label: string;
  hint: string;
  glyph: GlyphName;
  targetPath: string;
}
