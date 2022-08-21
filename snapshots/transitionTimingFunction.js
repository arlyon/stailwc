

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/transition-timing-function
theme`transitionTimingFunction.`

tw`ease-linear`
tw`ease-in`
tw`ease-out`
tw`ease-in-out`

tw`ease-[cubic-bezier(0.95, 0.05, 0.795, 0.035)]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/transition-timing-function
({
  DEFAULT: 'cubic-bezier(0.4, 0, 0.2, 1)',
  linear: 'linear',
  in: 'cubic-bezier(0.4, 0, 1, 1)',
  out: 'cubic-bezier(0, 0, 0.2, 1)',
  'in-out': 'cubic-bezier(0.4, 0, 0.2, 1)',
});
({
  transitionTimingFunction: 'linear',
});
({
  transitionTimingFunction: 'cubic-bezier(0.4, 0, 1, 1)',
});
({
  transitionTimingFunction: 'cubic-bezier(0, 0, 0.2, 1)',
});
({
  transitionTimingFunction: 'cubic-bezier(0.4, 0, 0.2, 1)',
});
({
  transitionTimingFunction: 'cubic-bezier(0.95, 0.05, 0.795, 0.035)',
})


