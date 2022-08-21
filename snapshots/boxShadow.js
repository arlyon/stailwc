

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/box-shadow
theme`boxShadow.`

tw`shadow-sm`
tw`shadow`
tw`shadow-md`
tw`shadow-lg`
tw`shadow-xl`
tw`shadow-2xl`
tw`shadow-inner`
tw`shadow-none`

tw`shadow-[0 35px 60px -15px rgba(0,0,0,0.3)]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/box-shadow
({
  sm: '0 1px 2px 0 rgb(0 0 0 / 0.05)',
  DEFAULT: '0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)',
  md: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)',
  lg: '0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)',
  xl: '0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)',
  '2xl': '0 25px 50px -12px rgb(0 0 0 / 0.25)',
  inner: 'inset 0 2px 4px 0 rgb(0 0 0 / 0.05)',
  none: 'none',
});
({
  '--tw-shadow': '0 1px 2px 0 rgb(0 0 0 / 0.05)',
  '--tw-shadow-colored': '0 1px 2px 0 var(--tw-shadow-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
});
({
  '--tw-shadow':
    '0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)',
  '--tw-shadow-colored':
    '0 1px 3px 0 var(--tw-shadow-color), 0 1px 2px -1px var(--tw-shadow-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
});
({
  '--tw-shadow':
    '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)',
  '--tw-shadow-colored':
    '0 4px 6px -1px var(--tw-shadow-color), 0 2px 4px -2px var(--tw-shadow-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
});
({
  '--tw-shadow':
    '0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)',
  '--tw-shadow-colored':
    '0 10px 15px -3px var(--tw-shadow-color), 0 4px 6px -4px var(--tw-shadow-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
});
({
  '--tw-shadow':
    '0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)',
  '--tw-shadow-colored':
    '0 20px 25px -5px var(--tw-shadow-color), 0 8px 10px -6px var(--tw-shadow-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
});
({
  '--tw-shadow': '0 25px 50px -12px rgb(0 0 0 / 0.25)',
  '--tw-shadow-colored': '0 25px 50px -12px var(--tw-shadow-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
});
({
  '--tw-shadow': 'inset 0 2px 4px 0 rgb(0 0 0 / 0.05)',
  '--tw-shadow-colored': 'inset 0 2px 4px 0 var(--tw-shadow-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
});
({
  '--tw-shadow': '0 0 #0000',
  '--tw-shadow-colored': '0 0 #0000',
  boxShadow:
    'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
});
({
  '--tw-shadow': '0 35px 60px -15px rgba(0,0,0,0.3)',
  '--tw-shadow-colored': '0 35px 60px -15px var(--tw-shadow-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)',
})


