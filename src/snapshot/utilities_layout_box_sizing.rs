use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`box-border`"#####, r#####"({
  boxSizing: "border-box",
})
;"##### ; "1")]
#[test_case(r#####"tw`box-content`"#####, r#####"({
  boxSizing: "content-box",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
