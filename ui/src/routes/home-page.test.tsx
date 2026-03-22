// @vitest-environment happy-dom

import type { ReactNode } from "react";
import { render, screen } from "@testing-library/react";
import { HomePage } from "./home-page";

vi.mock("@tanstack/react-router", () => ({
  Link: ({ children, className, to }: { children: ReactNode; className?: string; to: string }) => (
    <a className={className} href={to}>
      {children}
    </a>
  ),
}));

describe("home page", () => {
  it("renders the main entry points", () => {
    render(<HomePage />);

    expect(
      screen.getByRole("heading", {
        level: 2,
        name: /start with the real workflows/i,
      }),
    ).toBeInTheDocument();
    expect(screen.getByRole("link", { name: /open query explorer/i })).toHaveAttribute(
      "href",
      "/query",
    );
    expect(screen.getByRole("link", { name: /open statement browser/i })).toHaveAttribute(
      "href",
      "/statements",
    );
  });
});
