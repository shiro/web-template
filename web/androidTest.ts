import { test } from "@playwright/test";
import {
  _android as android,
  type AndroidWebView,
  type AndroidDevice,
} from "playwright";
import util from "util";
import child_process from "child_process";
import path from "path";
import fs from "fs";
import os from "os";

const exec = util.promisify(child_process.exec);
const mkdtemp = util.promisify(fs.mkdtemp);

const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export const androidTest = test.extend<
  {
    meta: {};
  },
  {
    device: AndroidDevice;
    webView: AndroidWebView;
  }
>({
  device: [
    async ({}, use) => {
      const [device] = await android.devices();
      await use(device);
    },
    { scope: "worker" },
  ],
  webView: [
    async ({ device }, use) => {
      const webView = await device.webView({ pkg: process.env.ANDROID_APPID });
      await use(webView);
    },
    { scope: "worker" },
  ],
  page: [
    async ({ webView }, use) => {
      const page = await webView.page();
      await use(page);
    },
    { scope: "test" },
  ],
  context: [
    async ({ page }, use) => {
      await use(page.context());
    },
    { scope: "test" },
  ],
  meta: [
    async ({ device, page, context }, use, testInfo) => {
      const localTempDir = await mkdtemp(path.join(os.tmpdir(), "playwright-"));
      const videoLocalFilePath = path.join(localTempDir, `video.mp4`);
      const traceLocalFilePath = path.join(localTempDir, `trace.zip`);
      const videoRemoteFilePath = (await device.shell("mktemp"))
        .toString()
        .trim();

      page.context().tracing.start({
        sources: true,
        screenshots: true,
        snapshots: true,
      });

      // run in background
      device.shell(`screenrecord ${videoRemoteFilePath}`).finally();

      await use({});
      console.log("done", traceLocalFilePath);

      await context.tracing.stop({ path: traceLocalFilePath });
      testInfo.attach("trace", { path: traceLocalFilePath });

      await device.shell("pkill -2 screenrecord");
      if (testInfo.error) {
        // wait for recording to finish writing
        await delay(500);
        await exec(`adb pull ${videoRemoteFilePath} ${videoLocalFilePath}`);
      }
      await device.shell("rm /sdcard/test.mp4");
      testInfo.attach("video", { path: videoLocalFilePath });

      await fs.promises.rm(localTempDir, { recursive: true });
    },
    { scope: "test", auto: true },
  ],
});
