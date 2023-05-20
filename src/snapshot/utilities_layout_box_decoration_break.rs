use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`decoration-clone`"#####, r#####"({
  boxDecorationBreak: "clone",
})
;"##### ; "1")]
#[test_case(r#####"tw`decoration-slice`"#####, r#####"({
  boxDecorationBreak: "slice",
})
;"##### ; "2")]
#[test_case(r#####"tw`box-decoration-clone`"#####, r#####"({
  boxDecorationBreak: "clone",
})
;"##### ; "3")]
#[test_case(r#####"tw`box-decoration-slice`"#####, r#####"({
  boxDecorationBreak: "slice",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
