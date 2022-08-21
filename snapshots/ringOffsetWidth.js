

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/ring-offset-width
theme`ringOffsetWidth`

tw`ring-offset-0`
tw`ring-offset-1`
tw`ring-offset-2`
tw`ring-offset-4`
tw`ring-offset-8`

tw`ring-offset-[3px]`
tw`ring-offset-[19rem]`
tw`ring-offset-[#76ad65]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/ring-offset-width
({
  0: '0px',
  1: '1px',
  2: '2px',
  4: '4px',
  8: '8px',
});
({
  '--tw-ring-offset-width': '0px',
});
({
  '--tw-ring-offset-width': '1px',
});
({
  '--tw-ring-offset-width': '2px',
});
({
  '--tw-ring-offset-width': '4px',
});
({
  '--tw-ring-offset-width': '8px',
});
({
  '--tw-ring-offset-width': '3px',
});
({
  '--tw-ring-offset-width': '19rem',
});
({
  '--tw-ring-offset-color': '#76ad65',
})


