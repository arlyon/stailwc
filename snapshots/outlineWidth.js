

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/outline-width
theme`outlineWidth`

tw`outline-0`
tw`outline-1`
tw`outline-2`
tw`outline-4`
tw`outline-8`

tw`outline outline-offset-2 outline-1`
tw`outline-[5px]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/outline-width
({
  0: '0px',
  1: '1px',
  2: '2px',
  4: '4px',
  8: '8px',
});
({
  outlineWidth: '0px',
});
({
  outlineWidth: '1px',
});
({
  outlineWidth: '2px',
});
({
  outlineWidth: '4px',
});
({
  outlineWidth: '8px',
});
({
  outlineStyle: 'solid',
  outlineOffset: '2px',
  outlineWidth: '1px',
});
({
  outlineWidth: '5px',
})


