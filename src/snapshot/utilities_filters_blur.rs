use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`blur.`"#####, r#####"({
  0: "0",
  none: "0",
  sm: "4px",
  DEFAULT: "8px",
  md: "12px",
  lg: "16px",
  xl: "24px",
  '2xl': "40px",
  '3xl': "64px",
})
;"##### ; "0")]
#[test_case(r#####"tw`blur-none`"#####, r#####"({
  '--tw-blur': "blur(0)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "1")]
#[test_case(r#####"tw`blur-sm`"#####, r#####"({
  '--tw-blur': "blur(4px)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "2")]
#[test_case(r#####"tw`blur`"#####, r#####"({
  '--tw-blur': "blur(8px)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "3")]
#[test_case(r#####"tw`blur-md`"#####, r#####"({
  '--tw-blur': "blur(12px)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "4")]
#[test_case(r#####"tw`blur-lg`"#####, r#####"({
  '--tw-blur': "blur(16px)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "5")]
#[test_case(r#####"tw`blur-xl`"#####, r#####"({
  '--tw-blur': "blur(24px)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "6")]
#[test_case(r#####"tw`blur-2xl`"#####, r#####"({
  '--tw-blur': "blur(40px)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "7")]
#[test_case(r#####"tw`blur-3xl`"#####, r#####"({
  '--tw-blur': "blur(64px)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "8")]
#[test_case(r#####"tw`blur-[2px]`"#####, r#####"({
  '--tw-blur': "blur(2px)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})"##### ; "9")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
