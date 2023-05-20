use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw.div`block`"#####, r#####"_styled("div", {
  display: "block",
})"##### ; "0")]
#[test_case(r#####"styled.div(tw`block`)"#####, r#####"_styled("div", {
  display: "block",
})"##### ; "1")]
#[test_case(r#####"styled.div({ display: "block" })"#####, r#####"_styled("div", {
  display: "block",
})"##### ; "2")]
#[test_case(r#####"styled("div", tw`block`)"#####, r#####"_styled("div", {
  display: "block",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
