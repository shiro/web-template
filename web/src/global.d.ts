/// <reference types="@solidjs/start/env" />
/// <reference types="solid-labels" />

// declare const IS_SERVER: boolean;
// declare const IS_DEV: boolean;
// declare const NATIVE_BUNDLE_VERSION: string;

// declare const logger: import("@core/logging/logger").Logger;

declare namespace NodeJS {
  interface ProcessEnv {
    readonly NODE_ENV: "development" | "production" | "test";

    readonly HTTPS_ENABLED: boolean;
    readonly PUBLIC_HOST: string;
    readonly PUBLIC_PORT: number;

    readonly PRIVATE_HOST: string;
    readonly PRIVATE_PORT: number;

    readonly LOG_LEVEL_SERVER: string;
    readonly LOG_LEVEL: string;

    readonly ANDROID_APPID: string;
    readonly ANDROID_APP_NAME: string;
    readonly ANDROID_FORCE_LOCAL: boolean;
  }
}

interface ImportMeta {
  env: ProcessEnv;
}
