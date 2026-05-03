#!/usr/bin/env node
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Color from "colorjs.io";

import { parseTokens } from "../src/contrast/parse-tokens.mjs";
import { PAIRS } from "../src/contrast/pairs.mjs";

const ROOT = dirname(dirname(fileURLToPath(import.meta.url)));
const TOKENS_CSS = join(ROOT, "dist", "tokens.css");
const REPORT_JSON = join(ROOT, "dist", "contrast-report.json");

const css = readFileSync(TOKENS_CSS, "utf8");
const tokens = parseTokens(css);

const records = PAIRS.map((p) => {
  const fgHex = tokens.get(p.fg_token);
  const bgHex = tokens.get(p.bg_token);
  if (!fgHex || !bgHex) {
    throw new Error(
      `Missing token in ${TOKENS_CSS}: ${p.fg_token}=${fgHex}, ${p.bg_token}=${bgHex}`,
    );
  }
  const ratio = Math.abs(new Color(fgHex).contrast(new Color(bgHex), "WCAG21"));
  const computed = Number(ratio.toFixed(2));
  const meets = computed >= p.target_ratio;
  const pass = p.severity === "informational" ? true : meets;
  const tool_result_id = `${p.pair_name}@${fgHex}-on-${bgHex}`;
  return {
    timestamp: new Date().toISOString(),
    level: pass ? "INFO" : "ERROR",
    target: "a11y::contrast",
    message: pass
      ? `WCAG ${p.wcag_criterion} pass for ${p.pair_name}`
      : `WCAG ${p.wcag_criterion} violation for ${p.pair_name}`,
    fields: {
      service: { name: "com.andromeda.pulse" },
      deployment: { environment: "production" },
      wcag_criterion: p.wcag_criterion,
      violation_type: "color-contrast",
      severity: p.severity,
      surface: "desktop-webview",
      token_name: p.pair_name,
      actual_contrast_ratio: computed,
      required_ratio: p.target_ratio,
      remediation: meets
        ? null
        : `adjust ${p.fg_token} or ${p.bg_token} to meet ${p.target_ratio}:1`,
      tool: "colorjs.io",
      tool_result_id,
      measured_value: computed,
      required_value: p.target_ratio,
      usage: p.usage,
      wcag_level: p.wcag_level,
      pass,
    },
  };
});

const report = {
  schema_version: 1,
  generated_at: new Date().toISOString(),
  records,
};
writeFileSync(REPORT_JSON, JSON.stringify(report, null, 2) + "\n");

const failures = records.filter(
  (r) => r.fields.severity === "critical" && !r.fields.pass,
);
if (failures.length > 0) {
  console.error(
    `verify-contrast: ${failures.length} critical failure(s) — see ${REPORT_JSON}`,
  );
  for (const r of failures) {
    console.error(
      `  ${r.fields.token_name}: ${r.fields.actual_contrast_ratio}:1 < ${r.fields.required_ratio}:1 (${r.fields.wcag_criterion})`,
    );
  }
  process.exit(1);
}
console.log(
  `verify-contrast: ${records.length} pairs evaluated, all critical pairs pass — see ${REPORT_JSON}`,
);
process.exit(0);
