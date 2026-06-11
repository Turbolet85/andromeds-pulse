#!/usr/bin/env node
// Chunk #99 — pa11y-ci stage wrapper. The npm chain previously invoked
// bare `pa11y-ci` with NO server (the playwright stage's webServer exits
// with playwright), so the stage died on ERR_CONNECTION_REFUSED the first
// time the chain ever reached it. This wrapper owns the static-server
// lifecycle around the pa11y-ci run and propagates its exit code.
import { spawn } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { startStaticServer } from "../helpers/static-server.mjs";

const ROOT = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const PORT = process.env.A11Y_PA11Y_PORT ?? "4173";

async function main() {
  let server;
  try {
    server = await startStaticServer(ROOT, PORT);
  } catch (e) {
    console.error(`run-pa11y: server start failed (${e.message})`);
    process.exit(1);
  }
  const bin = join(ROOT, "node_modules", "pa11y-ci", "bin", "pa11y-ci.js");
  const child = spawn(
    process.execPath,
    [bin, "--config", join(ROOT, "tests-a11y", "pa11y", "pa11y-ci-config.json")],
    { cwd: ROOT, stdio: "inherit", shell: false },
  );
  const code = await new Promise((resolve) => {
    child.on("close", resolve);
    child.on("error", () => resolve(1));
  });
  server.kill();
  process.exit(code ?? 1);
}

main().catch((e) => {
  console.error(`run-pa11y: ${e.stack ?? e.message}`);
  process.exit(1);
});
