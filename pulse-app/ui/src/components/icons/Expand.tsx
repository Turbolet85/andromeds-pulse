import { BaseIcon } from "./BaseIcon";
import type { IconProps } from "./types";

export function Expand(props: IconProps) {
  return (
    <BaseIcon {...props}>
      <path d="M 14 4 L 20 4 L 20 10" />
      <path d="M 20 4 L 13 11" />
      <path d="M 10 20 L 4 20 L 4 14" />
      <path d="M 4 20 L 11 13" />
    </BaseIcon>
  );
}
