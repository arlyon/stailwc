
// https://tailwindcss.com/docs/scroll-snap-type
tw`snap-none`
tw`snap-x`
tw`snap-y`
tw`snap-both`
tw`snap-mandatory`
tw`snap-proximity`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/scroll-snap-type
({
  scrollSnapType: 'none',
});
({
  scrollSnapType: 'x var(--tw-scroll-snap-strictness)',
});
({
  scrollSnapType: 'y var(--tw-scroll-snap-strictness)',
});
({
  scrollSnapType: 'both var(--tw-scroll-snap-strictness)',
});
({
  '--tw-scroll-snap-strictness': 'mandatory',
});
({
  '--tw-scroll-snap-strictness': 'proximity',
})


