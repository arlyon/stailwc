import { TailwindStyle } from "stailwc";

// currently needed due to a swc bug
// ===
import { css, Global } from "@emotion/react";
css;
Global;
// ===

export default function App({ Component, pageProps }) {
  return (
    <>
      <TailwindStyle />
      <Component {...pageProps} />
    </>
  );
}
