use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`dropShadow`"#####, r#####"({
  sm: "0 1px 1px rgb(0 0 0 / 0.05)",
  DEFAULT: ["0 1px 2px rgb(0 0 0 / 0.1)", "0 1px 1px rgb(0 0 0 / 0.06)"],
  md: ["0 4px 3px rgb(0 0 0 / 0.07)", "0 2px 2px rgb(0 0 0 / 0.06)"],
  lg: ["0 10px 8px rgb(0 0 0 / 0.04)", "0 4px 3px rgb(0 0 0 / 0.1)"],
  xl: ["0 20px 13px rgb(0 0 0 / 0.03)", "0 8px 5px rgb(0 0 0 / 0.08)"],
  "2xl": "0 25px 25px rgb(0 0 0 / 0.15)",
  none: "0 0 #0000",
})
;"##### ; "0")]
#[test_case(r#####"tw`drop-shadow-sm`"#####, r#####"({
  "--tw-drop-shadow": "drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "1")]
#[test_case(r#####"tw`drop-shadow`"#####, r#####"({
  "--tw-drop-shadow":
    "drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "2")]
#[test_case(r#####"tw`drop-shadow-md`"#####, r#####"({
  "--tw-drop-shadow":
    "drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "3")]
#[test_case(r#####"tw`drop-shadow-lg`"#####, r#####"({
  "--tw-drop-shadow":
    "drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "4")]
#[test_case(r#####"tw`drop-shadow-xl`"#####, r#####"({
  "--tw-drop-shadow":
    "drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "5")]
#[test_case(r#####"tw`drop-shadow-2xl`"#####, r#####"({
  "--tw-drop-shadow": "drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "6")]
#[test_case(r#####"tw`drop-shadow-none`"#####, r#####"({
  "--tw-drop-shadow": "drop-shadow(0 0 #0000)",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})
;"##### ; "7")]
#[test_case(r#####"tw`drop-shadow-[0 35px 35px rgba(0, 0, 0, 0.25)]`"#####, r#####"({
  "--tw-drop-shadow": "drop-shadow(0 35px 35px rgba(0, 0, 0, 0.25))",
  filter:
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow)",
})"##### ; "8")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
