use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`content-auto`"#####, r#####"({
  contentVisibility: "auto",
})
;"##### ; "0")]
#[test_case(r#####"tw`content-hidden`"#####, r#####"({
  contentVisibility: "hidden",
})
;"##### ; "1")]
#[test_case(r#####"tw`content-visible`"#####, r#####"({
  contentVisibility: "visible",
})
;"##### ; "2")]
#[test_case(r#####"tw`tab-1`"#####, r#####"({
  tabSizeTest: "1",
})
;"##### ; "3")]
#[test_case(r#####"tw`tab-2`"#####, r#####"({
  tabSizeTest: "2",
})
;"##### ; "4")]
#[test_case(r#####"tw`tab-4`"#####, r#####"({
  tabSizeTest: "4",
})
;"##### ; "5")]
#[test_case(r#####"tw`tab-8`"#####, r#####"({
  tabSizeTest: "8",
})
;"##### ; "6")]
#[test_case(r#####"tw`btn`"#####, r#####"({
  padding: ".5rem 1rem",
  borderRadius: ".25rem",
  fontWeight: "600",
})
;"##### ; "7")]
#[test_case(r#####"tw`btn-blue`"#####, r#####"({
  backgroundColor: "#3490dc",
  color: "#fff",
  ":hover": {
    backgroundColor: "#2779bd",
  },
})
;"##### ; "8")]
#[test_case(r#####"tw`btn-red`"#####, r#####"({
  backgroundColor: "#e3342f",
  color: "#fff",
  ":hover": {
    backgroundColor: "#cc1f1a",
  },
})
;"##### ; "9")]
#[test_case(r#####"tw`btn btn-blue btn-red`"#####, r#####"({
  padding: ".5rem 1rem",
  borderRadius: ".25rem",
  fontWeight: "600",
  backgroundColor: "#e3342f",
  color: "#fff",
  ":hover": {
    backgroundColor: "#cc1f1a",
  },
})
;"##### ; "10")]
#[test_case(r#####"globalStyles"#####, r#####"({
  "*, ::before, ::after": {
    "--tw-border-spacing-x": "0",
    "--tw-border-spacing-y": "0",
    "--tw-translate-x": "0",
    "--tw-translate-y": "0",
    "--tw-rotate": "0",
    "--tw-skew-x": "0",
    "--tw-skew-y": "0",
    "--tw-scale-x": "1",
    "--tw-scale-y": "1",
    "--tw-pan-x": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-pan-y": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-pinch-zoom": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-scroll-snap-strictness": "proximity",
    "--tw-ordinal": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-slashed-zero": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-numeric-figure": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-numeric-spacing": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-numeric-fraction": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-ring-offset-shadow": "0 0 #0000",
    "--tw-ring-shadow": "0 0 #0000",
    "--tw-shadow": "0 0 #0000",
    "--tw-shadow-colored": "0 0 #0000",
    "--tw-ring-inset": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-ring-offset-width": "0px",
    "--tw-ring-offset-color": "#fff",
    "--tw-ring-color": "rgb(59 130 246 / 0.5)",
    "--tw-blur": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-brightness": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-contrast": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-grayscale": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-hue-rotate": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-invert": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-saturate": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-sepia": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-drop-shadow": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-blur": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-brightness": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-contrast": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-grayscale": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-hue-rotate": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-invert": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-opacity": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-saturate": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-sepia": "var(--tw-empty,/*!*/ /*!*/)",
  },
  "::backdrop": {
    "--tw-border-spacing-x": "0",
    "--tw-border-spacing-y": "0",
    "--tw-translate-x": "0",
    "--tw-translate-y": "0",
    "--tw-rotate": "0",
    "--tw-skew-x": "0",
    "--tw-skew-y": "0",
    "--tw-scale-x": "1",
    "--tw-scale-y": "1",
    "--tw-pan-x": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-pan-y": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-pinch-zoom": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-scroll-snap-strictness": "proximity",
    "--tw-ordinal": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-slashed-zero": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-numeric-figure": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-numeric-spacing": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-numeric-fraction": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-ring-offset-shadow": "0 0 #0000",
    "--tw-ring-shadow": "0 0 #0000",
    "--tw-shadow": "0 0 #0000",
    "--tw-shadow-colored": "0 0 #0000",
    "--tw-ring-inset": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-ring-offset-width": "0px",
    "--tw-ring-offset-color": "#fff",
    "--tw-ring-color": "rgb(59 130 246 / 0.5)",
    "--tw-blur": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-brightness": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-contrast": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-grayscale": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-hue-rotate": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-invert": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-saturate": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-sepia": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-drop-shadow": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-blur": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-brightness": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-contrast": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-grayscale": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-hue-rotate": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-invert": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-opacity": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-saturate": "var(--tw-empty,/*!*/ /*!*/)",
    "--tw-backdrop-sepia": "var(--tw-empty,/*!*/ /*!*/)",
  },
})
;"##### ; "11")]
#[test_case(r#####"tw`test-1:block`"#####, r#####"({
  ":test1": {
    display: "block",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`test-2:block`"#####, r#####"({
  ":hover": {
    display: "block",
  },
  ":focus": {
    display: "block",
  },
})
;"##### ; "13")]
#[test_case(r#####"tw`test-3:block`"#####, r#####"({
  "@supports (display: grid)": {
    display: "block",
  },
})
;"##### ; "14")]
#[test_case(r#####"tw`test-4:block`"#####, r#####"({
  "html.dark &.something": {
    display: "block",
  },
})
;"##### ; "15")]
#[test_case(r#####"tw`potato-[yellow]:bg-yellow-200`"#####, r#####"({
  ".potato-yellow &": {
    "--tw-bg-opacity": "1",
    backgroundColor: "rgb(254 240 138 / var(--tw-bg-opacity))",
  },
})
;"##### ; "16")]
#[test_case(r#####"tw`potato-[baked]:w-3`"#####, r#####"({
  ".potato-baked &": {
    width: "0.75rem",
  },
})
;"##### ; "17")]
#[test_case(r#####"tw`tooltip-bottom:mt-5`"#####, r#####"({
  "&[data-location="bottom"]": {
    marginTop: "1.25rem",
  },
})
;"##### ; "18")]
#[test_case(r#####"tw`tooltip-top:mb-5`"#####, r#####"({
  "&[data-location="top"]": {
    marginBottom: "1.25rem",
  },
})
;"##### ; "19")]
#[test_case(r#####"tw`alphabet-c:underline `"#####, r#####"({
  "&[data-value="c"]": {
    textDecorationLine: "underline",
  },
})
;"##### ; "20")]
#[test_case(r#####"tw`alphabet-a:underline `"#####, r#####"({
  "&[data-value="a"]": {
    textDecorationLine: "underline",
  },
})
;"##### ; "21")]
#[test_case(r#####"tw`alphabet-d:underline `"#####, r#####"({
  "&[data-value="d"]": {
    textDecorationLine: "underline",
  },
})
;"##### ; "22")]
#[test_case(r#####"tw`alphabet-b:underline`"#####, r#####"({
  "&[data-value="b"]": {
    textDecorationLine: "underline",
  },
})
;"##### ; "23")]
#[test_case(r#####"tw`test-[a,b,c]:underline`"#####, r#####"({
  "&.a > *": {
    textDecorationLine: "underline",
  },
  "&.b > *": {
    textDecorationLine: "underline",
  },
  "&.c > *": {
    textDecorationLine: "underline",
  },
})
;"##### ; "24")]
#[test_case(r#####"tw`testmin-[500px]:underline`"#####, r#####"({
  "@media (min-width: 500px)": {
    textDecorationLine: "underline",
  },
})
;"##### ; "25")]
#[test_case(r#####"tw`testmin-[700px]:italic`"#####, r#####"({
  "@media (min-width: 700px)": {
    fontStyle: "italic",
  },
})"##### ; "26")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
