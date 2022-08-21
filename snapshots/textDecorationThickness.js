

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/text-decoration-thickness
theme`textDecorationThickness`

tw`decoration-auto`
tw`decoration-from-font`
tw`decoration-0`
tw`decoration-1`
tw`decoration-2`
tw`decoration-4`
tw`decoration-8`

tw`decoration-[length:10px]`
tw`decoration-[percentage:10%]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/text-decoration-thickness
({
  0: '0px',
  1: '1px',
  2: '2px',
  4: '4px',
  8: '8px',
  auto: 'auto',
  'from-font': 'from-font',
});
({
  textDecorationThickness: 'auto',
});
({
  textDecorationThickness: 'from-font',
});
({
  textDecorationThickness: '0px',
});
({
  textDecorationThickness: '1px',
});
({
  textDecorationThickness: '2px',
});
({
  textDecorationThickness: '4px',
});
({
  textDecorationThickness: '8px',
});
({
  textDecorationThickness: '10px',
});
({
  textDecorationThickness: '10%',
})


