

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/outline-offset
theme`outlineOffset`

tw`outline-offset-0`
tw`outline-offset-1`
tw`outline-offset-2`
tw`outline-offset-4`
tw`outline-offset-8`

tw`outline-offset-[3px]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/outline-offset
({
  0: '0px',
  1: '1px',
  2: '2px',
  4: '4px',
  8: '8px',
});
({
  outlineOffset: '0px',
});
({
  outlineOffset: '1px',
});
({
  outlineOffset: '2px',
});
({
  outlineOffset: '4px',
});
({
  outlineOffset: '8px',
});
({
  outlineOffset: '3px',
})


