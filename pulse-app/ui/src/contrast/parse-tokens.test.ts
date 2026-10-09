/// <reference types="node" />
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

// @ts-expect-error — .mjs source has no .d.ts but exports a typed Map
import { parseTokens } from "./parse-tokens.mjs";

const HERE = dirname(fileURLToPath(import.meta.url));
const TOKENS_CSS = join(HERE, "..", "..", "dist", "tokens.css");

describe("parseTokens", () => {
  it("parses Tailwind-minified output and exposes all 14 design tokens", () => {
    const css = readFileSync(TOKENS_CSS, "utf8");
    const tokens = parseTokens(css);

    const required = [
      "--color-primary",
      "--color-secondary",
      "--color-accent",
      "--color-base",
      "--color-raised-1",
      "--color-raised-2",
      "--color-raised-3",
      "--color-inset",
      "--color-text-primary",
      "--color-text-secondary",
      "--color-text-tertiary",
      "--color-text-muted",
      "--color-feedback-success",
      "--border-focus",
    ];
    for (const name of required) {
      expect(tokens.has(name), `missing token ${name}`).toBe(true);
    }
  });

  it("normalizes lowercase hex input to uppercase", () => {
    const css = `:root{--color-primary:#4a90e2;--color-base:#1a1d24;}`;
    const tokens = parseTokens(css);
    expect(tokens.get("--color-primary")).toBe("#4A90E2");
    expect(tokens.get("--color-base")).toBe("#1A1D24");
  });

  it("preserves uppercase hex input as-is", () => {
    const css = `:root{--color-accent:#8B2E3B;}`;
    const tokens = parseTokens(css);
    expect(tokens.get("--color-accent")).toBe("#8B2E3B");
  });

  it("ignores Tailwind banner comments and non-token CSS", () => {
    const css = `/*! tailwindcss v4.2.4 | MIT License */
@layer base{*{box-sizing:border-box;}}
:root,:host{--color-primary:#4A90E2;}
@font-face{font-family:IBM Plex Sans;src:url(/fonts/x.woff2)format("woff2")}`;
    const tokens = parseTokens(css);
    expect(tokens.size).toBe(1);
    expect(tokens.get("--color-primary")).toBe("#4A90E2");
  });

  it("returns an empty Map when no token declarations are present", () => {
    expect(parseTokens("").size).toBe(0);
    expect(parseTokens("body{margin:0;}").size).toBe(0);
  });

  it("supports both 6-digit and 8-digit hex (alpha) values", () => {
    const css = `:root{--color-base:#1A1D24;--shadow-overlay:#0000007F;}`;
    const tokens = parseTokens(css);
    expect(tokens.get("--color-base")).toBe("#1A1D24");
    expect(tokens.get("--shadow-overlay")).toBe("#0000007F");
  });

  it("de-duplicates repeated declarations by last-write-wins", () => {
    const css = `:root{--color-primary:#FFFFFF;}:host{--color-primary:#4A90E2;}`;
    const tokens = parseTokens(css);
    expect(tokens.get("--color-primary")).toBe("#4A90E2");
    expect(tokens.size).toBe(1);
  });
});
