

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/grayscale
theme`grayscale.`

tw`grayscale-0`
tw`grayscale`

tw`grayscale-[50%]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/grayscale
({
  0: '0',
  DEFAULT: '100%',
});
({
  '--tw-grayscale': 'grayscale(0)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-grayscale': 'grayscale(100%)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-grayscale': 'grayscale(50%)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
})


