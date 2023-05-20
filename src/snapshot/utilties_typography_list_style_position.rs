use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`list-inside`"#####, r#####"({
  listStylePosition: "inside",
})
;"##### ; "1")]
#[test_case(r#####"tw`list-outside`"#####, r#####"({
  listStylePosition: "outside",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
