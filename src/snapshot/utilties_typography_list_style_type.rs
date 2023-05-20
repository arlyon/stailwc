use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`listStyleType`"#####, r#####"({
  none: "none",
  disc: "disc",
  decimal: "decimal",
})
;"##### ; "1")]
#[test_case(r#####"tw`list-none`"#####, r#####"({
  listStyleType: "none",
})
;"##### ; "2")]
#[test_case(r#####"tw`list-disc`"#####, r#####"({
  listStyleType: "disc",
})
;"##### ; "3")]
#[test_case(r#####"tw`list-decimal`"#####, r#####"({
  listStyleType: "decimal",
})
;"##### ; "4")]
#[test_case(r#####"tw`list-[upper-roman]`"#####, r#####"({
  listStyleType: "upper-roman",
})
;"##### ; "5")]
#[test_case(r#####"tw`list-['1F44D']`"#####, r#####"({
  listStyleType: "'1F44D'",
})
;"##### ; "6")]
#[test_case(r#####"tw`list-[var(--value)]`"#####, r#####"({
  listStyleType: "var(--value)",
})"##### ; "7")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
