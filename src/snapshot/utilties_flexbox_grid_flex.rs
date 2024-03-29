use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`flex`"#####, r#####"({
  1: "1 1 0%",
  auto: "1 1 auto",
  initial: "0 1 auto",
  none: "none",
})
;"##### ; "0")]
#[test_case(r#####"tw`flex-1`"#####, r#####"({
  flex: "1 1 0%",
})
;"##### ; "1")]
#[test_case(r#####"tw`flex-auto`"#####, r#####"({
  flex: "1 1 auto",
})
;"##### ; "2")]
#[test_case(r#####"tw`flex-initial`"#####, r#####"({
  flex: "0 1 auto",
})
;"##### ; "3")]
#[test_case(r#####"tw`flex-none`"#####, r#####"({
  flex: "none",
})
;"##### ; "4")]
#[test_case(r#####"tw`flex-[2 2 0%]`"#####, r#####"({
  flex: "2 2 0%",
})
;"##### ; "5")]
#[test_case(r#####"tw`flex-[var(--flex)]`"#####, r#####"({
  flex: "var(--flex)",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
