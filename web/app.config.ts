import { defineConfig } from "@solidjs/start/config";
import path from "node:path";
import tsconfig from "./tsconfig.json";
import { linariaVitePlugin } from "./vite/linariaVitePlugin";
import tailwindcss from "@tailwindcss/vite";
// @ts-ignore
import babelPluginLazyPlus from "solid-lazy-plus/babel";

const babelPluginLabels = [
  "solid-labels/babel",
  { dev: process.env.NODE_ENV == "development" },
];

export default defineConfig({
  devOverlay: false,
  server: {
    baseURL: process.env.BASE_PATH,
  },
  ssr: false,

  solid: {
    babel: {
      plugins: [babelPluginLabels, babelPluginLazyPlus],
    },
    // the `solid` field is incorrectly typed
    ...({} as any),
  },

  extensions: ["mdx", "md"],

  vite(options) {
    return {
      // css: { postcss: "./postcss.config.js" },
      server: {
        port: 3000,
        warmup: { clientFiles: ["./src/app.tsx"] },
      },
      build: { sourcemap: true },
      resolve: {
        alias: Object.fromEntries(
          Object.entries(tsconfig.compilerOptions.paths).map(([key, value]) => [
            key.replace(/\/\*$/, ""),
            path.join(process.cwd(), value[0].replace(/\/\*$/, "")),
          ]),
        ),
      },
      plugins: [
        linariaVitePlugin({
          include: [/\/src\//],
          exclude: [/solid-refresh/, /\/@babel\/runtime\//, /\.import\./],
        }) as any,
        tailwindcss(),
      ],
    };
  },
});
