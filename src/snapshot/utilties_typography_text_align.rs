use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`text-left`"#####, r#####"({
  textAlign: "left",
})
;"##### ; "0")]
#[test_case(r#####"tw`text-center`"#####, r#####"({
  textAlign: "center",
})
;"##### ; "1")]
#[test_case(r#####"tw`text-right`"#####, r#####"({
  textAlign: "right",
})
;"##### ; "2")]
#[test_case(r#####"tw`text-justify`"#####, r#####"({
  textAlign: "justify",
})
;"##### ; "3")]
#[test_case(r#####"tw`text-start`"#####, r#####"({
  textAlign: "start",
})
;"##### ; "4")]
#[test_case(r#####"tw`text-end`"#####, r#####"({
  textAlign: "end",
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
