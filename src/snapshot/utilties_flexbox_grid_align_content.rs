use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`content-center`"#####, r#####"({
  alignContent: "center",
})
;"##### ; "0")]
#[test_case(r#####"tw`content-start`"#####, r#####"({
  alignContent: "flex-start",
})
;"##### ; "1")]
#[test_case(r#####"tw`content-end`"#####, r#####"({
  alignContent: "flex-end",
})
;"##### ; "2")]
#[test_case(r#####"tw`content-between`"#####, r#####"({
  alignContent: "space-between",
})
;"##### ; "3")]
#[test_case(r#####"tw`content-around`"#####, r#####"({
  alignContent: "space-around",
})
;"##### ; "4")]
#[test_case(r#####"tw`content-evenly`"#####, r#####"({
  alignContent: "space-evenly",
})
;"##### ; "5")]
#[test_case(r#####"tw`content-baseline`"#####, r#####"({
  alignContent: "baseline",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
