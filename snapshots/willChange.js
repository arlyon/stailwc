

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/will-change
theme`willChange`

tw`will-change-auto`
tw`will-change-scroll`
tw`will-change-contents`
tw`will-change-transform`
tw`will-change-[top, left]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/will-change
({
  auto: 'auto',
  scroll: 'scroll-position',
  contents: 'contents',
  transform: 'transform',
});
({
  willChange: 'auto',
});
({
  willChange: 'scroll-position',
});
({
  willChange: 'contents',
});
({
  willChange: 'transform',
});
({
  willChange: 'top, left',
})


