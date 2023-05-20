use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`items-start`"#####, r#####"({
  alignItems: "flex-start",
})
;"##### ; "1")]
#[test_case(r#####"tw`items-end`"#####, r#####"({
  alignItems: "flex-end",
})
;"##### ; "2")]
#[test_case(r#####"tw`items-center`"#####, r#####"({
  alignItems: "center",
})
;"##### ; "3")]
#[test_case(r#####"tw`items-baseline`"#####, r#####"({
  alignItems: "baseline",
})
;"##### ; "4")]
#[test_case(r#####"tw`items-stretch`"#####, r#####"({
  alignItems: "stretch",
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
