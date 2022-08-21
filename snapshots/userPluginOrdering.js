
tw`selector`

      ↓ ↓ ↓ ↓ ↓ ↓

({
  content: '.selector',
  margin: '1px',
  padding: 'padding',
  display: 'block',
  '& .selector2': {
    content: '.selector .selector2',
  },
  ':hover': {
    content: '.selector:hover',
  },
  '&:hover .selector3': {
    content: '.selector:hover .selector3',
  },
  '@media (min-width: 1px)': {
    content: '@media .selector',
    '& .selector3': {
      content: '.selector:hover @media .selector3',
    },
    '&:hover .selector2': {
      content: '.selector:hover @media .selector',
    },
  },
  '@media (min-width: 2px)': {
    content: '.selector @media',
  },
})


