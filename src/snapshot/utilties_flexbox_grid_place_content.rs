use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`place-content-center`"#####, r#####"({
  placeContent: "center",
})
;"##### ; "0")]
#[test_case(r#####"tw`place-content-start`"#####, r#####"({
  placeContent: "start",
})
;"##### ; "1")]
#[test_case(r#####"tw`place-content-end`"#####, r#####"({
  placeContent: "end",
})
;"##### ; "2")]
#[test_case(r#####"tw`place-content-between`"#####, r#####"({
  placeContent: "space-between",
})
;"##### ; "3")]
#[test_case(r#####"tw`place-content-around`"#####, r#####"({
  placeContent: "space-around",
})
;"##### ; "4")]
#[test_case(r#####"tw`place-content-evenly`"#####, r#####"({
  placeContent: "space-evenly",
})
;"##### ; "5")]
#[test_case(r#####"tw`place-content-stretch`"#####, r#####"({
  placeContent: "stretch",
})
;"##### ; "6")]
#[test_case(r#####"tw`place-content-baseline`"#####, r#####"({
  placeContent: "baseline",
})"##### ; "7")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
