import { TailwindStyle } from "stailwc";

// currently needed due to a swc bug
// ===
import { css, Global } from "@emotion/react";
css;
Global;
// ===

export default function App({ Component, pageProps }: { Component: any; pageProps: any }) {
  return (
    <>
      <TailwindStyle />
      <Component {...pageProps} />
    </>
  );
}
