

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/visibility
tw`visible`
tw`invisible`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/visibility
({
  visibility: 'visible',
});
({
  visibility: 'hidden',
})


