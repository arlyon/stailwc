use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`saturate`"#####, r#####"({
  0: "0",
  50: ".5",
  100: "1",
  150: "1.5",
  200: "2",
})
;"##### ; "0")]
#[test_case(r#####"tw`saturate-0`"#####, r#####"({
  '--tw-saturate': "saturate(0)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "1")]
#[test_case(r#####"tw`saturate-50`"#####, r#####"({
  '--tw-saturate': "saturate(.5)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "2")]
#[test_case(r#####"tw`saturate-100`"#####, r#####"({
  '--tw-saturate': "saturate(1)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "3")]
#[test_case(r#####"tw`saturate-150`"#####, r#####"({
  '--tw-saturate': "saturate(1.5)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "4")]
#[test_case(r#####"tw`saturate-200`"#####, r#####"({
  '--tw-saturate': "saturate(2)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "5")]
#[test_case(r#####"tw`saturate-[.25]`"#####, r#####"({
  '--tw-saturate': "saturate(.25)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
