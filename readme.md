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

Currently the setup process is a little bit convoluted, but it
will be cleaned up in the future, once we determine the best
way to package this. Place the following in your next.config.js:

`next.config.js`

```js
const initStailwc = () => {
  const resolveConfig = require("tailwindcss/resolveConfig");
  const defaultConfig = require("tailwindcss/defaultConfig");
  const override = require("./tailwind.config");

  const config = resolveConfig(override, defaultConfig);

  config.theme.colors = Object.entries(config.theme.colors)
    .flatMap(([k, v]) => {
      if (typeof v === "object") {
        const items = Object.entries(v).map(([k2, v2]) => [k + "-" + k2, v2]);
        return items;
      } else {
        return [[k, v]];
      }
    })
    .reduce(
      (acc, [k, v]) => ({
        ...acc,
        [k]: v,
      }),
      {}
    );

  return ["stailwc", { config }];
};

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  experimental: {
    swcPlugins: [initStailwc()],
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
visually on the screen. This will be supported down the line.
