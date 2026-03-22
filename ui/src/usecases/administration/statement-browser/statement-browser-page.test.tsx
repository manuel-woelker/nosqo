// @vitest-environment happy-dom

import { screen } from "@testing-library/react";
import { renderWithNosqoProviders } from "../../../test/render";
import { StatementBrowserPage } from "./statement-browser-page";

describe("statement browser page", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("renders an empty state when the statement query returns nothing", async () => {
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

    renderWithNosqoProviders(<StatementBrowserPage />);

    expect(
      await screen.findByRole("heading", {
        level: 3,
        name: /no statements matched/i,
      }),
    ).toBeInTheDocument();
  });
});
