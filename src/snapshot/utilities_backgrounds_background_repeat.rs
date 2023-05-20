use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`backgroundPosition`"#####, r#####"({
  bottom: "bottom",
  center: "center",
  left: "left",
  'left-bottom': "left bottom",
  'left-top': "left top",
  right: "right",
  'right-bottom': "right bottom",
  'right-top': "right top",
  top: "top",
})
;"##### ; "0")]
#[test_case(r#####"tw`bg-repeat`"#####, r#####"({
  backgroundRepeat: "repeat",
})
;"##### ; "1")]
#[test_case(r#####"tw`bg-no-repeat`"#####, r#####"({
  backgroundRepeat: "no-repeat",
})
;"##### ; "2")]
#[test_case(r#####"tw`bg-repeat-x`"#####, r#####"({
  backgroundRepeat: "repeat-x",
})
;"##### ; "3")]
#[test_case(r#####"tw`bg-repeat-y`"#####, r#####"({
  backgroundRepeat: "repeat-y",
})
;"##### ; "4")]
#[test_case(r#####"tw`bg-repeat-round`"#####, r#####"({
  backgroundRepeat: "round",
})
;"##### ; "5")]
#[test_case(r#####"tw`bg-repeat-space`"#####, r#####"({
  backgroundRepeat: "space",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
