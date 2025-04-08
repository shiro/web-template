import { expect } from "@playwright/test";
import { androidTest } from "../androidTest";

const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

androidTest("android test", async ({ page }, testInfo) => {
  await page.goto(testInfo.project.use.baseURL!);
  await delay(1000);
  await expect(page).toHaveTitle(/Test/);
  await delay(1000);
  await expect(page.getByText("hello world2")).toBeVisible();
  await delay(1000);
});

// await device.shell("cmd statusbar expand-notifications");
// adb shell dumpsys notification --noredact
