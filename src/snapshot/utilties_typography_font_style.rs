use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`italic`"#####, r#####"({
  fontStyle: "italic",
})
;"##### ; "0")]
#[test_case(r#####"tw`not-italic`"#####, r#####"({
  fontStyle: "normal",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
