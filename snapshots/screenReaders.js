
// https://tailwindcss.com/docs/screen-readers
tw`sr-only`
tw`not-sr-only`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/screen-readers
({
  position: 'absolute',
  width: '1px',
  height: '1px',
  padding: '0',
  margin: '-1px',
  overflow: 'hidden',
  clip: 'rect(0, 0, 0, 0)',
  whiteSpace: 'nowrap',
  borderWidth: '0',
});
({
  position: 'static',
  width: 'auto',
  height: 'auto',
  padding: '0',
  margin: '0',
  overflow: 'visible',
  clip: 'auto',
  whiteSpace: 'normal',
})


