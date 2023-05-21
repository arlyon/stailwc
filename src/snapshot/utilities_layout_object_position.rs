use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`objectPosition`"#####, r#####"({
  bottom: "bottom",
  center: "center",
  left: "left",
  "left-bottom": "left bottom",
  "left-top": "left top",
  right: "right",
  "right-bottom": "right bottom",
  "right-top": "right top",
  top: "top",
})
;"##### ; "0")]
#[test_case(r#####"tw`object-bottom`"#####, r#####"({
  objectPosition: "bottom",
})
;"##### ; "1")]
#[test_case(r#####"tw`object-center`"#####, r#####"({
  objectPosition: "center",
})
;"##### ; "2")]
#[test_case(r#####"tw`object-left`"#####, r#####"({
  objectPosition: "left",
})
;"##### ; "3")]
#[test_case(r#####"tw`object-left-bottom`"#####, r#####"({
  objectPosition: "left bottom",
})
;"##### ; "4")]
#[test_case(r#####"tw`object-left-top`"#####, r#####"({
  objectPosition: "left top",
})
;"##### ; "5")]
#[test_case(r#####"tw`object-right`"#####, r#####"({
  objectPosition: "right",
})
;"##### ; "6")]
#[test_case(r#####"tw`object-right-bottom`"#####, r#####"({
  objectPosition: "right bottom",
})
;"##### ; "7")]
#[test_case(r#####"tw`object-right-top`"#####, r#####"({
  objectPosition: "right top",
})
;"##### ; "8")]
#[test_case(r#####"tw`object-top`"#####, r#####"({
  objectPosition: "top",
})
;"##### ; "9")]
#[test_case(r#####"tw`object-[center bottom]`"#####, r#####"({
  objectPosition: "center bottom",
})
;"##### ; "10")]
#[test_case(r#####"tw`object-[var(--position)]`"#####, r#####"({
  objectPosition: "var(--position)",
})"##### ; "11")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
