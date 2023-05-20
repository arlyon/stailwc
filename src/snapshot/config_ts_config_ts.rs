use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`text-purple`"#####, r#####"({
  color: "custom",
})"##### ; "0")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
