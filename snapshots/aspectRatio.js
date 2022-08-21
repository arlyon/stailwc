

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/aspect-ratio
theme`aspectRatio`

tw`aspect-auto`
tw`aspect-square`
tw`aspect-video`
tw`aspect-[4/3]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/aspect-ratio
({
  auto: 'auto',
  square: '1 / 1',
  video: '16 / 9',
});
({
  aspectRatio: 'auto',
});
({
  aspectRatio: '1 / 1',
});
({
  aspectRatio: '16 / 9',
});
({
  aspectRatio: '4/3',
})


