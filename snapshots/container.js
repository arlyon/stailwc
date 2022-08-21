

import tw, { theme } from './macro'

// https://tailwindcss.com/docs/container
theme`container`
tw`container`
tw`md:container md:mx-auto`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/container
({
  padding: {
    DEFAULT: ['1rem', '2rem'],
    sm: ['2rem'],
    lg: '4rem',
    xl: '6rem',
    object: '8rem',
    'object-width': '10rem',
    'object-min-max': '12rem',
  },
  margin: {
    DEFAULT: ['2rem', '3rem'],
    sm: ['auto'],
    lg: '5rem',
    xl: '7rem',
  },
});
({
  width: '100%',
  paddingLeft: '1rem',
  paddingRight: '2rem',
  marginLeft: '2rem',
  marginRight: '3rem',
  '@media (min-width: 640px)': {
    maxWidth: '640px',
    paddingLeft: '2rem',
    marginLeft: 'auto',
  },
  '@media (min-width: 768px)': {
    maxWidth: '768px',
  },
  '@media (min-width: 1024px)': {
    maxWidth: '1024px',
    paddingLeft: '4rem',
    paddingRight: '4rem',
    marginLeft: '5rem',
    marginRight: '5rem',
  },
  '@media (min-width: 1280px)': {
    maxWidth: '1280px',
    paddingLeft: '6rem',
    paddingRight: '6rem',
    marginLeft: '7rem',
    marginRight: '7rem',
  },
  '@media (min-width: 1536px)': {
    maxWidth: '1536px',
  },
  '@media (min-width: 968px)': {
    maxWidth: '968px',
    paddingLeft: '8rem',
    paddingRight: '8rem',
  },
  '@media (min-width: 992px)': {
    maxWidth: '992px',
    paddingLeft: '10rem',
    paddingRight: '10rem',
  },
  '@media (min-width: 1200px)': {
    maxWidth: '1200px',
    paddingLeft: '12rem',
    paddingRight: '12rem',
  },
});
({
  '@media (min-width: 768px)': {
    width: '100%',
    paddingLeft: '1rem',
    paddingRight: '2rem',
    marginLeft: 'auto',
    marginRight: 'auto',
    '@media (min-width: 640px)': {
      maxWidth: '640px',
      paddingLeft: '2rem',
      marginLeft: 'auto',
    },
    '@media (min-width: 768px)': {
      maxWidth: '768px',
    },
    '@media (min-width: 1024px)': {
      maxWidth: '1024px',
      paddingLeft: '4rem',
      paddingRight: '4rem',
      marginLeft: '5rem',
      marginRight: '5rem',
    },
    '@media (min-width: 1280px)': {
      maxWidth: '1280px',
      paddingLeft: '6rem',
      paddingRight: '6rem',
      marginLeft: '7rem',
      marginRight: '7rem',
    },
    '@media (min-width: 1536px)': {
      maxWidth: '1536px',
    },
    '@media (min-width: 968px)': {
      maxWidth: '968px',
      paddingLeft: '8rem',
      paddingRight: '8rem',
    },
    '@media (min-width: 992px)': {
      maxWidth: '992px',
      paddingLeft: '10rem',
      paddingRight: '10rem',
    },
    '@media (min-width: 1200px)': {
      maxWidth: '1200px',
      paddingLeft: '12rem',
      paddingRight: '12rem',
    },
  },
})


