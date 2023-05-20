use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`visible`"#####, r#####"({
  visibility: "visible",
})
;"##### ; "1")]
#[test_case(r#####"tw`invisible`"#####, r#####"({
  visibility: "hidden",
})
;"##### ; "2")]
#[test_case(r#####"tw`collapse`"#####, r#####"({
  visibility: "collapse",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
