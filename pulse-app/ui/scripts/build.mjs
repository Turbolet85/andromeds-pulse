#!/usr/bin/env node
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, rmSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const ROOT = dirname(dirname(fileURLToPath(import.meta.url)));
const DIST = join(ROOT, "dist");
const SRC_TOKENS = join(ROOT, "src", "styles", "tokens.css");
const OUT_TOKENS = join(DIST, "tokens.css");

if (existsSync(DIST)) rmSync(DIST, { recursive: true, force: true });
mkdirSync(DIST, { recursive: true });

const tailwindEntry = join(ROOT, "node_modules", "@tailwindcss", "cli", "dist", "index.mjs");

execFileSync(process.execPath, [tailwindEntry, "-i", SRC_TOKENS, "-o", OUT_TOKENS, "--minify"], {
  stdio: "inherit",
  cwd: ROOT,
});

const viteEntry = join(ROOT, "node_modules", "vite", "bin", "vite.js");

execFileSync(process.execPath, [viteEntry, "build", "--emptyOutDir=false"], {
  stdio: "inherit",
  cwd: ROOT,
});
