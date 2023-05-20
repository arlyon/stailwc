use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`mix-blend-normal`"#####, r#####"({
  mixBlendMode: "normal",
})
;"##### ; "0")]
#[test_case(r#####"tw`mix-blend-multiply`"#####, r#####"({
  mixBlendMode: "multiply",
})
;"##### ; "1")]
#[test_case(r#####"tw`mix-blend-screen`"#####, r#####"({
  mixBlendMode: "screen",
})
;"##### ; "2")]
#[test_case(r#####"tw`mix-blend-overlay`"#####, r#####"({
  mixBlendMode: "overlay",
})
;"##### ; "3")]
#[test_case(r#####"tw`mix-blend-darken`"#####, r#####"({
  mixBlendMode: "darken",
})
;"##### ; "4")]
#[test_case(r#####"tw`mix-blend-lighten`"#####, r#####"({
  mixBlendMode: "lighten",
})
;"##### ; "5")]
#[test_case(r#####"tw`mix-blend-color-dodge`"#####, r#####"({
  mixBlendMode: "color-dodge",
})
;"##### ; "6")]
#[test_case(r#####"tw`mix-blend-color-burn`"#####, r#####"({
  mixBlendMode: "color-burn",
})
;"##### ; "7")]
#[test_case(r#####"tw`mix-blend-hard-light`"#####, r#####"({
  mixBlendMode: "hard-light",
})
;"##### ; "8")]
#[test_case(r#####"tw`mix-blend-soft-light`"#####, r#####"({
  mixBlendMode: "soft-light",
})
;"##### ; "9")]
#[test_case(r#####"tw`mix-blend-difference`"#####, r#####"({
  mixBlendMode: "difference",
})
;"##### ; "10")]
#[test_case(r#####"tw`mix-blend-exclusion`"#####, r#####"({
  mixBlendMode: "exclusion",
})
;"##### ; "11")]
#[test_case(r#####"tw`mix-blend-hue`"#####, r#####"({
  mixBlendMode: "hue",
})
;"##### ; "12")]
#[test_case(r#####"tw`mix-blend-saturation`"#####, r#####"({
  mixBlendMode: "saturation",
})
;"##### ; "13")]
#[test_case(r#####"tw`mix-blend-color`"#####, r#####"({
  mixBlendMode: "color",
})
;"##### ; "14")]
#[test_case(r#####"tw`mix-blend-luminosity`"#####, r#####"({
  mixBlendMode: "luminosity",
})
;"##### ; "15")]
#[test_case(r#####"tw`mix-blend-plus-lighter`"#####, r#####"({
  mixBlendMode: "plus-lighter",
})"##### ; "16")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
