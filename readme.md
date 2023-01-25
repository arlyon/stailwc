# stailwc (speedy tailwind compiler)

This is an experimental SWC transpiler to bring compile time
tailwind macros to SWC (and nextjs) a-la twin macro. The goal
is to give the same great performance and flexibility while
performing considerably better than babel-based alternatives.
Supports both `emotion` and `styled-components`.

## Compatibility Chart

We are currently testing against the following versions:

| stailwc | NextJS | Emotion | Styled Components |
| ------- | ------ | ------- | ----------------- |
| latest  | 13.0.1 | 11.10.5 | 5.3.6             |

## Feature Chart

| Feature                         | Emotion        | Styled Components |
| ------------------------------- | -------------- | ----------------- |
| `tw` jsx attribute              | ✅             | ✅                |
| `tw` template tag               | ✅             | ✅                |
| `tw` component syntax           | ✅<sup>1</sup> | ✅                |
| `tw` component extension syntax | ✅<sup>1</sup> | ✅                |
| Global styles                   | ✅<sup>2</sup> | ⛔<sup>3</sup>    |
| plugin parameter suggestions    | ✅             | ✅                |

1. Currently `emotion` requires a dummy import at the top of the file
   due to a swc bug. This will be fixed in the future.
2. Due to the same issue, a dummy import is required at the top of
   the for `emotion`. This will be fixed in the future.
3. Due to the same issue, `styled-components` global css does not work.

## Getting started

```bash
> npm add -D stailwc
> yarn add -D stailwc
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
    // or
    styledComponents: true,
  },
};

module.exports = nextConfig;
```

Optionally, you can also include the tailwind normalizer + forms
plugin by including the `<TailwindStyle />` component. This is
currently not possible with `styled-components`, but will be once
issue [#22](https://github.com/arlyon/stailwc/issues/23) is resolved.

`_document.tsx`

```tsx
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
```

Now get hacking!

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
// currently needed with emotion due to a swc bug
// ===
import _styled from "@emotion/styled";
_styled;
// ===

export const StyledButton = tw.button`p-1 m-4 text-green bg-white hover:(bg-gray text-yellow md:text-red) border-3`;
export const ShadowButton = tw(StyledButton)`shadow-lg`;
```

## Examples

There are examples available for both `emotion` and `styled-components`.
You can run them by cloning the repo and running `yarn` followed by
`yarn dev` in the example directory. You will need to `stailwc` first.

## Types

There is one step you need to take to get types working. You need to add `stailwc.d.ts` to the root of your source
folder.
