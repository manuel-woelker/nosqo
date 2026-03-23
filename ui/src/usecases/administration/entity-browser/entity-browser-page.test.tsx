// @vitest-environment happy-dom

import { fireEvent, screen, waitFor } from "@testing-library/react";
import { renderWithNosqoProviders } from "../../../test/render";
import { EntityBrowserPage } from "./entity-browser-page";

describe("entity browser page", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("searches by ontology-driven attributes and renders entity details", async () => {
    const fetchMock = vi.fn((input: string | URL | Request, init?: RequestInit) => {
      const url =
        typeof input === "string" ? input : input instanceof URL ? input.toString() : input.url;

      if (url.endsWith("/api/v1/ontology")) {
        return Promise.resolve(
          new Response(
            JSON.stringify({
              format: "nosqo-statement-json-v1",
              values: [
                "#Person",
                "~attribute",
                "~name",
                "~email",
                "~isA",
                "#Type",
                "~label",
                ["Person"],
                ["Name"],
                ["Email"],
                "#Predicate",
              ],
              statements: [
                [0, 1, 2],
                [0, 1, 3],
                [0, 4, 5],
                [0, 6, 7],
                [2, 4, 10],
                [2, 6, 8],
                [3, 4, 10],
                [3, 6, 9],
              ],
            }),
            {
              status: 200,
              headers: {
                "Content-Type": "application/json",
              },
            },
          ),
        );
      }

      if (url.endsWith("/api/v1/entities/search")) {
        expect(JSON.parse(String(init?.body))).toEqual({
          filters: {
            "~name": "Alice",
          },
          type: "#Person",
        });

        return Promise.resolve(
          new Response(
            JSON.stringify({
              results: [
                {
                  id: "alice",
                  nosqoId: "@alice",
                  label: "Alice",
                  typeIds: ["#Person"],
                },
              ],
            }),
            {
              status: 200,
              headers: {
                "Content-Type": "application/json",
              },
            },
          ),
        );
      }

      if (url.endsWith("/api/v1/entities/alice")) {
        return Promise.resolve(
          new Response(
            JSON.stringify({
              id: "alice",
              nosqoId: "@alice",
              label: "Alice",
              typeIds: ["#Person"],
              attributes: [
                {
                  predicateId: "~email",
                  label: "Email",
                  values: ["alice@example.com"],
                },
                {
                  predicateId: "~name",
                  label: "Name",
                  values: ["Alice"],
                },
              ],
            }),
            {
              status: 200,
              headers: {
                "Content-Type": "application/json",
              },
            },
          ),
        );
      }

      throw new Error(`Unexpected fetch URL: ${url}`);
    });

    vi.stubGlobal("fetch", fetchMock);

    renderWithNosqoProviders(<EntityBrowserPage />);

    expect(
      await screen.findByRole("heading", { level: 1, name: /entity browser/i }),
    ).toBeInTheDocument();

    fireEvent.change(screen.getByLabelText(/^name$/i), {
      target: { value: "Alice" },
    });
    fireEvent.click(screen.getByRole("button", { name: /^search$/i }));

    expect(await screen.findByRole("button", { name: /^alice$/i })).toBeInTheDocument();
    expect(await screen.findByRole("heading", { level: 3, name: /^alice$/i })).toBeInTheDocument();
    expect(screen.getByText("alice@example.com")).toBeInTheDocument();
    expect(fetchMock).toHaveBeenCalledTimes(3);
  });

  it("renders empty states for searches with no matches", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn((input: string | URL | Request) => {
        const url =
          typeof input === "string" ? input : input instanceof URL ? input.toString() : input.url;

        if (url.endsWith("/api/v1/ontology")) {
          return Promise.resolve(
            new Response(
              JSON.stringify({
                format: "nosqo-statement-json-v1",
                values: ["#Person", "~isA", "#Type", "~label", ["Person"]],
                statements: [
                  [0, 1, 2],
                  [0, 3, 4],
                ],
              }),
              {
                status: 200,
                headers: {
                  "Content-Type": "application/json",
                },
              },
            ),
          );
        }

        if (url.endsWith("/api/v1/entities/search")) {
          return Promise.resolve(
            new Response(JSON.stringify({ results: [] }), {
              status: 200,
              headers: {
                "Content-Type": "application/json",
              },
            }),
          );
        }

        throw new Error(`Unexpected fetch URL: ${url}`);
      }),
    );

    renderWithNosqoProviders(<EntityBrowserPage />);

    expect(
      await screen.findByRole("heading", { level: 1, name: /entity browser/i }),
    ).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /^search$/i }));

    expect(
      await screen.findByRole("heading", { level: 3, name: /no matching entities/i }),
    ).toBeInTheDocument();
  });

  it("renders search errors from failed requests", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn((input: string | URL | Request) => {
        const url =
          typeof input === "string" ? input : input instanceof URL ? input.toString() : input.url;

        if (url.endsWith("/api/v1/ontology")) {
          return Promise.resolve(
            new Response(
              JSON.stringify({
                format: "nosqo-statement-json-v1",
                values: ["#Person", "~attribute", "~name", "~isA", "#Type", "~label", ["Person"]],
                statements: [
                  [0, 1, 2],
                  [0, 3, 4],
                  [0, 5, 6],
                ],
              }),
              {
                status: 200,
                headers: {
                  "Content-Type": "application/json",
                },
              },
            ),
          );
        }

        if (url.endsWith("/api/v1/entities/search")) {
          return Promise.resolve(
            new Response("backend melted", {
              status: 500,
              headers: {
                "Content-Type": "text/plain",
              },
            }),
          );
        }

        throw new Error(`Unexpected fetch URL: ${url}`);
      }),
    );

    renderWithNosqoProviders(<EntityBrowserPage />);

    expect(
      await screen.findByRole("heading", { level: 1, name: /entity browser/i }),
    ).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /^search$/i }));

    await waitFor(() => {
      expect(screen.getByRole("alert")).toHaveTextContent("backend melted");
    });
  });
});
