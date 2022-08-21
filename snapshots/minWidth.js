

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/min-width
theme`minWidth`

tw`min-w-0`
tw`min-w-full`
tw`min-w-min`
tw`min-w-max`
tw`min-w-fit`

tw`min-w-[3.23rem]`
tw`min-w-[calc(100%+1rem)]`
tw`min-w-[var(--width)]`
tw`max-w-[3.23rem]`
tw`max-w-[calc(100%+1rem)]`
tw`max-w-[var(--width)]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/min-width
({
  0: '0px',
  full: '100%',
  min: 'min-content',
  max: 'max-content',
  fit: 'fit-content',
});
({
  minWidth: '0px',
});
({
  minWidth: '100%',
});
({
  minWidth: 'min-content',
});
({
  minWidth: 'max-content',
});
({
  minWidth: 'fit-content',
});
({
  minWidth: '3.23rem',
});
({
  minWidth: 'calc(100%+1rem)',
});
({
  minWidth: 'var(--width)',
});
({
  maxWidth: '3.23rem',
});
({
  maxWidth: 'calc(100%+1rem)',
});
({
  maxWidth: 'var(--width)',
})


