use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`transitionDuration.`"#####, r#####"({
  0: "0s",
  75: "75ms",
  100: "100ms",
  150: "150ms",
  200: "200ms",
  300: "300ms",
  500: "500ms",
  700: "700ms",
  1000: "1000ms",
  DEFAULT: "150ms",
})
;"##### ; "0")]
#[test_case(r#####"tw`duration-75`"#####, r#####"({
  transitionDuration: "75ms",
})
;"##### ; "1")]
#[test_case(r#####"tw`duration-100`"#####, r#####"({
  transitionDuration: "100ms",
})
;"##### ; "2")]
#[test_case(r#####"tw`duration-150`"#####, r#####"({
  transitionDuration: "150ms",
})
;"##### ; "3")]
#[test_case(r#####"tw`duration-200`"#####, r#####"({
  transitionDuration: "200ms",
})
;"##### ; "4")]
#[test_case(r#####"tw`duration-300`"#####, r#####"({
  transitionDuration: "300ms",
})
;"##### ; "5")]
#[test_case(r#####"tw`duration-500`"#####, r#####"({
  transitionDuration: "500ms",
})
;"##### ; "6")]
#[test_case(r#####"tw`duration-700`"#####, r#####"({
  transitionDuration: "700ms",
})
;"##### ; "7")]
#[test_case(r#####"tw`duration-1000`"#####, r#####"({
  transitionDuration: "1000ms",
})
;"##### ; "8")]
#[test_case(r#####"tw`duration-[2000ms]`"#####, r#####"({
  transitionDuration: "2000ms",
})
;"##### ; "9")]
#[test_case(r#####"tw`duration-[2s]`"#####, r#####"({
  transitionDuration: "2s",
})
;"##### ; "10")]
#[test_case(r#####"tw`duration-[var(--app-duration)]`"#####, r#####"({
  transitionDuration: "var(--app-duration)",
})"##### ; "11")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
