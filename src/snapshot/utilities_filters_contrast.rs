use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`contrast`"#####, r#####"({
  0: "0",
  50: ".5",
  75: ".75",
  100: "1",
  125: "1.25",
  150: "1.5",
  200: "2",
})
;"##### ; "0")]
#[test_case(r#####"tw`contrast-0`"#####, r#####"({
  '--tw-contrast': "contrast(0)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "1")]
#[test_case(r#####"tw`contrast-50`"#####, r#####"({
  '--tw-contrast': "contrast(.5)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "2")]
#[test_case(r#####"tw`contrast-75`"#####, r#####"({
  '--tw-contrast': "contrast(.75)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "3")]
#[test_case(r#####"tw`contrast-100`"#####, r#####"({
  '--tw-contrast': "contrast(1)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "4")]
#[test_case(r#####"tw`contrast-125`"#####, r#####"({
  '--tw-contrast': "contrast(1.25)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "5")]
#[test_case(r#####"tw`contrast-150`"#####, r#####"({
  '--tw-contrast': "contrast(1.5)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "6")]
#[test_case(r#####"tw`contrast-200`"#####, r#####"({
  '--tw-contrast': "contrast(2)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "7")]
#[test_case(r#####"tw`contrast-[.25]`"#####, r#####"({
  '--tw-contrast': "contrast(.25)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "8")]
#[test_case(r#####"tw`contrast-[-.25]`"#####, r#####"({
  '--tw-contrast': "contrast(-.25)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})"##### ; "9")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
