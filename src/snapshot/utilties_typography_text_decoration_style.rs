use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`decoration-solid`"#####, r#####"({
  textDecorationStyle: "solid",
})
;"##### ; "1")]
#[test_case(r#####"tw`decoration-double`"#####, r#####"({
  textDecorationStyle: "double",
})
;"##### ; "2")]
#[test_case(r#####"tw`decoration-dotted`"#####, r#####"({
  textDecorationStyle: "dotted",
})
;"##### ; "3")]
#[test_case(r#####"tw`decoration-dashed`"#####, r#####"({
  textDecorationStyle: "dashed",
})
;"##### ; "4")]
#[test_case(r#####"tw`decoration-wavy`"#####, r#####"({
  textDecorationStyle: "wavy",
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
