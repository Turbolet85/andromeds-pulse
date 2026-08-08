import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));

// CSP regression test for chunk #28 substrate per phase-25 combined.md
// Cross-domain rot warning #3 resolution: security plan §API Security row
// "CSP (webview content)" supplies the invariant; tests own the assertion
// mechanism. Verifies pulse-app/tauri.conf.json `app.security.csp` literal
// preserves the WebGPU `worker-src 'self' blob:` provision (chunk #29 needs
// it) and forbids `'unsafe-inline'` / `'unsafe-eval'` / remote origins on
// `script-src` (security plan §Anti-Patterns API + §Threat Model Summary
// "webview content").
//
// Path to tauri.conf.json: pulse-app/tauri.conf.json relative to repo root.
// This test runs from `pulse-app/ui/` (Vitest cwd), so the path resolves
// up two directories.

function readCsp(): string {
  const tauriConfPath = resolve(__dirname, "../../tauri.conf.json");
  const raw = readFileSync(tauriConfPath, "utf8");
  const conf = JSON.parse(raw) as { app?: { security?: { csp?: string } } };
  const csp = conf.app?.security?.csp;
  if (typeof csp !== "string") {
    throw new Error(
      "tauri.conf.json missing app.security.csp string — chunk #28 CSP regression invariant",
    );
  }
  return csp;
}

function extractDirective(csp: string, name: string): string | null {
  const segments = csp.split(";").map((s) => s.trim());
  const found = segments.find((s) => s.startsWith(`${name} `) || s === name);
  return found ?? null;
}

describe("tauri.conf.json CSP — chunk #28 regression invariants", () => {
  it("preserves `worker-src 'self' blob:` provision (WebGPU compute pipelines in chunk #29)", () => {
    const csp = readCsp();
    expect(csp).toContain("worker-src 'self' blob:");
  });

  it("preserves `script-src 'self'` without `'unsafe-inline'` / `'unsafe-eval'` widening", () => {
    const csp = readCsp();
    const scriptSrc = extractDirective(csp, "script-src");
    expect(scriptSrc).not.toBeNull();
    expect(scriptSrc).toContain("'self'");
    expect(scriptSrc).not.toContain("'unsafe-inline'");
    expect(scriptSrc).not.toContain("'unsafe-eval'");
  });

  it("does not introduce remote http(s) origins beyond http://ipc.localhost", () => {
    const csp = readCsp();
    const matches = csp.match(/https?:\/\/[^\s;]+/g) ?? [];
    for (const origin of matches) {
      expect(origin).toBe("http://ipc.localhost");
    }
  });

  it("preserves `default-src 'self' ipc: http://ipc.localhost`", () => {
    const csp = readCsp();
    const defaultSrc = extractDirective(csp, "default-src");
    expect(defaultSrc).not.toBeNull();
    expect(defaultSrc).toContain("'self'");
    expect(defaultSrc).toContain("ipc:");
    expect(defaultSrc).toContain("http://ipc.localhost");
  });

  it("preserves `connect-src 'self' ipc: http://ipc.localhost` (Tauri IPC connectivity)", () => {
    const csp = readCsp();
    const connectSrc = extractDirective(csp, "connect-src");
    expect(connectSrc).not.toBeNull();
    expect(connectSrc).toContain("'self'");
    expect(connectSrc).toContain("ipc:");
    expect(connectSrc).toContain("http://ipc.localhost");
  });
});
