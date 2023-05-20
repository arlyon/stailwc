use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`pointer-events-none`"#####, r#####"({
  pointerEvents: "none",
})
;"##### ; "0")]
#[test_case(r#####"tw`pointer-events-auto`"#####, r#####"({
  pointerEvents: "auto",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
