#!/usr/bin/env node
import { readFile, writeFile, mkdir, access } from "node:fs/promises";
import { homedir, platform } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { parseArgs } from "node:util";

const ROOT = dirname(dirname(fileURLToPath(import.meta.url)));
const DEFAULT_BASELINE = join(ROOT, "tests-a11y", "baselines", "a11y-violations-summary.json");

function resolveLogDir() {
  const override = process.env.ANDROMEDA_PULSE_DATA_DIR;
  if (override) return join(override, "logs");
  if (platform() === "win32") {
    const appdata = process.env.APPDATA;
    if (appdata) return join(appdata, "andromeda-pulse", "logs");
  } else if (platform() === "darwin") {
    return join(
      homedir(),
      "Library",
      "Application Support",
      "com.andromeda.pulse",
      "logs",
    );
  } else {
    const xdg = process.env.XDG_CONFIG_HOME;
    if (xdg) return join(xdg, "andromeda-pulse", "logs");
  }
  return join(homedir(), ".andromeda-pulse", "logs");
}

async function exists(path) {
  try {
    await access(path);
    return true;
  } catch {
    return false;
  }
}

function tupleKey(t) {
  return `${t.surface}|${t.wcag_criterion}|${t.selector}|${t.severity}|${t.violation_type}`;
}

async function main() {
  const { values } = parseArgs({
    options: {
      baseline: { type: "string" },
      current: { type: "string" },
    },
    strict: false,
  });

  const logDir = resolveLogDir();
  const currentPath = values.current ?? join(logDir, "a11y-violations-summary.json");
  const baselinePath = values.baseline ?? DEFAULT_BASELINE;

  if (!(await exists(currentPath))) {
    console.error(`regression-detector: current summary not found at ${currentPath}`);
    process.exit(1);
  }
  if (!(await exists(baselinePath))) {
    console.error(`regression-detector: baseline not found at ${baselinePath}; treating as empty baseline`);
  }

  const current = JSON.parse(await readFile(currentPath, "utf8"));
  const baseline = (await exists(baselinePath))
    ? JSON.parse(await readFile(baselinePath, "utf8"))
    : { per_surface: {} };

  const baselineKeys = new Set();
  for (const tuples of Object.values(baseline.per_surface ?? {})) {
    for (const t of tuples) baselineKeys.add(tupleKey(t));
  }

  const regressions = {};
  let regressionCount = 0;
  for (const [surface, tuples] of Object.entries(current.per_surface ?? {})) {
    const newOnes = tuples.filter((t) => !baselineKeys.has(tupleKey(t)));
    if (newOnes.length > 0) {
      regressions[surface] = newOnes;
      regressionCount += newOnes.length;
    }
  }

  const report = {
    schema_version: 1,
    generated_at: new Date().toISOString(),
    baseline_path: baselinePath,
    current_path: currentPath,
    regression_count: regressionCount,
    per_surface: regressions,
  };

  const outPath = join(ROOT, "tests-a11y", "regression-set.json");
  await mkdir(dirname(outPath), { recursive: true });
  await writeFile(outPath, JSON.stringify(report, null, 2) + "\n", "utf8");

  if (regressionCount > 0) {
    console.error(`regression-detector: ${regressionCount} new violation tuple(s) NOT in baseline`);
    for (const [surface, tuples] of Object.entries(regressions)) {
      console.error(`  ${surface}: ${tuples.length} new tuple(s)`);
      for (const t of tuples) {
        console.error(`    - ${t.wcag_criterion} ${t.violation_type} on ${t.selector}`);
      }
    }
    console.error(`Full diff at: ${outPath}`);
    process.exit(1);
  }
  console.log(`regression-detector: no new violations vs baseline (${regressionCount}/0) → ${outPath}`);
}

main().catch((e) => {
  console.error(`regression-detector: ${e.stack ?? e.message}`);
  process.exit(1);
});
