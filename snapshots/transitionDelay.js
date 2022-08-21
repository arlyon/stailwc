

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/transition-delay
theme`transitionDelay`

tw`delay-75`
tw`delay-100`
tw`delay-150`
tw`delay-200`
tw`delay-300`
tw`delay-500`
tw`delay-700`
tw`delay-1000`

tw`delay-[2000ms]`
tw`delay-[var(--delay)]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/transition-delay
({
  75: '75ms',
  100: '100ms',
  150: '150ms',
  200: '200ms',
  300: '300ms',
  500: '500ms',
  700: '700ms',
  1000: '1000ms',
});
({
  transitionDelay: '75ms',
});
({
  transitionDelay: '100ms',
});
({
  transitionDelay: '150ms',
});
({
  transitionDelay: '200ms',
});
({
  transitionDelay: '300ms',
});
({
  transitionDelay: '500ms',
});
({
  transitionDelay: '700ms',
});
({
  transitionDelay: '1000ms',
});
({
  transitionDelay: '2000ms',
});
({
  transitionDelay: 'var(--delay)',
})


