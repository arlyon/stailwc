use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`touch-auto`"#####, r#####"({
  touchAction: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`touch-none`"#####, r#####"({
  touchAction: "none",
})
;"##### ; "2")]
#[test_case(r#####"tw`touch-pan-x`"#####, r#####"({
  '--tw-pan-x': "pan-x",
  touchAction: "var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)",
})
;"##### ; "3")]
#[test_case(r#####"tw`touch-pan-left`"#####, r#####"({
  '--tw-pan-x': "pan-left",
  touchAction: "var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)",
})
;"##### ; "4")]
#[test_case(r#####"tw`touch-pan-right`"#####, r#####"({
  '--tw-pan-x': "pan-right",
  touchAction: "var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)",
})
;"##### ; "5")]
#[test_case(r#####"tw`touch-pan-y`"#####, r#####"({
  '--tw-pan-y': "pan-y",
  touchAction: "var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)",
})
;"##### ; "6")]
#[test_case(r#####"tw`touch-pan-up`"#####, r#####"({
  '--tw-pan-y': "pan-up",
  touchAction: "var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)",
})
;"##### ; "7")]
#[test_case(r#####"tw`touch-pan-down`"#####, r#####"({
  '--tw-pan-y': "pan-down",
  touchAction: "var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)",
})
;"##### ; "8")]
#[test_case(r#####"tw`touch-pinch-zoom`"#####, r#####"({
  '--tw-pinch-zoom': "pinch-zoom",
  touchAction: "var(--tw-pan-x) var(--tw-pan-y) var(--tw-pinch-zoom)",
})
;"##### ; "9")]
#[test_case(r#####"tw`touch-manipulation`"#####, r#####"({
  touchAction: "manipulation",
})"##### ; "10")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
