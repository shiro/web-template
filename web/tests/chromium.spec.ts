import { test, expect } from "@playwright/test";

const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

test("chrome test", async ({ page }, testInfo) => {
  page.goto(testInfo.project.use.baseURL!);
  await delay(1000);
  await expect(page).toHaveTitle(/Test/);
  await delay(1000);
  await expect(page.getByText("hello world2")).toBeVisible();
  await delay(1000);
});
