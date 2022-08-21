

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/object-position
theme`objectPosition`

tw`object-bottom`
tw`object-center`
tw`object-left`
tw`object-left-bottom`
tw`object-left-top`
tw`object-right`
tw`object-right-bottom`
tw`object-right-top`
tw`object-top`

tw`object-[center bottom]`
tw`object-[var(--position)]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/object-position
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
  objectPosition: 'bottom',
});
({
  objectPosition: 'center',
});
({
  objectPosition: 'left',
});
({
  objectPosition: 'left bottom',
});
({
  objectPosition: 'left top',
});
({
  objectPosition: 'right',
});
({
  objectPosition: 'right bottom',
});
({
  objectPosition: 'right top',
});
({
  objectPosition: 'top',
});
({
  objectPosition: 'center bottom',
});
({
  objectPosition: 'var(--position)',
})


