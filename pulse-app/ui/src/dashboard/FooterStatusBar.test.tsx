import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { FooterStatusBar } from "./FooterStatusBar";

// Mock the live-data hooks so the isolated footer render never reaches the real
// TauRPC proxy in jsdom; the line renders its honest empty state.
vi.mock("../hooks/use-service-constellation", () => ({ useServiceConstellation: () => [] }));
vi.mock("../hooks/use-connection-state", () => ({ useConnectionState: () => null }));
vi.mock("../hooks/use-ingest-stats", () => ({ useIngestStats: () => null }));

describe("FooterStatusBar", () => {
  it("renders a <footer> landmark", () => {
    render(<FooterStatusBar />);
    const footer = screen.getByRole("contentinfo");
    expect(footer.tagName).toBe("FOOTER");
  });

  it("uses the data-testid 'dashboard-footer'", () => {
    render(<FooterStatusBar />);
    expect(screen.getByTestId("dashboard-footer")).toBeDefined();
  });

  it("mounts the plain-language connection status line", () => {
    render(<FooterStatusBar />);
    expect(screen.getByTestId("connection-status-line")).toBeDefined();
  });
});
