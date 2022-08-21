

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/text-underline-offset
theme`textUnderlineOffset`

tw`underline-offset-auto`
tw`underline-offset-0`
tw`underline-offset-1`
tw`underline-offset-2`
tw`underline-offset-4`
tw`underline-offset-8`

tw`underline-offset-[3px]`
tw`underline-offset-[length:3px]`
tw`underline-offset-[30%]`
tw`underline-offset-[percentage:30%]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/text-underline-offset
({
  0: '0px',
  1: '1px',
  2: '2px',
  4: '4px',
  8: '8px',
  auto: 'auto',
});
({
  textUnderlineOffset: 'auto',
});
({
  textUnderlineOffset: '0px',
});
({
  textUnderlineOffset: '1px',
});
({
  textUnderlineOffset: '2px',
});
({
  textUnderlineOffset: '4px',
});
({
  textUnderlineOffset: '8px',
});
({
  textUnderlineOffset: '3px',
});
({
  textUnderlineOffset: '3px',
});
({
  textUnderlineOffset: '30%',
});
({
  textUnderlineOffset: '30%',
})


