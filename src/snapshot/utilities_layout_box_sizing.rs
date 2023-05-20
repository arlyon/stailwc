use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`box-border`"#####, r#####"({
  boxSizing: "border-box",
})
;"##### ; "0")]
#[test_case(r#####"tw`box-content`"#####, r#####"({
  boxSizing: "content-box",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
