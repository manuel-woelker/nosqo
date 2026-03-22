import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { RouterProvider } from "@tanstack/react-router";
import { router } from "./router";
import "./styles.css.ts";

const rootElement = document.getElementById("root");

if (!rootElement) {
  throw new Error("Missing root element for nosqo UI bootstrap.");
}

createRoot(rootElement).render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
);
