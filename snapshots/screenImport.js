

import tw, { styled, screen } from './macro'

// Media query only
screen`sm`
screen.md // Can't work with screen values that begin with a number, eg: screen.2xl
screen('lg')
screen(`xl`)

// Constructed media queries
;`
    ${screen`sm`} {
        display: block;
        ${tw`inline`}
    }
`
({ [screen`sm`]: `display: block; ${tw`inline`}` });
({ [screen`sm`]: { display: 'block', ...tw`inline` } })

// Media queries with styles
screen.sm({ color: `red` })
screen`md`({ color: `red` })
screen('lg')({ color: `red` })
screen(`xl`)({ color: `red` })
screen.sm`color: red;`
screen`md``color: red;`
screen('lg')`color: red;`
screen(`xl`)`color: red;`

screen.xl(tw`inline`)
screen.xl({ ...tw`inline` })
screen.xl({ ...tw`inline`, display: 'block' })
screen.xl`
    ${tw`inline`}
    display: block;
`
screen.xl`color: ${true && 'blue'};`

// Within template literals
;`${screen.lg}`
;`${screen`xl`}`
;`${screen(`xl`)}`
;`${screen('xl')}`

// Screen keys
;<div
  css={{
    [screen.xl]: { color: 'red' },
  }}
/>
;<div
  css={`
    ${{ [screen.xl]: { color: 'red' } }}
  `}
/>
;<div css={[{ [screen.xl]: { color: 'red' } }]} />
;<div
  css={`
    ${screen.xl} {
      color: red;
    }
  `}
/>

styled.div`
  ${{ [screen.xl]: { color: 'red' } }}
`
styled.div([{ [screen.xl]: { color: 'red' } }])

// Logical expressions
;<div
  css={{
    [true && screen.xl]: { color: 'red' },
  }}
/>
styled.div([{ [true && screen.xl]: { color: 'red' } }])

// Conditional expressions
;<div
  css={{
    // eslint-disable-next-line no-constant-condition
    [true ? screen.xl : screen.sm]: { color: 'red' },
  }}
/>
styled.div`
  ${{
    // eslint-disable-next-line no-constant-condition
    [true ? screen.xl : screen.sm]: { color: 'red' },
  }}
`

// Screen with values
;<div css={screen.xl({ color: 'red' })} />
;<div css={[screen.xl({ color: 'red' })]} />
;<div
  css={`
    ${screen.xl({ color: 'red' })}
  `}
/>
;<div css={screen.xl`color: red;`} />
;<div css={[screen.xl`color: red;`]} />
;<div
  css={`
    ${screen.xl`color: red;`}
  `}
/>

      ↓ ↓ ↓ ↓ ↓ ↓

import _styled from '@emotion/styled'
// Media query only
;('@media (min-width: 100px)')
;('@media (min-width: 200px)') // Can't work with screen values that begin with a number, eg: screen.2xl

;('@media (min-width: 300px)')
;('@media (min-width: 400px)') // Constructed media queries
;`
    ${'@media (min-width: 100px)'} {
        display: block;
        ${{
          display: 'inline',
        }}
    }
`
({
  '@media (min-width: 100px)': `display: block; ${{
    display: 'inline',
  }}`,
});
({
  '@media (min-width: 100px)': {
    display: 'block',
    ...{
      display: 'inline',
    },
  },
}) // Media queries with styles

({
  '@media (min-width: 100px)': {
    color: `red`,
  },
});
({
  '@media (min-width: 200px)': {
    color: `red`,
  },
});
({
  '@media (min-width: 300px)': {
    color: `red`,
  },
});
({
  '@media (min-width: 400px)': {
    color: `red`,
  },
});
`@media (min-width: 100px) { ${`color: red;`} }`
;`@media (min-width: 200px) { ${`color: red;`} }`
;`@media (min-width: 300px) { ${`color: red;`} }`
;`@media (min-width: 400px) { ${`color: red;`} }`
({
  '@media (min-width: 400px)': {
    display: 'inline',
  },
});
({
  '@media (min-width: 400px)': {
    ...{
      display: 'inline',
    },
  },
});
({
  '@media (min-width: 400px)': {
    ...{
      display: 'inline',
    },
    display: 'block',
  },
});
`@media (min-width: 400px) { ${`
    ${{
      display: 'inline',
    }}
    display: block;
`} }`
;`@media (min-width: 400px) { ${`color: ${true && 'blue'};`} }` // Within template literals
;`${'@media (min-width: 300px)'}`
;`${'@media (min-width: 400px)'}`
;`${'@media (min-width: 400px)'}`
;`${'@media (min-width: 400px)'}` // Screen keys
;<div
  css={{
    '@media (min-width: 400px)': {
      color: 'red',
    },
  }}
/>
;<div
  css={`
    ${{
      '@media (min-width: 400px)': {
        color: 'red',
      },
    }}
  `}
/>
;<div
  css={[
    {
      '@media (min-width: 400px)': {
        color: 'red',
      },
    },
  ]}
/>
;<div
  css={`
    ${'@media (min-width: 400px)'} {
      color: red;
    }
  `}
/>
_styled.div`
  ${{
    '@media (min-width: 400px)': {
      color: 'red',
    },
  }}
`

_styled.div([
  {
    '@media (min-width: 400px)': {
      color: 'red',
    },
  },
]) // Logical expressions

;<div
  css={{
    [true && '@media (min-width: 400px)']: {
      color: 'red',
    },
  }}
/>

_styled.div([
  {
    [true && '@media (min-width: 400px)']: {
      color: 'red',
    },
  },
]) // Conditional expressions

;<div
  css={{
    // eslint-disable-next-line no-constant-condition
    [true ? '@media (min-width: 400px)' : '@media (min-width: 100px)']: {
      color: 'red',
    },
  }}
/>
_styled.div`
  ${{
    // eslint-disable-next-line no-constant-condition
    [true ? '@media (min-width: 400px)' : '@media (min-width: 100px)']: {
      color: 'red',
    },
  }}
` // Screen with values
;<div
  css={{
    '@media (min-width: 400px)': {
      color: 'red',
    },
  }}
/>
;<div
  css={[
    {
      '@media (min-width: 400px)': {
        color: 'red',
      },
    },
  ]}
/>
;<div
  css={`
    ${{
      '@media (min-width: 400px)': {
        color: 'red',
      },
    }}
  `}
/>
;<div
  css={`
    @media (min-width: 400px) {
      ${`color: red;`}
    }
  `}
/>
;<div css={[`@media (min-width: 400px) { ${`color: red;`} }`]} />
;<div
  css={`
    ${`@media (min-width: 400px) { ${`color: red;`} }`}
  `}
/>


