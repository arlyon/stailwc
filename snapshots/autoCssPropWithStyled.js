

import tw from './macro' // twinImport

tw.div`block`
;<div tw="block" />

const Test = tw.div``
;<Test tw="block" />

      ↓ ↓ ↓ ↓ ↓ ↓

import _styled from 'styled-components'

// twinImport
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


