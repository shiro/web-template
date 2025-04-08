import { RouteDefinition } from "@solidjs/router";
import Main from "./Main";
import Page1 from "./pages/Page1";
import Page2 from "./pages/Page2";

export const routes: RouteDefinition[] = [
  { path: "/", component: Main },
  { path: "/page1", component: Page1 },
  { path: "/page2", component: Page2 },
];
