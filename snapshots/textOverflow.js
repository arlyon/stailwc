
// https://tailwindcss.com/docs/text-overflow
tw`truncate`
tw`text-ellipsis`
tw`text-clip`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/text-overflow
({
  overflow: 'hidden',
  textOverflow: 'ellipsis',
  whiteSpace: 'nowrap',
});
({
  textOverflow: 'ellipsis',
});
({
  textOverflow: 'clip',
})


