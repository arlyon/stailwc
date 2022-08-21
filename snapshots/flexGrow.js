

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/flex-grow
theme`flexGrow.`

tw`grow-0`
tw`grow`
tw`flex-grow-0` // Deprecated
tw`flex-grow` // Deprecated

tw`grow-[2]`
tw`flex-grow-[var(--grow)]` // Deprecated

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/flex-grow
({
  0: '0',
  DEFAULT: '1',
});
({
  flexGrow: '0',
});
({
  flexGrow: '1',
});
({
  flexGrow: '0',
}) // Deprecated

({
  flexGrow: '1',
}) // Deprecated

({
  flexGrow: '2',
});
({
  flexGrow: 'var(--grow)',
}) // Deprecated


