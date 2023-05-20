use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`-z-1`"#####, r#####"({
  zIndex: "-1",
})
;"##### ; "0")]
#[test_case(r#####"tw`-z-10`"#####, r#####"({
  zIndex: "-10",
})
;"##### ; "1")]
#[test_case(r#####"tw`-inset-10`"#####, r#####"({
  inset: "-2.5rem",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
