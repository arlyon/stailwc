
// Test the transform ordering - 'transform' should be moved to the start
// https://github.com/ben-rogerson/twin.macro/issues/363
tw`mt-5 translate-y-2 transform`
tw`translate-y-2 mt-5 md:(skew-y-6 transform) mb-5`

      ↓ ↓ ↓ ↓ ↓ ↓

// Test the transform ordering - 'transform' should be moved to the start
// https://github.com/ben-rogerson/twin.macro/issues/363
({
  transform:
    'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
  marginTop: '1.25rem',
  '--tw-translate-y': '0.5rem',
});
({
  '--tw-translate-y': '0.5rem',
  transform:
    'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
  marginTop: '1.25rem',
  marginBottom: '1.25rem',
  '@media (min-width: 768px)': {
    transform:
      'translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))',
    '--tw-skew-y': '6deg',
  },
})


