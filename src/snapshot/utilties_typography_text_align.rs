use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`text-left`"#####, r#####"({
  textAlign: "left",
})
;"##### ; "1")]
#[test_case(r#####"tw`text-center`"#####, r#####"({
  textAlign: "center",
})
;"##### ; "2")]
#[test_case(r#####"tw`text-right`"#####, r#####"({
  textAlign: "right",
})
;"##### ; "3")]
#[test_case(r#####"tw`text-justify`"#####, r#####"({
  textAlign: "justify",
})
;"##### ; "4")]
#[test_case(r#####"tw`text-start`"#####, r#####"({
  textAlign: "start",
})
;"##### ; "5")]
#[test_case(r#####"tw`text-end`"#####, r#####"({
  textAlign: "end",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
