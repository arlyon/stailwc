use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`flexShrink.`"#####, r#####"({
  0: "0",
  DEFAULT: "1",
})
;"##### ; "0")]
#[test_case(r#####"tw`shrink-0`"#####, r#####"({
  flexShrink: "0",
})
;"##### ; "1")]
#[test_case(r#####"tw`shrink`"#####, r#####"({
  flexShrink: "1",
})
;"##### ; "2")]
#[test_case(r#####"tw`flex-shrink-0`"#####, r#####"({
  flexShrink: "0",
}) // Deprecated

;"##### ; "3")]
#[test_case(r#####"tw`flex-shrink`"#####, r#####"({
  flexShrink: "1",
}) // Deprecated

;"##### ; "4")]
#[test_case(r#####"tw`flex-shrink-[var(--shrink)]`"#####, r#####"({
  flexShrink: "var(--shrink)",
}) // Deprecated

;"##### ; "5")]
#[test_case(r#####"tw`shrink-[var(--shrink)]`"#####, r#####"({
  flexShrink: "var(--shrink)",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
