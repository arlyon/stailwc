use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`objectPosition`"#####, r#####"({
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
#[test_case(r#####"tw`object-bottom`"#####, r#####"({
  objectPosition: "bottom",
})
;"##### ; "2")]
#[test_case(r#####"tw`object-center`"#####, r#####"({
  objectPosition: "center",
})
;"##### ; "3")]
#[test_case(r#####"tw`object-left`"#####, r#####"({
  objectPosition: "left",
})
;"##### ; "4")]
#[test_case(r#####"tw`object-left-bottom`"#####, r#####"({
  objectPosition: "left bottom",
})
;"##### ; "5")]
#[test_case(r#####"tw`object-left-top`"#####, r#####"({
  objectPosition: "left top",
})
;"##### ; "6")]
#[test_case(r#####"tw`object-right`"#####, r#####"({
  objectPosition: "right",
})
;"##### ; "7")]
#[test_case(r#####"tw`object-right-bottom`"#####, r#####"({
  objectPosition: "right bottom",
})
;"##### ; "8")]
#[test_case(r#####"tw`object-right-top`"#####, r#####"({
  objectPosition: "right top",
})
;"##### ; "9")]
#[test_case(r#####"tw`object-top`"#####, r#####"({
  objectPosition: "top",
})
;"##### ; "10")]
#[test_case(r#####"tw`object-[center bottom]`"#####, r#####"({
  objectPosition: "center bottom",
})
;"##### ; "11")]
#[test_case(r#####"tw`object-[var(--position)]`"#####, r#####"({
  objectPosition: "var(--position)",
})"##### ; "12")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
