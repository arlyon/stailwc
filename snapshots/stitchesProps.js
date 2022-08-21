// tw prop
;<div tw="block" />

// tw + css prop
;<div tw="block" css={{ color: 'black' }} />
;<div tw="block" css={tw`text-black`} />
;<div css={{ color: 'black' }} tw="block" />
;<div css={{ color: 'black' }} tw="block" thisShouldBePreserved="yup" />

// Extracted styles
const styles = {
  container: ({ isInline }) => ({ ...tw`block`, ...(isInline && tw`inline`) }),
}
;<div css={styles.container({ isInline: true })} />

// Dot syntax
const Component = { Sub: () => [] }
;<Component.Sub css={tw`fixed`} />
;<Component.Sub tw="fixed" />

      ↓ ↓ ↓ ↓ ↓ ↓

import { styled as _styled } from '__fixtures__/stitches/stitches.config.js'

const _TwComponent = _styled('div', {
  display: 'block',
})

;<_TwComponent /> // tw + css prop

const _TwComponent2 = _styled('div', {
  display: 'block',
})

const _TwComponent3 = _styled(_TwComponent2, {})

;<_TwComponent3
  css={{
    color: 'black',
  }}
/>

const _TwComponent4 = _styled('div', {
  display: 'block',
})

const _TwComponent5 = _styled(_TwComponent4, {})

;<_TwComponent5
  css={{
    '--tw-text-opacity': '1',
    color: 'rgb(0 0 0 / var(--tw-text-opacity))',
  }}
/>

const _TwComponent6 = _styled('div', {
  display: 'block',
})

const _TwComponent7 = _styled(_TwComponent6, {})

;<_TwComponent7
  css={{
    color: 'black',
  }}
/>

const _TwComponent8 = _styled('div', {
  display: 'block',
})

const _TwComponent9 = _styled(_TwComponent8, {})

;<_TwComponent9
  css={{
    color: 'black',
  }}
  thisShouldBePreserved="yup"
/> // Extracted styles

const styles = {
  container: ({ isInline }) => ({
    ...{
      display: 'block',
    },
    ...(isInline && {
      display: 'inline',
    }),
  }),
}

const _TwComponent10 = _styled('div', {})

;<_TwComponent10
  css={styles.container({
    isInline: true,
  })}
/> // Dot syntax

const Component = {
  Sub: () => [],
}

const _TwComponent11 = _styled(Component.Sub, {})

;<_TwComponent11
  css={{
    position: 'fixed',
  }}
/>

const _TwComponent12 = _styled(Component.Sub, {
  position: 'fixed',
})

;<_TwComponent12 />


