import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import { tabbable } from "tabbable";
import type { TemplateDistributionPayload } from "../../../bindings";
import {
  TemplateDistribution,
  __setProxyForTest,
} from "./TemplateDistribution";

function makePayload(
  overrides: Partial<TemplateDistributionPayload> = {},
): TemplateDistributionPayload {
  return {
    templates: [
      {
        id: 1,
        content: "user logged in from <IP>",
        occurrence_count: 42,
        drift_indicator: "Healthy",
      },
      {
        id: 2,
        content: "connection refused on port <NUM>",
        occurrence_count: 7,
        drift_indicator: "UnderClustered",
      },
      {
        id: 3,
        content: "<*> <*> <*> <*> <*>",
        occurrence_count: 99,
        drift_indicator: "OverGeneralized",
      },
    ],
    total_template_count: 3,
    last_updated_unix_nano: 1_700_000_000_000,
    ...overrides,
  };
}

let templateDistributionFn: ReturnType<typeof vi.fn>;

beforeEach(() => {
  templateDistributionFn = vi.fn().mockResolvedValue(makePayload());
  __setProxyForTest({
    diagnostics: { template_distribution: templateDistributionFn },
  } as never);
});

afterEach(() => {
  __setProxyForTest(null);
  vi.clearAllMocks();
  vi.unstubAllGlobals();
});

