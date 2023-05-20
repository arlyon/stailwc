use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`backdropHueRotate`"#####, r#####"({
  0: "0deg",
  15: "15deg",
  30: "30deg",
  60: "60deg",
  90: "90deg",
  180: "180deg",
})
;"##### ; "0")]
#[test_case(r#####"tw`backdrop-hue-rotate-0`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(0deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "1")]
#[test_case(r#####"tw`backdrop-hue-rotate-15`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(15deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "2")]
#[test_case(r#####"tw`backdrop-hue-rotate-30`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(30deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "3")]
#[test_case(r#####"tw`backdrop-hue-rotate-60`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(60deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "4")]
#[test_case(r#####"tw`backdrop-hue-rotate-90`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(90deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "5")]
#[test_case(r#####"tw`backdrop-hue-rotate-180`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(180deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "6")]
#[test_case(r#####"tw`-backdrop-hue-rotate-0`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(-0deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "7")]
#[test_case(r#####"tw`-backdrop-hue-rotate-15`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(-15deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "8")]
#[test_case(r#####"tw`-backdrop-hue-rotate-30`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(-30deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "9")]
#[test_case(r#####"tw`-backdrop-hue-rotate-60`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(-60deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "10")]
#[test_case(r#####"tw`-backdrop-hue-rotate-90`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(-90deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "11")]
#[test_case(r#####"tw`-backdrop-hue-rotate-180`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(-180deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "12")]
#[test_case(r#####"tw`backdrop-hue-rotate-[270deg]`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(270deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})
;"##### ; "13")]
#[test_case(r#####"tw`-backdrop-hue-rotate-[270deg]`"#####, r#####"({
  '--tw-backdrop-hue-rotate': "hue-rotate(-270deg)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
})"##### ; "14")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
