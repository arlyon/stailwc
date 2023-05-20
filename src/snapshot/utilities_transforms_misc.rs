use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`filter-none`"#####, r#####"({
  filter: "none",
})
;"##### ; "1")]
#[test_case(r#####"tw`filter`"#####, r#####"({
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
}) // Deprecated
// https://tailwindcss.com/docs/backdrop-filter

;"##### ; "2")]
#[test_case(r#####"tw`backdrop-filter`"#####, r#####"({
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
}) // Deprecated

;"##### ; "3")]
#[test_case(r#####"tw`backdrop-filter-none`"#####, r#####"({
  backdropFilter: "none",
}) // All

;"##### ; "4")]
#[test_case(r#####"tw`filter blur-2xl brightness-50 contrast-50 grayscale hue-rotate-180 invert saturate-50 sepia drop-shadow-2xl`"#####, r#####"({
  '--tw-blur': "blur(40px)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
  '--tw-brightness': "brightness(.5)",
  '--tw-contrast': "contrast(.5)",
  '--tw-drop-shadow': "drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))",
  '--tw-grayscale': "grayscale(100%)",
  '--tw-hue-rotate': "hue-rotate(180deg)",
  '--tw-invert': "invert(100%)",
  '--tw-saturate': "saturate(.5)",
  '--tw-sepia': "sepia(100%)",
}) // All

;"##### ; "5")]
#[test_case(r#####"tw`backdrop-filter backdrop-blur-2xl backdrop-brightness-50 backdrop-contrast-50 backdrop-grayscale backdrop-hue-rotate-180 backdrop-invert backdrop-opacity-50 backdrop-saturate-50 backdrop-sepia`"#####, r#####"({
  '--tw-backdrop-blur': "blur(40px)",
  backdropFilter:
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)",
  '--tw-backdrop-brightness': "brightness(.5)",
  '--tw-backdrop-contrast': "contrast(.5)",
  '--tw-backdrop-grayscale': "grayscale(100%)",
  '--tw-backdrop-hue-rotate': "hue-rotate(180deg)",
  '--tw-backdrop-invert': "invert(100%)",
  '--tw-backdrop-opacity': "opacity(0.5)",
  '--tw-backdrop-saturate': "saturate(.5)",
  '--tw-backdrop-sepia': "sepia(100%)",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
