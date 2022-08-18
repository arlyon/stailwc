# stailwc (speedy tailwind compiler)

This is an experimental SWC transpiler to bring compile time
tailwind macros to SWC (and nextjs) a-la twin macro. The goal
is to give the same great performance and flexibility while
performing considerably better than babel-based alternatives.

> 🚨 We currently only support NextJS 12.2.5

## Getting started

```bash
> yarn add stailwc
```

`next.config.js`

```js
/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  experimental: {
    swcPlugins: [["stailwc, {}]],
  },
  compiler: {
    emotion: true,
  },
};

module.exports = nextConfig;
```

Optionally, you can also include the tailwind normalizer.

`_app.tsx`

```tsx
import type { AppProps } from "next/app";
import Head from "next/head";

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
        <link
          rel="stylesheet"
          type="text/css"
          href="https://unpkg.com/tailwindcss@3.1.8/src/css/preflight.css"
        />
      </Head>
      <Component {...pageProps} />
    </>
  );
}

export default MyApp;
```

Now get hacking!

## Caveats

Next currently doesn't support the SWC error handler meaning
that errors are logged only to the command line, and not shown
visually on the screen. This will be supported down the line.
