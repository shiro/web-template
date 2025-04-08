import "~/style/global.style";

import { Meta, MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { Suspense } from "solid-js";
import { config } from "~/config";
import { routes } from "~/routes";
import Preload from "~/preload";

export default function App() {
  return (
    <MetaProvider>
      <Router
        base={config.base}
        root={(props) => {
          return (
            <>
              <Title>Test</Title>
              <Meta name="description" content="test app" />
              <div class="content-container">
                <Suspense>
                  <Preload />
                  {props.children}
                </Suspense>
              </div>
            </>
          );
        }}
      >
        {routes}
      </Router>
    </MetaProvider>
  );
}
