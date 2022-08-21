
tw`[mask-image:linear-gradient(180deg,white, rgba(255,255,255,0))]`
tw`[-webkit-property:bg-black]`
tw`[--my-var:blue]`
tw`[color:var(--my-var)]`
tw`bg-black md:[color:var(--my-var)]`
tw`[margin:2px_4px_5px_1px]`

      ↓ ↓ ↓ ↓ ↓ ↓

({
  maskImage: 'linear-gradient(180deg,white, rgba(255,255,255,0))',
});
({
  WebkitProperty: 'bg-black',
});
({
  '--my-var': 'blue',
});
({
  color: 'var(--my-var)',
});
({
  '--tw-bg-opacity': '1',
  backgroundColor: 'rgb(0 0 0 / var(--tw-bg-opacity))',
  '@media (min-width: 768px)': {
    color: 'var(--my-var)',
  },
});
({
  margin: '2px 4px 5px 1px',
})


