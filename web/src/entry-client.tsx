// @refresh reload
// import "solid-devtools";
import { mount, StartClient } from "@solidjs/start/client";
import { render } from "solid-js/web";
import App from "~/app";
import { logger } from "~/logging/logger";
import { isNative } from "~/platform";

console.log("app started wooooooo");
logger.info("UI", "woooooooooooooooooooooooooooooooo");

if (!isNative) {
  // console.log("client SECRET");
  mount(() => <StartClient />, document.getElementById("app")!);
} else {
  // console.log("foo SECRET");
  render(() => <App />, document.getElementById("app")!);
}
