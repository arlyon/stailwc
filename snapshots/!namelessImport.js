

import './macro'

const twPropertyString = <div tw="text-purple-500" />
const twPropertyExpression = <div tw={'text-purple-500'} />

      ↓ ↓ ↓ ↓ ↓ ↓

const twPropertyString = (
  <div
    css={{
      '--tw-text-opacity': '1',
      color: 'rgb(168 85 247 / var(--tw-text-opacity))',
    }}
  />
)
const twPropertyExpression = (
  <div
    css={{
      '--tw-text-opacity': '1',
      color: 'rgb(168 85 247 / var(--tw-text-opacity))',
    }}
  />
)


