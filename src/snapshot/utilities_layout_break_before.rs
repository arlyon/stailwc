use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`break-before-auto`"#####, r#####"({
  breakBefore: "auto",
})
;"##### ; "0")]
#[test_case(r#####"tw`break-before-avoid`"#####, r#####"({
  breakBefore: "avoid",
})
;"##### ; "1")]
#[test_case(r#####"tw`break-before-all`"#####, r#####"({
  breakBefore: "all",
})
;"##### ; "2")]
#[test_case(r#####"tw`break-before-avoid-page`"#####, r#####"({
  breakBefore: "avoid-page",
})
;"##### ; "3")]
#[test_case(r#####"tw`break-before-page`"#####, r#####"({
  breakBefore: "page",
})
;"##### ; "4")]
#[test_case(r#####"tw`break-before-left`"#####, r#####"({
  breakBefore: "left",
})
;"##### ; "5")]
#[test_case(r#####"tw`break-before-right`"#####, r#####"({
  breakBefore: "right",
})
;"##### ; "6")]
#[test_case(r#####"tw`break-before-column`"#####, r#####"({
  breakBefore: "column",
})"##### ; "7")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
