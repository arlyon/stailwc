
tw`[section]:hover:block`

tw`[p]:hover:block`
tw`hover:[p]:block`

tw`[* + *]:block` // Spaces
tw`[.class1 .class2]:block` // Classes

tw`[.class1]:[.class2]:block` // Multiple dynamic variants
tw`[.class1 .class2]:[.class3]:block` // Multiple dynamic variants

tw`[p]:placeholder-red-500/[var(--myvar)]`
tw`[p]:mt-[var(--myvar)]`
tw`[p]:marginTop[var(--myvar)]`
tw`[p]:[margin-top:var(--myvar)]`

tw`[p]:(mt-4 mb-4)`

      ↓ ↓ ↓ ↓ ↓ ↓

({
  section: {
    ':hover': {
      display: 'block',
    },
  },
});
({
  p: {
    ':hover': {
      display: 'block',
    },
  },
});
({
  ':hover': {
    p: {
      display: 'block',
    },
  },
});
({
  '* + *': {
    display: 'block',
  },
}) // Spaces

({
  '.class1 .class2': {
    display: 'block',
  },
}) // Classes

({
  '.class1': {
    '.class2': {
      display: 'block',
    },
  },
}) // Multiple dynamic variants

({
  '.class1 .class2': {
    '.class3': {
      display: 'block',
    },
  },
}) // Multiple dynamic variants

({
  p: {
    '::placeholder': {
      color: 'rgb(239 68 68 / var(--myvar))',
    },
  },
});
({
  p: {
    marginTop: 'var(--myvar)',
  },
});
({
  p: {
    marginTop: 'var(--myvar)',
  },
});
({
  p: {
    marginTop: 'var(--myvar)',
  },
});
({
  p: {
    marginTop: '1rem',
    marginBottom: '1rem',
  },
})


