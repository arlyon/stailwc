use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`transitionProperty.`"#####, r#####"({
  none: "none",
  all: "all",
  DEFAULT:
    "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter",
  colors:
    "color, background-color, border-color, text-decoration-color, fill, stroke",
  opacity: "opacity",
  shadow: "box-shadow",
  transform: "transform",
})
;"##### ; "1")]
#[test_case(r#####"tw`transition-none`"#####, r#####"({
  transitionProperty: "none",
})
;"##### ; "2")]
#[test_case(r#####"tw`transition-all`"#####, r#####"({
  transitionProperty: "all",
  transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
  transitionDuration: "150ms",
})
;"##### ; "3")]
#[test_case(r#####"tw`transition`"#####, r#####"({
  transitionProperty:
    "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter",
  transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
  transitionDuration: "150ms",
})
;"##### ; "4")]
#[test_case(r#####"tw`transition-colors`"#####, r#####"({
  transitionProperty:
    "color, background-color, border-color, text-decoration-color, fill, stroke",
  transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
  transitionDuration: "150ms",
})
;"##### ; "5")]
#[test_case(r#####"tw`transition-opacity`"#####, r#####"({
  transitionProperty: "opacity",
  transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
  transitionDuration: "150ms",
})
;"##### ; "6")]
#[test_case(r#####"tw`transition-shadow`"#####, r#####"({
  transitionProperty: "box-shadow",
  transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
  transitionDuration: "150ms",
})
;"##### ; "7")]
#[test_case(r#####"tw`transition-transform`"#####, r#####"({
  transitionProperty: "transform",
  transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
  transitionDuration: "150ms",
})
;"##### ; "8")]
#[test_case(r#####"tw`transition-[height]`"#####, r#####"({
  transitionProperty: "height",
  transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
  transitionDuration: "150ms",
})
;"##### ; "9")]
#[test_case(r#####"tw`transition-[lookup:green]`"#####, r#####"({
  transitionProperty: "green",
  transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
  transitionDuration: "150ms",
})"##### ; "10")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
