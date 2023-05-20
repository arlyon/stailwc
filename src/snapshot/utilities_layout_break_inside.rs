use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`break-inside-auto`"#####, r#####"({
  breakInside: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`break-inside-avoid`"#####, r#####"({
  breakInside: "avoid",
})
;"##### ; "2")]
#[test_case(r#####"tw`break-inside-avoid-page`"#####, r#####"({
  breakInside: "avoid-page",
})
;"##### ; "3")]
#[test_case(r#####"tw`break-inside-avoid-column`"#####, r#####"({
  breakInside: "avoid-column",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
