use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`break-after-auto`"#####, r#####"({
  breakAfter: "auto",
})
;"##### ; "0")]
#[test_case(r#####"tw`break-after-avoid`"#####, r#####"({
  breakAfter: "avoid",
})
;"##### ; "1")]
#[test_case(r#####"tw`break-after-all`"#####, r#####"({
  breakAfter: "all",
})
;"##### ; "2")]
#[test_case(r#####"tw`break-after-avoid-page`"#####, r#####"({
  breakAfter: "avoid-page",
})
;"##### ; "3")]
#[test_case(r#####"tw`break-after-page`"#####, r#####"({
  breakAfter: "page",
})
;"##### ; "4")]
#[test_case(r#####"tw`break-after-left`"#####, r#####"({
  breakAfter: "left",
})
;"##### ; "5")]
#[test_case(r#####"tw`break-after-right`"#####, r#####"({
  breakAfter: "right",
})
;"##### ; "6")]
#[test_case(r#####"tw`break-after-column`"#####, r#####"({
  breakAfter: "column",
})"##### ; "7")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
