

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/saturate
theme`saturate`

tw`saturate-0`
tw`saturate-50`
tw`saturate-100`
tw`saturate-150`
tw`saturate-200`

tw`saturate-[.25]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/saturate
({
  0: '0',
  50: '.5',
  100: '1',
  150: '1.5',
  200: '2',
});
({
  '--tw-saturate': 'saturate(0)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-saturate': 'saturate(.5)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-saturate': 'saturate(1)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-saturate': 'saturate(1.5)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-saturate': 'saturate(2)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-saturate': 'saturate(.25)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
})


