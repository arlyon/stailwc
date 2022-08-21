

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/transform-origin
theme`transformOrigin`

tw`origin-center`
tw`origin-top`
tw`origin-top-right`
tw`origin-right`
tw`origin-bottom-right`
tw`origin-bottom`
tw`origin-bottom-left`
tw`origin-left`
tw`origin-top-left`

tw`origin-[33% 75%]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/transform-origin
({
  center: 'center',
  top: 'top',
  'top-right': 'top right',
  right: 'right',
  'bottom-right': 'bottom right',
  bottom: 'bottom',
  'bottom-left': 'bottom left',
  left: 'left',
  'top-left': 'top left',
});
({
  transformOrigin: 'center',
});
({
  transformOrigin: 'top',
});
({
  transformOrigin: 'top right',
});
({
  transformOrigin: 'right',
});
({
  transformOrigin: 'bottom right',
});
({
  transformOrigin: 'bottom',
});
({
  transformOrigin: 'bottom left',
});
({
  transformOrigin: 'left',
});
({
  transformOrigin: 'top left',
});
({
  transformOrigin: '33% 75%',
})


