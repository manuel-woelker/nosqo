// @vitest-environment happy-dom

import { render, screen } from "@testing-library/react";
import { StatementsPage } from "./statements-page";

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

    render(<StatementsPage />);

    expect(
      await screen.findByRole("heading", {
        level: 3,
        name: /no statements matched/i,
      }),
    ).toBeInTheDocument();
  });
});
