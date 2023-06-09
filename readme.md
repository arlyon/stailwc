# stailwc (speedy tailwind compiler)

This is an experimental SWC transpiler to bring compile time
tailwind macros to SWC (and nextjs) a-la twin macro. The goal
is to give the same great compile-time ergonomics and flexibility
while performing considerably better than babel-based alternatives.
Supports both `emotion` and `styled-components` for CSS-in-JS, and
many build systems such as SWC, nextjs, Vite, etc.

## Compatibility Chart

We are currently testing against the following versions. Other versions outside these may still work, however.

| stailwc | NextJS | Emotion | Styled Components | swc    | Vite  |
| ------- | ------ | ------- | ----------------- | ------ | ----- |
| latest  | 13.4.3 | 11.10.5 | 5.3.6             | 1.3.24 | 4.1.0 |

## Feature Chart

| Feature                         | Emotion | Styled Components |
| ------------------------------- | ------- | ----------------- |
| `tw` jsx attribute              | ✅      | ✅                |
| `tw` template tag               | ✅      | ✅                |
| `tw` component syntax           | ✅      | ✅                |
| `tw` component extension syntax | ✅      | ✅                |
| Global styles                   | ✅      | ✅                |
| Plugin parameter suggestions    | ✅      | ✅                |

## Getting started

```bash
> npm add -D stailwc
> yarn add -D stailwc
> pnpm add -D stailwc
```

To get started with NextJS, place the following in your next.config.js.
For other framworks / tools, please see the examples.

`next.config.js`

```js
const stailwc = require("stailwc/install");

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  experimental: {
    swcPlugins: [
      stailwc({
        engine: "emotion", // or "styled-components"
      }),
    ],
  },
  compiler: {
    emotion: true,
    // or
    styledComponents: true,
  },
};

module.exports = nextConfig;
```

Optionally, you can also include the tailwind normalizer + forms
plugin by including the `<TailwindStyle />` component. Please see
the relevant examples.

`_document.tsx`

```tsx
import { TailwindStyle } from "stailwc";

export default function App({ Component, pageProps }) {
  return (
    <>
      <TailwindStyle />
      <Component {...pageProps} />
    </>
  );
}
```

### Types

There is one step you need to take to get types working. You need to add `stailwc.d.ts` to the root of your source
folder.

## Usage

### The `tw` tag

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

### Component syntax

You can also create styled components using the `tw` template tag.
This will automatically create the correct syntax for both `emotion`
and `styled-components`.

```tsx
export const StyledButton = tw.button`p-1 m-4 text-green bg-white hover:(bg-gray text-yellow md:text-red) border-3`;
export const ShadowButton = tw(StyledButton)`shadow-lg`;
```

## Examples

There are examples available for both `emotion` and `styled-components`.
You can run them by cloning the repo and running `yarn` followed by
`yarn dev` in the example directory. You will need to `stailwc` first.
