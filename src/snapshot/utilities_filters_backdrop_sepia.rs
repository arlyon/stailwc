use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`backdropSepia.`"#####, r#####"({
  0: "0",
  DEFAULT: "100%",
})
;"##### ; "0")]
#[test_case(r#####"tw`backdrop-sepia-0`"#####, r#####"({
  '--tw-backdrop-sepia': "sepia(0)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "1")]
#[test_case(r#####"tw`backdrop-sepia`"#####, r#####"({
  '--tw-backdrop-sepia': "sepia(100%)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "2")]
#[test_case(r#####"tw`backdrop-sepia-[.25]`"#####, r#####"({
  '--tw-backdrop-sepia': "sepia(.25)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
