use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`outline`"#####, r#####"({
  outlineStyle: "solid",
})
;"##### ; "0")]
#[test_case(r#####"tw`outline-none`"#####, r#####"({
  outline: "2px solid transparent",
  outlineOffset: "2px",
})
;"##### ; "1")]
#[test_case(r#####"tw`outline-dashed`"#####, r#####"({
  outlineStyle: "dashed",
})
;"##### ; "2")]
#[test_case(r#####"tw`outline-dotted`"#####, r#####"({
  outlineStyle: "dotted",
})
;"##### ; "3")]
#[test_case(r#####"tw`outline-double`"#####, r#####"({
  outlineStyle: "double",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
