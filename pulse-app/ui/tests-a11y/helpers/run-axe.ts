import AxeBuilder from "@axe-core/playwright";
import { expect, type Page } from "@playwright/test";
import { emitViolations } from "./emit-jsonl";

export interface AxeSweepOptions {
  surface: string;
  url?: string;
  setup?: (page: Page) => Promise<void>;
}

export async function runAxeSweep(page: Page, opts: AxeSweepOptions): Promise<void> {
  await page.goto(opts.url ?? "/");
  if (opts.setup) {
    await opts.setup(page);
  }
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  await emitViolations({ surface: opts.surface, violations: results.violations });
  const blocking = results.violations.filter(
    (v) => v.impact === "critical" || v.impact === "serious",
  );
  expect(
    blocking,
    `Surface "${opts.surface}" has ${blocking.length} critical/serious violation(s): ${blocking
      .map((v) => `${v.id}@${v.impact}`)
      .join(", ")}`,
  ).toEqual([]);
}
