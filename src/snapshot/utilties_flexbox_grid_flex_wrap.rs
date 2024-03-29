use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`flex-wrap`"#####, r#####"({
  flexWrap: "wrap",
})
;"##### ; "0")]
#[test_case(r#####"tw`flex-wrap-reverse`"#####, r#####"({
  flexWrap: "wrap-reverse",
})
;"##### ; "1")]
#[test_case(r#####"tw`flex-nowrap`"#####, r#####"({
  flexWrap: "nowrap",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
