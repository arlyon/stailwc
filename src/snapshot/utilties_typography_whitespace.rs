use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`whitespace-normal`"#####, r#####"({
  whiteSpace: "normal",
})
;"##### ; "1")]
#[test_case(r#####"tw`whitespace-nowrap`"#####, r#####"({
  whiteSpace: "nowrap",
})
;"##### ; "2")]
#[test_case(r#####"tw`whitespace-pre`"#####, r#####"({
  whiteSpace: "pre",
})
;"##### ; "3")]
#[test_case(r#####"tw`whitespace-pre-line`"#####, r#####"({
  whiteSpace: "pre-line",
})
;"##### ; "4")]
#[test_case(r#####"tw`whitespace-pre-wrap`"#####, r#####"({
  whiteSpace: "pre-wrap",
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
