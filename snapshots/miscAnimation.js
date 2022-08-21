
// Test the transition ordering - 'transition' should be moved to the start
// https://github.com/ben-rogerson/twin.macro/issues/363
tw`duration-75 ease-in transition`
tw`mt-5 md:(ml-5 ease-in transition) mb-5`

      ↓ ↓ ↓ ↓ ↓ ↓

// Test the transition ordering - 'transition' should be moved to the start
// https://github.com/ben-rogerson/twin.macro/issues/363
({
  transitionProperty:
    'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
  transitionTimingFunction: 'cubic-bezier(0.4, 0, 1, 1)',
  transitionDuration: '75ms',
});
({
  marginTop: '1.25rem',
  marginBottom: '1.25rem',
  '@media (min-width: 768px)': {
    transitionProperty:
      'color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter',
    transitionTimingFunction: 'cubic-bezier(0.4, 0, 1, 1)',
    transitionDuration: '150ms',
    marginLeft: '1.25rem',
  },
})


