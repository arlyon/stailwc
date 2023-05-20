use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`isolate`"#####, r#####"({
  isolation: "isolate",
})
;"##### ; "0")]
#[test_case(r#####"tw`isolation-auto`"#####, r#####"({
  isolation: "auto",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
