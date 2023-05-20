use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`float-right`"#####, r#####"({
  float: "right",
})
;"##### ; "0")]
#[test_case(r#####"tw`float-left`"#####, r#####"({
  float: "left",
})
;"##### ; "1")]
#[test_case(r#####"tw`float-none`"#####, r#####"({
  float: "none",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
