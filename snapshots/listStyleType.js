

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/list-style-type
theme`listStyleType`

tw`list-none`
tw`list-disc`
tw`list-decimal`

tw`list-[upper-roman]`
tw`list-['1F44D']`
tw`list-[var(--value)]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/list-style-type
({
  none: 'none',
  disc: 'disc',
  decimal: 'decimal',
});
({
  listStyleType: 'none',
});
({
  listStyleType: 'disc',
});
({
  listStyleType: 'decimal',
});
({
  listStyleType: 'upper-roman',
});
({
  listStyleType: "'1F44D'",
});
({
  listStyleType: 'var(--value)',
})


