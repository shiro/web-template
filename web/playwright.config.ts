import { defineConfig, devices } from "@playwright/test";

class Launcher {
  async setup(worker: number, headless: boolean, workerInfo: any) {
    console.log("hi");
  }
}

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  testDir: "./tests",
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [["html", { open: "never" }]],
  use: {
    baseURL: "http://localhost:3000",
    trace: "on",
    video: "on",
  },

  /* Configure projects for major browsers */
  projects: [
    {
      name: "chromium",
      testMatch: "**/chromium.spec.ts",
      fullyParallel: true,
      use: { ...devices["Desktop Chrome"] },
    },
    {
      name: "webview",
      testMatch: "**/android.spec.ts",
      use: {
        ...devices["Desktop Chrome"],
        baseURL: "http://localhost",
      },
    },
  ],

  /* Run your local dev server before starting the tests */
  // webServer: {
  //   command: 'npm run start',
  //   url: 'http://127.0.0.1:3000',
  //   reuseExistingServer: !process.env.CI,
  // },
});
