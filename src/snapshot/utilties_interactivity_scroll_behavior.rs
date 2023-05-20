use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`scroll-auto`"#####, r#####"({
  scrollBehavior: "auto",
})
;"##### ; "0")]
#[test_case(r#####"tw`scroll-smooth`"#####, r#####"({
  scrollBehavior: "smooth",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
