use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`static`"#####, r#####"({
  position: "static",
})
;"##### ; "0")]
#[test_case(r#####"tw`fixed`"#####, r#####"({
  position: "fixed",
})
;"##### ; "1")]
#[test_case(r#####"tw`absolute`"#####, r#####"({
  position: "absolute",
})
;"##### ; "2")]
#[test_case(r#####"tw`relative`"#####, r#####"({
  position: "relative",
})
;"##### ; "3")]
#[test_case(r#####"tw`sticky`"#####, r#####"({
  position: "sticky",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
