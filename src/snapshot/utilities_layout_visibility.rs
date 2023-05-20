use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`visible`"#####, r#####"({
  visibility: "visible",
})
;"##### ; "0")]
#[test_case(r#####"tw`invisible`"#####, r#####"({
  visibility: "hidden",
})
;"##### ; "1")]
#[test_case(r#####"tw`collapse`"#####, r#####"({
  visibility: "collapse",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
