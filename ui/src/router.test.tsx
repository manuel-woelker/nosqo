import { screen } from "@testing-library/react";
import { navigateTo, renderApp } from "./test/render-app";

describe("router", () => {
  it("renders the app shell and navigates between routes", async () => {
    const { router } = await renderApp(["/"]);

    expect(
      screen.getByRole("heading", {
        level: 2,
        name: /start with the real workflows/i,
      }),
    ).toBeInTheDocument();

    expect(screen.getByRole("link", { name: "Query" })).toBeInTheDocument();
    await navigateTo(router, "/query");

    expect(
      await screen.findByRole("heading", {
        level: 2,
        name: /run a query against the loaded knowledge base/i,
      }),
    ).toBeInTheDocument();
  });
});
