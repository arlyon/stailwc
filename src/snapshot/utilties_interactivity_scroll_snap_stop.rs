use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`snap-normal`"#####, r#####"({
  scrollSnapStop: "normal",
})
;"##### ; "0")]
#[test_case(r#####"tw`snap-always`"#####, r#####"({
  scrollSnapStop: "always",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
