use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`table-auto`"#####, r#####"({
  tableLayout: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`table-fixed`"#####, r#####"({
  tableLayout: "fixed",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
