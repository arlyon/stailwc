use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`gridTemplateRows`"#####, r#####"({
  1: "repeat(1, minmax(0, 1fr))",
  2: "repeat(2, minmax(0, 1fr))",
  3: "repeat(3, minmax(0, 1fr))",
  4: "repeat(4, minmax(0, 1fr))",
  5: "repeat(5, minmax(0, 1fr))",
  6: "repeat(6, minmax(0, 1fr))",
  none: "none",
})
;"##### ; "0")]
#[test_case(r#####"tw`grid-rows-1`"#####, r#####"({
  gridTemplateRows: "repeat(1, minmax(0, 1fr))",
})
;"##### ; "1")]
#[test_case(r#####"tw`grid-rows-2`"#####, r#####"({
  gridTemplateRows: "repeat(2, minmax(0, 1fr))",
})
;"##### ; "2")]
#[test_case(r#####"tw`grid-rows-3`"#####, r#####"({
  gridTemplateRows: "repeat(3, minmax(0, 1fr))",
})
;"##### ; "3")]
#[test_case(r#####"tw`grid-rows-4`"#####, r#####"({
  gridTemplateRows: "repeat(4, minmax(0, 1fr))",
})
;"##### ; "4")]
#[test_case(r#####"tw`grid-rows-5`"#####, r#####"({
  gridTemplateRows: "repeat(5, minmax(0, 1fr))",
})
;"##### ; "5")]
#[test_case(r#####"tw`grid-rows-6`"#####, r#####"({
  gridTemplateRows: "repeat(6, minmax(0, 1fr))",
})
;"##### ; "6")]
#[test_case(r#####"tw`grid-rows-none`"#####, r#####"({
  gridTemplateRows: "none",
})
;"##### ; "7")]
#[test_case(r#####"tw`grid-rows-[200px minmax(900px, 1fr) 100px]`"#####, r#####"({
  gridTemplateRows: "200px minmax(900px, 1fr) 100px",
})"##### ; "8")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
