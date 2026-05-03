import { BaseIcon } from "./BaseIcon";
import type { IconProps } from "./types";

export function Star(props: IconProps) {
  return (
    <BaseIcon {...props}>
      <path d="M 12 4 L 13.8 9.6 L 19.6 9.5 L 14.9 12.9 L 16.7 18.5 L 12 15 L 7.3 18.5 L 9.2 12.9 L 4.4 9.5 L 10.2 9.6 Z" />
    </BaseIcon>
  );
}
