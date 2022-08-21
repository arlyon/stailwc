

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/transition-duration
theme`transitionDuration.`

tw`duration-75`
tw`duration-100`
tw`duration-150`
tw`duration-200`
tw`duration-300`
tw`duration-500`
tw`duration-700`
tw`duration-1000`

tw`duration-[2000ms]`
tw`duration-[2s]`
tw`duration-[var(--app-duration)]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/transition-duration
({
  75: '75ms',
  100: '100ms',
  150: '150ms',
  200: '200ms',
  300: '300ms',
  500: '500ms',
  700: '700ms',
  1000: '1000ms',
  DEFAULT: '150ms',
});
({
  transitionDuration: '75ms',
});
({
  transitionDuration: '100ms',
});
({
  transitionDuration: '150ms',
});
({
  transitionDuration: '200ms',
});
({
  transitionDuration: '300ms',
});
({
  transitionDuration: '500ms',
});
({
  transitionDuration: '700ms',
});
({
  transitionDuration: '1000ms',
});
({
  transitionDuration: '2000ms',
});
({
  transitionDuration: '2s',
});
({
  transitionDuration: 'var(--app-duration)',
})


