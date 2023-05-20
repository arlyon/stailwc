use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`select-none`"#####, r#####"({
  userSelect: "none",
})
;"##### ; "1")]
#[test_case(r#####"tw`select-text`"#####, r#####"({
  userSelect: "text",
})
;"##### ; "2")]
#[test_case(r#####"tw`select-all`"#####, r#####"({
  userSelect: "all",
})
;"##### ; "3")]
#[test_case(r#####"tw`select-auto`"#####, r#####"({
  userSelect: "auto",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
