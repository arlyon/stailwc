use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`bg-fixed`"#####, r#####"({
  backgroundAttachment: "fixed",
})
;"##### ; "0")]
#[test_case(r#####"tw`bg-local`"#####, r#####"({
  backgroundAttachment: "local",
})
;"##### ; "1")]
#[test_case(r#####"tw`bg-scroll`"#####, r#####"({
  backgroundAttachment: "scroll",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
