use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`flexGrow.`"#####, r#####"({
  0: "0",
  DEFAULT: "1",
})
;"##### ; "0")]
#[test_case(r#####"tw`grow-0`"#####, r#####"({
  flexGrow: "0",
})
;"##### ; "1")]
#[test_case(r#####"tw`grow`"#####, r#####"({
  flexGrow: "1",
})
;"##### ; "2")]
#[test_case(r#####"tw`flex-grow-0`"#####, r#####"({
  flexGrow: "0",
}) // Deprecated

;"##### ; "3")]
#[test_case(r#####"tw`flex-grow`"#####, r#####"({
  flexGrow: "1",
}) // Deprecated

;"##### ; "4")]
#[test_case(r#####"tw`grow-[2]`"#####, r#####"({
  flexGrow: "2",
})
;"##### ; "5")]
#[test_case(r#####"tw`flex-grow-[var(--grow)]`"#####, r#####"({
  flexGrow: "var(--grow)",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
