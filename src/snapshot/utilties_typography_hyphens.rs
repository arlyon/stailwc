use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`hyphens-none`"#####, r#####"({
  hyphens: "none",
})
;"##### ; "1")]
#[test_case(r#####"tw`hyphens-manual`"#####, r#####"({
  hyphens: "manual",
})
;"##### ; "2")]
#[test_case(r#####"tw`hyphens-auto`"#####, r#####"({
  hyphens: "auto",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
