import { WinstonAxiomTransport } from "@server/logging/winstonAxiomTransport";
import { winstonConfig } from "@config/winston.config";
import fs from "fs";
import winston, { format } from "winston";
import DailyRotateFile from "winston-daily-rotate-file";

const { combine, printf } = format;
const timeFormat = format.timestamp({
    format: "YYYY-MM-DD HH:mm:ss",
});

const logFormat = printf((info) => {
    const { level, message, timestamp, tag, durationMs } = info;
    const { err, ...meta } = info.meta ?? {};

    const durationString = durationMs != null ? ` (${durationMs}ms)` : "";

    const logLine = `${timestamp} [${tag}] ${level}: ${message}${durationString}`;

    return (
        `${logLine}` +
        `${meta ? `\nmeta: ${JSON.stringify(meta, null, 2)}` : ""}` +
        (err
            ? `\nmessage: ${err.message ?? "[no message]"}` +
              `${
                  err.data
                      ? `\n data: ${JSON.stringify(err.data, null, 2)}`
                      : ""
              }` +
              `${
                  err.stack
                      ? `\n${err.stack.split("\n").slice(1, 21).join(`\n`)}`
                      : ""
              }`
            : "")
    );
});

const logLevels = {
    levels: {
        error: 0,
        warn: 1,
        info: 2,
        verbose: 3,
    },
    colors: {
        error: "red",
        warn: "yellow",
        info: "blue",
        verbose: "white",
    },
};
winston.addColors(logLevels.colors);

// ensure the log folder exists
fs.mkdirSync(winstonConfig.logFolder, { recursive: true });

// write all logs with level `error`
const errorFileLogTransport = new DailyRotateFile({
    level: "error",
    filename: `${winstonConfig.logFolder}${winstonConfig.errorLogFile}`,
    datePattern: "YYYY-MM-DD-HH",
    maxSize: winstonConfig.errorLogSizeLimit,
    maxFiles: winstonConfig.errorLogFileLimit,
});

const filterErrors = winston.format((info) =>
    info.level == "error" ? false : info
);

// write all logs except level `error`
const otherFileLogTransport = new DailyRotateFile({
    filename: `${winstonConfig.logFolder}${winstonConfig.combinedLogFile}`,
    datePattern: "YYYY-MM-DD-HH",
    maxSize: winstonConfig.combinedLogSizeLimit,
    maxFiles: winstonConfig.combinedLogFileLimit,
    format: combine(filterErrors(), timeFormat, logFormat),
});

const consoleTransport = new winston.transports.Console({
    format: combine(winston.format.colorize(), timeFormat, logFormat),
});

const axiomTransport = new WinstonAxiomTransport();

export const winstonLogConfiguration = {
    levels: logLevels.levels,
    format: combine(timeFormat, logFormat),
    defaultMeta: { tag: "UNDEFINED" },
    transports: [
        errorFileLogTransport,
        otherFileLogTransport,
        consoleTransport,
        axiomTransport,
    ],
};
