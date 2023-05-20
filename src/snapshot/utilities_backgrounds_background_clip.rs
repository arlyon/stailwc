use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`bg-clip-border`"#####, r#####"({
  backgroundClip: "border-box",
})
;"##### ; "0")]
#[test_case(r#####"tw`bg-clip-padding`"#####, r#####"({
  backgroundClip: "padding-box",
})
;"##### ; "1")]
#[test_case(r#####"tw`bg-clip-content`"#####, r#####"({
  backgroundClip: "content-box",
})
;"##### ; "2")]
#[test_case(r#####"tw`bg-clip-text`"#####, r#####"({
  WebkitBackgroundClip: "text",
  backgroundClip: "text",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
