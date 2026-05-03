import { BaseIcon } from "./BaseIcon";
import type { IconProps } from "./types";

export function Telescope(props: IconProps) {
  return (
    <BaseIcon {...props}>
      <path d="M 5 18 L 18 5 L 21 8 L 8 21 Z" />
      <path d="M 12 12 L 9 22" />
      <path d="M 12 12 L 15 22" />
    </BaseIcon>
  );
}
