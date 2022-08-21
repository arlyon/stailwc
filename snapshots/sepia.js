

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/sepia
theme`sepia.`

tw`sepia-0`
tw`sepia`

tw`sepia-[.25]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/sepia
({
  0: '0',
  DEFAULT: '100%',
});
({
  '--tw-sepia': 'sepia(0)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-sepia': 'sepia(100%)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-sepia': 'sepia(.25)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
})


