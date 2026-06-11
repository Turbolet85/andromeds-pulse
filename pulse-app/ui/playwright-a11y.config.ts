import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./tests-a11y",
  testMatch: ["axe/**/*.spec.ts", "reduced-motion.spec.ts", "keyboard-focus/**/*.spec.ts"],
  reporter: [
    ["json", { outputFile: "../../target/playwright-a11y-report/results.json" }],
  ],
  use: {
    baseURL: "http://localhost:4173",
    trace: "off",
    screenshot: "off",
    video: "off",
  },
  projects: [
    {
      name: "chromium",
      use: { browserName: "chromium" },
    },
  ],
  webServer: {
    // SPA fallback via self-proxy (`-P <self>?`): the dashboard router uses
    // BROWSER history (no hash history), so direct route URLs like
    // /settings must rewrite to index.html. Chunk #99 finding: the specs'
    // former `/#/route` URLs never routed — every route-specific audit
    // silently swept the traces index instead of its declared surface.
    command: "npx http-server dist -p 4173 --silent -P http://localhost:4173?",
    port: 4173,
    reuseExistingServer: !process.env.CI,
    timeout: 30_000,
  },
});
