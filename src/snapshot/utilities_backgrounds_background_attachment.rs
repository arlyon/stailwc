use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`bg-fixed`"#####, r#####"({
  backgroundAttachment: "fixed",
})
;"##### ; "1")]
#[test_case(r#####"tw`bg-local`"#####, r#####"({
  backgroundAttachment: "local",
})
;"##### ; "2")]
#[test_case(r#####"tw`bg-scroll`"#####, r#####"({
  backgroundAttachment: "scroll",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
