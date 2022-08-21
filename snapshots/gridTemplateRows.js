

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/grid-template-rows
theme`gridTemplateRows`

tw`grid-rows-1`
tw`grid-rows-2`
tw`grid-rows-3`
tw`grid-rows-4`
tw`grid-rows-5`
tw`grid-rows-6`
tw`grid-rows-none`

tw`grid-rows-[200px minmax(900px, 1fr) 100px]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/grid-template-rows
({
  1: 'repeat(1, minmax(0, 1fr))',
  2: 'repeat(2, minmax(0, 1fr))',
  3: 'repeat(3, minmax(0, 1fr))',
  4: 'repeat(4, minmax(0, 1fr))',
  5: 'repeat(5, minmax(0, 1fr))',
  6: 'repeat(6, minmax(0, 1fr))',
  none: 'none',
});
({
  gridTemplateRows: 'repeat(1, minmax(0, 1fr))',
});
({
  gridTemplateRows: 'repeat(2, minmax(0, 1fr))',
});
({
  gridTemplateRows: 'repeat(3, minmax(0, 1fr))',
});
({
  gridTemplateRows: 'repeat(4, minmax(0, 1fr))',
});
({
  gridTemplateRows: 'repeat(5, minmax(0, 1fr))',
});
({
  gridTemplateRows: 'repeat(6, minmax(0, 1fr))',
});
({
  gridTemplateRows: 'none',
});
({
  gridTemplateRows: '200px minmax(900px, 1fr) 100px',
})


