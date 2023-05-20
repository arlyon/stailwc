use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`outline`"#####, r#####"({
  outlineStyle: "solid",
})
;"##### ; "1")]
#[test_case(r#####"tw`outline-none`"#####, r#####"({
  outline: "2px solid transparent",
  outlineOffset: "2px",
})
;"##### ; "2")]
#[test_case(r#####"tw`outline-dashed`"#####, r#####"({
  outlineStyle: "dashed",
})
;"##### ; "3")]
#[test_case(r#####"tw`outline-dotted`"#####, r#####"({
  outlineStyle: "dotted",
})
;"##### ; "4")]
#[test_case(r#####"tw`outline-double`"#####, r#####"({
  outlineStyle: "double",
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
