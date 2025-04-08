import { registerPlugin } from "@capacitor/core";

export interface LoggerPlugin {
  echo(options: { value: string }): Promise<{ value: string }>;
}

const LoggerPlugin = registerPlugin<LoggerPlugin>("Logger");

export default LoggerPlugin;
