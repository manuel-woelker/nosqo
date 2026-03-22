import {
  Navigate,
  Outlet,
  createRootRoute,
  createRoute,
  createRouter,
} from "@tanstack/react-router";
import type { RouterHistory } from "@tanstack/react-router";
import { NosqoAppShell } from "./common/components/nosqo-app-shell";
import { routePaths } from "./infrastructure/routing/route-paths";
import { OntologyViewerPage } from "./usecases/administration/ontology/ontology-viewer-page";
import { HomePage } from "./usecases/home/home-page";
import { QueryExplorerPage } from "./usecases/administration/query-explorer/query-explorer-page";
import { StatementBrowserPage } from "./usecases/administration/statement-browser/statement-browser-page";
import { NotFoundPage } from "./usecases/system/not-found-page";
import { RouteErrorPage } from "./usecases/system/route-error-page";

function RootLayout() {
  return (
    <NosqoAppShell>
      <main>
        <Outlet />
      </main>
    </NosqoAppShell>
  );
}

const rootRoute = createRootRoute({
  component: RootLayout,
  errorComponent: RouteErrorPage,
  notFoundComponent: NotFoundPage,
});

const homeRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: routePaths.home,
  component: HomePage,
});

const queryRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: routePaths.queryExplorer,
  component: QueryExplorerPage,
});

const ontologyRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: routePaths.ontology,
  component: OntologyViewerPage,
});

const statementsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: routePaths.statementBrowser,
  component: StatementBrowserPage,
});

const legacyQueryRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: routePaths.legacyQueryExplorer,
  component: () => <Navigate replace to={routePaths.queryExplorer} />,
});

const legacyStatementsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: routePaths.legacyStatementBrowser,
  component: () => <Navigate replace to={routePaths.statementBrowser} />,
});

export const routeTree = rootRoute.addChildren([
  homeRoute,
  ontologyRoute,
  queryRoute,
  statementsRoute,
  legacyQueryRoute,
  legacyStatementsRoute,
]);

export function createAppRouter(history?: RouterHistory) {
  return createRouter({
    routeTree,
    defaultPreload: "intent",
    history,
  });
}

export const router = createAppRouter();

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
