

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/background-image
theme`backgroundImage`

tw`bg-none`
tw`bg-gradient-to-t`
tw`bg-gradient-to-tr`
tw`bg-gradient-to-r`
tw`bg-gradient-to-br`
tw`bg-gradient-to-b`
tw`bg-gradient-to-bl`
tw`bg-gradient-to-l`
tw`bg-gradient-to-tl`

tw`bg-[image:custom]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/background-image
({
  none: 'none',
  'gradient-to-t': 'linear-gradient(to top, var(--tw-gradient-stops))',
  'gradient-to-tr': 'linear-gradient(to top right, var(--tw-gradient-stops))',
  'gradient-to-r': 'linear-gradient(to right, var(--tw-gradient-stops))',
  'gradient-to-br':
    'linear-gradient(to bottom right, var(--tw-gradient-stops))',
  'gradient-to-b': 'linear-gradient(to bottom, var(--tw-gradient-stops))',
  'gradient-to-bl': 'linear-gradient(to bottom left, var(--tw-gradient-stops))',
  'gradient-to-l': 'linear-gradient(to left, var(--tw-gradient-stops))',
  'gradient-to-tl': 'linear-gradient(to top left, var(--tw-gradient-stops))',
});
({
  backgroundImage: 'none',
});
({
  backgroundImage: 'linear-gradient(to top, var(--tw-gradient-stops))',
});
({
  backgroundImage: 'linear-gradient(to top right, var(--tw-gradient-stops))',
});
({
  backgroundImage: 'linear-gradient(to right, var(--tw-gradient-stops))',
});
({
  backgroundImage: 'linear-gradient(to bottom right, var(--tw-gradient-stops))',
});
({
  backgroundImage: 'linear-gradient(to bottom, var(--tw-gradient-stops))',
});
({
  backgroundImage: 'linear-gradient(to bottom left, var(--tw-gradient-stops))',
});
({
  backgroundImage: 'linear-gradient(to left, var(--tw-gradient-stops))',
});
({
  backgroundImage: 'linear-gradient(to top left, var(--tw-gradient-stops))',
});
({
  backgroundImage: 'custom',
})


