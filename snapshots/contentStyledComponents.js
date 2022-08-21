

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/content
theme`content`

tw`content`
tw`content-test`
tw`content-['hello']`
tw`content-[attr(content-before)]`
tw`content-['>']`
tw`content-none`
tw`before:block`
tw`peer-focus:before:block`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/content
"\"default\""
({
  '--tw-content': '"default"',
  content: 'var(--tw-content)',
});
({
  '--tw-content': '"hi"',
  content: 'var(--tw-content)',
});
({
  '--tw-content': "'hello'",
  content: 'var(--tw-content)',
});
({
  '--tw-content': 'attr(content-before)',
  content: 'var(--tw-content)',
});
({
  '--tw-content': "'>'",
  content: 'var(--tw-content)',
});
({
  '--tw-content': 'none',
  content: 'var(--tw-content)',
});
({
  ':before': {
    content: 'var(--tw-content)',
    display: 'block',
  },
});
({
  '.peer:focus ~ &': {
    ':before': {
      content: 'var(--tw-content)',
      display: 'block',
    },
  },
})


