use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`backdropBlur.`"#####, r#####"({
  0: "0",
  none: "0",
  sm: "4px",
  DEFAULT: "8px",
  md: "12px",
  lg: "16px",
  xl: "24px",
  "2xl": "40px",
  "3xl": "64px",
})
;"##### ; "0")]
#[test_case(r#####"tw`backdrop-blur-none`"#####, r#####"({
  "--tw-backdrop-blur": "blur(0)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "1")]
#[test_case(r#####"tw`backdrop-blur-sm`"#####, r#####"({
  "--tw-backdrop-blur": "blur(4px)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "2")]
#[test_case(r#####"tw`backdrop-blur`"#####, r#####"({
  "--tw-backdrop-blur": "blur(8px)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "3")]
#[test_case(r#####"tw`backdrop-blur-md`"#####, r#####"({
  "--tw-backdrop-blur": "blur(12px)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "4")]
#[test_case(r#####"tw`backdrop-blur-lg`"#####, r#####"({
  "--tw-backdrop-blur": "blur(16px)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "5")]
#[test_case(r#####"tw`backdrop-blur-xl`"#####, r#####"({
  "--tw-backdrop-blur": "blur(24px)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "6")]
#[test_case(r#####"tw`backdrop-blur-2xl`"#####, r#####"({
  "--tw-backdrop-blur": "blur(40px)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "7")]
#[test_case(r#####"tw`backdrop-blur-3xl`"#####, r#####"({
  "--tw-backdrop-blur": "blur(64px)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "8")]
#[test_case(r#####"tw`backdrop-blur-[2px]`"#####, r#####"({
  "--tw-backdrop-blur": "blur(2px)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})"##### ; "9")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
