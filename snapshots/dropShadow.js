

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/drop-shadow
theme`dropShadow`

tw`drop-shadow-sm`
tw`drop-shadow`
tw`drop-shadow-md`
tw`drop-shadow-lg`
tw`drop-shadow-xl`
tw`drop-shadow-2xl`
tw`drop-shadow-none`

tw`drop-shadow-[0 35px 35px rgba(0, 0, 0, 0.25)]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/drop-shadow
;['0 1px 2px rgb(0 0 0 / 0.1)', '0 1px 1px rgb(0 0 0 / 0.06)']
({
  '--tw-drop-shadow': 'drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-drop-shadow':
    'drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-drop-shadow':
    'drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-drop-shadow':
    'drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-drop-shadow':
    'drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-drop-shadow': 'drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-drop-shadow': 'drop-shadow(0 0 #0000)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-drop-shadow': 'drop-shadow(0 35px 35px rgba(0, 0, 0, 0.25))',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
})


