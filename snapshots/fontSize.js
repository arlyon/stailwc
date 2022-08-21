

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/font-size
theme`fontSize`

tw`text-xs`
tw`text-sm`
tw`text-base`
tw`text-lg`
tw`text-xl`
tw`text-2xl`
tw`text-3xl`
tw`text-4xl`
tw`text-5xl`
tw`text-6xl`
tw`text-7xl`
tw`text-8xl`
tw`text-9xl`

tw`text-[2.23rem]`
tw`text-[length:var(--font-size)]`

tw`text-arraystring`
tw`text-arrayobject`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/font-size
({
  xs: [
    '0.75rem',
    {
      lineHeight: '1rem',
    },
  ],
  sm: [
    '0.875rem',
    {
      lineHeight: '1.25rem',
    },
  ],
  base: [
    '1rem',
    {
      lineHeight: '1.5rem',
    },
  ],
  lg: [
    '1.125rem',
    {
      lineHeight: '1.75rem',
    },
  ],
  xl: [
    '1.25rem',
    {
      lineHeight: '1.75rem',
    },
  ],
  '2xl': [
    '1.5rem',
    {
      lineHeight: '2rem',
    },
  ],
  '3xl': [
    '1.875rem',
    {
      lineHeight: '2.25rem',
    },
  ],
  '4xl': [
    '2.25rem',
    {
      lineHeight: '2.5rem',
    },
  ],
  '5xl': [
    '3rem',
    {
      lineHeight: '1',
    },
  ],
  '6xl': [
    '3.75rem',
    {
      lineHeight: '1',
    },
  ],
  '7xl': [
    '4.5rem',
    {
      lineHeight: '1',
    },
  ],
  '8xl': [
    '6rem',
    {
      lineHeight: '1',
    },
  ],
  '9xl': [
    '8rem',
    {
      lineHeight: '1',
    },
  ],
  arraystring: ['0.875rem', '1.5'],
  arrayobject: [
    '0.875rem',
    {
      lineHeight: '2rem',
      color: 'red',
    },
  ],
});
({
  fontSize: '0.75rem',
  lineHeight: '1rem',
});
({
  fontSize: '0.875rem',
  lineHeight: '1.25rem',
});
({
  fontSize: '1rem',
  lineHeight: '1.5rem',
});
({
  fontSize: '1.125rem',
  lineHeight: '1.75rem',
});
({
  fontSize: '1.25rem',
  lineHeight: '1.75rem',
});
({
  fontSize: '1.5rem',
  lineHeight: '2rem',
});
({
  fontSize: '1.875rem',
  lineHeight: '2.25rem',
});
({
  fontSize: '2.25rem',
  lineHeight: '2.5rem',
});
({
  fontSize: '3rem',
  lineHeight: '1',
});
({
  fontSize: '3.75rem',
  lineHeight: '1',
});
({
  fontSize: '4.5rem',
  lineHeight: '1',
});
({
  fontSize: '6rem',
  lineHeight: '1',
});
({
  fontSize: '8rem',
  lineHeight: '1',
});
({
  fontSize: '2.23rem',
});
({
  fontSize: 'var(--font-size)',
});
({
  fontSize: '0.875rem',
  lineHeight: '1.5',
});
({
  fontSize: '0.875rem',
  lineHeight: '2rem',
  color: 'red',
})


