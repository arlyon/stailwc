

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/stroke-width
theme`strokeWidth`

tw`stroke-0`
tw`stroke-1`
tw`stroke-2`

tw`stroke-[2px]`

tw`stroke-[color:red]`
tw`stroke-[length:2px]`
tw`stroke-[number:10]`
tw`stroke-[percentage:10%]`
tw`stroke-[url:url(hand.cur)_2_2, pointer]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/stroke-width
({
  0: '0',
  1: '1',
  2: '2',
});
({
  strokeWidth: '0',
});
({
  strokeWidth: '1',
});
({
  strokeWidth: '2',
});
({
  strokeWidth: '2px',
});
({
  stroke: 'red',
});
({
  strokeWidth: '2px',
});
({
  strokeWidth: '10',
});
({
  strokeWidth: '10%',
});
({
  stroke: 'url(hand.cur) 2 2, pointer',
})


