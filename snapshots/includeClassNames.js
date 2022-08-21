
const SkipEmptyClassName = <div className="" />
const OnlyUppercaseConverted = <div className="uppercase spare-class" />
const AllConverted = <div className="uppercase block" />
const SkippedCurlies = <div className={'mt-1'} />
const SkippedConditionals = <div className={true && 'mt-1'} />
const SkippedGroup = <div className="group" />

// css + className
const CssPropFirst = (
  <div
    css={`
      color: red;
    `}
    className="block"
  />
)
const CssPropLast = (
  <div
    className="block"
    css={`
      color: red;
    `}
  />
)

// tw + className
const TwPropFirst = <div tw="block" className="mt-1" />
const TwPropLast = <div className="mt-1" tw="block" />

// tw + css + className
const TwThenCssThenClassName = (
  <div
    tw="block"
    css={`
      color: red;
    `}
    className="mt-1"
  />
)
const TwThenClassNameThenCss = (
  <div
    tw="block"
    className="mt-1"
    css={`
      color: red;
    `}
  />
)
const ClassNameThenTwThenCss = (
  <div
    className="mt-1"
    tw="block"
    css={`
      color: red;
    `}
  />
)
const ClassNameThenCssThenTw = (
  <div
    className="mt-1"
    css={`
      color: red;
    `}
    tw="block"
  />
)
const CssThenClassNameThenTw = (
  <div
    css={`
      color: red;
    `}
    className="mt-1"
    tw="block"
  />
)
const CssThenTwThenClassName = (
  <div
    css={`
      color: red;
    `}
    tw="block"
    className="mt-1"
  />
)

// styled + everything
const Button = tw.div``

const StyledTwThenCssThenClassName = (
  <Button
    tw="block"
    css={`
      color: red;
    `}
    className="mt-1"
  />
)
const StyledTwThenClassNameThenCss = (
  <Button
    tw="block"
    className="mt-1"
    css={`
      color: red;
    `}
  />
)
const StyledClassNameThenTwThenCss = (
  <Button
    className="mt-1"
    tw="block"
    css={`
      color: red;
    `}
  />
)
const StyledClassNameThenCssThenTw = (
  <Button
    className="mt-1"
    css={`
      color: red;
    `}
    tw="block"
  />
)
const StyledCssThenClassNameThenTw = (
  <Button
    css={`
      color: red;
    `}
    className="mt-1"
    tw="block"
  />
)
const StyledCssThenTwThenClassName = (
  <Button
    css={`
      color: red;
    `}
    tw="block"
    className="mt-1"
  />
)
// All four css props
const TwThenClassNameThenCsThenCss = (
  <Button
    tw="block"
    className="ml-1"
    cs="content['cs']"
    css={`
      content: 'css';
    `}
  />
)
const TwThenClassNameThenCssThenCs = (
  <Button
    tw="block"
    className="ml-1"
    css={`
      content: 'css';
    `}
    cs="content['cs']"
  />
)
const TwThenCssThenClassNameThenCs = (
  <Button
    tw="block"
    css={`
      content: 'css';
    `}
    className="ml-1"
    cs="content['cs']"
  />
)
const CssThenTwThenClassNameThenCs = (
  <Button
    css={`
      content: 'css';
    `}
    tw="block"
    className="ml-1"
    cs="content['cs']"
  />
)

      ↓ ↓ ↓ ↓ ↓ ↓

import _styled from '@emotion/styled'
const SkipEmptyClassName = <div className="" />
const OnlyUppercaseConverted = (
  <div
    className="spare-class"
    css={{
      textTransform: 'uppercase',
    }}
  />
)
const AllConverted = (
  <div
    css={{
      textTransform: 'uppercase',
      display: 'block',
    }}
  />
)
const SkippedCurlies = <div className={'mt-1'} />
const SkippedConditionals = <div className={true && 'mt-1'} />
const SkippedGroup = <div className="group" /> // css + className

const CssPropFirst = (
  <div
    css={[
      `
      color: red;
    `,
      {
        display: 'block',
      },
    ]}
  />
)
const CssPropLast = (
  <div
    css={[
      {
        display: 'block',
      },
      `
      color: red;
    `,
    ]}
  />
) // tw + className

