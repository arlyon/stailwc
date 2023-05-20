use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`truncate`"#####, r#####"({
  overflow: "hidden",
  textOverflow: "ellipsis",
  whiteSpace: "nowrap",
})
;"##### ; "1")]
#[test_case(r#####"tw`text-ellipsis`"#####, r#####"({
  textOverflow: "ellipsis",
})
;"##### ; "2")]
#[test_case(r#####"tw`text-clip`"#####, r#####"({
  textOverflow: "clip",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
