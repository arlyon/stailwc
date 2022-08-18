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
    swcPlugins: [["stailwc", {}]],
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

const MyComponent = () => {
  const [clicked, setClicked] = useState(0);
  return (
    <div tw="bg-white border-2 border-orange m-2">
      <button
        tw="p-1"
        css={clicked % 2 == 0 ? tw`text-green` : tw`text-blue`}
        onClick={() => setClicked(clicked + 1)}
      >
        Clicked {clicked} times
      </button>
    </div>
  );
};
```

## Caveats

Next currently doesn't support the SWC error handler meaning
that errors are logged only to the command line, and not shown
visually on the screen. This will be supported down the line.
