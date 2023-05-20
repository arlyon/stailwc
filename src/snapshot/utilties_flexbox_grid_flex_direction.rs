use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`flex-row`"#####, r#####"({
  flexDirection: "row",
})
;"##### ; "1")]
#[test_case(r#####"tw`flex-row-reverse`"#####, r#####"({
  flexDirection: "row-reverse",
})
;"##### ; "2")]
#[test_case(r#####"tw`flex-col`"#####, r#####"({
  flexDirection: "column",
})
;"##### ; "3")]
#[test_case(r#####"tw`flex-col-reverse`"#####, r#####"({
  flexDirection: "column-reverse",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
