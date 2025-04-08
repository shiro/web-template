import { registerPlugin } from "@capacitor/core";

export interface EchoPlugin {
  echo(options: { value: string }): Promise<{ value: string }>;
}

const EchoPlugin = registerPlugin<EchoPlugin>("Echo");

export default EchoPlugin;
