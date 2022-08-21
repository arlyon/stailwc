

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/backdrop-sepia
theme`backdropSepia.`

tw`backdrop-sepia-0`
tw`backdrop-sepia`

tw`backdrop-sepia-[.25]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/backdrop-sepia
({
  0: '0',
  DEFAULT: '100%',
});
({
  '--tw-backdrop-sepia': 'sepia(0)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
});
({
  '--tw-backdrop-sepia': 'sepia(100%)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
});
({
  '--tw-backdrop-sepia': 'sepia(.25)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
})


