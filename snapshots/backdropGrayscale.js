

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/backdrop-grayscale
theme`backdropGrayscale.`

tw`backdrop-grayscale-0`
tw`backdrop-grayscale`

tw`backdrop-grayscale-[.5]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/backdrop-grayscale
({
  0: '0',
  DEFAULT: '100%',
});
({
  '--tw-backdrop-grayscale': 'grayscale(0)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
});
({
  '--tw-backdrop-grayscale': 'grayscale(100%)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
});
({
  '--tw-backdrop-grayscale': 'grayscale(.5)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
})


