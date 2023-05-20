use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`list-image-none`"#####, r#####"({
  listStyleImage: "none",
})
;"##### ; "0")]
#[test_case(r#####"tw`list-image-[url(checkmark.png)]`"#####, r#####"({
  listStyleImage: "url(checkmark.png)",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
