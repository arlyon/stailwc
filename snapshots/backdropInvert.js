

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/backdrop-invert
theme`backdropInvert.`

tw`backdrop-invert-0`
tw`backdrop-invert`

tw`backdrop-invert-[.25]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/backdrop-invert
({
  0: '0',
  DEFAULT: '100%',
});
({
  '--tw-backdrop-invert': 'invert(0)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
});
({
  '--tw-backdrop-invert': 'invert(100%)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
});
({
  '--tw-backdrop-invert': 'invert(.25)',
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
})


