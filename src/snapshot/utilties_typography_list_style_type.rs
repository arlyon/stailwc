use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`listStyleType`"#####, r#####"({
  none: "none",
  disc: "disc",
  decimal: "decimal",
})
;"##### ; "0")]
#[test_case(r#####"tw`list-none`"#####, r#####"({
  listStyleType: "none",
})
;"##### ; "1")]
#[test_case(r#####"tw`list-disc`"#####, r#####"({
  listStyleType: "disc",
})
;"##### ; "2")]
#[test_case(r#####"tw`list-decimal`"#####, r#####"({
  listStyleType: "decimal",
})
;"##### ; "3")]
#[test_case(r#####"tw`list-[upper-roman]`"#####, r#####"({
  listStyleType: "upper-roman",
})
;"##### ; "4")]
#[test_case(r#####"tw`list-['1F44D']`"#####, r#####"({
  listStyleType: "'1F44D'",
})
;"##### ; "5")]
#[test_case(r#####"tw`list-[var(--value)]`"#####, r#####"({
  listStyleType: "var(--value)",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
