
/**
 * Test comments
 */

// singleline
;<div css={tw`// comment`} />

// multiline
;<div css={tw`/* comment */`} />

// mixture
;<div
  css={tw`// comment  
/*
multline
comment
*/
block
// comment
`}
/>

// multiline comment
;<div
  css={tw`/*  block  
comment too
*/`}
/>

// singleline comment with class
;<div
  css={tw`// a comment
block
`}
/>

// multiline comment out a singleline comment with class
;<div
  css={tw`/*
// comment */
block
`}
/>

// mixture with single and multiline on same line
;<div
  css={tw`// hi
// ho /*
hum
*/`}
/>

// comment in variant group and consecutive strings
;<div css={tw`md:(text-xl/* text-yellow-500 */font-black)`} />

// break right bracket
;<div
  css={tw`2xl:(// ####@@@@ 
  [background:/*start*/rgb(191, 201/*inner*/, 211)])`}
/>

// comments within multiline comment
;<div
  css={tw`relative
  lg:(
    /***
    helloworld
    /****/
    //***
    flex
    text-5xl
    border-yellow-500
    /****/
)!`}
/>

      ↓ ↓ ↓ ↓ ↓ ↓

;<div css={{}} data-tw={'// comment'} /> // multiline
;<div css={{}} data-tw={'/* comment */'} /> // mixture
;<div
  css={{
    display: 'block',
  }}
  data-tw={'// comment /* multline comment */ block // comment'}
/> // multiline comment
;<div css={{}} data-tw={'/* block comment too */'} /> // singleline comment with class
;<div
  css={{
    display: 'block',
  }}
  data-tw={'// a comment block'}
/> // multiline comment out a singleline comment with class
;<div
  css={{
    display: 'block',
  }}
  data-tw={'/* // comment */ block'}
/> // mixture with single and multiline on same line
;<div css={{}} data-tw={'// hi // ho /* hum */'} /> // comment in variant group and consecutive strings
;<div
  css={{
    '@media (min-width: 768px)': {
      fontSize: '1.25rem',
      lineHeight: '1.75rem',
      fontWeight: '900',
    },
  }}
  data-tw={'md:(text-xl/* text-yellow-500 */font-black)'}
/> // break right bracket
;<div
  css={{
    '@media (min-width: 1536px)': {
      background: 'rgb(191, 201, 211)',
    },
  }}
  data-tw={
    '2xl:(// ####@@@@ [background:/*start*/rgb(191, 201/*inner*/, 211)])'
  }
/> // comments within multiline comment
;<div
  css={{
    position: 'relative',
    '@media (min-width: 1024px)': {
      display: 'flex !important',
      fontSize: '3rem !important',
      lineHeight: '1 !important',
      '--tw-border-opacity': '1',
      borderColor: 'rgb(234 179 8 / var(--tw-border-opacity)) !important',
    },
  }}
  data-tw={
    'relative lg:( /*** helloworld /****/ //*** flex text-5xl border-yellow-500 /****/ )!'
  }
/>


