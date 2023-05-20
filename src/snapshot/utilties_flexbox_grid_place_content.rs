use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`place-content-center`"#####, r#####"({
  placeContent: "center",
})
;"##### ; "1")]
#[test_case(r#####"tw`place-content-start`"#####, r#####"({
  placeContent: "start",
})
;"##### ; "2")]
#[test_case(r#####"tw`place-content-end`"#####, r#####"({
  placeContent: "end",
})
;"##### ; "3")]
#[test_case(r#####"tw`place-content-between`"#####, r#####"({
  placeContent: "space-between",
})
;"##### ; "4")]
#[test_case(r#####"tw`place-content-around`"#####, r#####"({
  placeContent: "space-around",
})
;"##### ; "5")]
#[test_case(r#####"tw`place-content-evenly`"#####, r#####"({
  placeContent: "space-evenly",
})
;"##### ; "6")]
#[test_case(r#####"tw`place-content-stretch`"#####, r#####"({
  placeContent: "stretch",
})
;"##### ; "7")]
#[test_case(r#####"tw`place-content-baseline`"#####, r#####"({
  placeContent: "baseline",
})"##### ; "8")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
