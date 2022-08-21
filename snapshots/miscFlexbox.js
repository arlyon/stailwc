
// col-span-x should always be moved to the start
// https://github.com/ben-rogerson/twin.macro/issues/363
tw`col-start-3 col-span-3`
tw`col-span-3 col-start-3`
tw`col-end-3 col-span-3`
tw`col-span-3 col-end-3`
tw`col-start-3 col-end-3 col-span-3`
tw`col-span-3 col-start-3 col-end-3`
tw`grid col-span-3`
tw`grid col-start-3 col-end-3`
tw`col-start-3 col-span-3 col-end-3 grid`
tw`col-start-3 mt-4 md:(mt-3 col-span-3) mb-4 col-end-3 col-span-3`

      ↓ ↓ ↓ ↓ ↓ ↓

// col-span-x should always be moved to the start
// https://github.com/ben-rogerson/twin.macro/issues/363
({
  gridColumn: 'span 3 / span 3',
  gridColumnStart: '3',
});
({
  gridColumn: 'span 3 / span 3',
  gridColumnStart: '3',
});
({
  gridColumn: 'span 3 / span 3',
  gridColumnEnd: '3',
});
({
  gridColumn: 'span 3 / span 3',
  gridColumnEnd: '3',
});
({
  gridColumn: 'span 3 / span 3',
  gridColumnStart: '3',
  gridColumnEnd: '3',
});
({
  gridColumn: 'span 3 / span 3',
  gridColumnStart: '3',
  gridColumnEnd: '3',
});
({
  gridColumn: 'span 3 / span 3',
  display: 'grid',
});
({
  display: 'grid',
  gridColumnStart: '3',
  gridColumnEnd: '3',
});
({
  gridColumn: 'span 3 / span 3',
  gridColumnStart: '3',
  gridColumnEnd: '3',
  display: 'grid',
});
({
  gridColumn: 'span 3 / span 3',
  gridColumnStart: '3',
  marginTop: '1rem',
  marginBottom: '1rem',
  gridColumnEnd: '3',
  '@media (min-width: 768px)': {
    gridColumn: 'span 3 / span 3',
    marginTop: '0.75rem',
  },
})


