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
    command: "npx http-server dist -p 4173 --silent",
    port: 4173,
    reuseExistingServer: !process.env.CI,
    timeout: 30_000,
  },
});
