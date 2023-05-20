use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
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
;"##### ; "1")]
#[test_case(r#####"tw`bg-repeat`"#####, r#####"({
  backgroundRepeat: "repeat",
})
;"##### ; "2")]
#[test_case(r#####"tw`bg-no-repeat`"#####, r#####"({
  backgroundRepeat: "no-repeat",
})
;"##### ; "3")]
#[test_case(r#####"tw`bg-repeat-x`"#####, r#####"({
  backgroundRepeat: "repeat-x",
})
;"##### ; "4")]
#[test_case(r#####"tw`bg-repeat-y`"#####, r#####"({
  backgroundRepeat: "repeat-y",
})
;"##### ; "5")]
#[test_case(r#####"tw`bg-repeat-round`"#####, r#####"({
  backgroundRepeat: "round",
})
;"##### ; "6")]
#[test_case(r#####"tw`bg-repeat-space`"#####, r#####"({
  backgroundRepeat: "space",
})"##### ; "7")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
