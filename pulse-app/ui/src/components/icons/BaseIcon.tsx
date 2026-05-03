import type { BaseIconProps } from "./types";

export function BaseIcon({
  size = 24,
  "aria-label": ariaLabel,
  "aria-labelledby": ariaLabelledBy,
  title,
  className,
  children,
}: BaseIconProps) {
  const isMeaningful = ariaLabel !== undefined || ariaLabelledBy !== undefined;
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      strokeLinecap="round"
      strokeLinejoin="round"
      focusable="false"
      aria-hidden={isMeaningful ? undefined : true}
      role={isMeaningful ? "img" : undefined}
      aria-label={ariaLabel}
      aria-labelledby={ariaLabelledBy}
      className={className}
    >
      {title ? <title>{title}</title> : null}
      {children}
    </svg>
  );
}
