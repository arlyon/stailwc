use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`border-t`"#####, r#####"({
  borderTopWidth: "1px",
})
;"##### ; "0")]
#[test_case(r#####"tw`border-r`"#####, r#####"({
  borderRightWidth: "1px",
})
;"##### ; "1")]
#[test_case(r#####"tw`border-b`"#####, r#####"({
  borderBottomWidth: "1px",
})
;"##### ; "2")]
#[test_case(r#####"tw`border-l`"#####, r#####"({
  borderLeftWidth: "1px",
})
;"##### ; "3")]
#[test_case(r#####"tw`border-t-transparent`"#####, r#####"({
  borderTopColor: "transparent",
})
;"##### ; "4")]
#[test_case(r#####"tw`border-t-current`"#####, r#####"({
  borderTopColor: "currentColor",
})
;"##### ; "5")]
#[test_case(r#####"tw`border-t-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "6")]
#[test_case(r#####"tw`border-r-transparent`"#####, r#####"({
  borderRightColor: "transparent",
})
;"##### ; "7")]
#[test_case(r#####"tw`border-r-current`"#####, r#####"({
  borderRightColor: "currentColor",
})
;"##### ; "8")]
#[test_case(r#####"tw`border-r-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "9")]
#[test_case(r#####"tw`border-b-transparent`"#####, r#####"({
  borderBottomColor: "transparent",
})
;"##### ; "10")]
#[test_case(r#####"tw`border-b-current`"#####, r#####"({
  borderBottomColor: "currentColor",
})
;"##### ; "11")]
#[test_case(r#####"tw`border-b-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "12")]
#[test_case(r#####"tw`border-l-transparent`"#####, r#####"({
  borderLeftColor: "transparent",
})
;"##### ; "13")]
#[test_case(r#####"tw`border-l-current`"#####, r#####"({
  borderLeftColor: "currentColor",
})
;"##### ; "14")]
#[test_case(r#####"tw`border-l-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "15")]
#[test_case(r#####"tw`border-l-gray-50/20`"#####, r#####"({
  borderLeftColor: "rgb(249 250 251 / 0.2)",
})
;"##### ; "16")]
#[test_case(r#####"tw`border-t-4`"#####, r#####"({
  borderTopWidth: "4px",
})
;"##### ; "17")]
#[test_case(r#####"tw`border-r-4`"#####, r#####"({
  borderRightWidth: "4px",
})
;"##### ; "18")]
#[test_case(r#####"tw`border-b-4`"#####, r#####"({
  borderBottomWidth: "4px",
})
;"##### ; "19")]
#[test_case(r#####"tw`border-l-4`"#####, r#####"({
  borderLeftWidth: "4px",
})"##### ; "20")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
