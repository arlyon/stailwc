use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`select-none`"#####, r#####"({
  userSelect: "none",
})
;"##### ; "0")]
#[test_case(r#####"tw`select-text`"#####, r#####"({
  userSelect: "text",
})
;"##### ; "1")]
#[test_case(r#####"tw`select-all`"#####, r#####"({
  userSelect: "all",
})
;"##### ; "2")]
#[test_case(r#####"tw`select-auto`"#####, r#####"({
  userSelect: "auto",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
