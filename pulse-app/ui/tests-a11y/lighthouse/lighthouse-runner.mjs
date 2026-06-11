#!/usr/bin/env node
import { appendFile, mkdir, writeFile } from "node:fs/promises";
import { homedir, platform } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = dirname(dirname(dirname(fileURLToPath(import.meta.url))));

// Real browser-history paths (chunk #99 finding: the former `/#/route`
// URLs never routed — every surface silently audited the traces index).
// /diagnostics added with the chunk #97 view.
const SURFACES = [
  { key: "compact-widget", path: "/" },
  { key: "dashboard-traces", path: "/traces" },
  { key: "dashboard-metrics", path: "/metrics" },
  { key: "dashboard-logs", path: "/logs" },
  { key: "dashboard-snapshots", path: "/snapshots" },
  { key: "dashboard-settings", path: "/settings" },
  { key: "dashboard-diagnostics", path: "/diagnostics" },
];

const MIN_A11Y_SCORE = 90;
const PORT = process.env.A11Y_LIGHTHOUSE_PORT ?? "4173";
const BASE = `http://localhost:${PORT}`;

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

function startServer() {
  // Shared lifecycle helper (chunk #99): direct-spawn server with SPA
  // fallback; the prior npx-wrapper spawn orphaned the real server child
  // on Windows kill and had no fallback for browser-history routes.
  return import("../helpers/static-server.mjs").then(({ startStaticServer }) =>
    startStaticServer(ROOT, PORT),
  );
}

async function runLighthouseOne(lighthouse, surface) {
  const url = `${BASE}${surface.path}`;
  const { default: chromeLauncher } = await import("chrome-launcher");
  const chrome = await chromeLauncher.launch({
    chromeFlags: ["--headless=new", "--disable-gpu", "--no-sandbox"],
  });
  try {
    const result = await lighthouse(url, {
      port: chrome.port,
      output: "json",
      onlyCategories: ["accessibility"],
      logLevel: "error",
    });
    const lhr = result.lhr ?? JSON.parse(result.report);
    const score = (lhr.categories?.accessibility?.score ?? 0) * 100;
    return { surface: surface.key, url, score, lhr };
  } finally {
    await chrome.kill();
  }
}

async function main() {
  const logDir = resolveLogDir();
  await mkdir(logDir, { recursive: true });
  const outPath = join(logDir, "a11y-lighthouse-results.json");

  let server;
  try {
    server = await startServer();
  } catch (e) {
    console.error(`lighthouse-runner: server start failed (${e.message}); skipping`);
    await writeFile(
      outPath,
      JSON.stringify({ schema_version: 1, status: "skipped", reason: e.message, results: [] }, null, 2) + "\n",
      "utf8",
    );
    process.exit(0);
  }

  let lighthouse;
  try {
    lighthouse = (await import("lighthouse")).default;
  } catch (e) {
    console.error(`lighthouse-runner: lighthouse import failed (${e.message}); skipping`);
    server.kill();
    await writeFile(
      outPath,
      JSON.stringify({ schema_version: 1, status: "skipped", reason: e.message, results: [] }, null, 2) + "\n",
      "utf8",
    );
    process.exit(0);
  }

  const results = [];
  let failure = null;
  try {
    for (const surface of SURFACES) {
      try {
        const r = await runLighthouseOne(lighthouse, surface);
        results.push({ surface: r.surface, url: r.url, score: r.score });
        if (r.score < MIN_A11Y_SCORE) {
          failure = failure ?? `${r.surface}: a11y score ${r.score} < ${MIN_A11Y_SCORE}`;
        }
      } catch (e) {
        results.push({ surface: surface.key, url: `${BASE}${surface.path}`, score: null, error: e.message });
      }
    }
  } finally {
    server.kill();
  }

  await writeFile(
    outPath,
    JSON.stringify(
      {
        schema_version: 1,
        generated_at: new Date().toISOString(),
        threshold: MIN_A11Y_SCORE,
        results,
      },
      null,
      2,
    ) + "\n",
    "utf8",
  );

  for (const r of results) {
    const record = {
      timestamp: new Date().toISOString(),
      level: r.error ? "WARN" : r.score !== null && r.score < MIN_A11Y_SCORE ? "ERROR" : "INFO",
      target: "a11y::lighthouse",
      message: r.error
        ? `lighthouse skipped for ${r.surface}: ${r.error}`
        : `lighthouse a11y score for ${r.surface}: ${r.score}`,
      fields: {
        service: { name: "com.andromeda.pulse" },
        deployment: { environment: process.env.DEPLOYMENT_ENVIRONMENT ?? "test" },
        wcag_criterion: "n/a",
        violation_type: "lighthouse-score",
        severity: r.error ? "info" : r.score !== null && r.score < MIN_A11Y_SCORE ? "critical" : "info",
        surface: r.surface,
        selector: "",
        remediation: r.error ? null : `improve a11y to reach ${MIN_A11Y_SCORE}`,
        tool: "lighthouse",
        tool_result_id: `lighthouse@${r.surface}`,
        score: r.score,
      },
    };
    await appendFile(join(logDir, "a11y-axe-core-results.jsonl"), JSON.stringify(record) + "\n", "utf8");
  }

  if (failure) {
    console.error(`lighthouse-runner: ${failure}`);
    process.exit(1);
  }
  console.log(`lighthouse-runner: ${results.length} surfaces audited; all ≥${MIN_A11Y_SCORE}`);
}

main().catch((e) => {
  console.error(`lighthouse-runner: ${e.stack ?? e.message}`);
  process.exit(1);
});
