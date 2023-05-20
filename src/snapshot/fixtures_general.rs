use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####"import _styled from '@emotion/styled'"##### ; "0")]
#[test_case(r#####"const styles = tw`uppercase`"#####, r#####"const styles = {
  textTransform: "uppercase",
}"##### ; "1")]
#[test_case(r#####"const Box = tw.div`text-red-500`"#####, r#####"const Box = _styled.div({
  '--tw-text-opacity': "1",
  color: "rgb(239 68 68 / var(--tw-text-opacity))",
})"##### ; "2")]
#[test_case(r#####"const BoxExtended = tw(Box)`bg-blue-500`"#####, r#####"const BoxExtended = _styled(Box)({
  '--tw-bg-opacity': "1",
  backgroundColor: "rgb(59 130 246 / var(--tw-bg-opacity))",
})"##### ; "3")]
#[test_case(r#####"const MediaProperty = tw`lg:uppercase`"#####, r#####"const MediaProperty = {
  '@media (min-width: 1024px)': {
    textTransform: "uppercase",
  },
}"##### ; "4")]
#[test_case(r#####"const MediaColorProperty = tw.div`lg:text-red-500`"#####, r#####"const MediaColorProperty = _styled.div({
  '@media (min-width: 1024px)': {
    '--tw-text-opacity': "1",
    color: "rgb(239 68 68 / var(--tw-text-opacity))",
  },
})"##### ; "5")]
#[test_case(r#####"const ElementMediaColorProperty = tw(Box)`lg:bg-blue-500`"#####, r#####"const ElementMediaColorProperty = _styled(Box)({
  '@media (min-width: 1024px)': {
    '--tw-bg-opacity': "1",
    backgroundColor: "rgb(59 130 246 / var(--tw-bg-opacity))",
  },
})"##### ; "6")]
#[test_case(r#####"const MediaPropertyDuplicates = tw`lg:bg-blue-500 lg:bg-black`"#####, r#####"const MediaPropertyDuplicates = {
  '@media (min-width: 1024px)': {
    '--tw-bg-opacity': "1",
    backgroundColor: "rgb(59 130 246 / var(--tw-bg-opacity))",
  },
}"##### ; "7")]
#[test_case(r#####"const plainConditional = true && "red""#####, r#####"const plainConditional = true && "red""##### ; "8")]
#[test_case(r#####"const plainVariable = `bg-${plainConditional}-500`"#####, r#####"const plainVariable = `bg-${plainConditional}-500`
;"##### ; "9")]
#[test_case(r#####"tw`${plainVariable}`"#####, r#####"({
  '--tw-bg-opacity': "1",
  backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
})"##### ; "10")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
