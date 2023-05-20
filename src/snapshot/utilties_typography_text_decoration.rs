use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`underline`"#####, r#####"({
  textDecorationLine: "underline",
})
;"##### ; "0")]
#[test_case(r#####"tw`overline`"#####, r#####"({
  textDecorationLine: "overline",
})
;"##### ; "1")]
#[test_case(r#####"tw`line-through`"#####, r#####"({
  textDecorationLine: "line-through",
})
;"##### ; "2")]
#[test_case(r#####"tw`no-underline`"#####, r#####"({
  textDecorationLine: "none",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
