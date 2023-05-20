use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`place-self-auto`"#####, r#####"({
  placeSelf: "auto",
})
;"##### ; "0")]
#[test_case(r#####"tw`place-self-start`"#####, r#####"({
  placeSelf: "start",
})
;"##### ; "1")]
#[test_case(r#####"tw`place-self-end`"#####, r#####"({
  placeSelf: "end",
})
;"##### ; "2")]
#[test_case(r#####"tw`place-self-center`"#####, r#####"({
  placeSelf: "center",
})
;"##### ; "3")]
#[test_case(r#####"tw`place-self-stretch`"#####, r#####"({
  placeSelf: "stretch",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
