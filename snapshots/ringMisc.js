
// Combined ring classes
tw`ring ring-inset ring-purple-500 ring-offset-black ring-offset-4 ring-opacity-50`
tw`ring ring-inset ring-purple-500 ring-offset-black ring-offset-4`
tw`ring ring-purple-500 ring-offset-black ring-offset-4`
tw`ring ring-offset-black ring-offset-4`
tw`ring ring-offset-4`

// Test the ring-opacity ordering - 'ring-opacity-x' should be moved to the end
// https://github.com/ben-rogerson/twin.macro/issues/374
tw`ring-4 ring-opacity-20 ring-green-500`
tw`mt-5 md:(ring-opacity-20 ring-4 ring-green-500) mb-5`

tw`ring-[10px]`

      ↓ ↓ ↓ ↓ ↓ ↓

// Combined ring classes
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
  '--tw-ring-inset': 'inset',
  '--tw-ring-opacity': '0.5',
  '--tw-ring-color': 'rgb(168 85 247 / var(--tw-ring-opacity))',
  '--tw-ring-offset-color': '#000',
  '--tw-ring-offset-width': '4px',
});
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
  '--tw-ring-inset': 'inset',
  '--tw-ring-opacity': '1',
  '--tw-ring-color': 'rgb(168 85 247 / var(--tw-ring-opacity))',
  '--tw-ring-offset-color': '#000',
  '--tw-ring-offset-width': '4px',
});
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
  '--tw-ring-opacity': '1',
  '--tw-ring-color': 'rgb(168 85 247 / var(--tw-ring-opacity))',
  '--tw-ring-offset-color': '#000',
  '--tw-ring-offset-width': '4px',
});
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
  '--tw-ring-offset-color': '#000',
  '--tw-ring-offset-width': '4px',
});
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
  '--tw-ring-offset-width': '4px',
}) // Test the ring-opacity ordering - 'ring-opacity-x' should be moved to the end
// https://github.com/ben-rogerson/twin.macro/issues/374

({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
  '--tw-ring-opacity': '0.2',
  '--tw-ring-color': 'rgb(34 197 94 / var(--tw-ring-opacity))',
});
({
  marginTop: '1.25rem',
  marginBottom: '1.25rem',
  '@media (min-width: 768px)': {
    '--tw-ring-offset-shadow':
      'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
    '--tw-ring-shadow':
      'var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
    boxShadow:
      'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
    '--tw-ring-opacity': '0.2',
    '--tw-ring-color': 'rgb(34 197 94 / var(--tw-ring-opacity))',
  },
});
({
  '--tw-ring-offset-shadow':
    'var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)',
  '--tw-ring-shadow':
    'var(--tw-ring-inset) 0 0 0 calc(10px + var(--tw-ring-offset-width)) var(--tw-ring-color)',
  boxShadow:
    'var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)',
})


