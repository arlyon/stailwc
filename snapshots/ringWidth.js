

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/ring-width
theme`ringWidth.`

tw`ring-0`
tw`ring-1`
tw`ring-2`
tw`ring`
tw`ring-4`
tw`ring-8`
tw`ring-inset`

tw`ring-[10px]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/ring-width
({
  0: '0px',
  1: '1px',
  2: '2px',
  4: '4px',
  8: '8px',
  DEFAULT: '3px',
});
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(0px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
});
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
});
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
});
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
});
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
});
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(8px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
});
({
  '--tw-ring-inset': 'inset',
});
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(10px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
})


