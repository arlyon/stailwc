

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/background-size
theme`backgroundSize`

tw`bg-auto`
tw`bg-cover`
tw`bg-contain`

tw`bg-[length:var(--value)]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/background-size
({
  auto: 'auto',
  cover: 'cover',
  contain: 'contain',
});
({
  backgroundSize: 'auto',
});
({
  backgroundSize: 'cover',
});
({
  backgroundSize: 'contain',
});
({
  backgroundSize: 'var(--value)',
})


