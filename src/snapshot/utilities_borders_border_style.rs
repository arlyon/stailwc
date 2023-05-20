use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`border-solid`"#####, r#####"({
  borderStyle: "solid",
})
;"##### ; "0")]
#[test_case(r#####"tw`border-dashed`"#####, r#####"({
  borderStyle: "dashed",
})
;"##### ; "1")]
#[test_case(r#####"tw`border-dotted`"#####, r#####"({
  borderStyle: "dotted",
})
;"##### ; "2")]
#[test_case(r#####"tw`border-double`"#####, r#####"({
  borderStyle: "double",
})
;"##### ; "3")]
#[test_case(r#####"tw`border-hidden`"#####, r#####"({
  borderStyle: "hidden",
})
;"##### ; "4")]
#[test_case(r#####"tw`border-none`"#####, r#####"({
  borderStyle: "none",
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
