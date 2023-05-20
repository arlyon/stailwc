use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`bg-clip-border`"#####, r#####"({
  backgroundClip: "border-box",
})
;"##### ; "1")]
#[test_case(r#####"tw`bg-clip-padding`"#####, r#####"({
  backgroundClip: "padding-box",
})
;"##### ; "2")]
#[test_case(r#####"tw`bg-clip-content`"#####, r#####"({
  backgroundClip: "content-box",
})
;"##### ; "3")]
#[test_case(r#####"tw`bg-clip-text`"#####, r#####"({
  WebkitBackgroundClip: "text",
  backgroundClip: "text",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
