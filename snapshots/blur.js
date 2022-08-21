

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/blur
theme`blur.`

tw`blur-none`
tw`blur-sm`
tw`blur`
tw`blur-md`
tw`blur-lg`
tw`blur-xl`
tw`blur-2xl`
tw`blur-3xl`

tw`blur-[2px]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/blur
({
  0: '0',
  none: '0',
  sm: '4px',
  DEFAULT: '8px',
  md: '12px',
  lg: '16px',
  xl: '24px',
  '2xl': '40px',
  '3xl': '64px',
});
({
  '--tw-blur': 'blur(0)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-blur': 'blur(4px)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-blur': 'blur(8px)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-blur': 'blur(12px)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-blur': 'blur(16px)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-blur': 'blur(24px)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-blur': 'blur(40px)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-blur': 'blur(64px)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
});
({
  '--tw-blur': 'blur(2px)',
  filter:
    'var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)',
})


