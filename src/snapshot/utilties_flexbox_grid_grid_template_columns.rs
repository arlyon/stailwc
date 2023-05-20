use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`gridTemplateColumns`"#####, r#####"({
  1: "repeat(1, minmax(0, 1fr))",
  2: "repeat(2, minmax(0, 1fr))",
  3: "repeat(3, minmax(0, 1fr))",
  4: "repeat(4, minmax(0, 1fr))",
  5: "repeat(5, minmax(0, 1fr))",
  6: "repeat(6, minmax(0, 1fr))",
  7: "repeat(7, minmax(0, 1fr))",
  8: "repeat(8, minmax(0, 1fr))",
  9: "repeat(9, minmax(0, 1fr))",
  10: "repeat(10, minmax(0, 1fr))",
  11: "repeat(11, minmax(0, 1fr))",
  12: "repeat(12, minmax(0, 1fr))",
  none: "none",
})
;"##### ; "1")]
#[test_case(r#####"tw`grid-cols-1`"#####, r#####"({
  gridTemplateColumns: "repeat(1, minmax(0, 1fr))",
})
;"##### ; "2")]
#[test_case(r#####"tw`grid-cols-2`"#####, r#####"({
  gridTemplateColumns: "repeat(2, minmax(0, 1fr))",
})
;"##### ; "3")]
#[test_case(r#####"tw`grid-cols-3`"#####, r#####"({
  gridTemplateColumns: "repeat(3, minmax(0, 1fr))",
})
;"##### ; "4")]
#[test_case(r#####"tw`grid-cols-4`"#####, r#####"({
  gridTemplateColumns: "repeat(4, minmax(0, 1fr))",
})
;"##### ; "5")]
#[test_case(r#####"tw`grid-cols-5`"#####, r#####"({
  gridTemplateColumns: "repeat(5, minmax(0, 1fr))",
})
;"##### ; "6")]
#[test_case(r#####"tw`grid-cols-6`"#####, r#####"({
  gridTemplateColumns: "repeat(6, minmax(0, 1fr))",
})
;"##### ; "7")]
#[test_case(r#####"tw`grid-cols-7`"#####, r#####"({
  gridTemplateColumns: "repeat(7, minmax(0, 1fr))",
})
;"##### ; "8")]
#[test_case(r#####"tw`grid-cols-8`"#####, r#####"({
  gridTemplateColumns: "repeat(8, minmax(0, 1fr))",
})
;"##### ; "9")]
#[test_case(r#####"tw`grid-cols-9`"#####, r#####"({
  gridTemplateColumns: "repeat(9, minmax(0, 1fr))",
})
;"##### ; "10")]
#[test_case(r#####"tw`grid-cols-10`"#####, r#####"({
  gridTemplateColumns: "repeat(10, minmax(0, 1fr))",
})
;"##### ; "11")]
#[test_case(r#####"tw`grid-cols-11`"#####, r#####"({
  gridTemplateColumns: "repeat(11, minmax(0, 1fr))",
})
;"##### ; "12")]
#[test_case(r#####"tw`grid-cols-12`"#####, r#####"({
  gridTemplateColumns: "repeat(12, minmax(0, 1fr))",
})
;"##### ; "13")]
#[test_case(r#####"tw`grid-cols-none`"#####, r#####"({
  gridTemplateColumns: "none",
})
;"##### ; "14")]
#[test_case(r#####"tw`grid-cols-[200px minmax(900px, 1fr) 100px]`"#####, r#####"({
  gridTemplateColumns: "200px minmax(900px, 1fr) 100px",
})"##### ; "15")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
