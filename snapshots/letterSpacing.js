

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/letter-spacing
theme`letterSpacing`

tw`tracking-tighter`
tw`tracking-tight`
tw`tracking-normal`
tw`tracking-wide`
tw`tracking-wider`
tw`tracking-widest`

tw`-tracking-tighter`
tw`-tracking-tight`
tw`-tracking-normal`
tw`-tracking-wide`
tw`-tracking-wider`
tw`-tracking-widest`

tw`-tracking-[var(--tracking)]`
tw`tracking-[var(--tracking)]`
tw`-tracking-[2em]`
tw`tracking-[.25em]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/letter-spacing
({
  tighter: '-0.05em',
  tight: '-0.025em',
  normal: '0em',
  wide: '0.025em',
  wider: '0.05em',
  widest: '0.1em',
});
({
  letterSpacing: '-0.05em',
});
({
  letterSpacing: '-0.025em',
});
({
  letterSpacing: '0em',
});
({
  letterSpacing: '0.025em',
});
({
  letterSpacing: '0.05em',
});
({
  letterSpacing: '0.1em',
});
({
  letterSpacing: '0.05em',
});
({
  letterSpacing: '0.025em',
});
({
  letterSpacing: '-0em',
});
({
  letterSpacing: '-0.025em',
});
({
  letterSpacing: '-0.05em',
});
({
  letterSpacing: '-0.1em',
});
({
  letterSpacing: 'calc(var(--tracking) * -1)',
});
({
  letterSpacing: 'var(--tracking)',
});
({
  letterSpacing: '-2em',
});
({
  letterSpacing: '.25em',
})


