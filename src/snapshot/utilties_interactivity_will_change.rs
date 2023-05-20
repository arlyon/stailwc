use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`willChange`"#####, r#####"({
  auto: "auto",
  scroll: "scroll-position",
  contents: "contents",
  transform: "transform",
})
;"##### ; "1")]
#[test_case(r#####"tw`will-change-auto`"#####, r#####"({
  willChange: "auto",
})
;"##### ; "2")]
#[test_case(r#####"tw`will-change-scroll`"#####, r#####"({
  willChange: "scroll-position",
})
;"##### ; "3")]
#[test_case(r#####"tw`will-change-contents`"#####, r#####"({
  willChange: "contents",
})
;"##### ; "4")]
#[test_case(r#####"tw`will-change-transform`"#####, r#####"({
  willChange: "transform",
})
;"##### ; "5")]
#[test_case(r#####"tw`will-change-[top, left]`"#####, r#####"({
  willChange: "top, left",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
