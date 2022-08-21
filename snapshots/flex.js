

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/flex
theme`flex`

tw`flex-1`
tw`flex-auto`
tw`flex-initial`
tw`flex-none`

tw`flex-[2 2 0%]`
tw`flex-[var(--flex)]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/flex
({
  1: '1 1 0%',
  auto: '1 1 auto',
  initial: '0 1 auto',
  none: 'none',
});
({
  flex: '1 1 0%',
});
({
  flex: '1 1 auto',
});
({
  flex: '0 1 auto',
});
({
  flex: 'none',
});
({
  flex: '2 2 0%',
});
({
  flex: 'var(--flex)',
})


