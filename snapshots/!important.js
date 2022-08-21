
const Box = tw.div`text-red-500`

const Important = tw`lg:uppercase!`
const MediaImportant = tw.div`lg:text-red-500!`
const ElementMediaImportant = tw(Box)`lg:bg-blue-500!`

const PlaceholderImportant = tw.input`placeholder-red-500!`
const StateImportant = tw.input`hover:text-red-500!`
const StatePlaceholderImportant = tw.input`hover:placeholder-red-500!`
const StateStatePlaceholderImportant = tw.input`active:hover:placeholder-red-500!`
const StateMultiplePropertiesImportant = tw.input`hover:truncate!`
const MediaStateMultiplePropertiesImportant = tw.input`lg:hover:truncate!`
const ElementMediaStateMultiplePropertiesImportant = tw(Box)`lg:hover:truncate!`

const JsxPlaceholderImportant = () => <input tw="placeholder-green-500!" />

const ImportantPrefixPrefix = tw`lg:!uppercase`
const MediaImportantPrefix = tw.div`lg:!text-red-500`
const ElementMediaImportantPrefix = tw(Box)`lg:!bg-blue-500`

const PlaceholderImportantPrefix = tw.input`!placeholder-red-500`
const StateImportantPrefix = tw.input`hover:!text-red-500`
const StatePlaceholderImportantPrefix = tw.input`hover:!placeholder-red-500`
const StateStatePlaceholderImportantPrefix = tw.input`active:hover:!placeholder-red-500`
const StateMultiplePropertiesImportantPrefix = tw.input`hover:!truncate`
const MediaStateMultiplePropertiesImportantPrefix = tw.input`lg:hover:!truncate`
const ElementMediaStateMultiplePropertiesImportantPrefix = tw(
  Box
)`lg:hover:!truncate`
const VariantImportantPrefixMergeCheck = tw.div`md:!from-black to-[#dc4fc2] bg-gradient-to-r`
const MultiVariantImportantPrefixMergeCheck = tw.div`first:md:!from-black to-[#dc4fc2] bg-gradient-to-r`

const JsxPlaceholderImportantPrefix = () => (
  <input tw="!placeholder-green-500" />
)

      ↓ ↓ ↓ ↓ ↓ ↓

import _styled from '@emotion/styled'

const Box = _styled.div({
  '--tw-text-opacity': '1',
  color: 'rgb(239 68 68 / var(--tw-text-opacity))',
})

const Important = {
  '@media (min-width: 1024px)': {
    textTransform: 'uppercase !important',
  },
}

const MediaImportant = _styled.div({
  '@media (min-width: 1024px)': {
    '--tw-text-opacity': '1',
    color: 'rgb(239 68 68 / var(--tw-text-opacity)) !important',
  },
})

const ElementMediaImportant = _styled(Box)({
  '@media (min-width: 1024px)': {
    '--tw-bg-opacity': '1',
    backgroundColor: 'rgb(59 130 246 / var(--tw-bg-opacity)) !important',
  },
})

const PlaceholderImportant = _styled.input({
  '::placeholder': {
    '--tw-placeholder-opacity': '1',
    color: 'rgb(239 68 68 / var(--tw-placeholder-opacity)) !important',
  },
})

const StateImportant = _styled.input({
  ':hover': {
    '--tw-text-opacity': '1',
    color: 'rgb(239 68 68 / var(--tw-text-opacity)) !important',
  },
})

const StatePlaceholderImportant = _styled.input({
  ':hover': {
    '::placeholder': {
      '--tw-placeholder-opacity': '1',
      color: 'rgb(239 68 68 / var(--tw-placeholder-opacity)) !important',
    },
  },
})

const StateStatePlaceholderImportant = _styled.input({
  ':active': {
    ':hover': {
      '::placeholder': {
        '--tw-placeholder-opacity': '1',
        color: 'rgb(239 68 68 / var(--tw-placeholder-opacity)) !important',
      },
    },
  },
})

const StateMultiplePropertiesImportant = _styled.input({
  ':hover': {
    overflow: 'hidden !important',
    textOverflow: 'ellipsis !important',
    whiteSpace: 'nowrap !important',
  },
})

const MediaStateMultiplePropertiesImportant = _styled.input({
  '@media (min-width: 1024px)': {
    ':hover': {
      overflow: 'hidden !important',
      textOverflow: 'ellipsis !important',
      whiteSpace: 'nowrap !important',
    },
  },
})

