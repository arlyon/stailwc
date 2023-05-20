use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`place-items-start`"#####, r#####"({
  placeItems: "start",
})
;"##### ; "0")]
#[test_case(r#####"tw`place-items-end`"#####, r#####"({
  placeItems: "end",
})
;"##### ; "1")]
#[test_case(r#####"tw`place-items-center`"#####, r#####"({
  placeItems: "center",
})
;"##### ; "2")]
#[test_case(r#####"tw`place-items-stretch`"#####, r#####"({
  placeItems: "stretch",
})
;"##### ; "3")]
#[test_case(r#####"tw`place-items-baseline`"#####, r#####"({
  placeItems: "baseline",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
