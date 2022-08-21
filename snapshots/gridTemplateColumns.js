

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/grid-template-columns
theme`gridTemplateColumns`

tw`grid-cols-1`
tw`grid-cols-2`
tw`grid-cols-3`
tw`grid-cols-4`
tw`grid-cols-5`
tw`grid-cols-6`
tw`grid-cols-7`
tw`grid-cols-8`
tw`grid-cols-9`
tw`grid-cols-10`
tw`grid-cols-11`
tw`grid-cols-12`
tw`grid-cols-none`

tw`grid-cols-[200px minmax(900px, 1fr) 100px]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/grid-template-columns
({
  1: 'repeat(1, minmax(0, 1fr))',
  2: 'repeat(2, minmax(0, 1fr))',
  3: 'repeat(3, minmax(0, 1fr))',
  4: 'repeat(4, minmax(0, 1fr))',
  5: 'repeat(5, minmax(0, 1fr))',
  6: 'repeat(6, minmax(0, 1fr))',
  7: 'repeat(7, minmax(0, 1fr))',
  8: 'repeat(8, minmax(0, 1fr))',
  9: 'repeat(9, minmax(0, 1fr))',
  10: 'repeat(10, minmax(0, 1fr))',
  11: 'repeat(11, minmax(0, 1fr))',
  12: 'repeat(12, minmax(0, 1fr))',
  none: 'none',
});
({
  gridTemplateColumns: 'repeat(1, minmax(0, 1fr))',
});
({
  gridTemplateColumns: 'repeat(2, minmax(0, 1fr))',
});
({
  gridTemplateColumns: 'repeat(3, minmax(0, 1fr))',
});
({
  gridTemplateColumns: 'repeat(4, minmax(0, 1fr))',
});
({
  gridTemplateColumns: 'repeat(5, minmax(0, 1fr))',
});
({
  gridTemplateColumns: 'repeat(6, minmax(0, 1fr))',
});
({
  gridTemplateColumns: 'repeat(7, minmax(0, 1fr))',
});
({
  gridTemplateColumns: 'repeat(8, minmax(0, 1fr))',
});
({
  gridTemplateColumns: 'repeat(9, minmax(0, 1fr))',
});
({
  gridTemplateColumns: 'repeat(10, minmax(0, 1fr))',
});
({
  gridTemplateColumns: 'repeat(11, minmax(0, 1fr))',
});
({
  gridTemplateColumns: 'repeat(12, minmax(0, 1fr))',
});
({
  gridTemplateColumns: 'none',
});
({
  gridTemplateColumns: '200px minmax(900px, 1fr) 100px',
})


