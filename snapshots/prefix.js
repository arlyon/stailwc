
// tw prop prefix
;<div tw="tw-text-black" />

// tw import prefix
;<div css={tw`tw-bg-red-500`} />

// tw prop + import prefix
;<div tw="tw-text-black" css={tw`lg:tw-bg-red-500`} />

// tw import + short css
;<div css={tw`lg:tw-bg-red-500 max-width[100vw]`} />

// tw import + arbitrary property
;<div css={tw`lg:tw-bg-red-500 [max-width:100vw]`} />

// className should be ignored without the prefix
;<div className="block" />

// className should be converted with a prefix
;<div className="tw-block" />

// group
;<div tw="hover:(lg:tw-bg-red-500)" />
;<div tw="hover:(lg:tw-bg-red-500 max-width[100vw])" />
;<div tw="hover:(lg:tw-bg-red-500 [max-width:100vw])" />
;<div css={tw`hover:(lg:tw-bg-red-500)`} />
;<div css={tw`hover:(lg:tw-bg-red-500 max-width[100vw])`} />
;<div css={tw`hover:(lg:tw-bg-red-500 [max-width:100vw])`} />

// custom plugin classes
;<div tw="tw-plugin-class" />
;<div tw="tw-test-1" />
;<div tw="tw-test-2" />

      ↓ ↓ ↓ ↓ ↓ ↓

;<div
  css={{
    '--tw-text-opacity': '1',
    color: 'rgb(0 0 0 / var(--tw-text-opacity))',
  }}
  data-tw="tw-text-black"
/> // tw import prefix
;<div
  css={{
    '--tw-bg-opacity': '1',
    backgroundColor: 'rgb(239 68 68 / var(--tw-bg-opacity))',
  }}
  data-tw={'tw-bg-red-500'}
/> // tw prop + import prefix
;<div
  css={[
    {
      '--tw-text-opacity': '1',
      color: 'rgb(0 0 0 / var(--tw-text-opacity))',
    },
    {
      '@media (min-width: 1024px)': {
        '--tw-bg-opacity': '1',
        backgroundColor: 'rgb(239 68 68 / var(--tw-bg-opacity))',
      },
    },
  ]}
  data-tw={'tw-text-black | lg:tw-bg-red-500'}
/> // tw import + short css
;<div
  css={{
    maxWidth: '100vw',
    '@media (min-width: 1024px)': {
      '--tw-bg-opacity': '1',
      backgroundColor: 'rgb(239 68 68 / var(--tw-bg-opacity))',
    },
  }}
  data-tw={'lg:tw-bg-red-500 max-width[100vw]'}
/> // tw import + arbitrary property
;<div
  css={{
    maxWidth: '100vw',
    '@media (min-width: 1024px)': {
      '--tw-bg-opacity': '1',
      backgroundColor: 'rgb(239 68 68 / var(--tw-bg-opacity))',
    },
  }}
  data-tw={'lg:tw-bg-red-500 [max-width:100vw]'}
/> // className should be ignored without the prefix
;<div className="block" /> // className should be converted with a prefix
;<div
  css={{
    display: 'block',
  }}
  data-tw="tw-block"
/> // group
;<div
  css={{
    '@media (min-width: 1024px)': {
      ':hover': {
        '--tw-bg-opacity': '1',
        backgroundColor: 'rgb(239 68 68 / var(--tw-bg-opacity))',
      },
    },
  }}
  data-tw="hover:(lg:tw-bg-red-500)"
/>
;<div
  css={{
    '@media (min-width: 1024px)': {
      ':hover': {
        '--tw-bg-opacity': '1',
        backgroundColor: 'rgb(239 68 68 / var(--tw-bg-opacity))',
      },
    },
    ':hover': {
      maxWidth: '100vw',
    },
  }}
  data-tw="hover:(lg:tw-bg-red-500 max-width[100vw])"
/>
;<div
  css={{
    '@media (min-width: 1024px)': {
      ':hover': {
        '--tw-bg-opacity': '1',
        backgroundColor: 'rgb(239 68 68 / var(--tw-bg-opacity))',
      },
    },
    ':hover': {
      maxWidth: '100vw',
    },
  }}
  data-tw="hover:(lg:tw-bg-red-500 [max-width:100vw])"
/>
;<div
  css={{
    '@media (min-width: 1024px)': {
      ':hover': {
        '--tw-bg-opacity': '1',
        backgroundColor: 'rgb(239 68 68 / var(--tw-bg-opacity))',
      },
    },
  }}
  data-tw={'hover:(lg:tw-bg-red-500)'}
/>
;<div
  css={{
    '@media (min-width: 1024px)': {
      ':hover': {
        '--tw-bg-opacity': '1',
        backgroundColor: 'rgb(239 68 68 / var(--tw-bg-opacity))',
      },
    },
    ':hover': {
      maxWidth: '100vw',
    },
  }}
  data-tw={'hover:(lg:tw-bg-red-500 max-width[100vw])'}
/>
;<div
  css={{
    '@media (min-width: 1024px)': {
      ':hover': {
        '--tw-bg-opacity': '1',
        backgroundColor: 'rgb(239 68 68 / var(--tw-bg-opacity))',
      },
    },
    ':hover': {
      maxWidth: '100vw',
    },
  }}
  data-tw={'hover:(lg:tw-bg-red-500 [max-width:100vw])'}
/> // custom plugin classes
;<div
  css={{
    content: 'working',
  }}
  data-tw="tw-plugin-class"
/>
;<div
  css={{
    background: '5px',
    '.a-class & .some-class': {
      margin: '10px',
    },
    '.a-class & > *': {
      margin: '20px',
    },
  }}
  data-tw="tw-test-1"
/>
;<div
  css={{
    '.a-class & .some-class': {
      margin: '10px',
    },
    '.a-class & > *': {
      margin: '20px',
    },
  }}
  data-tw="tw-test-2"
/>


