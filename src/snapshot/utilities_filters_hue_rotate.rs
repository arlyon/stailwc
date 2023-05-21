use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`hueRotate`"#####, r#####"({
  0: "0deg",
  15: "15deg",
  30: "30deg",
  60: "60deg",
  90: "90deg",
  180: "180deg",
})
;"##### ; "0")]
#[test_case(r#####"tw`hue-rotate-0`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(0deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "1")]
#[test_case(r#####"tw`hue-rotate-15`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(15deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "2")]
#[test_case(r#####"tw`hue-rotate-30`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(30deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "3")]
#[test_case(r#####"tw`hue-rotate-60`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(60deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "4")]
#[test_case(r#####"tw`hue-rotate-90`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(90deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "5")]
#[test_case(r#####"tw`hue-rotate-180`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(180deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "6")]
#[test_case(r#####"tw`-hue-rotate-0`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(-0deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "7")]
#[test_case(r#####"tw`-hue-rotate-15`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(-15deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "8")]
#[test_case(r#####"tw`-hue-rotate-30`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(-30deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "9")]
#[test_case(r#####"tw`-hue-rotate-60`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(-60deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "10")]
#[test_case(r#####"tw`-hue-rotate-90`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(-90deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "11")]
#[test_case(r#####"tw`-hue-rotate-180`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(-180deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "12")]
#[test_case(r#####"tw`hue-rotate-[270deg]`"#####, r#####"({
  "--tw-hue-rotate": "hue-rotate(270deg)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})"##### ; "13")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
