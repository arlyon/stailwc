

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/min-height
theme`minHeight`

tw`min-h-0`
tw`min-h-full`
tw`min-h-screen`
tw`min-h-min`
tw`min-h-max`
tw`min-h-fit`

tw`min-h-[3.23rem]`
tw`min-h-[calc(100%+1rem)]`
tw`min-h-[var(--height)]`
tw`max-h-[3.23rem]`
tw`max-h-[calc(100%+1rem)]`
tw`max-h-[var(--height)]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/min-height
({
  0: '0px',
  full: '100%',
  screen: '100vh',
  min: 'min-content',
  max: 'max-content',
  fit: 'fit-content',
});
({
  minHeight: '0px',
});
({
  minHeight: '100%',
});
({
  minHeight: '100vh',
});
({
  minHeight: 'min-content',
});
({
  minHeight: 'max-content',
});
({
  minHeight: 'fit-content',
});
({
  minHeight: '3.23rem',
});
({
  minHeight: 'calc(100%+1rem)',
});
({
  minHeight: 'var(--height)',
});
({
  maxHeight: '3.23rem',
});
({
  maxHeight: 'calc(100%+1rem)',
});
({
  maxHeight: 'var(--height)',
})


