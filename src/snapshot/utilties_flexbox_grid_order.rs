use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`order`"#####, r#####"({
  1: "1",
  2: "2",
  3: "3",
  4: "4",
  5: "5",
  6: "6",
  7: "7",
  8: "8",
  9: "9",
  10: "10",
  11: "11",
  12: "12",
  first: "-9999",
  last: "9999",
  none: "0",
})
;"##### ; "0")]
#[test_case(r#####"tw`order-1`"#####, r#####"({
  order: "1",
})
;"##### ; "1")]
#[test_case(r#####"tw`order-2`"#####, r#####"({
  order: "2",
})
;"##### ; "2")]
#[test_case(r#####"tw`order-3`"#####, r#####"({
  order: "3",
})
;"##### ; "3")]
#[test_case(r#####"tw`order-4`"#####, r#####"({
  order: "4",
})
;"##### ; "4")]
#[test_case(r#####"tw`order-5`"#####, r#####"({
  order: "5",
})
;"##### ; "5")]
#[test_case(r#####"tw`order-6`"#####, r#####"({
  order: "6",
})
;"##### ; "6")]
#[test_case(r#####"tw`order-7`"#####, r#####"({
  order: "7",
})
;"##### ; "7")]
#[test_case(r#####"tw`order-8`"#####, r#####"({
  order: "8",
})
;"##### ; "8")]
#[test_case(r#####"tw`order-9`"#####, r#####"({
  order: "9",
})
;"##### ; "9")]
#[test_case(r#####"tw`order-10`"#####, r#####"({
  order: "10",
})
;"##### ; "10")]
#[test_case(r#####"tw`order-11`"#####, r#####"({
  order: "11",
})
;"##### ; "11")]
#[test_case(r#####"tw`order-12`"#####, r#####"({
  order: "12",
})
;"##### ; "12")]
#[test_case(r#####"tw`order-first`"#####, r#####"({
  order: "-9999",
})
;"##### ; "13")]
#[test_case(r#####"tw`order-last`"#####, r#####"({
  order: "9999",
})
;"##### ; "14")]
#[test_case(r#####"tw`order-none`"#####, r#####"({
  order: "0",
})
;"##### ; "15")]
#[test_case(r#####"tw`order-[13]`"#####, r#####"({
  order: "13",
})"##### ; "16")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
