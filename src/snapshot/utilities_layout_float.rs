use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`float-right`"#####, r#####"({
  float: "right",
})
;"##### ; "1")]
#[test_case(r#####"tw`float-left`"#####, r#####"({
  float: "left",
})
;"##### ; "2")]
#[test_case(r#####"tw`float-none`"#####, r#####"({
  float: "none",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
