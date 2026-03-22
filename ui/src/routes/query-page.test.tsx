import { fireEvent, screen, waitFor } from "@testing-library/react";
import { renderApp } from "../test/render-app";

describe("query page", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("renders successful query results", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue(
        new Response(
          JSON.stringify({
            columns: ["?person", "?name"],
            rows: [["@frodo_baggins", '"Frodo Baggins"']],
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

    await renderApp(["/query"]);

    fireEvent.click(screen.getByRole("button", { name: /run query/i }));

    expect(await screen.findByText("@frodo_baggins")).toBeInTheDocument();
    expect(screen.getByText('"Frodo Baggins"')).toBeInTheDocument();
  });

  it("renders API errors from failed queries", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue(
        new Response(
          JSON.stringify({
            error: "query must contain at least one pattern",
          }),
          {
            status: 400,
            headers: {
              "Content-Type": "application/json",
            },
          },
        ),
      ),
    );

    await renderApp(["/query"]);

    fireEvent.click(screen.getByRole("button", { name: /run query/i }));

    await waitFor(() => {
      expect(screen.getByRole("alert")).toHaveTextContent(
        "query must contain at least one pattern",
      );
    });
  });
});
