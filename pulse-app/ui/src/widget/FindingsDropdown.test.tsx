import { createRef, type RefObject } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { FindingsDropdown } from "./FindingsDropdown";
import { FINDINGS_DROPDOWN_PANEL_ID } from "./FindingsCounter";
import type { FindingsRow } from "./findings-types";

const NOW_NANO = 1_700_000_000_000;
const SECOND = 1_000_000_000;
const MINUTE = 60 * SECOND;

function row(overrides: Partial<FindingsRow> = {}): FindingsRow {
  return {
    id: 1,
    priorityTier: "suggested",
    title: "Sample finding",
    openedAtUnixNano: NOW_NANO - 5 * MINUTE,
    ...overrides,
  };
}

function makeTriggerRef(): RefObject<HTMLButtonElement | null> {
  const ref = createRef<HTMLButtonElement>();
  const trigger = document.createElement("button");
  trigger.setAttribute("data-testid", "trigger-stub");
  document.body.appendChild(trigger);
  (ref as { current: HTMLButtonElement | null }).current = trigger;
  return ref;
}

afterEach(() => {
  document.body.innerHTML = "";
});

describe("FindingsDropdown — visibility", () => {
  it("renders nothing when isOpen is false", () => {
    const { container } = render(
      <FindingsDropdown
        rows={[row()]}
        isOpen={false}
        onClose={() => {}}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={makeTriggerRef()}
        nowUnixNano={NOW_NANO}
      />,
    );
    expect(container.firstChild).toBeNull();
  });

  it("renders the panel when isOpen is true", () => {
    render(
      <FindingsDropdown
        rows={[row()]}
        isOpen={true}
        onClose={() => {}}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={makeTriggerRef()}
        nowUnixNano={NOW_NANO}
      />,
    );
    expect(screen.getByTestId("findings-dropdown")).toBeDefined();
    expect(screen.getByTestId("findings-dropdown").id).toBe(FINDINGS_DROPDOWN_PANEL_ID);
  });
});

describe("FindingsDropdown — empty state", () => {
  it("shows 'No unread findings' message when rows is empty", () => {
    render(
      <FindingsDropdown
        rows={[]}
        isOpen={true}
        onClose={() => {}}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={makeTriggerRef()}
        nowUnixNano={NOW_NANO}
      />,
    );
    expect(screen.getByTestId("findings-dropdown-empty").textContent).toBe("No unread findings");
    expect(screen.queryByTestId("findings-dropdown-list")).toBeNull();
  });

  it("renders the row list when rows are present", () => {
    render(
      <FindingsDropdown
        rows={[row()]}
        isOpen={true}
        onClose={() => {}}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={makeTriggerRef()}
        nowUnixNano={NOW_NANO}
      />,
    );
    expect(screen.getByTestId("findings-dropdown-list")).toBeDefined();
    expect(screen.queryByTestId("findings-dropdown-empty")).toBeNull();
  });
});

describe("FindingsDropdown — row rendering", () => {
  const rows: FindingsRow[] = [
    row({ id: 10, priorityTier: "autonomous", title: "Database timeout" }),
    row({ id: 20, priorityTier: "suggested", title: "Latency regression" }),
    row({ id: 30, priorityTier: "curious", title: "Slow startup" }),
  ];

  it("renders one row per FindingsRow in input order (sorted upstream)", () => {
    render(
      <FindingsDropdown
        rows={rows}
        isOpen={true}
        onClose={() => {}}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={makeTriggerRef()}
        nowUnixNano={NOW_NANO}
      />,
    );
    const renderedRows = screen.getAllByTestId("findings-dropdown-row");
    expect(renderedRows).toHaveLength(3);
    expect(renderedRows.map((r) => r.dataset.rowId)).toEqual(["10", "20", "30"]);
    expect(renderedRows.map((r) => r.dataset.priorityTier)).toEqual([
      "autonomous",
      "suggested",
      "curious",
    ]);
  });

  it("renders rows as native <button> elements", () => {
    render(
      <FindingsDropdown
        rows={[row()]}
        isOpen={true}
        onClose={() => {}}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={makeTriggerRef()}
        nowUnixNano={NOW_NANO}
      />,
    );
    const row1 = screen.getByTestId("findings-dropdown-row");
    expect(row1.tagName).toBe("BUTTON");
  });

  it("composes row aria-label as 'tier: title, relative-time'", () => {
    render(
      <FindingsDropdown
        rows={[
          row({
            id: 1,
            priorityTier: "autonomous",
            title: "Database connection timeout",
            openedAtUnixNano: NOW_NANO - 3 * MINUTE,
          }),
        ]}
        isOpen={true}
        onClose={() => {}}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={makeTriggerRef()}
        nowUnixNano={NOW_NANO}
      />,
    );
    expect(screen.getByTestId("findings-dropdown-row").getAttribute("aria-label")).toBe(
      "Autonomous: Database connection timeout, 3m ago",
    );
  });
});

