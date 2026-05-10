import { forwardRef, type ButtonHTMLAttributes, type CSSProperties } from "react";
import { Icon } from "./icons";

export type InvestigateButtonVariant =
  | "main"
  | "widget-titlebar"
  | "trace-row-inline";

interface InvestigateButtonProps
  extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, "type"> {
  variant: InvestigateButtonVariant;
  ariaBusy?: boolean;
  "data-testid"?: string;
}

export const InvestigateButton = forwardRef<
  HTMLButtonElement,
  InvestigateButtonProps
>(function InvestigateButton(
  { variant, ariaBusy = false, onClick, style: styleOverride, ...rest },
  ref,
) {
  const iconSize = variant === "trace-row-inline" ? 16 : 20;
  const showLabel = variant === "main";

  const baseStyle: CSSProperties = {
    display: "inline-flex",
    alignItems: "center",
    gap: "var(--spacing-xs)",
    border: 0,
    cursor: "pointer",
    fontFamily: "var(--font-body)",
    fontSize: "14px",
    transitionDuration: "var(--duration-fast)",
    transitionTimingFunction: "var(--easing-out)",
  };

  const variantStyle: CSSProperties =
    variant === "main"
      ? {
          background: "var(--color-primary)",
          color: "var(--color-base)",
          borderRadius: "var(--radius-sm)",
          padding: "var(--spacing-xs) var(--spacing-sm)",
          fontWeight: 500,
          minHeight: "var(--target-input-min)",
        }
      : {
          background: "transparent",
          color: "var(--color-text-secondary)",
          borderRadius: "var(--radius-sm)",
          padding: "var(--spacing-xs)",
          minWidth: "var(--target-input-min)",
          minHeight: "var(--target-input-min)",
        };

  const defaultAriaLabel =
    variant === "main" ? undefined : "Investigate";

  return (
    <button
      ref={ref}
      type="button"
      aria-busy={ariaBusy ? "true" : "false"}
      aria-label={rest["aria-label"] ?? defaultAriaLabel}
      onClick={onClick}
      className="motion-reduce:transition-none motion-reduce:duration-0"
      data-variant={variant}
      data-testid={rest["data-testid"] ?? `investigate-button-${variant}`}
      style={{ ...baseStyle, ...variantStyle, ...styleOverride }}
      {...rest}
    >
      <Icon
        glyph="telescope"
        size={iconSize}
        aria-hidden={showLabel ? "true" : undefined}
      />
      {showLabel ? <span>Investigate</span> : null}
    </button>
  );
});
