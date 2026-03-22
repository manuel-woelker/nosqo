import { screen } from "@testing-library/react";
import { renderApp } from "../test/render-app";

describe("statements page", () => {
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

    await renderApp(["/statements"]);

    expect(
      await screen.findByRole("heading", {
        level: 3,
        name: /no statements matched/i,
      }),
    ).toBeInTheDocument();
  });
});
