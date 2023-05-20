use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`textDecorationThickness`"#####, r#####"({
  0: "0px",
  1: "1px",
  2: "2px",
  4: "4px",
  8: "8px",
  auto: "auto",
  'from-font': "from-font",
})
;"##### ; "1")]
#[test_case(r#####"tw`decoration-auto`"#####, r#####"({
  textDecorationThickness: "auto",
})
;"##### ; "2")]
#[test_case(r#####"tw`decoration-from-font`"#####, r#####"({
  textDecorationThickness: "from-font",
})
;"##### ; "3")]
#[test_case(r#####"tw`decoration-0`"#####, r#####"({
  textDecorationThickness: "0px",
})
;"##### ; "4")]
#[test_case(r#####"tw`decoration-1`"#####, r#####"({
  textDecorationThickness: "1px",
})
;"##### ; "5")]
#[test_case(r#####"tw`decoration-2`"#####, r#####"({
  textDecorationThickness: "2px",
})
;"##### ; "6")]
#[test_case(r#####"tw`decoration-4`"#####, r#####"({
  textDecorationThickness: "4px",
})
;"##### ; "7")]
#[test_case(r#####"tw`decoration-8`"#####, r#####"({
  textDecorationThickness: "8px",
})
;"##### ; "8")]
#[test_case(r#####"tw`decoration-[length:10px]`"#####, r#####"({
  textDecorationThickness: "10px",
})
;"##### ; "9")]
#[test_case(r#####"tw`decoration-[percentage:10%]`"#####, r#####"({
  textDecorationThickness: "10%",
})"##### ; "10")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
