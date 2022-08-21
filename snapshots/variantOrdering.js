
tw`before:valid:rtl:motion-safe:contrast-more:dark:print:portrait:any-pointer-fine:block`

tw`any-pointer-fine:portrait:print:dark:contrast-more:motion-safe:rtl:valid:before:mt-5`

      ↓ ↓ ↓ ↓ ↓ ↓

({
  '@media (prefers-reduced-motion: no-preference)': {
    '@media (prefers-contrast: more)': {
      '@media (prefers-color-scheme: dark)': {
        '@media print': {
          '@media (orientation: portrait)': {
            '@media (any-pointer: fine)': {
              ':before': {
                ':valid': {
                  '[dir="rtl"] &': {
                    content: '""',
                    display: 'block',
                  },
                },
              },
            },
          },
        },
      },
    },
  },
});
({
  '@media (any-pointer: fine)': {
    '@media (orientation: portrait)': {
      '@media print': {
        '@media (prefers-color-scheme: dark)': {
          '@media (prefers-contrast: more)': {
            '@media (prefers-reduced-motion: no-preference)': {
              '[dir="rtl"] &': {
                ':valid': {
                  ':before': {
                    content: '""',
                    marginTop: '1.25rem',
                  },
                },
              },
            },
          },
        },
      },
    },
  },
})


