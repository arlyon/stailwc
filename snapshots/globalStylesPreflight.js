

import { globalStyles } from './macro'

globalStyles

      ↓ ↓ ↓ ↓ ↓ ↓

({
  '@keyframes spin': {
    to: {
      transform: 'rotate(360deg)',
    },
  },
  '@keyframes ping': {
    '75%, 100%': {
      transform: 'scale(2)',
      opacity: '0',
    },
  },
  '@keyframes pulse': {
    '50%': {
      opacity: '.5',
    },
  },
  '@keyframes bounce': {
    '0%, 100%': {
      transform: 'translateY(-25%)',
      animationTimingFunction: 'cubic-bezier(0.8,0,1,1)',
    },
    '50%': {
      transform: 'none',
      animationTimingFunction: 'cubic-bezier(0,0,0.2,1)',
    },
  },
  '*, ::before, ::after': {
    '--tw-translate-x': '0',
    '--tw-translate-y': '0',
    '--tw-rotate': '0',
    '--tw-skew-x': '0',
    '--tw-skew-y': '0',
    '--tw-scale-x': '1',
    '--tw-scale-y': '1',
    '--tw-pan-x': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-pan-y': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-pinch-zoom': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-scroll-snap-strictness': 'proximity',
    '--tw-ordinal': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-slashed-zero': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-numeric-figure': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-numeric-spacing': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-numeric-fraction': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-ring-inset': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-ring-offset-width': '0px',
    '--tw-ring-offset-color': '#fff',
    '--tw-ring-color': 'rgb(59 130 246 / 0.5)',
    '--tw-ring-offset-shadow': '0 0 #0000',
    '--tw-ring-shadow': '0 0 #0000',
    '--tw-shadow': '0 0 #0000',
    '--tw-shadow-colored': '0 0 #0000',
    '--tw-blur': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-brightness': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-contrast': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-grayscale': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-hue-rotate': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-invert': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-saturate': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-sepia': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-drop-shadow': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-blur': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-brightness': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-contrast': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-grayscale': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-hue-rotate': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-invert': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-opacity': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-saturate': 'var(--tw-empty,/*!*/ /*!*/)',
    '--tw-backdrop-sepia': 'var(--tw-empty,/*!*/ /*!*/)',
  },
})


