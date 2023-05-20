use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`break-keep`"#####, r#####"({
  wordBreak: "keep-all",
})
;"##### ; "0")]
#[test_case(r#####"tw`break-normal`"#####, r#####"({
  overflowWrap: "normal",
  wordBreak: "normal",
})
;"##### ; "1")]
#[test_case(r#####"tw`break-words`"#####, r#####"({
  overflowWrap: "break-word",
})
;"##### ; "2")]
#[test_case(r#####"tw`break-all`"#####, r#####"({
  wordBreak: "break-all",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
