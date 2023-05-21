use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`backdropContrast`"#####, r#####"({
  0: "0",
  50: ".5",
  75: ".75",
  100: "1",
  125: "1.25",
  150: "1.5",
  200: "2",
})
;"##### ; "0")]
#[test_case(r#####"tw`backdrop-contrast-0`"#####, r#####"({
  "--tw-backdrop-contrast": "contrast(0)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "1")]
#[test_case(r#####"tw`backdrop-contrast-50`"#####, r#####"({
  "--tw-backdrop-contrast": "contrast(.5)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "2")]
#[test_case(r#####"tw`backdrop-contrast-75`"#####, r#####"({
  "--tw-backdrop-contrast": "contrast(.75)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "3")]
#[test_case(r#####"tw`backdrop-contrast-100`"#####, r#####"({
  "--tw-backdrop-contrast": "contrast(1)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "4")]
#[test_case(r#####"tw`backdrop-contrast-125`"#####, r#####"({
  "--tw-backdrop-contrast": "contrast(1.25)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "5")]
#[test_case(r#####"tw`backdrop-contrast-150`"#####, r#####"({
  "--tw-backdrop-contrast": "contrast(1.5)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "6")]
#[test_case(r#####"tw`backdrop-contrast-200`"#####, r#####"({
  "--tw-backdrop-contrast": "contrast(2)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "7")]
#[test_case(r#####"tw`backdrop-contrast-[.25]`"#####, r#####"({
  "--tw-backdrop-contrast": "contrast(.25)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})"##### ; "8")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
