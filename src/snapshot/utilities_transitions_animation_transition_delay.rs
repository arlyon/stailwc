use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`transitionDelay`"#####, r#####"({
  0: "0s",
  75: "75ms",
  100: "100ms",
  150: "150ms",
  200: "200ms",
  300: "300ms",
  500: "500ms",
  700: "700ms",
  1000: "1000ms",
})
;"##### ; "1")]
#[test_case(r#####"tw`delay-75`"#####, r#####"({
  transitionDelay: "75ms",
})
;"##### ; "2")]
#[test_case(r#####"tw`delay-100`"#####, r#####"({
  transitionDelay: "100ms",
})
;"##### ; "3")]
#[test_case(r#####"tw`delay-150`"#####, r#####"({
  transitionDelay: "150ms",
})
;"##### ; "4")]
#[test_case(r#####"tw`delay-200`"#####, r#####"({
  transitionDelay: "200ms",
})
;"##### ; "5")]
#[test_case(r#####"tw`delay-300`"#####, r#####"({
  transitionDelay: "300ms",
})
;"##### ; "6")]
#[test_case(r#####"tw`delay-500`"#####, r#####"({
  transitionDelay: "500ms",
})
;"##### ; "7")]
#[test_case(r#####"tw`delay-700`"#####, r#####"({
  transitionDelay: "700ms",
})
;"##### ; "8")]
#[test_case(r#####"tw`delay-1000`"#####, r#####"({
  transitionDelay: "1000ms",
})
;"##### ; "9")]
#[test_case(r#####"tw`delay-[2000ms]`"#####, r#####"({
  transitionDelay: "2000ms",
})
;"##### ; "10")]
#[test_case(r#####"tw`delay-[var(--delay)]`"#####, r#####"({
  transitionDelay: "var(--delay)",
})"##### ; "11")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
