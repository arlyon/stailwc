use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`align-baseline`"#####, r#####"({
  verticalAlign: "baseline",
})
;"##### ; "1")]
#[test_case(r#####"tw`align-top`"#####, r#####"({
  verticalAlign: "top",
})
;"##### ; "2")]
#[test_case(r#####"tw`align-middle`"#####, r#####"({
  verticalAlign: "middle",
})
;"##### ; "3")]
#[test_case(r#####"tw`align-bottom`"#####, r#####"({
  verticalAlign: "bottom",
})
;"##### ; "4")]
#[test_case(r#####"tw`align-text-top`"#####, r#####"({
  verticalAlign: "text-top",
})
;"##### ; "5")]
#[test_case(r#####"tw`align-text-bottom`"#####, r#####"({
  verticalAlign: "text-bottom",
})
;"##### ; "6")]
#[test_case(r#####"tw`align-sub`"#####, r#####"({
  verticalAlign: "sub",
})
;"##### ; "7")]
#[test_case(r#####"tw`align-super`"#####, r#####"({
  verticalAlign: "super",
})
;"##### ; "8")]
#[test_case(r#####"tw`align-[something]`"#####, r#####"({
  verticalAlign: "something",
})"##### ; "9")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
