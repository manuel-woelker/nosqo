// @vitest-environment happy-dom

import { screen, waitFor } from "@testing-library/react";
import { RouterProvider } from "@tanstack/react-router";
import { act } from "react";
import { createAppRouter } from "../../router";
import { routePaths } from "../../infrastructure/routing/route-paths";
import { renderWithNosqoProviders } from "../../test/render";

async function renderRouterAtPath(pathname: string) {
  window.history.pushState({}, "", pathname);
  const router = createAppRouter();

  await act(async () => {
    renderWithNosqoProviders(<RouterProvider router={router} />);
    await router.load();
  });
}

describe("nosqo app shell", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    window.history.pushState({}, "", "/");
  });

  it("renders the shell navigation and breadcrumbs for the query explorer route", async () => {
    await renderRouterAtPath(routePaths.queryExplorer);

    const breadcrumbNavigation = screen.getByRole("navigation", { name: "Breadcrumbs" });

    expect(await screen.findByRole("link", { name: "nosqo" })).toHaveAttribute("href", "/");
    expect(screen.getByRole("link", { name: /query explorer/i })).toHaveAttribute(
      "data-active",
      "true",
    );
    expect(breadcrumbNavigation).toHaveTextContent("Administration");
    expect(breadcrumbNavigation).toHaveTextContent("Query Explorer");
  });

  it("renders ontology breadcrumbs inside the shared shell", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue(
        new Response(
          JSON.stringify({
            format: "nosqo-statement-json-v1",
            values: [],
            statements: [],
          }),
          {
            status: 200,
            headers: {
              "Content-Type": "application/json",
            },
          },
        ),
      ),
    );

    await renderRouterAtPath(routePaths.ontology);

    const breadcrumbNavigation = screen.getByRole("navigation", { name: "Breadcrumbs" });

    await waitFor(() => {
      expect(breadcrumbNavigation).toHaveTextContent("Ontology");
    });

    expect(screen.getByRole("link", { name: /model ontology/i })).toHaveAttribute(
      "data-active",
      "true",
    );
  });

  it("renders statement browser breadcrumbs inside the shared shell", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue(
        new Response("", {
          status: 200,
          headers: {
            "Content-Type": "text/plain",
          },
        }),
      ),
    );

    await renderRouterAtPath(routePaths.statementBrowser);

    const breadcrumbNavigation = screen.getByRole("navigation", { name: "Breadcrumbs" });

    await waitFor(() => {
      expect(breadcrumbNavigation).toHaveTextContent("Statement Browser");
    });

    expect(screen.getByRole("link", { name: /statement browser/i })).toHaveAttribute(
      "data-active",
      "true",
    );
  });
});
