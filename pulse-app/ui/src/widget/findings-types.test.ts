import { describe, expect, it } from "vitest";
import type { IncidentRecord } from "../bindings";
import {
  counterAriaLabel,
  dropdownRowAriaLabel,
  formatRelativeTime,
  maxPriorityTier,
  priorityTierColorVar,
  priorityTierForegroundVar,
  priorityTierLabel,
  selectUnreadRows,
  type FindingsRow,
} from "./findings-types";

const NOW = 1_700_000_000_000;
const SECOND = 1_000_000_000;
const MINUTE = 60 * SECOND;
const HOUR = 60 * MINUTE;
const DAY = 24 * HOUR;

function record(overrides: Partial<IncidentRecord>): IncidentRecord {
  return {
    id: 1,
    workspace: "ws-a",
    kind: "error_rate_spike",
    scope: "service",
    status: "active",
    severity: "warn",
    priority_tier: "suggested",
    title: "sample finding",
    detail: "[redacted]",
    opened_at_unix_nano: NOW - 5 * MINUTE,
    updated_at_unix_nano: NOW - 5 * MINUTE,
    acknowledged_at_unix_nano: null,
    resolved_at_unix_nano: null,
    read_at_unix_nano: null,
    evidence_count: 0,
    ...overrides,
  };
}

describe("selectUnreadRows", () => {
  it("filters out resolved incidents", () => {
    const rows = selectUnreadRows([
      record({ id: 1, status: "resolved", resolved_at_unix_nano: NOW }),
      record({ id: 2 }),
    ]);
    expect(rows).toHaveLength(1);
    expect(rows[0].id).toBe(2);
  });

  it("filters out already-read incidents", () => {
    const rows = selectUnreadRows([
      record({ id: 1, read_at_unix_nano: NOW - 60 * SECOND }),
      record({ id: 2 }),
    ]);
    expect(rows).toHaveLength(1);
    expect(rows[0].id).toBe(2);
  });

  it("sorts by priority_tier descending then opened_at descending", () => {
    const rows = selectUnreadRows([
      record({ id: 10, priority_tier: "curious", opened_at_unix_nano: NOW - 1 * MINUTE }),
      record({ id: 20, priority_tier: "autonomous", opened_at_unix_nano: NOW - 10 * MINUTE }),
      record({ id: 30, priority_tier: "suggested", opened_at_unix_nano: NOW - 2 * MINUTE }),
      record({ id: 40, priority_tier: "autonomous", opened_at_unix_nano: NOW - 1 * MINUTE }),
    ]);
    expect(rows.map((r) => r.id)).toEqual([40, 20, 30, 10]);
  });

  it("returns empty array for empty input", () => {
    expect(selectUnreadRows([])).toEqual([]);
  });
});

describe("maxPriorityTier", () => {
  it("returns null for empty rows", () => {
    expect(maxPriorityTier([])).toBeNull();
  });

  it("identifies autonomous as the highest tier", () => {
    const rows: FindingsRow[] = [
      { id: 1, priorityTier: "curious", title: "a", openedAtUnixNano: NOW },
      { id: 2, priorityTier: "autonomous", title: "b", openedAtUnixNano: NOW },
      { id: 3, priorityTier: "suggested", title: "c", openedAtUnixNano: NOW },
    ];
    expect(maxPriorityTier(rows)).toBe("autonomous");
  });

  it("ranks suggested above curious", () => {
    const rows: FindingsRow[] = [
      { id: 1, priorityTier: "curious", title: "a", openedAtUnixNano: NOW },
      { id: 2, priorityTier: "suggested", title: "b", openedAtUnixNano: NOW },
    ];
    expect(maxPriorityTier(rows)).toBe("suggested");
  });
});

describe("priorityTierLabel / priorityTierColorVar / priorityTierForegroundVar", () => {
  it("labels priority tiers с capitalized form", () => {
    expect(priorityTierLabel("autonomous")).toBe("Autonomous");
    expect(priorityTierLabel("suggested")).toBe("Suggested");
    expect(priorityTierLabel("curious")).toBe("Curious");
  });

  it("maps autonomous → accent burgundy", () => {
    expect(priorityTierColorVar("autonomous")).toBe("var(--color-accent)");
  });

  it("maps suggested → primary earth blue", () => {
    expect(priorityTierColorVar("suggested")).toBe("var(--color-primary)");
  });

  it("maps curious → text secondary", () => {
    expect(priorityTierColorVar("curious")).toBe("var(--color-text-secondary)");
  });

  it("pairs accent / primary with text-primary foreground", () => {
    expect(priorityTierForegroundVar("autonomous")).toBe("var(--color-text-primary)");
    expect(priorityTierForegroundVar("suggested")).toBe("var(--color-text-primary)");
  });

  it("pairs curious с base foreground (dark text on light gray dot)", () => {
    expect(priorityTierForegroundVar("curious")).toBe("var(--color-base)");
  });
});

describe("formatRelativeTime", () => {
  it("returns 'just now' for sub-minute deltas", () => {
    expect(formatRelativeTime(NOW - 30 * SECOND, NOW)).toBe("just now");
  });

  it("returns 'just now' for negative deltas (clock skew)", () => {
    expect(formatRelativeTime(NOW + 5 * SECOND, NOW)).toBe("just now");
  });

  it("returns minutes form for sub-hour deltas", () => {
    expect(formatRelativeTime(NOW - 3 * MINUTE, NOW)).toBe("3m ago");
    expect(formatRelativeTime(NOW - 59 * MINUTE, NOW)).toBe("59m ago");
  });

  it("returns hours form for sub-day deltas", () => {
    expect(formatRelativeTime(NOW - 2 * HOUR, NOW)).toBe("2h ago");
    expect(formatRelativeTime(NOW - 23 * HOUR, NOW)).toBe("23h ago");
  });

  it("returns days form for ≥1 day deltas", () => {
    expect(formatRelativeTime(NOW - 2 * DAY, NOW)).toBe("2d ago");
  });

  it("returns placeholder for non-finite inputs", () => {
    expect(formatRelativeTime(Number.NaN, NOW)).toBe("—");
    expect(formatRelativeTime(NOW, Number.POSITIVE_INFINITY)).toBe("—");
  });
});

describe("counterAriaLabel", () => {
  it("uses singular for count=1", () => {
    expect(counterAriaLabel(1, "autonomous")).toBe(
      "Findings: 1 unread incident, Autonomous severity",
    );
  });

  it("uses plural for count>1", () => {
    expect(counterAriaLabel(3, "suggested")).toBe(
      "Findings: 3 unread incidents, Suggested severity",
    );
  });

  it("omits severity suffix when tier is null", () => {
    expect(counterAriaLabel(0, null)).toBe("Findings: 0 unread incidents");
  });
});

describe("dropdownRowAriaLabel", () => {
  it("combines tier + title + relative time", () => {
    const row: FindingsRow = {
      id: 1,
      priorityTier: "autonomous",
      title: "Database connection timeout",
      openedAtUnixNano: NOW - 3 * MINUTE,
    };
    expect(dropdownRowAriaLabel(row, NOW)).toBe(
      "Autonomous: Database connection timeout, 3m ago",
    );
  });
});
