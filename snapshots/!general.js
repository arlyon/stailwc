
/**
 * Misc usage tests
 */

const styles = tw`uppercase`
const Box = tw.div`text-red-500`
const BoxExtended = tw(Box)`bg-blue-500`

// Media queries

const MediaProperty = tw`lg:uppercase`
const MediaColorProperty = tw.div`lg:text-red-500`
const ElementMediaColorProperty = tw(Box)`lg:bg-blue-500`
const MediaPropertyDuplicates = tw`lg:bg-blue-500 lg:bg-black`

// Only basic evaluations supported
// No functions or "beyond basic" conditionals.
const plainConditional = true && "red"
const plainVariable = `bg-${plainConditional}-500`
tw`${plainVariable}`

      ↓ ↓ ↓ ↓ ↓ ↓

import _styled from "@emotion/styled"

/**
 * Misc usage tests
 */
const styles = {
  textTransform: "uppercase",
}

const Box = _styled.div({
  "--tw-text-opacity": "1",
  color: "rgb(239 68 68 / var(--tw-text-opacity))",
})

const BoxExtended = _styled(Box)({
  "--tw-bg-opacity": "1",
  backgroundColor: "rgb(59 130 246 / var(--tw-bg-opacity))",
}) // Media queries

const MediaProperty = {
  "@media (min-width: 1024px)": {
    textTransform: "uppercase",
  },
}

const MediaColorProperty = _styled.div({
  "@media (min-width: 1024px)": {
    "--tw-text-opacity": "1",
    color: "rgb(239 68 68 / var(--tw-text-opacity))",
  },
})

const ElementMediaColorProperty = _styled(Box)({
  "@media (min-width: 1024px)": {
    "--tw-bg-opacity": "1",
    backgroundColor: "rgb(59 130 246 / var(--tw-bg-opacity))",
  },
})

const MediaPropertyDuplicates = {
  "@media (min-width: 1024px)": {
    "--tw-bg-opacity": "1",
    backgroundColor: "rgb(0 0 0 / var(--tw-bg-opacity))",
  },
} // Only basic evaluations supported
// No functions or "beyond basic" conditionals.

const plainConditional = true && "red"
const plainVariable = `bg-${plainConditional}-500`
({
  "--tw-bg-opacity": "1",
  backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
})