describe("FindingsDropdown — footer action", () => {
  it("renders 'Mark all as read' button regardless of row count", () => {
    render(
      <FindingsDropdown
        rows={[]}
        isOpen={true}
        onClose={() => {}}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={makeTriggerRef()}
        nowUnixNano={NOW_NANO}
      />,
    );
    const footerBtn = screen.getByTestId("findings-dropdown-mark-all-read");
    expect(footerBtn.tagName).toBe("BUTTON");
    expect(footerBtn.textContent).toBe("Mark all as read");
  });

  it("invokes onMarkAllRead when the footer button is clicked", async () => {
    const onMarkAllRead = vi.fn();
    const user = userEvent.setup();
    render(
      <FindingsDropdown
        rows={[row()]}
        isOpen={true}
        onClose={() => {}}
        onMarkAllRead={onMarkAllRead}
        onRowClick={() => {}}
        triggerRef={makeTriggerRef()}
        nowUnixNano={NOW_NANO}
      />,
    );
    await user.click(screen.getByTestId("findings-dropdown-mark-all-read"));
    expect(onMarkAllRead).toHaveBeenCalledTimes(1);
  });
});

describe("FindingsDropdown — Escape closes + focus restoration", () => {
  it("invokes onClose + restores focus к trigger on Escape", async () => {
    const onClose = vi.fn();
    const user = userEvent.setup();
    const triggerRef = makeTriggerRef();
    render(
      <FindingsDropdown
        rows={[row()]}
        isOpen={true}
        onClose={onClose}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={triggerRef}
        nowUnixNano={NOW_NANO}
      />,
    );
    await user.keyboard("{Escape}");
    expect(onClose).toHaveBeenCalledTimes(1);
    expect(document.activeElement).toBe(triggerRef.current);
  });

  it("does NOT fire onClose on Escape when closed", async () => {
    const onClose = vi.fn();
    const user = userEvent.setup();
    render(
      <FindingsDropdown
        rows={[row()]}
        isOpen={false}
        onClose={onClose}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={makeTriggerRef()}
        nowUnixNano={NOW_NANO}
      />,
    );
    await user.keyboard("{Escape}");
    expect(onClose).not.toHaveBeenCalled();
  });
});

describe("FindingsDropdown — click-outside closes", () => {
  it("invokes onClose when mousedown fires outside the panel and trigger", async () => {
    const onClose = vi.fn();
    const triggerRef = makeTriggerRef();
    render(
      <FindingsDropdown
        rows={[row()]}
        isOpen={true}
        onClose={onClose}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={triggerRef}
        nowUnixNano={NOW_NANO}
      />,
    );
    const outside = document.createElement("div");
    document.body.appendChild(outside);
    outside.dispatchEvent(new MouseEvent("mousedown", { bubbles: true }));
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("does NOT close when click is inside the panel", () => {
    const onClose = vi.fn();
    render(
      <FindingsDropdown
        rows={[row()]}
        isOpen={true}
        onClose={onClose}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={makeTriggerRef()}
        nowUnixNano={NOW_NANO}
      />,
    );
    const panel = screen.getByTestId("findings-dropdown");
    panel.dispatchEvent(new MouseEvent("mousedown", { bubbles: true }));
    expect(onClose).not.toHaveBeenCalled();
  });

  it("does NOT close when click is on the trigger button", () => {
    const onClose = vi.fn();
    const triggerRef = makeTriggerRef();
    render(
      <FindingsDropdown
        rows={[row()]}
        isOpen={true}
        onClose={onClose}
        onMarkAllRead={() => {}}
        onRowClick={() => {}}
        triggerRef={triggerRef}
        nowUnixNano={NOW_NANO}
      />,
    );
    triggerRef.current?.dispatchEvent(new MouseEvent("mousedown", { bubbles: true }));
    expect(onClose).not.toHaveBeenCalled();
  });
});
