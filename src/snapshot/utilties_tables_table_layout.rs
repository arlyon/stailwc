use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`table-auto`"#####, r#####"({
  tableLayout: "auto",
})
;"##### ; "0")]
#[test_case(r#####"tw`table-fixed`"#####, r#####"({
  tableLayout: "fixed",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
