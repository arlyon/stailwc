use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`uppercase`"#####, r#####"({
  textTransform: "uppercase",
})
;"##### ; "0")]
#[test_case(r#####"tw`lowercase`"#####, r#####"({
  textTransform: "lowercase",
})
;"##### ; "1")]
#[test_case(r#####"tw`capitalize`"#####, r#####"({
  textTransform: "capitalize",
})
;"##### ; "2")]
#[test_case(r#####"tw`normal-case`"#####, r#####"({
  textTransform: "none",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
