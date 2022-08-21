

import tw, { styled } from './macro'

tw.div`block`
styled.div(tw`block`)
styled.div({ display: 'block' })

// Classic syntax
styled('div', tw`block`)

      ↓ ↓ ↓ ↓ ↓ ↓

import { styled as _styled } from '__fixtures__/stitches/stitches.config.js'

_styled('div', {
  display: 'block',
})

_styled('div', {
  display: 'block',
})

_styled('div', {
  display: 'block',
}) // Classic syntax

_styled('div', {
  display: 'block',
})


