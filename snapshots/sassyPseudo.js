
tw`hover:block first:mt-2 last-of-type:max-width[20px]`
tw`hover:block first:mt-2 last-of-type:[max-width:20px]`

tw`my-class1`
tw`my-class2`
tw`my-class3`

      ↓ ↓ ↓ ↓ ↓ ↓

({
  '&:hover': {
    display: 'block',
  },
  '&:first-child': {
    marginTop: '0.5rem',
  },
  '&:last-of-type': {
    maxWidth: '20px',
  },
});
({
  '&:hover': {
    display: 'block',
  },
  '&:first-child': {
    marginTop: '0.5rem',
  },
  '&:last-of-type': {
    maxWidth: '20px',
  },
});
({
  '&:hover': {
    backgroundColor: 'pink',
  },
});
({
  '& :hover': {
    backgroundColor: 'orange',
  },
});
({
  '.test & :hover': {
    backgroundColor: 'orange',
  },
})


