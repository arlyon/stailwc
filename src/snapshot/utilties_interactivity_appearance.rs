use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`appearance-none`"#####, r#####"({
  appearance: "none",
})"##### ; "0")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
