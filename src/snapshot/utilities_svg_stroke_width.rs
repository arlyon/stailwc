use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`strokeWidth`"#####, r#####"({
  0: "0",
  1: "1",
  2: "2",
})
;"##### ; "0")]
#[test_case(r#####"tw`stroke-0`"#####, r#####"({
  strokeWidth: "0",
})
;"##### ; "1")]
#[test_case(r#####"tw`stroke-1`"#####, r#####"({
  strokeWidth: "1",
})
;"##### ; "2")]
#[test_case(r#####"tw`stroke-2`"#####, r#####"({
  strokeWidth: "2",
})
;"##### ; "3")]
#[test_case(r#####"tw`stroke-[2px]`"#####, r#####"({
  strokeWidth: "2px",
})
;"##### ; "4")]
#[test_case(r#####"tw`stroke-[color:red]`"#####, r#####"({
  stroke: "red",
})
;"##### ; "5")]
#[test_case(r#####"tw`stroke-[length:2px]`"#####, r#####"({
  strokeWidth: "2px",
})
;"##### ; "6")]
#[test_case(r#####"tw`stroke-[number:10]`"#####, r#####"({
  strokeWidth: "10",
})
;"##### ; "7")]
#[test_case(r#####"tw`stroke-[percentage:10%]`"#####, r#####"({
  strokeWidth: "10%",
})
;"##### ; "8")]
#[test_case(r#####"tw`stroke-[url:url(hand.cur)_2_2, pointer]`"#####, r#####"({
  stroke: "url(hand.cur) 2 2, pointer",
})"##### ; "9")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
