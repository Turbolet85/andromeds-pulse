#!/usr/bin/env node
import { execFileSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, rmSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const ROOT = dirname(dirname(fileURLToPath(import.meta.url)));
const DIST = join(ROOT, "dist");
const SRC_TOKENS = join(ROOT, "src", "styles", "tokens.css");
const OUT_TOKENS = join(DIST, "tokens.css");
const INDEX_HTML = join(ROOT, "index.html");
const FONTS_SRC = join(ROOT, "public", "fonts");
const FONTS_DIST = join(DIST, "fonts");

if (existsSync(DIST)) rmSync(DIST, { recursive: true, force: true });
mkdirSync(DIST, { recursive: true });
mkdirSync(FONTS_DIST, { recursive: true });

cpSync(FONTS_SRC, FONTS_DIST, { recursive: true });
cpSync(INDEX_HTML, join(DIST, "index.html"));

const tailwindEntry = join(ROOT, "node_modules", "@tailwindcss", "cli", "dist", "index.mjs");

execFileSync(process.execPath, [tailwindEntry, "-i", SRC_TOKENS, "-o", OUT_TOKENS, "--minify"], {
  stdio: "inherit",
  cwd: ROOT,
});
