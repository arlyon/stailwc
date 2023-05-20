use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`bg-origin-border`"#####, r#####"({
  backgroundOrigin: "border-box",
})
;"##### ; "0")]
#[test_case(r#####"tw`bg-origin-padding`"#####, r#####"({
  backgroundOrigin: "padding-box",
})
;"##### ; "1")]
#[test_case(r#####"tw`bg-origin-content`"#####, r#####"({
  backgroundOrigin: "content-box",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
