use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`static`"#####, r#####"({
  position: "static",
})
;"##### ; "1")]
#[test_case(r#####"tw`fixed`"#####, r#####"({
  position: "fixed",
})
;"##### ; "2")]
#[test_case(r#####"tw`absolute`"#####, r#####"({
  position: "absolute",
})
;"##### ; "3")]
#[test_case(r#####"tw`relative`"#####, r#####"({
  position: "relative",
})
;"##### ; "4")]
#[test_case(r#####"tw`sticky`"#####, r#####"({
  position: "sticky",
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
