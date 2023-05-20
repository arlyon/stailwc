use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`list-image-none`"#####, r#####"({
  listStyleImage: "none",
})
;"##### ; "1")]
#[test_case(r#####"tw`list-image-[url(checkmark.png)]`"#####, r#####"({
  listStyleImage: "url(checkmark.png)",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
