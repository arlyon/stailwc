use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`grid-flow-row`"#####, r#####"({
  gridAutoFlow: "row",
})
;"##### ; "0")]
#[test_case(r#####"tw`grid-flow-col`"#####, r#####"({
  gridAutoFlow: "column",
})
;"##### ; "1")]
#[test_case(r#####"tw`grid-flow-row-dense`"#####, r#####"({
  gridAutoFlow: "row dense",
})
;"##### ; "2")]
#[test_case(r#####"tw`grid-flow-col-dense`"#####, r#####"({
  gridAutoFlow: "column dense",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
