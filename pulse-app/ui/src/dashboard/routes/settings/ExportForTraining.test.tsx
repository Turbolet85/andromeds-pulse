import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { ExportForTraining, __setProxyForTest } from "./ExportForTraining";

type Category = { dimension: string; label: string; count: number };
type ExportPreviewPayload = {
  categories: Category[];
  total_records: number;
  date_range_start_unix_nano: number | null;
  date_range_end_unix_nano: number | null;
  anonymization_confirmed: boolean;
  written: boolean;
  written_path_basename: string | null;
};

const previewPayload: ExportPreviewPayload = {
  categories: [
    { dimension: "status", label: "active", count: 1 },
    { dimension: "status", label: "resolved", count: 2 },
    { dimension: "severity", label: "error", count: 3 },
  ],
  total_records: 3,
  date_range_start_unix_nano: 1_000_000_000_000,
  date_range_end_unix_nano: 1_000_000_000_000,
  anonymization_confirmed: true,
  written: false,
  written_path_basename: null,
};

function setProxy(exportFn: ReturnType<typeof vi.fn>): void {
  __setProxyForTest({
    storage: { export_for_training: exportFn },
  } as unknown as Parameters<typeof __setProxyForTest>[0]);
}

afterEach(() => {
  cleanup();
  __setProxyForTest(null);
});

describe("ExportForTraining", () => {
  it("opens the preview dialog with the anonymization indicator + category summary", async () => {
    const exportFn = vi.fn().mockResolvedValue(previewPayload);
    setProxy(exportFn);
    const user = userEvent.setup();
    render(<ExportForTraining />);

    await user.click(screen.getByTestId("export-training-trigger"));

    await waitFor(() => {
      expect(screen.getByTestId("modal-dialog")).toBeTruthy();
    });
    // Preview is the no-write path.
    expect(exportFn).toHaveBeenCalledWith(null, false);
    // Not-color-alone: the anonymization indicator carries a text label.
    expect(
      screen.getByTestId("export-anonymization-confirmed").textContent,
    ).toContain("Anonymized");
    expect(
      screen.getByTestId("export-category-breakdown").textContent,
    ).toContain("3");
  });

  it("does not write a file on preview (preview-before-write gate)", async () => {
    const exportFn = vi.fn().mockResolvedValue(previewPayload);
    setProxy(exportFn);
    const user = userEvent.setup();
    render(<ExportForTraining />);

    await user.click(screen.getByTestId("export-training-trigger"));
    await waitFor(() => screen.getByTestId("modal-dialog"));

    // Only the preview (confirm=false) call — no write triggered.
    expect(exportFn).toHaveBeenCalledTimes(1);
    expect(exportFn).toHaveBeenCalledWith(null, false);
  });

  it("writes on confirm and announces success (no auto-submission)", async () => {
    const exportFn = vi
      .fn()
      .mockResolvedValueOnce(previewPayload)
      .mockResolvedValueOnce({
        ...previewPayload,
        written: true,
        written_path_basename: "pulse-corpus-export-123.jsonl",
      });
    setProxy(exportFn);
    const user = userEvent.setup();
    render(<ExportForTraining />);

    await user.click(screen.getByTestId("export-training-trigger"));
    await waitFor(() => screen.getByTestId("export-training-confirm"));
    await user.click(screen.getByTestId("export-training-confirm"));

    await waitFor(() => {
      expect(
        screen.getByTestId("export-training-status").textContent,
      ).toContain("pulse-corpus-export-123.jsonl");
    });
    // Second call is the write path (confirm=true).
    expect(exportFn).toHaveBeenNthCalledWith(2, null, true);
  });

  it("surfaces an export error via role=alert", async () => {
    const exportFn = vi
      .fn()
      .mockRejectedValue({ kind: "storage", message: "export file write failed" });
    setProxy(exportFn);
    const user = userEvent.setup();
    render(<ExportForTraining />);

    await user.click(screen.getByTestId("export-training-trigger"));

    await waitFor(() => {
      expect(
        screen.getByTestId("export-training-error").textContent,
      ).toContain("export file write failed");
    });
  });
});
