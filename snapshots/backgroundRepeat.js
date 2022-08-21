

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/background-repeat
theme`backgroundPosition`

tw`bg-repeat`
tw`bg-no-repeat`
tw`bg-repeat-x`
tw`bg-repeat-y`
tw`bg-repeat-round`
tw`bg-repeat-space`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/background-repeat
({
  bottom: 'bottom',
  center: 'center',
  left: 'left',
  'left-bottom': 'left bottom',
  'left-top': 'left top',
  right: 'right',
  'right-bottom': 'right bottom',
  'right-top': 'right top',
  top: 'top',
});
({
  backgroundRepeat: 'repeat',
});
({
  backgroundRepeat: 'no-repeat',
});
({
  backgroundRepeat: 'repeat-x',
});
({
  backgroundRepeat: 'repeat-y',
});
({
  backgroundRepeat: 'round',
});
({
  backgroundRepeat: 'space',
})


