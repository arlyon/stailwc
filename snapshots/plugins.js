
// Tailwind plugin tests

tw`type-sm`

const addUtilitiesTest = tw`type-sm text-red-500 lg:type-sm md:type-sm!`

const addUtilitiesTest2 = tw`skew-15deg`
const addUtilitiesTest2Important = tw`skew-15deg! type-sm!`
const addUtilitiesTest2Media = tw`sm:skew-15deg lg:type-sm`
const addUtilitiesTest2Variants = tw`hover:active:skew-15deg even:visited:skew-15deg`

const addComponentsTest = tw`btn btn-blue btn-red`
// const addComponentsTestImportant = tw`btn! btn-blue!` // TODO: Issue showing sub selectors and important
const addComponentsTestMedia = tw`xl:btn sm:btn-blue lg:btn-red`
const addComponentsTestVariants = tw`hover:active:btn hocus:before:btn-blue even:visited:btn-red`

const addComponentsTestElementPrefixes = tw`prefixes`
const addComponentsTestElementScreenReplacements = tw`screenies`

      ↓ ↓ ↓ ↓ ↓ ↓

// Tailwind plugin tests
({
  fontSize: '0.875rem',
  fontWeight: '500',
  lineHeight: '1.25',
})
const addUtilitiesTest = {
  fontSize: '0.875rem',
  fontWeight: '500',
  lineHeight: '1.25',
  '--tw-text-opacity': '1',
  color: 'rgb(239 68 68 / var(--tw-text-opacity))',
  '@media (min-width: 768px)': {
    fontSize: '0.875rem !important',
    fontWeight: '500 !important',
    lineHeight: '1.25 !important',
  },
  '@media (min-width: 1024px)': {
    fontSize: '0.875rem',
    fontWeight: '500',
    lineHeight: '1.25',
  },
}
const addUtilitiesTest2 = {
  transform: 'skewY(-15deg)',
}
const addUtilitiesTest2Important = {
  transform: 'skewY(-15deg) !important',
  fontSize: '0.875rem !important',
  fontWeight: '500 !important',
  lineHeight: '1.25 !important',
}
const addUtilitiesTest2Media = {
  '@media (min-width: 640px)': {
    transform: 'skewY(-15deg)',
  },
  '@media (min-width: 1024px)': {
    fontSize: '0.875rem',
    fontWeight: '500',
    lineHeight: '1.25',
  },
}
const addUtilitiesTest2Variants = {
  ':hover': {
    ':active': {
      transform: 'skewY(-15deg)',
    },
  },
  ':nth-child(even)': {
    ':visited': {
      transform: 'skewY(-15deg)',
    },
  },
}
const addComponentsTest = {
  padding: '.5rem 1rem',
  borderRadius: '.25rem',
  fontWeight: '600',
  backgroundColor: '#e3342f',
  color: '#fff',
  ':hover': {
    backgroundColor: '#cc1f1a',
  },
} // const addComponentsTestImportant = tw`btn! btn-blue!` // TODO: Issue showing sub selectors and important

const addComponentsTestMedia = {
  '@media (min-width: 640px)': {
    backgroundColor: '#3490dc',
    color: '#fff',
    ':hover': {
      backgroundColor: '#2779bd',
    },
  },
  '@media (min-width: 1024px)': {
    backgroundColor: '#e3342f',
    color: '#fff',
    ':hover': {
      backgroundColor: '#cc1f1a',
    },
  },
  '@media (min-width: 1280px)': {
    padding: '.5rem 1rem',
    borderRadius: '.25rem',
    fontWeight: '600',
  },
}
const addComponentsTestVariants = {
  ':hover': {
    ':active': {
      padding: '.5rem 1rem',
      borderRadius: '.25rem',
      fontWeight: '600',
    },
  },
  ':hover, :focus': {
    ':before': {
      content: '""',
      backgroundColor: '#3490dc',
      color: '#fff',
      ':hover': {
        backgroundColor: '#2779bd',
      },
    },
  },
  ':nth-child(even)': {
    ':visited': {
      backgroundColor: '#e3342f',
      color: '#fff',
      ':hover': {
        backgroundColor: '#cc1f1a',
      },
    },
  },
}
const addComponentsTestElementPrefixes = {
  '& h1': {
    margin: 'auto',
    marginRight: '10px',
  },
  '& h2:hover': {
    color: 'red',
  },
  '& h3:hover': {
    color: 'green',
  },
  '& h3:active': {
    color: 'green',
  },
  '& :focus': {
    display: 'none',
  },
}
const addComponentsTestElementScreenReplacements = {
  '@media (min-width: 640px)': {
    display: 'block',
  },
  '@media (min-width: 768px)': {
    display: 'flex',
  },
  '@media (min-width: 1024px)': {
    display: 'inline-block',
  },
  '@media (min-width: 1280px)': {
    '& h1': {
      marginTop: '50px',
    },
    '& h1:hover': {
      color: 'blue',
    },
    '& h1:focus': {
      color: 'blue',
    },
  },
}


