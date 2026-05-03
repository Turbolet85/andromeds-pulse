import { BaseIcon } from "./BaseIcon";
import type { IconProps } from "./types";

export function ConstellationGrid(props: IconProps) {
  return (
    <BaseIcon {...props}>
      <circle cx="12" cy="12" r="9" />
      <path d="M 3 12 L 21 12" />
      <path d="M 12 3 L 12 21" />
      <path d="M 6 12 Q 12 20 18 12" />
      <circle cx="12" cy="6" r="1" fill="currentColor" stroke="none" />
      <circle cx="6" cy="12" r="1" fill="currentColor" stroke="none" />
      <circle cx="18" cy="12" r="1" fill="currentColor" stroke="none" />
      <circle cx="12" cy="18" r="1" fill="currentColor" stroke="none" />
    </BaseIcon>
  );
}