describe("TemplateDistribution", () => {
  describe("rendering states", () => {
    it("renders loading state initially with aria-busy='true'", () => {
      // Pending promise — keep mock unresolved to capture loading state.
      templateDistributionFn.mockReturnValueOnce(new Promise(() => {}));
      __setProxyForTest({
        diagnostics: { template_distribution: templateDistributionFn },
      } as never);
      render(<TemplateDistribution />);
      const loading = screen.getByTestId("template-distribution-loading");
      expect(loading.getAttribute("aria-busy")).toBe("true");
    });

    it("renders empty state when TauRPC returns zero templates", async () => {
      templateDistributionFn.mockResolvedValueOnce(
        makePayload({ templates: [], total_template_count: 0 }),
      );
      __setProxyForTest({
        diagnostics: { template_distribution: templateDistributionFn },
      } as never);
      render(<TemplateDistribution />);
      const empty = await screen.findByTestId("template-distribution-empty");
      expect(empty.textContent).toBe("No templates yet");
      // Chunk #99 re-audit: 12px body text on raised-1 needs 4.5:1, so the
      // empty state uses text-secondary (tertiary measured 3.92:1 — the
      // original "tertiary per design extract" encoded the violation).
      expect(empty.getAttribute("style")).toContain(
        "var(--color-text-secondary)",
      );
    });

    it("renders error state when TauRPC promise rejects (role='alert')", async () => {
      templateDistributionFn.mockRejectedValueOnce({
        kind: "storage",
        message: "drain template miner failed",
      });
      __setProxyForTest({
        diagnostics: { template_distribution: templateDistributionFn },
      } as never);
      render(<TemplateDistribution />);
      const err = await screen.findByTestId("template-distribution-error");
      expect(err.getAttribute("role")).toBe("alert");
      expect(err.textContent).toContain("drain template miner failed");
    });

    it("falls back to internal AppError on unknown rejection shape", async () => {
      templateDistributionFn.mockRejectedValueOnce(new Error("network down"));
      __setProxyForTest({
        diagnostics: { template_distribution: templateDistributionFn },
      } as never);
      render(<TemplateDistribution />);
      const err = await screen.findByTestId("template-distribution-error");
      expect(err.textContent).toContain(
        "diagnostics.template_distribution failed",
      );
    });
  });

  describe("table rendering", () => {
    it("renders semantic <table> with <thead> + <tbody> + <th scope='col'>", async () => {
      render(<TemplateDistribution />);
      const table = await screen.findByTestId("template-distribution-table");
      expect(table.tagName).toBe("TABLE");
      const thead = table.querySelector("thead");
      const tbody = table.querySelector("tbody");
      expect(thead).not.toBeNull();
      expect(tbody).not.toBeNull();
      const headers = table.querySelectorAll("th");
      for (const th of headers) {
        expect(th.getAttribute("scope")).toBe("col");
      }
    });

    it("renders one row per template returned by TauRPC", async () => {
      render(<TemplateDistribution />);
      const table = await screen.findByTestId("template-distribution-table");
      const dataRows = table.querySelectorAll("tbody tr");
      expect(dataRows.length).toBe(3);
    });

    it("caps rendered rows at 50 even when TauRPC returns more", async () => {
      const bigTemplates = Array.from({ length: 75 }, (_, i) => ({
        id: i + 1,
        content: `event_${i}`,
        occurrence_count: 100 - i,
        drift_indicator: "Healthy" as const,
      }));
      templateDistributionFn.mockResolvedValueOnce(
        makePayload({
          templates: bigTemplates,
          total_template_count: bigTemplates.length,
        }),
      );
      __setProxyForTest({
        diagnostics: { template_distribution: templateDistributionFn },
      } as never);
      render(<TemplateDistribution />);
      const table = await screen.findByTestId("template-distribution-table");
      const dataRows = table.querySelectorAll("tbody tr");
      expect(dataRows.length).toBeLessThanOrEqual(50);
    });

    it("renders Template ID / Count / Drift / Sample message headers", async () => {
      render(<TemplateDistribution />);
      const table = await screen.findByTestId("template-distribution-table");
      const headerTexts = Array.from(table.querySelectorAll("thead th")).map(
        (h) => h.textContent,
      );
      expect(headerTexts).toEqual([
        "Template ID",
        "Count",
        "Drift",
        "Sample message",
      ]);
    });

    it("renders the showing-N-of-M counter when data present", async () => {
      render(<TemplateDistribution />);
      const data = await screen.findByTestId("template-distribution-data");
      expect(data.textContent).toContain("Showing top 3 of 3 templates");
    });
  });

  describe("drift indicator (SC 1.4.1 not-color-alone)", () => {
    it("pairs each drift indicator color with a text label", async () => {
      render(<TemplateDistribution />);
      const table = await screen.findByTestId("template-distribution-table");
      const indicators = table.querySelectorAll("[data-drift-indicator]");
      expect(indicators.length).toBe(3);
      const labels = Array.from(indicators).map((el) => el.textContent);
      expect(labels).toEqual(
        expect.arrayContaining([
          expect.stringContaining("Healthy"),
          expect.stringContaining("Over-generalized"),
          expect.stringContaining("Under-clustered"),
        ]),
      );
    });

    it("flags each drift indicator with the canonical data attribute", async () => {
      render(<TemplateDistribution />);
      const table = await screen.findByTestId("template-distribution-table");
      const indicatorValues = Array.from(
        table.querySelectorAll("[data-drift-indicator]"),
      ).map((el) => el.getAttribute("data-drift-indicator"));
      expect(indicatorValues).toEqual(
        expect.arrayContaining(["Healthy", "OverGeneralized", "UnderClustered"]),
      );
    });
  });

  describe("a11y compliance", () => {
    it("introduces no new focusable elements (read-only panel — no keyboard trap risk)", async () => {
      const { container } = render(<TemplateDistribution />);
      await screen.findByTestId("template-distribution-table");
      const focusables = tabbable(container);
      expect(focusables).toEqual([]);
    });

    it("uses design-token color variables (no hardcoded hex in sample data row colors)", async () => {
      render(<TemplateDistribution />);
      const table = await screen.findByTestId("template-distribution-table");
      const indicator = table.querySelector("[data-drift-indicator='Healthy']");
      expect(indicator).not.toBeNull();
      const styleAttr = indicator!.getAttribute("style") ?? "";
      // Healthy uses --color-feedback-success per driftColorVar()
      expect(styleAttr).toContain("var(--color-feedback-success)");
      // No hardcoded hex appears in the data row inline styles.
      expect(styleAttr).not.toMatch(/#[0-9a-fA-F]{3,8}\b/);
    });
  });

  describe("sample message column", () => {
    it("renders the title attribute on the sample message cell for overflow tooltip", async () => {
      render(<TemplateDistribution />);
      const table = await screen.findByTestId("template-distribution-table");
      const rows = table.querySelectorAll("tbody tr");
      const firstRowSampleCell = rows[0].querySelectorAll("td")[3];
      expect(firstRowSampleCell.getAttribute("title")).toBe(
        "user logged in from <IP>",
      );
    });
  });

  describe("invocation", () => {
    it("invokes diagnostics.template_distribution exactly once on mount", async () => {
      render(<TemplateDistribution />);
      await waitFor(() => {
        expect(templateDistributionFn).toHaveBeenCalledTimes(1);
      });
    });

    it("does not pass any arguments to template_distribution()", async () => {
      render(<TemplateDistribution />);
      await waitFor(() => {
        expect(templateDistributionFn).toHaveBeenCalledWith();
      });
    });
  });
});
