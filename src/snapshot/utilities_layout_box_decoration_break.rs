use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`decoration-clone`"#####, r#####"({
  boxDecorationBreak: "clone",
})
;"##### ; "0")]
#[test_case(r#####"tw`decoration-slice`"#####, r#####"({
  boxDecorationBreak: "slice",
})
;"##### ; "1")]
#[test_case(r#####"tw`box-decoration-clone`"#####, r#####"({
  boxDecorationBreak: "clone",
})
;"##### ; "2")]
#[test_case(r#####"tw`box-decoration-slice`"#####, r#####"({
  boxDecorationBreak: "slice",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
