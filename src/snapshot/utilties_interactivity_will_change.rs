use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`willChange`"#####, r#####"({
  auto: "auto",
  scroll: "scroll-position",
  contents: "contents",
  transform: "transform",
})
;"##### ; "0")]
#[test_case(r#####"tw`will-change-auto`"#####, r#####"({
  willChange: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`will-change-scroll`"#####, r#####"({
  willChange: "scroll-position",
})
;"##### ; "2")]
#[test_case(r#####"tw`will-change-contents`"#####, r#####"({
  willChange: "contents",
})
;"##### ; "3")]
#[test_case(r#####"tw`will-change-transform`"#####, r#####"({
  willChange: "transform",
})
;"##### ; "4")]
#[test_case(r#####"tw`will-change-[top, left]`"#####, r#####"({
  willChange: "top, left",
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
