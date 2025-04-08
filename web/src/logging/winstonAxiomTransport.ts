import Transport from 'winston-transport';
import {Client as AxiomClient} from "@axiomhq/axiom-node";
import {serverConfig} from "@config/server.config";


export class WinstonAxiomTransport extends Transport {
    private readonly axiomClient?: AxiomClient;

    constructor() {
        super();
        if (serverConfig.flags.axiomLoggingEnabled) {
            this.axiomClient = new AxiomClient();
        }
    }
    log(info, callback) {
        const {level, tag, message, err, durationMs: duration, meta} = info;
        if (!this.axiomClient) {
            callback();
            return;
        }

        const ev = {
            severity: level,
            message,
            tag,
            ...(meta ? {meta} : {}),
            ...(duration != null ? {duration} : {}),
            ...(err ? {
                error: {
                    // code: err.code,
                    name: err.name,
                    message: err.message,
                    stack: err.stack?.split("\n").map(s => s.trim()),
                    ...(err.code ? {code: err.code} : {}),
                }
            } : {}),
        };

        this.axiomClient.ingestEvents("production", ev)
            .catch(err => {
                // logger.error("api", "logging to axiom failed", {err});
            })
            .finally();

        callback();
    }
}