

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/filter
tw`filter-none`
tw`filter` // Deprecated

// https://tailwindcss.com/docs/backdrop-filter
tw`backdrop-filter` // Deprecated
tw`backdrop-filter-none`

// All
tw`filter blur-2xl brightness-50 contrast-50 grayscale hue-rotate-180 invert saturate-50 sepia drop-shadow-2xl`

// All
tw`backdrop-filter backdrop-blur-2xl backdrop-brightness-50 backdrop-contrast-50 backdrop-grayscale backdrop-hue-rotate-180 backdrop-invert backdrop-opacity-50 backdrop-saturate-50 backdrop-sepia`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/filter
({
  filter: 'none',
});
({
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
}) // Deprecated
// https://tailwindcss.com/docs/backdrop-filter

({
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
}) // Deprecated

({
  backdropFilter: 'none',
}) // All

({
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
  '--tw-blur': 'blur(40px)',
  '--tw-brightness': 'brightness(.5)',
  '--tw-contrast': 'contrast(.5)',
  '--tw-grayscale': 'grayscale(100%)',
  '--tw-hue-rotate': 'hue-rotate(180deg)',
  '--tw-invert': 'invert(100%)',
  '--tw-saturate': 'saturate(.5)',
  '--tw-sepia': 'sepia(100%)',
  '--tw-drop-shadow': 'drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))',
}) // All

({
  backdropFilter:
    'var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)',
  '--tw-backdrop-blur': 'blur(40px)',
  '--tw-backdrop-brightness': 'brightness(.5)',
  '--tw-backdrop-contrast': 'contrast(.5)',
  '--tw-backdrop-grayscale': 'grayscale(100%)',
  '--tw-backdrop-hue-rotate': 'hue-rotate(180deg)',
  '--tw-backdrop-invert': 'invert(100%)',
  '--tw-backdrop-opacity': 'opacity(0.5)',
  '--tw-backdrop-saturate': 'saturate(.5)',
  '--tw-backdrop-sepia': 'sepia(100%)',
})


