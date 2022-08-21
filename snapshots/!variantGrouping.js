
const basic = tw`group-hover:(flex m-10)`
const subMediaQuery = tw`focus-within:(md:flex mt-5)`
const multipleClasses = tw`hover:(bg-black text-white underline)`
const pseudoElement = tw`before:(w-10 h-10 block bg-black)`
const pseudoElementNoContent = tw`before:(w-10 h-10 block bg-black)`
const pseudoElementsNoContent = tw`before:(w-10 h-10) after:(w-10 h-10)`
const mediaHover = tw`sm:hover:(bg-black text-white)`
const sloppySpacing = tw` last:( flex  mt-5)`
const multipleGroups = tw`focus:(w-10 h-10 block bg-black) focus-within:(md:flex mt-5)`
const nestedGroups = tw`md:(w-10 hocus:(h-10 block bg-black))`

tw`(block w-10)`
tw`((block w-10))`
tw`hover:(block w-10)`
tw`md:(hover:(block w-10))`
tw`md:(hover:(block (block) w-10))`
tw`md:(hover:(block (h-10) w-10))`

// Important
tw`(block w-10)!`
tw`hover:(block w-10)!`
tw`md:(hover:(block (block)! w-10))`
tw`md:(hover:(block (h-10)! w-10))`
tw`md:(hover:(block w-10)!)`

// Ignored
tw``
tw`()`
tw`md:()`
tw`md:(hover:())`

// Slash opacity
tw`first:(bg-[black]/20 block)`
tw`first:(bg-[black]/[.20] block)`
tw`first:(bg-[black]/[.20] inline last:(bg-[black]/[.20] block))`
tw`first:(block bg-[black]/[.20])`

      ↓ ↓ ↓ ↓ ↓ ↓

const basic = {
  '.group:hover &': {
    display: 'flex',
    margin: '2.5rem',
  },
}
const subMediaQuery = {
  '@media (min-width: 768px)': {
    ':focus-within': {
      display: 'flex',
    },
  },
  ':focus-within': {
    marginTop: '1.25rem',
  },
}
const multipleClasses = {
  ':hover': {
    '--tw-bg-opacity': '1',
    backgroundColor: 'rgb(0 0 0 / var(--tw-bg-opacity))',
    '--tw-text-opacity': '1',
    color: 'rgb(255 255 255 / var(--tw-text-opacity))',
    textDecorationLine: 'underline',
  },
}
const pseudoElement = {
  ':before': {
    content: '""',
    width: '2.5rem',
    height: '2.5rem',
    display: 'block',
    '--tw-bg-opacity': '1',
    backgroundColor: 'rgb(0 0 0 / var(--tw-bg-opacity))',
  },
}
const pseudoElementNoContent = {
  ':before': {
    content: '""',
    width: '2.5rem',
    height: '2.5rem',
    display: 'block',
    '--tw-bg-opacity': '1',
    backgroundColor: 'rgb(0 0 0 / var(--tw-bg-opacity))',
  },
}
const pseudoElementsNoContent = {
  ':before': {
    content: '""',
    width: '2.5rem',
    height: '2.5rem',
  },
  ':after': {
    content: '""',
    width: '2.5rem',
    height: '2.5rem',
  },
}
const mediaHover = {
  '@media (min-width: 640px)': {
    ':hover': {
      '--tw-bg-opacity': '1',
      backgroundColor: 'rgb(0 0 0 / var(--tw-bg-opacity))',
      '--tw-text-opacity': '1',
      color: 'rgb(255 255 255 / var(--tw-text-opacity))',
    },
  },
}
const sloppySpacing = {
  ':last-child': {
    display: 'flex',
    marginTop: '1.25rem',
  },
}
const multipleGroups = {
  ':focus': {
    width: '2.5rem',
    height: '2.5rem',
    display: 'block',
    '--tw-bg-opacity': '1',
    backgroundColor: 'rgb(0 0 0 / var(--tw-bg-opacity))',
  },
  '@media (min-width: 768px)': {
    ':focus-within': {
      display: 'flex',
    },
  },
  ':focus-within': {
    marginTop: '1.25rem',
  },
}
const nestedGroups = {
  '@media (min-width: 768px)': {
    width: '2.5rem',
    ':hover, :focus': {
      height: '2.5rem',
      display: 'block',
      '--tw-bg-opacity': '1',
      backgroundColor: 'rgb(0 0 0 / var(--tw-bg-opacity))',
    },
  },
}
({
  display: 'block',
  width: '2.5rem',
});
({
  display: 'block',
  width: '2.5rem',
});
({
  ':hover': {
    display: 'block',
    width: '2.5rem',
  },
});
({
  '@media (min-width: 768px)': {
    ':hover': {
      display: 'block',
      width: '2.5rem',
    },
  },
});
({
  '@media (min-width: 768px)': {
    ':hover': {
      display: 'block',
      width: '2.5rem',
    },
  },
});
({
  '@media (min-width: 768px)': {
    ':hover': {
      display: 'block',
      height: '2.5rem',
      width: '2.5rem',
    },
  },
}) // Important

({
  display: 'block !important',
  width: '2.5rem !important',
});
({
  ':hover': {
    display: 'block !important',
    width: '2.5rem !important',
  },
});
({
  '@media (min-width: 768px)': {
    ':hover': {
      display: 'block !important',
      width: '2.5rem',
    },
  },
});
({
  '@media (min-width: 768px)': {
    ':hover': {
      display: 'block',
      height: '2.5rem !important',
      width: '2.5rem',
    },
  },
});
({
  '@media (min-width: 768px)': {
    ':hover': {
      display: 'block !important',
      width: '2.5rem !important',
    },
  },
}) // Ignored

({});
({});
({});
({}) // Slash opacity

({
  ':first-child': {
    backgroundColor: 'rgb(0 0 0 / 0.2)',
    display: 'block',
  },
});
({
  ':first-child': {
    backgroundColor: 'rgb(0 0 0 / .20)',
    display: 'block',
  },
});
({
  ':first-child': {
    backgroundColor: 'rgb(0 0 0 / .20)',
    display: 'inline',
    ':last-child': {
      backgroundColor: 'rgb(0 0 0 / .20)',
      display: 'block',
    },
  },
});
({
  ':first-child': {
    display: 'block',
    backgroundColor: 'rgb(0 0 0 / .20)',
  },
})


