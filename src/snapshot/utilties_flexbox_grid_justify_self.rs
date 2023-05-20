use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`justify-self-auto`"#####, r#####"({
  justifySelf: "auto",
})
;"##### ; "0")]
#[test_case(r#####"tw`justify-self-start`"#####, r#####"({
  justifySelf: "start",
})
;"##### ; "1")]
#[test_case(r#####"tw`justify-self-end`"#####, r#####"({
  justifySelf: "end",
})
;"##### ; "2")]
#[test_case(r#####"tw`justify-self-center`"#####, r#####"({
  justifySelf: "center",
})
;"##### ; "3")]
#[test_case(r#####"tw`justify-self-stretch`"#####, r#####"({
  justifySelf: "stretch",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
