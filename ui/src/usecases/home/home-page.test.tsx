// @vitest-environment happy-dom

import type { ReactNode } from "react";
import { screen } from "@testing-library/react";
import { renderWithNosqoProviders } from "../../test/render";
import { HomePage } from "./home-page";

vi.mock("@tanstack/react-router", () => ({
  Link: ({ children, className, to }: { children: ReactNode; className?: string; to: string }) => (
    <a className={className} href={to}>
      {children}
    </a>
  ),
}));

describe("home page", () => {
  it("renders the administration entry points", () => {
    renderWithNosqoProviders(<HomePage />);

    expect(
      screen.getByRole("heading", {
        level: 1,
        name: /start with the real workflows/i,
      }),
    ).toBeInTheDocument();
    expect(screen.getByRole("img", { name: /nosqo logo/i })).toHaveAttribute(
      "src",
      "/nosqo-logo.png",
    );
    expect(screen.getByRole("link", { name: /open entity browser/i })).toHaveAttribute(
      "href",
      "/administration/entity-browser",
    );
    expect(screen.getByRole("link", { name: /open ontology/i })).toHaveAttribute(
      "href",
      "/administration/ontology",
    );
    expect(screen.getByRole("link", { name: /open query explorer/i })).toHaveAttribute(
      "href",
      "/administration/query-explorer",
    );
    expect(screen.getByRole("link", { name: /open statement browser/i })).toHaveAttribute(
      "href",
      "/administration/statement-browser",
    );
  });
});
