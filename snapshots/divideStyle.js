
// https://tailwindcss.com/docs/divide-style
tw`divide-solid`
tw`divide-dashed`
tw`divide-dotted`
tw`divide-double`
tw`divide-none`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/divide-style
({
  '> :not([hidden]) ~ :not([hidden])': {
    borderStyle: 'solid',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    borderStyle: 'dashed',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    borderStyle: 'dotted',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    borderStyle: 'double',
  },
});
({
  '> :not([hidden]) ~ :not([hidden])': {
    borderStyle: 'none',
  },
})


