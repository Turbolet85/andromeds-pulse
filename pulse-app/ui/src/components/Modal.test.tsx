import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useRef, type ReactNode } from "react";
import { tabbable } from "tabbable";
import { hasReducedMotionListener, prefersReducedMotion } from "motion-dom";
import { mockReducedMotion } from "../hooks/mock-reduced-motion";
import { Modal } from "./Modal";

interface HarnessProps {
  open: boolean;
  onClose: () => void;
  busy?: boolean;
  liveRegionLevel?: "polite" | "assertive";
  liveMessage?: string;
  clickOutsideDeactivates?: boolean;
  childContent?: ReactNode;
}

function Harness({
  open,
  onClose,
  busy,
  liveRegionLevel,
  liveMessage,
  clickOutsideDeactivates,
  childContent,
}: HarnessProps) {
  const triggerRef = useRef<HTMLButtonElement | null>(null);
  return (
    <>
      <button ref={triggerRef} type="button" data-testid="modal-trigger">
        open modal
      </button>
      <Modal
        open={open}
        onClose={onClose}
        triggerRef={triggerRef}
        titleId="test-modal-title"
        title="Test Modal"
        busy={busy}
        liveRegionLevel={liveRegionLevel}
        liveMessage={liveMessage}
        clickOutsideDeactivates={clickOutsideDeactivates}
      >
        {childContent ?? <p>Modal content</p>}
      </Modal>
    </>
  );
}

describe("Modal", () => {
  beforeEach(() => {
    hasReducedMotionListener.current = false;
    prefersReducedMotion.current = null;
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  describe("open/close lifecycle", () => {
    it("renders nothing when open=false", () => {
      render(<Harness open={false} onClose={vi.fn()} />);
      expect(screen.queryByRole("dialog")).toBeNull();
    });

    it("renders dialog with aria-modal='true' when open=true", () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      const dialog = screen.getByRole("dialog");
      expect(dialog.getAttribute("aria-modal")).toBe("true");
    });

    it("renders dialog with aria-labelledby pointing to the title heading", () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      const dialog = screen.getByRole("dialog");
      expect(dialog.getAttribute("aria-labelledby")).toBe("test-modal-title");
      const title = document.getElementById("test-modal-title");
      expect(title).not.toBeNull();
      expect(title?.textContent).toBe("Test Modal");
    });

    it("renders <h2> heading with the title prop", () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      const heading = screen.getByRole("heading", { name: "Test Modal" });
      expect(heading.tagName).toBe("H2");
    });
  });

  describe("close button", () => {
    it("renders a button with accessible name 'Close'", () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      const closeButton = screen.getByRole("button", { name: "Close" });
      expect(closeButton).toBeDefined();
    });

    it("triggers onClose when clicked", async () => {
      const onClose = vi.fn();
      const user = userEvent.setup();
      render(<Harness open={true} onClose={onClose} />);
      await user.click(screen.getByRole("button", { name: "Close" }));
      expect(onClose).toHaveBeenCalled();
    });
  });

  describe("Esc key", () => {
    it("Esc triggers onClose", async () => {
      const onClose = vi.fn();
      const user = userEvent.setup();
      render(<Harness open={true} onClose={onClose} />);
      await user.keyboard("{Escape}");
      expect(onClose).toHaveBeenCalled();
    });
  });

  describe("focus trap", () => {
    it("close button is reachable via tabbable() enumeration of the dialog", () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      const dialog = screen.getByRole("dialog");
      // jsdom returns 0×0 getBoundingClientRect for all elements; without
      // displayCheck: 'none', tabbable filters everything out as not-visible.
      // Per .claude/rules/testing.md Session Additions 2026-05-09 last entry.
      const focusables = tabbable(dialog, { displayCheck: "none" });
      const closeButton = screen.getByRole("button", { name: "Close" });
      expect(focusables).toContain(closeButton);
    });
  });

  describe("aria-busy hook", () => {
    it("reflects aria-busy='false' when busy is undefined (default)", () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      expect(screen.getByRole("dialog").getAttribute("aria-busy")).toBe("false");
    });

    it("reflects aria-busy='true' when busy=true", () => {
      render(<Harness open={true} onClose={vi.fn()} busy={true} />);
      expect(screen.getByRole("dialog").getAttribute("aria-busy")).toBe("true");
    });

    it("reflects aria-busy='false' when busy=false", () => {
      render(<Harness open={true} onClose={vi.fn()} busy={false} />);
      expect(screen.getByRole("dialog").getAttribute("aria-busy")).toBe("false");
    });
  });

  describe("aria-live hook", () => {
    it("renders a status region with default polite aria-live", () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      const status = screen.getByRole("status");
      expect(status.getAttribute("aria-live")).toBe("polite");
    });

    it("reflects aria-live='assertive' when liveRegionLevel='assertive'", () => {
      render(
        <Harness open={true} onClose={vi.fn()} liveRegionLevel="assertive" />,
      );
      const status = screen.getByRole("status");
      expect(status.getAttribute("aria-live")).toBe("assertive");
    });

    it("renders liveMessage in the status region", () => {
      render(
        <Harness
          open={true}
          onClose={vi.fn()}
          liveMessage="Saving settings..."
        />,
      );
      const status = screen.getByRole("status");
      expect(status.textContent).toBe("Saving settings...");
    });

    it("renders empty status region when liveMessage absent", () => {
      render(<Harness open={true} onClose={vi.fn()} />);
      const status = screen.getByRole("status");
      expect(status.textContent).toBe("");
    });
  });

  describe("reduced motion", () => {
    it("applies motion-reduce class to the overlay backdrop", () => {
      mockReducedMotion(true);
      render(<Harness open={true} onClose={vi.fn()} />);
      const overlay = screen.getByTestId("modal-overlay");
      expect(overlay.className).toContain("motion-reduce:transition-none");
      expect(overlay.className).toContain("motion-reduce:duration-0");
    });
  });

  describe("click-outside dismissal", () => {
    it("does NOT call onClose on backdrop click when clickOutsideDeactivates is unset (default false)", async () => {
      const onClose = vi.fn();
      const user = userEvent.setup();
      render(<Harness open={true} onClose={onClose} />);
      const overlay = screen.getByTestId("modal-overlay");
      await user.click(overlay);
      expect(onClose).not.toHaveBeenCalled();
    });

    it("calls onClose on backdrop click when clickOutsideDeactivates=true", async () => {
      const onClose = vi.fn();
      const user = userEvent.setup();
      render(
        <Harness
          open={true}
          onClose={onClose}
          clickOutsideDeactivates={true}
        />,
      );
      const overlay = screen.getByTestId("modal-overlay");
      await user.click(overlay);
      expect(onClose).toHaveBeenCalled();
    });

    it("does NOT call onClose when clicking inner content (event bubbles up but target is inner element)", async () => {
      const onClose = vi.fn();
      const user = userEvent.setup();
      render(
        <Harness
          open={true}
          onClose={onClose}
          clickOutsideDeactivates={true}
          childContent={<p data-testid="inner-content">click me</p>}
        />,
      );
      await user.click(screen.getByTestId("inner-content"));
      expect(onClose).not.toHaveBeenCalled();
    });
  });
});
