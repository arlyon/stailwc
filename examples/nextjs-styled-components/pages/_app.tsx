import { TailwindStyle } from "stailwc";

export default function App({ Component, pageProps }) {
  return (
    <>
      <TailwindStyle />
      <Component {...pageProps} />
    </>
  );
}
