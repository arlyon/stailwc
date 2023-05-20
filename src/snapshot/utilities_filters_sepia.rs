use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`sepia.`"#####, r#####"({
  0: "0",
  DEFAULT: "100%",
})
;"##### ; "0")]
#[test_case(r#####"tw`sepia-0`"#####, r#####"({
  '--tw-sepia': "sepia(0)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "1")]
#[test_case(r#####"tw`sepia`"#####, r#####"({
  '--tw-sepia': "sepia(100%)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "2")]
#[test_case(r#####"tw`sepia-[.25]`"#####, r#####"({
  '--tw-sepia': "sepia(.25)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
