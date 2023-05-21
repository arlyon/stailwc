use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`grayscale.`"#####, r#####"({
  0: "0",
  DEFAULT: "100%",
})
;"##### ; "0")]
#[test_case(r#####"tw`grayscale-0`"#####, r#####"({
  "--tw-grayscale": "grayscale(0)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "1")]
#[test_case(r#####"tw`grayscale`"#####, r#####"({
  "--tw-grayscale": "grayscale(100%)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "2")]
#[test_case(r#####"tw`grayscale-[50%]`"#####, r#####"({
  "--tw-grayscale": "grayscale(50%)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
