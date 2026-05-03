import { BaseIcon } from "./BaseIcon";
import type { IconProps } from "./types";

export function Aperture(props: IconProps) {
  return (
    <BaseIcon {...props}>
      <circle cx="12" cy="12" r="9" />
      <path d="M 12 8 L 15.46 10 L 15.46 14 L 12 16 L 8.54 14 L 8.54 10 Z" />
      <path d="M 12 3 L 12 8" />
      <path d="M 19.79 7.5 L 15.46 10" />
      <path d="M 19.79 16.5 L 15.46 14" />
      <path d="M 12 21 L 12 16" />
      <path d="M 4.21 16.5 L 8.54 14" />
      <path d="M 4.21 7.5 L 8.54 10" />
    </BaseIcon>
  );
}
