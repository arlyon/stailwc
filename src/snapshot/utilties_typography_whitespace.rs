use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`whitespace-normal`"#####, r#####"({
  whiteSpace: "normal",
})
;"##### ; "0")]
#[test_case(r#####"tw`whitespace-nowrap`"#####, r#####"({
  whiteSpace: "nowrap",
})
;"##### ; "1")]
#[test_case(r#####"tw`whitespace-pre`"#####, r#####"({
  whiteSpace: "pre",
})
;"##### ; "2")]
#[test_case(r#####"tw`whitespace-pre-line`"#####, r#####"({
  whiteSpace: "pre-line",
})
;"##### ; "3")]
#[test_case(r#####"tw`whitespace-pre-wrap`"#####, r#####"({
  whiteSpace: "pre-wrap",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
