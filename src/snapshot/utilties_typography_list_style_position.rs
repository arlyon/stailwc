use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`list-inside`"#####, r#####"({
  listStylePosition: "inside",
})
;"##### ; "0")]
#[test_case(r#####"tw`list-outside`"#####, r#####"({
  listStylePosition: "outside",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