const ElementMediaStateMultiplePropertiesImportant = _styled(Box)({
  '@media (min-width: 1024px)': {
    ':hover': {
      overflow: 'hidden !important',
      textOverflow: 'ellipsis !important',
      whiteSpace: 'nowrap !important',
    },
  },
})

const JsxPlaceholderImportant = () => (
  <input
    css={{
      '::placeholder': {
        '--tw-placeholder-opacity': '1',
        color: 'rgb(34 197 94 / var(--tw-placeholder-opacity)) !important',
      },
    }}
  />
)

const ImportantPrefixPrefix = {
  '@media (min-width: 1024px)': {
    textTransform: 'uppercase !important',
  },
}

const MediaImportantPrefix = _styled.div({
  '@media (min-width: 1024px)': {
    '--tw-text-opacity': '1',
    color: 'rgb(239 68 68 / var(--tw-text-opacity)) !important',
  },
})

const ElementMediaImportantPrefix = _styled(Box)({
  '@media (min-width: 1024px)': {
    '--tw-bg-opacity': '1',
    backgroundColor: 'rgb(59 130 246 / var(--tw-bg-opacity)) !important',
  },
})

const PlaceholderImportantPrefix = _styled.input({
  '::placeholder': {
    '--tw-placeholder-opacity': '1',
    color: 'rgb(239 68 68 / var(--tw-placeholder-opacity)) !important',
  },
})

const StateImportantPrefix = _styled.input({
  ':hover': {
    '--tw-text-opacity': '1',
    color: 'rgb(239 68 68 / var(--tw-text-opacity)) !important',
  },
})

const StatePlaceholderImportantPrefix = _styled.input({
  ':hover': {
    '::placeholder': {
      '--tw-placeholder-opacity': '1',
      color: 'rgb(239 68 68 / var(--tw-placeholder-opacity)) !important',
    },
  },
})

const StateStatePlaceholderImportantPrefix = _styled.input({
  ':active': {
    ':hover': {
      '::placeholder': {
        '--tw-placeholder-opacity': '1',
        color: 'rgb(239 68 68 / var(--tw-placeholder-opacity)) !important',
      },
    },
  },
})

const StateMultiplePropertiesImportantPrefix = _styled.input({
  ':hover': {
    overflow: 'hidden !important',
    textOverflow: 'ellipsis !important',
    whiteSpace: 'nowrap !important',
  },
})

const MediaStateMultiplePropertiesImportantPrefix = _styled.input({
  '@media (min-width: 1024px)': {
    ':hover': {
      overflow: 'hidden !important',
      textOverflow: 'ellipsis !important',
      whiteSpace: 'nowrap !important',
    },
  },
})

const ElementMediaStateMultiplePropertiesImportantPrefix = _styled(Box)({
  '@media (min-width: 1024px)': {
    ':hover': {
      overflow: 'hidden !important',
      textOverflow: 'ellipsis !important',
      whiteSpace: 'nowrap !important',
    },
  },
})

const VariantImportantPrefixMergeCheck = _styled.div({
  '--tw-gradient-to': '#dc4fc2',
  backgroundImage: 'linear-gradient(to right, var(--tw-gradient-stops))',
  '@media (min-width: 768px)': {
    '--tw-gradient-from': '#000 !important',
    '--tw-gradient-stops':
      'var(--tw-gradient-from), var(--tw-gradient-to, rgb(0 0 0 / 0) !important)',
  },
})

const MultiVariantImportantPrefixMergeCheck = _styled.div({
  '@media (min-width: 768px)': {
    ':first-child': {
      '--tw-gradient-from': '#000 !important',
      '--tw-gradient-stops':
        'var(--tw-gradient-from), var(--tw-gradient-to, rgb(0 0 0 / 0) !important)',
    },
  },
  '--tw-gradient-to': '#dc4fc2',
  backgroundImage: 'linear-gradient(to right, var(--tw-gradient-stops))',
})

const JsxPlaceholderImportantPrefix = () => (
  <input
    css={{
      '::placeholder': {
        '--tw-placeholder-opacity': '1',
        color: 'rgb(34 197 94 / var(--tw-placeholder-opacity)) !important',
      },
    }}
  />
)


