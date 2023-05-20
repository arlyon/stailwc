use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`outlineOffset`"#####, r#####"({
  0: "0px",
  1: "1px",
  2: "2px",
  4: "4px",
  8: "8px",
})
;"##### ; "0")]
#[test_case(r#####"tw`outline-offset-0`"#####, r#####"({
  outlineOffset: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`outline-offset-1`"#####, r#####"({
  outlineOffset: "1px",
})
;"##### ; "2")]
#[test_case(r#####"tw`outline-offset-2`"#####, r#####"({
  outlineOffset: "2px",
})
;"##### ; "3")]
#[test_case(r#####"tw`outline-offset-4`"#####, r#####"({
  outlineOffset: "4px",
})
;"##### ; "4")]
#[test_case(r#####"tw`outline-offset-8`"#####, r#####"({
  outlineOffset: "8px",
})
;"##### ; "5")]
#[test_case(r#####"tw`-outline-offset-1`"#####, r#####"({
  outlineOffset: "-1px",
})
;"##### ; "6")]
#[test_case(r#####"tw`outline-offset-[3px]`"#####, r#####"({
  outlineOffset: "3px",
})"##### ; "7")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
