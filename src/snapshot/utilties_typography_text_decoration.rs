use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`underline`"#####, r#####"({
  textDecorationLine: "underline",
})
;"##### ; "1")]
#[test_case(r#####"tw`overline`"#####, r#####"({
  textDecorationLine: "overline",
})
;"##### ; "2")]
#[test_case(r#####"tw`line-through`"#####, r#####"({
  textDecorationLine: "line-through",
})
;"##### ; "3")]
#[test_case(r#####"tw`no-underline`"#####, r#####"({
  textDecorationLine: "none",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
