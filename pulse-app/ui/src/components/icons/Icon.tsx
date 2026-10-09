import type { FC } from "react";
import { Aperture } from "./Aperture";
import { CircularPulse } from "./CircularPulse";
import { ConstellationGrid } from "./ConstellationGrid";
import { Expand } from "./Expand";
import { Star } from "./Star";
import { Telescope } from "./Telescope";
import type { GlyphName, IconProps } from "./types";

const map: Record<GlyphName, FC<IconProps>> = {
  aperture: Aperture,
  telescope: Telescope,
  "constellation-grid": ConstellationGrid,
  expand: Expand,
  star: Star,
  "circular-pulse": CircularPulse,
};

export interface IconDispatchProps extends IconProps {
  glyph: GlyphName;
}

export function Icon({ glyph, ...rest }: IconDispatchProps) {
  const Component = map[glyph];
  return <Component {...rest} />;
}
