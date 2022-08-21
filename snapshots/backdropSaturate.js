

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/backdrop-saturate
theme`backdropSaturate`

tw`backdrop-saturate-0`
tw`backdrop-saturate-50`
tw`backdrop-saturate-100`
tw`backdrop-saturate-150`
tw`backdrop-saturate-200`

tw`backdrop-saturate-[.25]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/backdrop-saturate
({
  0: '0',
  50: '.5',
  100: '1',
  150: '1.5',
  200: '2',
});
({
  '--tw-backdrop-saturate': 'saturate(0)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
});
({
  '--tw-backdrop-saturate': 'saturate(.5)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
});
({
  '--tw-backdrop-saturate': 'saturate(1)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
});
({
  '--tw-backdrop-saturate': 'saturate(1.5)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
});
({
  '--tw-backdrop-saturate': 'saturate(2)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
});
({
  '--tw-backdrop-saturate': 'saturate(.25)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
})


