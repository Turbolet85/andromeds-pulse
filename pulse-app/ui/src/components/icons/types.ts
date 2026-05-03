import type { ReactNode } from "react";

export type GlyphName =
  | "aperture"
  | "telescope"
  | "constellation-grid"
  | "star"
  | "circular-pulse";

export interface IconProps {
  size?: 16 | 20 | 24;
  "aria-label"?: string;
  "aria-labelledby"?: string;
  title?: string;
  className?: string;
}

export interface BaseIconProps extends IconProps {
  children: ReactNode;
}
