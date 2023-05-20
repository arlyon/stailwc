use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`outlineWidth`"#####, r#####"({
  0: "0px",
  1: "1px",
  2: "2px",
  4: "4px",
  8: "8px",
})
;"##### ; "0")]
#[test_case(r#####"tw`outline-0`"#####, r#####"({
  outlineWidth: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`outline-1`"#####, r#####"({
  outlineWidth: "1px",
})
;"##### ; "2")]
#[test_case(r#####"tw`outline-2`"#####, r#####"({
  outlineWidth: "2px",
})
;"##### ; "3")]
#[test_case(r#####"tw`outline-4`"#####, r#####"({
  outlineWidth: "4px",
})
;"##### ; "4")]
#[test_case(r#####"tw`outline-8`"#####, r#####"({
  outlineWidth: "8px",
})
;"##### ; "5")]
#[test_case(r#####"tw`outline outline-offset-2 outline-1`"#####, r#####"({
  outlineStyle: "solid",
  outlineWidth: "1px",
  outlineOffset: "2px",
})
;"##### ; "6")]
#[test_case(r#####"tw`outline-[5px]`"#####, r#####"({
  outlineWidth: "5px",
})"##### ; "7")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
