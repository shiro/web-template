import { isNative } from "~/platform";
import type { Logger, LogLevel, LogMeta } from "~/logging/logger";
import { omit, isEmpty } from "lodash-es";
// import { clientConfig } from "@config/client.config";

const levelToColor: Record<LogLevel, string> = {
  error: "red",
  warn: "yellow",
  info: "dodgerBlue",
  verbose: "grey",
};

// const makeAPIClient = () => {
//     // const { createTRPCProxyClient } = require("@trpc/client");
//     // const { trpcConfig } = require("@server/trpc/trpcConfig");
//     return createTRPCProxyClient<AppRouter>(trpcConfigStart());
// };

// type APIClient = ReturnType<typeof makeAPIClient>;

interface LogEntry {
  level: LogLevel;
  tag: string;
  message: string;
  meta?: LogMeta;
  err?: Error;
  timestamp?: string;
}

export class ClientLogger<LogTag extends string> implements Logger<LogTag> {
  private readonly level: LogLevel;
  // private apiClient: APIClient | undefined;
  private logBuffer: LogEntry[];

  constructor() {
    this.logBuffer = [];
    // this.apiClient = undefined;
    // this.level = clientConfig.logLevel;
    this.level = "info";
  }

  private sendMessage(entry: LogEntry) {
    // if (!this.apiClient) this.apiClient = makeAPIClient();
    // this.apiClient;

    if (entry.meta?.err) {
      entry.meta.err = {
        name: entry.meta.err.name,
        message: entry.meta.err.message,
        stack: entry.meta.err.stack?.split("\n").slice(0, 20).join("\n"),
      };
    }

    // this.apiClient.reporting.reportError.mutate({
    //   ...entry,
    //   meta: omit(entry.meta, "online"),
    //   // only push log buffer on error
    //   ...(entry.level == "error" && this.logBuffer.length
    //     ? { log: this.logBuffer }
    //     : {}),
    // });
  }

  private log(
    level: LogLevel,
    tag: LogTag,
    message: string,
    meta: LogMeta = {},
  ) {
    meta.platform = isNative ? "android" : "web";

    const printMeta = omit(meta, ["platform", "online"]);

    if (isNative) {
      const msg = `[${tag.toUpperCase()}] ${level}: ${message}`;
      if (!isEmpty(printMeta))
        console.log(msg, JSON.stringify(printMeta, null, 4));
      else console.log(msg);
    } else {
      const msg = `[${tag.toUpperCase()}] %c${level}%c: ${message}`;
      const opts = [`color: ${levelToColor[level]}`, "color: white"];

      if (!isEmpty(printMeta)) console.log(msg, ...opts, printMeta);
      else console.log(msg, ...opts);
    }

    const entry = { level, tag, message, meta };

    if (level == "error" || meta.online === true) {
      this.sendMessage(entry);
      if (level == "error") this.logBuffer = [];
    } else {
      this.pushLog(entry);
    }
  }

  private pushLog(entry: LogEntry) {
    entry.timestamp = new Date().toISOString();
    this.logBuffer.push(entry);
    if (this.logBuffer.length > 50) {
      this.logBuffer.splice(0, 1);
    }
  }

  verbose(tag: LogTag, message: string, meta?: LogMeta) {
    if (this.level != "verbose") return;
    this.log("verbose", tag, message, meta);
  }

  info(tag: LogTag, message: string, meta?: LogMeta) {
    if (this.level != "verbose" && this.level != "info") return;
    this.log("info", tag, message, meta);
  }

  warn(tag: LogTag, message: string, meta?: LogMeta) {
    if (this.level != "verbose" && this.level != "info" && this.level != "warn")
      return;
    this.log("warn", tag, message, meta);
  }

  error(tag: LogTag, message: string, meta?: LogMeta) {
    console.error(`[${tag.toUpperCase()}] ${message}`, meta);
    this.log("error", tag, message, meta);
  }

  verboseProfiler() {
    // return new LoggerProfiler("verbose", this.logger.startTimer());
  }
}
