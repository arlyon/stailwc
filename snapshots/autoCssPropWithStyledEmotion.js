

import tw from './macro' // twinImport

// Css prop isn't handled by twin
tw.div`block`
;<div tw="block" />

const Test = tw.div``
;<Test tw="block" />

      ↓ ↓ ↓ ↓ ↓ ↓

import _styled from '@emotion/styled'

// twinImport
// Css prop isn't handled by twin
_styled.div({
  display: 'block',
})

;<div
  css={{
    display: 'block',
  }}
/>

const Test = _styled.div({})

;<Test
  css={{
    display: 'block',
  }}
/>


