use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`justify-items-start`"#####, r#####"({
  justifyItems: "start",
})
;"##### ; "0")]
#[test_case(r#####"tw`justify-items-end`"#####, r#####"({
  justifyItems: "end",
})
;"##### ; "1")]
#[test_case(r#####"tw`justify-items-center`"#####, r#####"({
  justifyItems: "center",
})
;"##### ; "2")]
#[test_case(r#####"tw`justify-items-stretch`"#####, r#####"({
  justifyItems: "stretch",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
