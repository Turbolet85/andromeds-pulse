import { BaseIcon } from "./BaseIcon";
import type { IconProps } from "./types";

export function CircularPulse(props: IconProps) {
  return (
    <BaseIcon {...props}>
      <circle cx="12" cy="12" r="9" />
      <path d="M 7 12 A 5 5 0 0 1 17 12" />
      <path d="M 9.5 12 A 2.5 2.5 0 0 0 14.5 12" />
      <circle cx="12" cy="12" r="0.8" fill="currentColor" stroke="none" />
    </BaseIcon>
  );
}
