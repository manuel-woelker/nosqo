// @vitest-environment happy-dom

import { StrictMode } from "react";
import { fireEvent, screen, within } from "@testing-library/react";
import { OntologyViewerPage } from "./ontology-viewer-page";
import { renderWithNosqoProviders } from "../../../test/render";
import { useOntologyViewerStore } from "./use-ontology-viewer-store";

describe("ontology viewer page", () => {
  afterEach(() => {
    useOntologyViewerStore.getState().reset();
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
    expect(screen.getByRole("heading", { level: 3, name: /attributes/i })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /^name$/i })).toBeInTheDocument();
    expect(screen.getByText("Human-readable name.")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /^string$/i })).toBeInTheDocument();
    expect(screen.queryByText("#Person")).not.toBeInTheDocument();

    fireEvent.click(
      within(screen.getByRole("list", { name: /ontology entities/i })).getByRole("button", {
        name: /name/i,
      }),
    );

    expect(screen.getByRole("heading", { level: 3, name: /source types/i })).toBeInTheDocument();
    expect(screen.getByRole("heading", { level: 3, name: /target types/i })).toBeInTheDocument();
    expect(screen.getByText("A human individual.")).toBeInTheDocument();
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

  it("only loads the ontology once under strict mode", async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      new Response(
        JSON.stringify({
          format: "nosqo-statement-json-v1",
          values: ["#Type", "~label", ["Type"]],
          statements: [[0, 1, 2]],
        }),
        {
          status: 200,
          headers: {
            "Content-Type": "application/json",
          },
        },
      ),
    );

    vi.stubGlobal("fetch", fetchMock);

    renderWithNosqoProviders(
      <StrictMode>
        <OntologyViewerPage />
      </StrictMode>,
    );

    expect(await screen.findByRole("heading", { level: 1, name: /ontology/i })).toBeInTheDocument();
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });
});
