import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { TailwindStyle } from "stailwc";

// currently needed due to a swc bug
// ===
import { css, Global } from "@emotion/react";
css;
Global;
// ===

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <TailwindStyle />
    <App />
  </StrictMode>
);
