use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####"import _styled from '@emotion/styled'"##### ; "0")]
#[test_case(r#####"const SkipEmptyClassName = <div className="" />"#####, r#####"const SkipEmptyClassName = <div className="" />"##### ; "1")]
#[test_case(r#####"const OnlyUppercaseConverted = <div className="uppercase spare-class" />"#####, r#####"const OnlyUppercaseConverted = (
  <div
    className="spare-class"
    css={{
      textTransform: "uppercase",
    }}
  />
)"##### ; "2")]
#[test_case(r#####"const AllConverted = <div className="uppercase block" />"#####, r#####"const AllConverted = (
  <div
    css={{
      display: "block",
      textTransform: "uppercase",
    }}
  />
)"##### ; "3")]
#[test_case(r#####"const SkippedCurlies = <div className={"mt-1"} />"#####, r#####"const SkippedCurlies = <div className={"mt-1"} />"##### ; "4")]
#[test_case(r#####"const SkippedConditionals = <div className={true && "mt-1"} />"#####, r#####"const SkippedConditionals = <div className={true && "mt-1"} />"##### ; "5")]
#[test_case(r#####"const SkippedGroup = <div className="group" />"#####, r#####"const SkippedGroup = <div className="group" />"##### ; "6")]
#[test_case(r#####"const CssPropFirst = (
  <div
    css={`
      color: red;
    `}
    className="block"
  />
)"#####, r#####"const CssPropFirst = (
  <div
    css={[
      `
      color: red;
    `,
      {
        display: "block",
      },
    ]}
  />
)"##### ; "7")]
#[test_case(r#####"const CssPropLast = (
  <div
    className="block"
    css={`
      color: red;
    `}
  />
)"#####, r#####"const CssPropLast = (
  <div
    css={[
      {
        display: "block",
      },
      `
      color: red;
    `,
    ]}
  />
)"##### ; "8")]
#[test_case(r#####"const TwPropFirst = <div tw="block" className="mt-1" />"#####, r#####"const TwPropFirst = (
  <div
    css={[
      {
        display: "block",
      },
      {
        marginTop: "0.25rem",
      },
    ]}
  />
)"##### ; "9")]
#[test_case(r#####"const TwPropLast = <div className="mt-1" tw="block" />"#####, r#####"const TwPropLast = (
  <div
    css={[
      {
        marginTop: "0.25rem",
      },
      {
        display: "block",
      },
    ]}
  />
)"##### ; "10")]
#[test_case(r#####"const TwThenCssThenClassName = (
  <div
    tw="block"
    css={`
      color: red;
    `}
    className="mt-1"
  />
)"#####, r#####"const TwThenCssThenClassName = (
  <div
    css={[
      {
        display: "block",
      },
      `
      color: red;
    `,
      {
        marginTop: "0.25rem",
      },
    ]}
  />
)"##### ; "11")]
#[test_case(r#####"const TwThenClassNameThenCss = (
  <div
    tw="block"
    className="mt-1"
    css={`
      color: red;
    `}
  />
)"#####, r#####"const TwThenClassNameThenCss = (
  <div
    css={[
      {
        display: "block",
      },
      {
        marginTop: "0.25rem",
      },
      `
      color: red;
    `,
    ]}
  />
)"##### ; "12")]
#[test_case(r#####"const ClassNameThenTwThenCss = (
  <div
    className="mt-1"
    tw="block"
    css={`
      color: red;
    `}
  />
)"#####, r#####"const ClassNameThenTwThenCss = (
  <div
    css={[
      {
        marginTop: "0.25rem",
      },
      {
        display: "block",
      },
      `
      color: red;
    `,
    ]}
  />
)"##### ; "13")]
#[test_case(r#####"const ClassNameThenCssThenTw = (
  <div
    className="mt-1"
    css={`
      color: red;
    `}
    tw="block"
  />
)"#####, r#####"const ClassNameThenCssThenTw = (
  <div
    css={[
      {
        marginTop: "0.25rem",
      },
      `
      color: red;
    `,
      {
        display: "block",
      },
    ]}
  />
)"##### ; "14")]
#[test_case(r#####"const CssThenClassNameThenTw = (
  <div
    css={`
      color: red;
    `}
    className="mt-1"
    tw="block"
  />
)"#####, r#####"const CssThenClassNameThenTw = (
  <div
    css={[
      `
      color: red;
    `,
      {
        marginTop: "0.25rem",
      },
      {
        display: "block",
      },
    ]}
  />
)"##### ; "15")]
#[test_case(r#####"const CssThenTwThenClassName = (
  <div
    css={`
      color: red;
    `}
    tw="block"
    className="mt-1"
  />
)"#####, r#####"const CssThenTwThenClassName = (
  <div
    css={[
      `
      color: red;
    `,
      {
        display: "block",
      },
      {
        marginTop: "0.25rem",
      },
    ]}
  />
)"##### ; "16")]
#[test_case(r#####"const Button = tw.div``"#####, r#####"const Button = _styled.div({})"##### ; "17")]
#[test_case(r#####"const StyledTwThenCssThenClassName = (
  <Button
    tw="block"
    css={`
      color: red;
    `}
    className="mt-1"
  />
)"#####, r#####"const StyledTwThenCssThenClassName = (
  <Button
    css={[
      {
        display: "block",
      },
      `
      color: red;
    `,
      {
        marginTop: "0.25rem",
      },
    ]}
  />
)"##### ; "18")]
#[test_case(r#####"const StyledTwThenClassNameThenCss = (
  <Button
    tw="block"
    className="mt-1"
    css={`
      color: red;
    `}
  />
)"#####, r#####"const StyledTwThenClassNameThenCss = (
  <Button
    css={[
      {
        display: "block",
      },
      {
        marginTop: "0.25rem",
      },
      `
      color: red;
    `,
    ]}
  />
)"##### ; "19")]
#[test_case(r#####"const StyledClassNameThenTwThenCss = (
  <Button
    className="mt-1"
    tw="block"
    css={`
      color: red;
    `}
  />
)"#####, r#####"const StyledClassNameThenTwThenCss = (
  <Button
    css={[
      {
        marginTop: "0.25rem",
      },
      {
        display: "block",
      },
      `
      color: red;
    `,
    ]}
  />
)"##### ; "20")]
#[test_case(r#####"const StyledClassNameThenCssThenTw = (
  <Button
    className="mt-1"
    css={`
      color: red;
    `}
    tw="block"
  />
)"#####, r#####"const StyledClassNameThenCssThenTw = (
  <Button
    css={[
      {
        marginTop: "0.25rem",
      },
      `
      color: red;
    `,
      {
        display: "block",
      },
    ]}
  />
)"##### ; "21")]
#[test_case(r#####"const StyledCssThenClassNameThenTw = (
  <Button
    css={`
      color: red;
    `}
    className="mt-1"
    tw="block"
  />
)"#####, r#####"const StyledCssThenClassNameThenTw = (
  <Button
    css={[
      `
      color: red;
    `,
      {
        marginTop: "0.25rem",
      },
      {
        display: "block",
      },
    ]}
  />
)"##### ; "22")]
#[test_case(r#####"const StyledCssThenTwThenClassName = (
  <Button
    css={`
      color: red;
    `}
    tw="block"
    className="mt-1"
  />
)"#####, r#####"const StyledCssThenTwThenClassName = (
  <Button
    css={[
      `
      color: red;
    `,
      {
        display: "block",
      },
      {
        marginTop: "0.25rem",
      },
    ]}
  />
)"##### ; "23")]
#[test_case(r#####"const TwThenClassNameThenCsThenCss = (
  <Button
    tw="block"
    className="ml-1"
    cs="content['cs']"
    css={`
      content: 'css';
    `}
  />
)"#####, r#####"const TwThenClassNameThenCsThenCss = (
  <Button
    css={[
      {
        display: "block",
      },
      {
        marginLeft: "0.25rem",
      },
      {
        content: "'cs'",
      },
      `
      content: 'css';
    `,
    ]}
  />
)"##### ; "24")]
#[test_case(r#####"const TwThenClassNameThenCssThenCs = (
  <Button
    tw="block"
    className="ml-1"
    css={`
      content: 'css';
    `}
    cs="content['cs']"
  />
)"#####, r#####"const TwThenClassNameThenCssThenCs = (
  <Button
    css={[
      {
        display: "block",
      },
      {
        marginLeft: "0.25rem",
      },
      `
      content: 'css';
    `,
      {
        content: "'cs'",
      },
    ]}
  />
)"##### ; "25")]
#[test_case(r#####"const TwThenCssThenClassNameThenCs = (
  <Button
    tw="block"
    css={`
      content: 'css';
    `}
    className="ml-1"
    cs="content['cs']"
  />
)"#####, r#####"const TwThenCssThenClassNameThenCs = (
  <Button
    css={[
      {
        display: "block",
      },
      `
      content: 'css';
    `,
      {
        marginLeft: "0.25rem",
      },
      {
        content: "'cs'",
      },
    ]}
  />
)"##### ; "26")]
#[test_case(r#####"const CssThenTwThenClassNameThenCs = (
  <Button
    css={`
      content: 'css';
    `}
    tw="block"
    className="ml-1"
    cs="content['cs']"
  />
)"#####, r#####"const CssThenTwThenClassNameThenCs = (
  <Button
    css={[
      `
      content: 'css';
    `,
      {
        display: "block",
      },
      {
        marginLeft: "0.25rem",
      },
      {
        content: "'cs'",
      },
    ]}
  />
)"##### ; "27")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
