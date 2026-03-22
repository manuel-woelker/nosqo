import type { ReactElement } from "react";
import { render } from "@testing-library/react";

export function renderWithNosqoProviders(ui: ReactElement) {
  return render(ui);
}
