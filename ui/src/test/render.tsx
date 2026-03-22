import type { ReactElement } from "react";
import { MantineProvider } from "@mantine/core";
import { render } from "@testing-library/react";
import { nosqoTheme } from "../infrastructure/theme/nosqo-theme";

export function renderWithNosqoProviders(ui: ReactElement) {
  return render(
    <MantineProvider defaultColorScheme="dark" theme={nosqoTheme}>
      {ui}
    </MantineProvider>,
  );
}
