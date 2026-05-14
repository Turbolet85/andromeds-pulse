#!/usr/bin/env node
import { readFile, writeFile, mkdir, access } from "node:fs/promises";
import { homedir, platform } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = dirname(dirname(fileURLToPath(import.meta.url)));

const CANONICAL_SURFACES = [
  "compact-widget",
  "dashboard-traces",
  "dashboard-metrics",
  "dashboard-logs",
  "dashboard-snapshots",
  "dashboard-settings",
  "tray-icon-halo",
];

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

async function readJsonl(path) {
  if (!(await exists(path))) return [];
  const content = await readFile(path, "utf8");
  return content
    .split("\n")
    .filter((line) => line.trim().length > 0)
    .map((line) => {
      try {
        return JSON.parse(line);
      } catch {
        return null;
      }
    })
    .filter((v) => v !== null);
}

async function readJson(path) {
  if (!(await exists(path))) return null;
  try {
    return JSON.parse(await readFile(path, "utf8"));
  } catch {
    return null;
  }
}

function tupleFromAxeRecord(rec) {
  const f = rec?.fields;
  if (!f) return null;
  return {
    surface: f.surface ?? "unknown",
    wcag_criterion: f.wcag_criterion ?? "unknown",
    selector: f.selector ?? "",
    severity: f.severity ?? "info",
    tool: f.tool ?? "unknown",
    violation_type: f.violation_type ?? "unknown",
  };
}

function tupleFromContrastRecord(rec) {
  const f = rec?.fields;
  if (!f) return null;
  if (f.pass === true) return null;
  return {
    surface: f.surface ?? "desktop-webview",
    wcag_criterion: f.wcag_criterion ?? "unknown",
    selector: f.token_name ?? "",
    severity: f.severity ?? "info",
    tool: f.tool ?? "colorjs.io",
    violation_type: f.violation_type ?? "color-contrast",
  };
}

function tupleFromPa11yIssue(issue, url) {
  const surface = surfaceFromUrl(url);
  return {
    surface,
    wcag_criterion: issue.code ?? "unknown",
    selector: issue.selector ?? "",
    severity: issue.type === "error" ? "serious" : "moderate",
    tool: "pa11y",
    violation_type: issue.code ?? "unknown",
  };
}

function surfaceFromUrl(url) {
  if (!url) return "unknown";
  if (url.endsWith("/")) return "compact-widget";
  const m = /#\/([a-z-]+)/.exec(url);
  if (m) return `dashboard-${m[1]}`;
  return "unknown";
}

async function main() {
  const logDir = resolveLogDir();
  await mkdir(logDir, { recursive: true });

  const perSurface = Object.fromEntries(CANONICAL_SURFACES.map((s) => [s, []]));

  const axeRecords = await readJsonl(join(logDir, "a11y-axe-core-results.jsonl"));
  for (const rec of axeRecords) {
    const t = tupleFromAxeRecord(rec);
    if (!t) continue;
    if (t.violation_type === "none") continue;
    if (perSurface[t.surface]) perSurface[t.surface].push(t);
  }

  const contrastReport = await readJson(join(ROOT, "dist", "contrast-report.json"));
  if (contrastReport?.records) {
    for (const rec of contrastReport.records) {
      const t = tupleFromContrastRecord(rec);
      if (!t) continue;
      const key = perSurface[t.surface] ? t.surface : "dashboard-traces";
      perSurface[key].push(t);
    }
  }

  const pa11yResults = await readJson(join(logDir, "a11y-pa11y-results.json"));
  if (Array.isArray(pa11yResults?.results?.documentTitle ?? null)) {
    // pa11y-ci array form
  }
  if (Array.isArray(pa11yResults)) {
    for (const entry of pa11yResults) {
      for (const issue of entry.issues ?? []) {
        const t = tupleFromPa11yIssue(issue, entry.pageUrl ?? entry.documentTitle ?? "");
        if (perSurface[t.surface]) perSurface[t.surface].push(t);
      }
    }
  } else if (pa11yResults?.results) {
    for (const [url, entry] of Object.entries(pa11yResults.results)) {
      for (const issue of entry.issues ?? entry ?? []) {
        const t = tupleFromPa11yIssue(issue, url);
        if (perSurface[t.surface]) perSurface[t.surface].push(t);
      }
    }
  }

  for (const surface of Object.keys(perSurface)) {
    perSurface[surface].sort((a, b) => {
      const ak = `${a.wcag_criterion}|${a.selector}|${a.violation_type}`;
      const bk = `${b.wcag_criterion}|${b.selector}|${b.violation_type}`;
      return ak.localeCompare(bk);
    });
  }

  const total = Object.values(perSurface).reduce((acc, arr) => acc + arr.length, 0);
  const summary = {
    schema_version: 1,
    generated_at: new Date().toISOString(),
    total_tuple_count: total,
    per_surface: perSurface,
  };
  const outPath = join(logDir, "a11y-violations-summary.json");
  await writeFile(outPath, JSON.stringify(summary, null, 2) + "\n", "utf8");
  console.log(
    `aggregator: ${total} violation tuple(s) across ${CANONICAL_SURFACES.length} surfaces → ${outPath}`,
  );
}

main().catch((e) => {
  console.error(`aggregator: ${e.stack ?? e.message}`);
  process.exit(1);
});
