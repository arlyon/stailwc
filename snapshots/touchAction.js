
// https://tailwindcss.com/docs/touch-action
tw`touch-auto`
tw`touch-none`
tw`touch-pan-x`
tw`touch-pan-left`
tw`touch-pan-right`
tw`touch-pan-y`
tw`touch-pan-up`
tw`touch-pan-down`
tw`touch-pinch-zoom`
tw`touch-manipulation`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/touch-action
({
  touchAction: 'auto',
});
({
  touchAction: 'none',
});
({
  '--tw-pan-x': 'pan-x',
  touchAction: 'var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)',
});
({
  '--tw-pan-x': 'pan-left',
  touchAction: 'var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)',
});
({
  '--tw-pan-x': 'pan-right',
  touchAction: 'var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)',
});
({
  '--tw-pan-y': 'pan-y',
  touchAction: 'var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)',
});
({
  '--tw-pan-y': 'pan-up',
  touchAction: 'var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)',
});
({
  '--tw-pan-y': 'pan-down',
  touchAction: 'var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)',
});
({
  '--tw-pinch-zoom': 'pinch-zoom',
  touchAction: 'var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)',
});
({
  touchAction: 'manipulation',
})


