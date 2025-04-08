import winston from "winston";
import { winstonLogConfiguration } from "@server/logging/winstonLogger";
import { Logger, LogLevel, LogMeta, LogTag } from "@core/logging/logger";
import { serverConfig } from "@config/server.config";

class LoggerProfiler {
    private readonly level: LogLevel;
    private profiler: winston.Profiler;

    constructor(level: LogLevel, profiler: winston.Profiler) {
        this.level = level;
        this.profiler = profiler;
    }

    done(tag: LogTag, message: string, meta: LogMeta = {}) {
        this.profiler.done({
            level: this.level,
            tag,
            message,
            meta,
        });
    }
}

export class ServerLogger implements Logger {
    private logger: winston.Logger;

    constructor() {
        this.logger = winston.createLogger(winstonLogConfiguration);
        this.logger.level = serverConfig.logLevel;
    }

    info(tag: LogTag, message: string, meta: LogMeta = {}) {
        if (!meta.platform) meta.platform = "server";
        this.logger.info(message, { meta, tag: tag.toUpperCase() });
    }

    error(tag: LogTag, message: string, meta: LogMeta = {}) {
        if (!meta.platform) meta.platform = "server";
        this.logger.error(message, { meta, tag: tag.toUpperCase() });
    }

    warn(tag: LogTag, message: string, meta: LogMeta = {}) {
        if (!meta.platform) meta.platform = "server";
        this.logger.warn(message, { meta, tag: tag.toUpperCase() });
    }

    verbose(tag: LogTag, message: string, meta: LogMeta = {}) {
        if (!meta.platform) meta.platform = "server";
        this.logger.verbose(message, { meta, tag: tag.toUpperCase() });
    }

    verboseProfiler() {
        return new LoggerProfiler("verbose", this.logger.startTimer());
    }
}
