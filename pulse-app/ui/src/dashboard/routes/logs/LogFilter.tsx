import {
  ALL_SEVERITY_TIERS,
  type LogFilterState,
  type SeverityTier,
} from "./use-log-filter";

interface LogFilterProps {
  state: LogFilterState;
  onSearchChange: (query: string) => void;
  onToggleTier: (tier: SeverityTier) => void;
}

const TIER_LABEL: Record<SeverityTier, string> = {
  error: "ERROR",
  warn: "WARN",
  info: "INFO",
  debug: "DEBUG",
};

export function LogFilter({
  state,
  onSearchChange,
  onToggleTier,
}: LogFilterProps) {
  return (
    <div
      data-testid="log-filter"
      style={{
        display: "flex",
        flexDirection: "row",
        gap: "var(--spacing-md)",
        alignItems: "center",
        padding: "var(--spacing-sm)",
        background: "var(--color-raised-1)",
        border: "1px solid rgba(74, 144, 226, 0.3)",
        borderRadius: "var(--radius-md)",
      }}
    >
      <label
        htmlFor="log-search"
        style={{
          fontFamily: "var(--font-body)",
          fontSize: "12px",
          color: "var(--color-text-secondary)",
        }}
      >
        Search logs
      </label>
      <input
        id="log-search"
        type="search"
        value={state.searchQuery}
        onChange={(e) => onSearchChange(e.target.value)}
        placeholder="Filter by body…"
        aria-describedby="log-search-hint"
        data-testid="log-search-input"
        style={{
          flex: 1,
          background: "var(--color-inset)",
          border: "1px solid rgba(74, 144, 226, 0.3)",
          borderRadius: "var(--radius-sm)",
          padding: "var(--spacing-xs) var(--spacing-sm)",
          color: "var(--color-text-primary)",
          fontFamily: "var(--font-body)",
          fontSize: "14px",
        }}
      />
      <span
        id="log-search-hint"
        style={{
          fontFamily: "var(--font-body)",
          fontSize: "12px",
          // text-secondary, not tertiary: small body text needs 4.5:1
                  // (chunk #99 pa11y finding 3.92:1)
                  color: "var(--color-text-secondary)",
        }}
      >
        Case-insensitive
      </span>
      <div
        role="group"
        aria-label="Severity filters"
        style={{ display: "flex", gap: "var(--spacing-xs)" }}
      >
        {ALL_SEVERITY_TIERS.map((tier) => {
          const enabled = state.enabledTiers.has(tier);
          return (
            <button
              key={tier}
              type="button"
              aria-pressed={enabled}
              onClick={() => onToggleTier(tier)}
              data-testid={`severity-chip-${tier}`}
              style={{
                background: enabled ? "var(--color-raised-2)" : "var(--color-inset)",
                border: enabled
                  ? "1px solid #4A90E2"
                  : "1px solid rgba(74, 144, 226, 0.3)",
                borderRadius: "var(--radius-sm)",
                padding: "var(--spacing-xs) var(--spacing-sm)",
                color: "var(--color-text-primary)",
                fontFamily: "var(--font-body)",
                fontSize: "12px",
                cursor: "pointer",
                opacity: enabled ? 1 : 0.6,
              }}
            >
              {TIER_LABEL[tier]}
            </button>
          );
        })}
      </div>
    </div>
  );
}
