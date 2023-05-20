use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`hyphens-none`"#####, r#####"({
  hyphens: "none",
})
;"##### ; "0")]
#[test_case(r#####"tw`hyphens-manual`"#####, r#####"({
  hyphens: "manual",
})
;"##### ; "1")]
#[test_case(r#####"tw`hyphens-auto`"#####, r#####"({
  hyphens: "auto",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
