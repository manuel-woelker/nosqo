import { Link, Outlet, createRootRoute, createRoute, createRouter } from "@tanstack/react-router";
import type { RouterHistory } from "@tanstack/react-router";
import { HomePage } from "./routes/home-page";
import { NotFoundPage } from "./routes/not-found-page";
import { QueryPage } from "./routes/query-page";
import { RouteErrorPage } from "./routes/route-error-page";
import { StatementsPage } from "./routes/statements-page";

function RootLayout() {
  return (
    <div className="app-shell">
      <header className="hero">
        <div className="hero__copy">
          <p className="eyebrow">nosqo</p>
          <h1>Query a knowledge graph without pretending it is a spreadsheet.</h1>
          <p className="hero__lede">
            The UI bootstrap stays intentionally small: real routes, real API calls, and enough
            surface area to prove the stack.
          </p>
        </div>
        <nav aria-label="Primary">
          <ul className="nav-list">
            <li>
              <Link
                activeProps={{ className: "nav-link nav-link--active" }}
                className="nav-link"
                to="/"
              >
                Home
              </Link>
            </li>
            <li>
              <Link
                activeProps={{ className: "nav-link nav-link--active" }}
                className="nav-link"
                to="/query"
              >
                Query
              </Link>
            </li>
            <li>
              <Link
                activeProps={{ className: "nav-link nav-link--active" }}
                className="nav-link"
                to="/statements"
              >
                Statements
              </Link>
            </li>
          </ul>
        </nav>
      </header>
      <main className="page-content">
        <Outlet />
      </main>
    </div>
  );
}

const rootRoute = createRootRoute({
  component: RootLayout,
  errorComponent: RouteErrorPage,
  notFoundComponent: NotFoundPage,
});

const homeRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: HomePage,
});

const queryRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/query",
  component: QueryPage,
});

const statementsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/statements",
  component: StatementsPage,
});

export const routeTree = rootRoute.addChildren([homeRoute, queryRoute, statementsRoute]);

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
