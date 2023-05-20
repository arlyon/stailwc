use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`self-auto`"#####, r#####"({
  alignSelf: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`self-start`"#####, r#####"({
  alignSelf: "flex-start",
})
;"##### ; "2")]
#[test_case(r#####"tw`self-end`"#####, r#####"({
  alignSelf: "flex-end",
})
;"##### ; "3")]
#[test_case(r#####"tw`self-center`"#####, r#####"({
  alignSelf: "center",
})
;"##### ; "4")]
#[test_case(r#####"tw`self-stretch`"#####, r#####"({
  alignSelf: "stretch",
})
;"##### ; "5")]
#[test_case(r#####"tw`self-baseline`"#####, r#####"({
  alignSelf: "baseline",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