const TwPropFirst = (
  <div
    css={[
      {
        display: 'block',
      },
      {
        marginTop: '0.25rem',
      },
    ]}
  />
)
const TwPropLast = (
  <div
    css={[
      {
        marginTop: '0.25rem',
      },
      {
        display: 'block',
      },
    ]}
  />
) // tw + css + className

const TwThenCssThenClassName = (
  <div
    css={[
      {
        display: 'block',
      },
      `
      color: red;
    `,
      {
        marginTop: '0.25rem',
      },
    ]}
  />
)
const TwThenClassNameThenCss = (
  <div
    css={[
      {
        display: 'block',
      },
      {
        marginTop: '0.25rem',
      },
      `
      color: red;
    `,
    ]}
  />
)
const ClassNameThenTwThenCss = (
  <div
    css={[
      {
        marginTop: '0.25rem',
      },
      {
        display: 'block',
      },
      `
      color: red;
    `,
    ]}
  />
)
const ClassNameThenCssThenTw = (
  <div
    css={[
      {
        marginTop: '0.25rem',
      },
      `
      color: red;
    `,
      {
        display: 'block',
      },
    ]}
  />
)
const CssThenClassNameThenTw = (
  <div
    css={[
      `
      color: red;
    `,
      {
        marginTop: '0.25rem',
      },
      {
        display: 'block',
      },
    ]}
  />
)
const CssThenTwThenClassName = (
  <div
    css={[
      `
      color: red;
    `,
      {
        display: 'block',
      },
      {
        marginTop: '0.25rem',
      },
    ]}
  />
) // styled + everything

const Button = _styled.div({})

const StyledTwThenCssThenClassName = (
  <Button
    css={[
      {
        display: 'block',
      },
      `
      color: red;
    `,
      {
        marginTop: '0.25rem',
      },
    ]}
  />
)
const StyledTwThenClassNameThenCss = (
  <Button
    css={[
      {
        display: 'block',
      },
      {
        marginTop: '0.25rem',
      },
      `
      color: red;
    `,
    ]}
  />
)
const StyledClassNameThenTwThenCss = (
  <Button
    css={[
      {
        marginTop: '0.25rem',
      },
      {
        display: 'block',
      },
      `
      color: red;
    `,
    ]}
  />
)
const StyledClassNameThenCssThenTw = (
  <Button
    css={[
      {
        marginTop: '0.25rem',
      },
      `
      color: red;
    `,
      {
        display: 'block',
      },
    ]}
  />
)
const StyledCssThenClassNameThenTw = (
  <Button
    css={[
      `
      color: red;
    `,
      {
        marginTop: '0.25rem',
      },
      {
        display: 'block',
      },
    ]}
  />
)
const StyledCssThenTwThenClassName = (
  <Button
    css={[
      `
      color: red;
    `,
      {
        display: 'block',
      },
      {
        marginTop: '0.25rem',
      },
    ]}
  />
) // All four css props

const TwThenClassNameThenCsThenCss = (
  <Button
    css={[
      {
        display: 'block',
      },
      {
        marginLeft: '0.25rem',
      },
      {
        content: "'cs'",
      },
      `
      content: 'css';
    `,
    ]}
  />
)
const TwThenClassNameThenCssThenCs = (
  <Button
    css={[
      {
        display: 'block',
      },
      {
        marginLeft: '0.25rem',
      },
      `
      content: 'css';
    `,
      {
        content: "'cs'",
      },
    ]}
  />
)
const TwThenCssThenClassNameThenCs = (
  <Button
    css={[
      {
        display: 'block',
      },
      `
      content: 'css';
    `,
      {
        marginLeft: '0.25rem',
      },
      {
        content: "'cs'",
      },
    ]}
  />
)
const CssThenTwThenClassNameThenCs = (
  <Button
    css={[
      `
      content: 'css';
    `,
      {
        display: 'block',
      },
      {
        marginLeft: '0.25rem',
      },
      {
        content: "'cs'",
      },
    ]}
  />
)


