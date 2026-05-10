import { forwardRef, useImperativeHandle, useRef } from "react";
import { Icon } from "./icons";
import { usePlatform } from "../hooks/use-platform";
import { WindowControls } from "./WindowControls";

interface TitlebarProps {
  // Optional override for the title text — defaults to "andromeda-pulse"
  // for the compact-widget; the full dashboard appends a view variant
  // (chunk #31 will pass `${appName} — ${view}` once routing arrives).
  title?: string;
  onSettingsClick?: () => void;
  // chunk #42 — Investigate trigger placement on the compact-widget titlebar
  // per layout-templates.md §Wireframe — Compact widget IA notes ("actionable
  // element belongs in titlebar segment"). When undefined, the button is not
  // rendered (preserves chunk #24 default shape for surfaces without an
  // Investigate flow).
  onInvestigateClick?: (trigger: HTMLElement | null) => void;
}

export interface TitlebarHandle {
  investigateButton: HTMLButtonElement | null;
}

// Frameless custom titlebar for the desktop-webview surface (chunk #24).
//
// Per layout-templates.md §Component — Custom titlebar Layout: 32px height
// (hardcoded; see Implementation note in plan.md — design says 32px but
// `--spacing-lg` resolves to 24px; reconciliation deferred to chunk #34
// settings panel via a dedicated --space-titlebar token), drag region across
// the bg via `data-tauri-drag-region` except button hit-targets, segment
// order [app-icon + title | grow | settings | window-controls].
//
// Per a11y plan §7 Landmark roles: wrapped in <header>; window-control
// buttons + settings button are native <button> with aria-label per §11
// "semantic HTML first". Drag region carries no tabindex per §11 Keyboard.
export const Titlebar = forwardRef<TitlebarHandle, TitlebarProps>(
  function Titlebar(
    { title = "andromeda-pulse", onSettingsClick, onInvestigateClick },
    ref,
  ) {
    const platform = usePlatform();
    const investigateRef = useRef<HTMLButtonElement | null>(null);

    useImperativeHandle(
      ref,
      () => ({
        get investigateButton() {
          return investigateRef.current;
        },
      }),
      [],
    );

    return (
    <header
      className="titlebar"
      data-tauri-drag-region
      data-testid="titlebar"
      style={{
        height: "32px",
        display: "flex",
        alignItems: "center",
        gap: "var(--spacing-sm)",
        padding: "0 var(--spacing-sm)",
        background: "var(--color-base)",
        borderBottom: "1px solid rgba(74, 144, 226, 0.1)",
        userSelect: "none",
      }}
    >
      <span
        className="titlebar__app-icon"
        aria-hidden="true"
        data-tauri-drag-region
      >
        <Icon glyph="constellation-grid" size={16} />
      </span>
      <span
        className="titlebar__title"
        data-tauri-drag-region
        style={{
          fontFamily: "var(--font-body)",
          fontSize: "12px",
          fontWeight: 500,
          color: "var(--color-text-primary)",
          letterSpacing: 0,
        }}
      >
        {title}
      </span>
      <span className="titlebar__grow" data-tauri-drag-region style={{ flex: 1 }} />
      {onInvestigateClick ? (
        <button
          ref={investigateRef}
          type="button"
          aria-label="Investigate"
          onClick={(event) => onInvestigateClick(event.currentTarget)}
          className="titlebar__investigate motion-reduce:transition-none motion-reduce:duration-0"
          data-testid="titlebar-investigate"
          style={{
            background: "transparent",
            border: 0,
            color: "var(--color-text-secondary)",
            cursor: "pointer",
            padding: "var(--spacing-xs)",
            minWidth: "var(--target-input-min)",
            minHeight: "var(--target-input-min)",
            transitionDuration: "var(--duration-fast)",
            transitionTimingFunction: "var(--easing-out)",
          }}
        >
          <Icon glyph="telescope" size={20} aria-hidden="true" />
        </button>
      ) : null}
      <button
        type="button"
        aria-label="Open settings"
        onClick={onSettingsClick}
        className="titlebar__settings motion-reduce:transition-none motion-reduce:duration-0"
        style={{
          background: "transparent",
          border: 0,
          color: "var(--color-text-secondary)",
          cursor: "pointer",
          padding: "var(--spacing-xs)",
          minWidth: "var(--target-input-min)",
          minHeight: "var(--target-input-min)",
          transitionDuration: "var(--duration-fast)",
          transitionTimingFunction: "var(--easing-out)",
        }}
      >
        <Icon glyph="aperture" size={20} />
      </button>
      <WindowControls platform={platform} />
    </header>
  );
});
