use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`mix-blend-normal`"#####, r#####"({
  mixBlendMode: "normal",
})
;"##### ; "1")]
#[test_case(r#####"tw`mix-blend-multiply`"#####, r#####"({
  mixBlendMode: "multiply",
})
;"##### ; "2")]
#[test_case(r#####"tw`mix-blend-screen`"#####, r#####"({
  mixBlendMode: "screen",
})
;"##### ; "3")]
#[test_case(r#####"tw`mix-blend-overlay`"#####, r#####"({
  mixBlendMode: "overlay",
})
;"##### ; "4")]
#[test_case(r#####"tw`mix-blend-darken`"#####, r#####"({
  mixBlendMode: "darken",
})
;"##### ; "5")]
#[test_case(r#####"tw`mix-blend-lighten`"#####, r#####"({
  mixBlendMode: "lighten",
})
;"##### ; "6")]
#[test_case(r#####"tw`mix-blend-color-dodge`"#####, r#####"({
  mixBlendMode: "color-dodge",
})
;"##### ; "7")]
#[test_case(r#####"tw`mix-blend-color-burn`"#####, r#####"({
  mixBlendMode: "color-burn",
})
;"##### ; "8")]
#[test_case(r#####"tw`mix-blend-hard-light`"#####, r#####"({
  mixBlendMode: "hard-light",
})
;"##### ; "9")]
#[test_case(r#####"tw`mix-blend-soft-light`"#####, r#####"({
  mixBlendMode: "soft-light",
})
;"##### ; "10")]
#[test_case(r#####"tw`mix-blend-difference`"#####, r#####"({
  mixBlendMode: "difference",
})
;"##### ; "11")]
#[test_case(r#####"tw`mix-blend-exclusion`"#####, r#####"({
  mixBlendMode: "exclusion",
})
;"##### ; "12")]
#[test_case(r#####"tw`mix-blend-hue`"#####, r#####"({
  mixBlendMode: "hue",
})
;"##### ; "13")]
#[test_case(r#####"tw`mix-blend-saturation`"#####, r#####"({
  mixBlendMode: "saturation",
})
;"##### ; "14")]
#[test_case(r#####"tw`mix-blend-color`"#####, r#####"({
  mixBlendMode: "color",
})
;"##### ; "15")]
#[test_case(r#####"tw`mix-blend-luminosity`"#####, r#####"({
  mixBlendMode: "luminosity",
})
;"##### ; "16")]
#[test_case(r#####"tw`mix-blend-plus-lighter`"#####, r#####"({
  mixBlendMode: "plus-lighter",
})"##### ; "17")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
