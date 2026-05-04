import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./tests-e2e",
  testIgnore: ["**/*"],
  reporter: [
    ["json", { outputFile: "../../target/playwright-report/results.json" }],
  ],
});
