// @vitest-environment happy-dom

import { screen } from "@testing-library/react";
import { OntologyViewerPage } from "./ontology-viewer-page";
import { renderWithNosqoProviders } from "../../../test/render";

describe("ontology viewer page", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("renders a parsed ontology snapshot from the server", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue(
        new Response(
          JSON.stringify({
            format: "nosqo-statement-json-v1",
            values: [
              "#Person",
              "~attribute",
              "~name",
              "~description",
              ["A human individual."],
              "~isA",
              "#Type",
              "~label",
              ["Person"],
              ["name"],
              "#Predicate",
              ["Human-readable name."],
              "#String",
              "~targetType",
            ],
            statements: [
              [0, 1, 2],
              [0, 3, 4],
              [0, 5, 6],
              [0, 7, 8],
              [2, 3, 11],
              [2, 5, 10],
              [2, 7, 9],
              [2, 13, 12],
            ],
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

    renderWithNosqoProviders(<OntologyViewerPage />);

    expect(await screen.findByRole("heading", { level: 1, name: /ontology/i })).toBeInTheDocument();
    expect(screen.getByText("Read-only")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /person/i })).toBeInTheDocument();
    expect(screen.getByText("Allowed attributes")).toBeInTheDocument();
    expect(screen.getAllByText("~name").length).toBeGreaterThan(0);
  });

  it("renders an empty state when the ontology endpoint returns no content", async () => {
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

    renderWithNosqoProviders(<OntologyViewerPage />);

    expect(
      await screen.findByRole("heading", {
        level: 3,
        name: /no ontology entities/i,
      }),
    ).toBeInTheDocument();
  });

  it("renders API errors from failed ontology requests", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue(
        new Response("backend melted", {
          status: 500,
          headers: {
            "Content-Type": "text/plain",
          },
        }),
      ),
    );

    renderWithNosqoProviders(<OntologyViewerPage />);

    expect(await screen.findByRole("alert")).toHaveTextContent("backend melted");
  });
});
