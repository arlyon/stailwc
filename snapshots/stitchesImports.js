

import tw, { css, styled } from './macro'

css(tw`block`)
tw.div`block`
styled.div(tw`block`)

      ↓ ↓ ↓ ↓ ↓ ↓

import { css as _css } from '__fixtures__/stitches/stitches.config.js'
import { styled as _styled } from '__fixtures__/stitches/stitches.config.js'

_css({
  display: 'block',
})

_styled('div', {
  display: 'block',
})

_styled('div', {
  display: 'block',
})


