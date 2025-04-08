import { isServer } from "solid-js/web";
// import { ServerLogger } from "./serverLogger";
import { ClientLogger } from "./clientLogger";

type Tag =
  | "api"
  | "sql"
  | "service"
  | "seed"
  | "stream"
  | "pdf"
  | "http"
  | "android"
  | "script"
  | "UI"
  | "other";

export type LogLevel = "verbose" | "info" | "warn" | "error";

export interface LogMeta {
  [key: string]: any;
  err?: Error;
}

let _logger: Logger<Tag>;

export interface Logger<LogTag extends string> {
  verbose(tag: LogTag, message: string, meta?: LogMeta): void;
  info(tag: LogTag, message: string, meta?: LogMeta): void;
  warn(tag: LogTag, message: string, meta?: LogMeta): void;
  error(tag: LogTag, message: string, meta?: LogMeta): void;
  verboseProfiler(): any;
}

// const testLogger: Logger = {
//     info: console.log,
//     warn: console.log,
//     error: console.log,
//     verbose: console.log,
//     verboseProfiler: () => ({ done: () => {} }),
// };
// _logger = testLogger;

// if (isServer) {
// const { ServerLogger } = await import("@server/logging/serverLogger");
// _logger = new ServerLogger();
// } else {
// const { ClientLogger } = await import("@core/logging/clientLogger");
_logger = new ClientLogger();
// }

export const logger = _logger;
