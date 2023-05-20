use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`break-inside-auto`"#####, r#####"({
  breakInside: "auto",
})
;"##### ; "0")]
#[test_case(r#####"tw`break-inside-avoid`"#####, r#####"({
  breakInside: "avoid",
})
;"##### ; "1")]
#[test_case(r#####"tw`break-inside-avoid-page`"#####, r#####"({
  breakInside: "avoid-page",
})
;"##### ; "2")]
#[test_case(r#####"tw`break-inside-avoid-column`"#####, r#####"({
  breakInside: "avoid-column",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
