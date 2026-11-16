import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./ui/app/App";

// TODO(T-020): initialise i18n before rendering.
// TODO(T-021): import the theme stylesheets.

const rootElement = document.getElementById("root");
if (rootElement === null) {
  throw new Error("Missing #root element in index.html");
}

createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
