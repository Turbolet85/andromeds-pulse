// Findings counter + dropdown shared types + pure-function helpers per
// chunk #87 §Phase 8 §86. Severity encoding uses the PriorityTier enum
// (Autonomous / Suggested / Curious) per a11y plan §6 + the existing
// chunk #78 IncidentRecord shape. Sort + format helpers live here so
// FindingsCounter + FindingsDropdown stay presentational.

import type { IncidentRecord, PriorityTier } from "../bindings";

export const SECONDS_PER_MINUTE = 60;
export const SECONDS_PER_HOUR = 3600;
export const SECONDS_PER_DAY = 86400;

const NANOS_PER_SECOND = 1_000_000_000n;

const PRIORITY_TIER_RANK: Record<PriorityTier, number> = {
  autonomous: 3,
  suggested: 2,
  curious: 1,
};

export interface FindingsRow {
  id: number;
  priorityTier: PriorityTier;
  title: string;
  openedAtUnixNano: number;
}

export type FindingsSeverityKey = PriorityTier;

// Returns the unread incident rows in display order: priority_tier
// descending (Autonomous → Suggested → Curious), then opened_at_unix_nano
// descending (newest unread first per chronological sort tie-break per
// project-doc §86 "sorted by severity descending then chronologically").
export function selectUnreadRows(records: IncidentRecord[]): FindingsRow[] {
  const unread = records
    .filter((r) => r.read_at_unix_nano === null && r.status !== "resolved")
    .map<FindingsRow>((r) => ({
      id: r.id,
      priorityTier: r.priority_tier,
      title: r.title,
      openedAtUnixNano: r.opened_at_unix_nano,
    }));

  unread.sort((a, b) => {
    const rankDelta = PRIORITY_TIER_RANK[b.priorityTier] - PRIORITY_TIER_RANK[a.priorityTier];
    if (rankDelta !== 0) return rankDelta;
    return b.openedAtUnixNano - a.openedAtUnixNano;
  });

  return unread;
}

// Returns the highest priority_tier present in the unread set, or null if
// the set is empty. Used by FindingsCounter accessible name + background
// color encoding.
export function maxPriorityTier(rows: FindingsRow[]): PriorityTier | null {
  if (rows.length === 0) return null;
  let best: PriorityTier = "curious";
  for (const row of rows) {
    if (PRIORITY_TIER_RANK[row.priorityTier] > PRIORITY_TIER_RANK[best]) {
      best = row.priorityTier;
    }
  }
  return best;
}

// Human-readable tier label for accessible names and dropdown row text.
// Capitalized form per a11y extract: "Autonomous" / "Suggested" / "Curious".
export function priorityTierLabel(tier: PriorityTier): string {
  switch (tier) {
    case "autonomous":
      return "Autonomous";
    case "suggested":
      return "Suggested";
    case "curious":
      return "Curious";
  }
}

// CSS custom-property name for the severity-color encoding. Maps to design
// tokens from design-system.md §Color Palette; counter background +
// dropdown row dots consume this. NOT color-alone per a11y SC 1.4.1 —
// callers MUST pair color with text label (priorityTierLabel) per
// state-not-color-alone discipline.
export function priorityTierColorVar(tier: PriorityTier): string {
  switch (tier) {
    case "autonomous":
      return "var(--color-accent)";
    case "suggested":
      return "var(--color-primary)";
    case "curious":
      return "var(--color-text-secondary)";
  }
}

// Returns a contrast-safe text-color CSS variable for content rendered on
// top of priorityTierColorVar(). Pairs Autonomous (Alert Burgundy) with
// text-primary (Status White-Blue) and Suggested (Earth Blue) with
// text-primary; Curious (text-secondary) needs base-dark text.
export function priorityTierForegroundVar(tier: PriorityTier): string {
  switch (tier) {
    case "autonomous":
    case "suggested":
      return "var(--color-text-primary)";
    case "curious":
      return "var(--color-base)";
  }
}

// Returns a human-readable relative timestamp suitable for dropdown row
// display: "just now" / "Nm ago" / "Nh ago" / "Nd ago". Inputs are unix
// nano timestamps as numbers (within JS Number precision per CLAUDE.md
// testing.md 2026-05-10 no-loss-of-precision discipline). Negative deltas
// (clock skew) collapse to "just now"; non-finite inputs collapse к "—".
export function formatRelativeTime(thenUnixNano: number, nowUnixNano: number): string {
  if (!Number.isFinite(thenUnixNano) || !Number.isFinite(nowUnixNano)) {
    return "—";
  }
  const deltaNanos = nowUnixNano - thenUnixNano;
  if (deltaNanos < 0) return "just now";
  const deltaSeconds = Math.floor(deltaNanos / Number(NANOS_PER_SECOND));
  if (deltaSeconds < SECONDS_PER_MINUTE) {
    return "just now";
  }
  if (deltaSeconds < SECONDS_PER_HOUR) {
    return `${Math.floor(deltaSeconds / SECONDS_PER_MINUTE)}m ago`;
  }
  if (deltaSeconds < SECONDS_PER_DAY) {
    return `${Math.floor(deltaSeconds / SECONDS_PER_HOUR)}h ago`;
  }
  return `${Math.floor(deltaSeconds / SECONDS_PER_DAY)}d ago`;
}

// Builds the accessible name for the counter trigger button per a11y
// extract: "Findings: {N} unread, {highest tier} severity". Singular vs
// plural correct ("1 unread" / "3 unread"); singleton tier label
// capitalized; null tier collapses к count-only ("Findings: 0 unread" —
// only used during transient zero-count render before counter unmounts).
export function counterAriaLabel(count: number, tier: PriorityTier | null): string {
  const plural = count === 1 ? "" : "s";
  const tierSuffix = tier ? `, ${priorityTierLabel(tier)} severity` : "";
  return `Findings: ${count} unread incident${plural}${tierSuffix}`;
}

// Returns the accessible name for а dropdown row per а11y extract:
// "{tier}: {title}, {relative-time}". Severity word IS the non-color
// supplement к the colored dot per a11y SC 1.4.1.
export function dropdownRowAriaLabel(row: FindingsRow, nowUnixNano: number): string {
  return `${priorityTierLabel(row.priorityTier)}: ${row.title}, ${formatRelativeTime(row.openedAtUnixNano, nowUnixNano)}`;
}
