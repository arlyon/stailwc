

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/divide-width
theme`divideWidth.`

tw`divide-x-0`
tw`divide-x-2`
tw`divide-x-4`
tw`divide-x-8`
tw`divide-x`
tw`divide-y-0`
tw`divide-y-2`
tw`divide-y-4`
tw`divide-y-8`
tw`divide-y`
tw`divide-x-reverse`
tw`divide-y-reverse`

tw`divide-x-[3px]`
tw`divide-y-[3px]`

tw`divide-x-[line-width:3px]`
tw`divide-x-[length:3px]`

tw`divide-y-[line-width:3px]`
tw`divide-y-[length:3px]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/divide-width
({
  0: '0px',
  2: '2px',
  4: '4px',
  8: '8px',
  DEFAULT: '1px',
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': '0',
    borderRightWidth: 'calc(0px * var(--tw-divide-x-reverse))',
    borderLeftWidth: 'calc(0px * calc(1 - var(--tw-divide-x-reverse)))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': '0',
    borderRightWidth: 'calc(2px * var(--tw-divide-x-reverse))',
    borderLeftWidth: 'calc(2px * calc(1 - var(--tw-divide-x-reverse)))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': '0',
    borderRightWidth: 'calc(4px * var(--tw-divide-x-reverse))',
    borderLeftWidth: 'calc(4px * calc(1 - var(--tw-divide-x-reverse)))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': '0',
    borderRightWidth: 'calc(8px * var(--tw-divide-x-reverse))',
    borderLeftWidth: 'calc(8px * calc(1 - var(--tw-divide-x-reverse)))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': '0',
    borderRightWidth: 'calc(1px * var(--tw-divide-x-reverse))',
    borderLeftWidth: 'calc(1px * calc(1 - var(--tw-divide-x-reverse)))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': '0',
    borderTopWidth: 'calc(0px * calc(1 - var(--tw-divide-y-reverse)))',
    borderBottomWidth: 'calc(0px * var(--tw-divide-y-reverse))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': '0',
    borderTopWidth: 'calc(2px * calc(1 - var(--tw-divide-y-reverse)))',
    borderBottomWidth: 'calc(2px * var(--tw-divide-y-reverse))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': '0',
    borderTopWidth: 'calc(4px * calc(1 - var(--tw-divide-y-reverse)))',
    borderBottomWidth: 'calc(4px * var(--tw-divide-y-reverse))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': '0',
    borderTopWidth: 'calc(8px * calc(1 - var(--tw-divide-y-reverse)))',
    borderBottomWidth: 'calc(8px * var(--tw-divide-y-reverse))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': '0',
    borderTopWidth: 'calc(1px * calc(1 - var(--tw-divide-y-reverse)))',
    borderBottomWidth: 'calc(1px * var(--tw-divide-y-reverse))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': '1',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': '1',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': '0',
    borderRightWidth: 'calc(3px * var(--tw-divide-x-reverse))',
    borderLeftWidth: 'calc(3px * calc(1 - var(--tw-divide-x-reverse)))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': '0',
    borderTopWidth: 'calc(3px * calc(1 - var(--tw-divide-y-reverse)))',
    borderBottomWidth: 'calc(3px * var(--tw-divide-y-reverse))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': '0',
    borderRightWidth: 'calc(3px * var(--tw-divide-x-reverse))',
    borderLeftWidth: 'calc(3px * calc(1 - var(--tw-divide-x-reverse)))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': '0',
    borderRightWidth: 'calc(3px * var(--tw-divide-x-reverse))',
    borderLeftWidth: 'calc(3px * calc(1 - var(--tw-divide-x-reverse)))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': '0',
    borderTopWidth: 'calc(3px * calc(1 - var(--tw-divide-y-reverse)))',
    borderBottomWidth: 'calc(3px * var(--tw-divide-y-reverse))',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': '0',
    borderTopWidth: 'calc(3px * calc(1 - var(--tw-divide-y-reverse)))',
    borderBottomWidth: 'calc(3px * var(--tw-divide-y-reverse))',
  },
})


