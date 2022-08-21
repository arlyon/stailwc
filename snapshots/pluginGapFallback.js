
tw`flex-gap-0`
tw`flex-gap-3`
tw`flex-gap-x-3`
tw`flex-gap-y-3`
tw`flex-gap-px`
tw`flex-gap-x-1.5`

tw`gap-0`
tw`gap-3`
tw`gap-x-3`
tw`gap-y-3`
tw`gap-px`
tw`gap-x-1.5`

tw`test-1`
tw`test-2`

      ↓ ↓ ↓ ↓ ↓ ↓

({
  margin: '-0px',
  '& > *': {
    margin: '0px',
  },
});
({
  margin: '-0.375rem',
  '& > *': {
    margin: '0.375rem',
  },
});
({
  marginRight: '-0.375rem',
  marginLeft: '-0.375rem',
  '& > *': {
    marginRight: '0.375rem',
    marginLeft: '0.375rem',
  },
});
({
  marginTop: '-0.375rem',
  marginBottom: '-0.375rem',
  '& > *': {
    marginTop: '0.375rem',
    marginBottom: '0.375rem',
  },
});
({
  margin: '-0.5px',
  '& > *': {
    margin: '0.5px',
  },
});
({
  '&': {
    marginRight: '-0.1875rem',
    marginLeft: '-0.1875rem',
  },
  '& > *': {
    marginRight: '0.1875rem',
    marginLeft: '0.1875rem',
  },
});
({
  '.no-flex-gap &': {
    margin: '-0px',
  },
  '.no-flex-gap & > *': {
    margin: '0px',
  },
});
({
  '.no-flex-gap &': {
    margin: '-0.375rem',
  },
  '.no-flex-gap & > *': {
    margin: '0.375rem',
  },
});
({
  '.no-flex-gap &': {
    marginRight: '-0.375rem',
    marginLeft: '-0.375rem',
  },
  '.no-flex-gap & > *': {
    marginRight: '0.375rem',
    marginLeft: '0.375rem',
  },
});
({
  '.no-flex-gap &': {
    marginTop: '-0.375rem',
    marginBottom: '-0.375rem',
  },
  '.no-flex-gap & > *': {
    marginTop: '0.375rem',
    marginBottom: '0.375rem',
  },
});
({
  '.no-flex-gap &': {
    margin: '-0.5px',
  },
  '.no-flex-gap & > *': {
    margin: '0.5px',
  },
});
({
  '.no-flex-gap &': {
    marginRight: '-0.1875rem',
    marginLeft: '-0.1875rem',
  },
  '.no-flex-gap & > *': {
    marginRight: '0.1875rem',
    marginLeft: '0.1875rem',
  },
});
({
  background: '5px',
  '.a-class & .some-class': {
    margin: '10px',
  },
  '.a-class & > *': {
    margin: '20px',
  },
});
({
  '.a-class & .some-class': {
    margin: '10px',
  },
  '.a-class & > *': {
    margin: '20px',
  },
})


