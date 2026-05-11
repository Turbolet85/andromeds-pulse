import { type CSSProperties } from "react";
import type { PresetPrompt } from "../dashboard/preset-prompts";

interface PresetPromptListProps {
  prompts: ReadonlyArray<PresetPrompt>;
  onPick: (prompt: PresetPrompt) => void;
  "data-testid"?: string;
}

export function PresetPromptList({
  prompts,
  onPick,
  "data-testid": dataTestId,
}: PresetPromptListProps) {
  const listStyle: CSSProperties = {
    display: "flex",
    flexDirection: "column",
    gap: "var(--spacing-xs)",
    margin: 0,
    padding: 0,
    listStyle: "none",
  };

  const buttonStyle: CSSProperties = {
    display: "block",
    width: "100%",
    textAlign: "left",
    background: "var(--color-raised-1)",
    color: "var(--color-text-primary)",
    border: "1px solid var(--border-subtle)",
    borderRadius: "var(--radius-sm)",
    padding: "var(--spacing-xs) var(--spacing-sm)",
    minHeight: "var(--target-button-min)",
    minWidth: "var(--target-button-min)",
    fontFamily: "var(--font-body)",
    fontSize: "12px",
    fontWeight: 500,
    cursor: "pointer",
    transitionDuration: "var(--duration-fast)",
    transitionTimingFunction: "var(--easing-out)",
  };

  return (
    <ul
      data-testid={dataTestId ?? "preset-prompt-list"}
      style={listStyle}
    >
      {prompts.map((prompt) => (
        <li key={prompt.id}>
          <button
            type="button"
            aria-label={`Insert prompt: ${prompt.label}`}
            data-testid={`preset-prompt-${prompt.id}`}
            onClick={() => onPick(prompt)}
            className="motion-reduce:transition-none motion-reduce:duration-0 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[color:var(--border-focus)]"
            style={buttonStyle}
          >
            {prompt.label}
          </button>
        </li>
      ))}
    </ul>
  );
}
