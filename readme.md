# stailwc (speedy tailwind compiler)

This is an experimental SWC transpiler to bring compile time
tailwind macros to SWC (and nextjs) a-la twin macro. The goal
is to give the same great performance and flexibility while
performing considerably better than babel-based alternatives
(about 11x faster in my experience, proper benchmarks coming soon!)

> 🚨 We currently only support NextJS 13.0.1

## Getting started

```bash
> yarn add stailwc
```

Currently the setup process is a little bit convoluted, but it
will be cleaned up in the future, once we determine the best
way to package this. Place the following in your next.config.js:

`next.config.js`

```js
const stailwc = require("stailwc/install");

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  experimental: {
    swcPlugins: [stailwc()],
  },
  compiler: {
    emotion: true,
  },
};

module.exports = nextConfig;
```

Optionally, you can also include the tailwind normalizer.

`_document.tsx`

```tsx
import { Html, Head, Main, NextScript } from "next/document";

export default function Document() {
  return (
    <Html>
      <Head>
        <link
          rel="stylesheet"
          type="text/css"
          href="https://unpkg.com/tailwindcss@3.1.8/src/css/preflight.css"
        />
      </Head>
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
```

Now get hacking!

## Usage

You can interact with stailwc in two ways. The first is through
the `tw` JSW attribute, and the second is via the `tw` template
tag.

```tsx
import { useState } from "react";

export const ColorButton = () => {
  const [clicked, setClicked] = useState(0);
  return (
    <button
      tw="p-1 m-4 text-green bg-white hover:(bg-gray text-yellow md:text-red) border-3"
      css={clicked % 2 == 0 ? tw`border-green` : tw`border-blue`}
      onClick={() => setClicked(clicked + 1)}
    >
      Clicked {clicked} times
    </button>
  );
};
```

## Caveats

Next currently doesn't support the SWC error handler meaning
that errors are logged only to the command line, and not shown
visually on the screen. This will be supported down the line
(see here: https://github.com/vercel/next.js/discussions/39779).
