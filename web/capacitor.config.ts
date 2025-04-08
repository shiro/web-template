// import "./loader/hook";
import { CapacitorConfig } from "@capacitor/cli";
// import { mainConfig } from "./config/main.config";

// console.log("env", import.meta.env.FOO);
// require("./loader/hook");

(global as any).IS_DEV = process.env.NODE_ENV == "development";
// const { mainConfig } = require("./config/main.config");

// const url = `${getEnv("HTTPS_ENABLED") ? "https" : "http"}://${getEnv("PUBLIC_HOST")}:${getEnv("PUBLIC_PORT")}`;
const url = `${process.env.HTTPS_ENABLED ? "https" : "http"}://${process.env.PUBLIC_HOST}:${process.env.PUBLIC_PORT}`;
// const url = `http://localhost:80`;

const useDevServer =
  process.env.NODE_ENV == "development" && !process.env.FORCE_LOCAL_ANDROID;

console.log("webview destination:", useDevServer ? url : "local");

const config: CapacitorConfig = {
  appId: process.env.ANDROID_APPID,
  appName: "myapp",
  webDir: "../android/build_android",
  loggingBehavior: "production",
  android: {
    path: "../android",
    // flavor: "universal",
    allowMixedContent: true,
  },
  plugins: {
    CapacitorCookies: {
      enabled: true,
    },
    SplashScreen: {
      androidSplashResourceName: "splash_screen",
    },
    update: {
      enabled: !useDevServer,
    },
  },
  server: useDevServer
    ? {
        url,
        cleartext: true,
      }
    : {
        androidScheme: "http",
        cleartext: true,
      },
};

export default config;
