import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { MantineProvider } from "@mantine/core";
import { RouterProvider } from "@tanstack/react-router";
import "@mantine/core/styles.css";
import { nosqoTheme } from "./infrastructure/theme/nosqo-theme";
import { router } from "./router";
import "./styles.css";

const rootElement = document.getElementById("root");

if (!rootElement) {
  throw new Error("Missing root element for nosqo UI bootstrap.");
}

createRoot(rootElement).render(
  <StrictMode>
    <MantineProvider defaultColorScheme="dark" theme={nosqoTheme}>
      <RouterProvider router={router} />
    </MantineProvider>
  </StrictMode>,
);
