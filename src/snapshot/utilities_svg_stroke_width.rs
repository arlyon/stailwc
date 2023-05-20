use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`strokeWidth`"#####, r#####"({
  0: "0",
  1: "1",
  2: "2",
})
;"##### ; "1")]
#[test_case(r#####"tw`stroke-0`"#####, r#####"({
  strokeWidth: "0",
})
;"##### ; "2")]
#[test_case(r#####"tw`stroke-1`"#####, r#####"({
  strokeWidth: "1",
})
;"##### ; "3")]
#[test_case(r#####"tw`stroke-2`"#####, r#####"({
  strokeWidth: "2",
})
;"##### ; "4")]
#[test_case(r#####"tw`stroke-[2px]`"#####, r#####"({
  strokeWidth: "2px",
})
;"##### ; "5")]
#[test_case(r#####"tw`stroke-[color:red]`"#####, r#####"({
  stroke: "red",
})
;"##### ; "6")]
#[test_case(r#####"tw`stroke-[length:2px]`"#####, r#####"({
  strokeWidth: "2px",
})
;"##### ; "7")]
#[test_case(r#####"tw`stroke-[number:10]`"#####, r#####"({
  strokeWidth: "10",
})
;"##### ; "8")]
#[test_case(r#####"tw`stroke-[percentage:10%]`"#####, r#####"({
  strokeWidth: "10%",
})
;"##### ; "9")]
#[test_case(r#####"tw`stroke-[url:url(hand.cur)_2_2, pointer]`"#####, r#####"({
  stroke: "url(hand.cur) 2 2, pointer",
})"##### ; "10")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
