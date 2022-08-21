
tw`my-class1`
tw`my-class2`
tw`my-class3`
tw`my-class4`
tw`my-class5`
tw`my-class6`

      ↓ ↓ ↓ ↓ ↓ ↓

({
  backgroundColor: 'black',
  '& h2': {
    backgroundColor: 'red',
  },
});
({
  backgroundColor: 'green',
  'h2 &': {
    backgroundColor: 'yellow',
  },
});
({
  backgroundColor: 'green',
  '.dark &:hover': {
    backgroundColor: 'yellow',
  },
});
({
  '.test & :hover': {
    backgroundColor: 'orange',
  },
});
({
  backgroundColor: 'brown',
  ':hover': {
    backgroundColor: 'pink',
  },
});
({
  backgroundColor: 'blue',
  '& :hover': {
    backgroundColor: 'orange',
  },
})


