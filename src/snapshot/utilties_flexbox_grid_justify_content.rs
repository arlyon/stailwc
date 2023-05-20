use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`justify-start`"#####, r#####"({
  justifyContent: "flex-start",
})
;"##### ; "1")]
#[test_case(r#####"tw`justify-end`"#####, r#####"({
  justifyContent: "flex-end",
})
;"##### ; "2")]
#[test_case(r#####"tw`justify-center`"#####, r#####"({
  justifyContent: "center",
})
;"##### ; "3")]
#[test_case(r#####"tw`justify-between`"#####, r#####"({
  justifyContent: "space-between",
})
;"##### ; "4")]
#[test_case(r#####"tw`justify-around`"#####, r#####"({
  justifyContent: "space-around",
})
;"##### ; "5")]
#[test_case(r#####"tw`justify-evenly`"#####, r#####"({
  justifyContent: "space-evenly",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
