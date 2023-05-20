use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`bg-blend-normal`"#####, r#####"({
  backgroundBlendMode: "normal",
})
;"##### ; "0")]
#[test_case(r#####"tw`bg-blend-multiply`"#####, r#####"({
  backgroundBlendMode: "multiply",
})
;"##### ; "1")]
#[test_case(r#####"tw`bg-blend-screen`"#####, r#####"({
  backgroundBlendMode: "screen",
})
;"##### ; "2")]
#[test_case(r#####"tw`bg-blend-overlay`"#####, r#####"({
  backgroundBlendMode: "overlay",
})
;"##### ; "3")]
#[test_case(r#####"tw`bg-blend-darken`"#####, r#####"({
  backgroundBlendMode: "darken",
})
;"##### ; "4")]
#[test_case(r#####"tw`bg-blend-lighten`"#####, r#####"({
  backgroundBlendMode: "lighten",
})
;"##### ; "5")]
#[test_case(r#####"tw`bg-blend-color-dodge`"#####, r#####"({
  backgroundBlendMode: "color-dodge",
})
;"##### ; "6")]
#[test_case(r#####"tw`bg-blend-color-burn`"#####, r#####"({
  backgroundBlendMode: "color-burn",
})
;"##### ; "7")]
#[test_case(r#####"tw`bg-blend-hard-light`"#####, r#####"({
  backgroundBlendMode: "hard-light",
})
;"##### ; "8")]
#[test_case(r#####"tw`bg-blend-soft-light`"#####, r#####"({
  backgroundBlendMode: "soft-light",
})
;"##### ; "9")]
#[test_case(r#####"tw`bg-blend-difference`"#####, r#####"({
  backgroundBlendMode: "difference",
})
;"##### ; "10")]
#[test_case(r#####"tw`bg-blend-exclusion`"#####, r#####"({
  backgroundBlendMode: "exclusion",
})
;"##### ; "11")]
#[test_case(r#####"tw`bg-blend-hue`"#####, r#####"({
  backgroundBlendMode: "hue",
})
;"##### ; "12")]
#[test_case(r#####"tw`bg-blend-saturation`"#####, r#####"({
  backgroundBlendMode: "saturation",
})
;"##### ; "13")]
#[test_case(r#####"tw`bg-blend-color`"#####, r#####"({
  backgroundBlendMode: "color",
})
;"##### ; "14")]
#[test_case(r#####"tw`bg-blend-luminosity`"#####, r#####"({
  backgroundBlendMode: "luminosity",
})"##### ; "15")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
