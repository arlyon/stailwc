use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`border-t`"#####, r#####"({
  borderTopWidth: "1px",
})
;"##### ; "1")]
#[test_case(r#####"tw`border-r`"#####, r#####"({
  borderRightWidth: "1px",
})
;"##### ; "2")]
#[test_case(r#####"tw`border-b`"#####, r#####"({
  borderBottomWidth: "1px",
})
;"##### ; "3")]
#[test_case(r#####"tw`border-l`"#####, r#####"({
  borderLeftWidth: "1px",
})
;"##### ; "4")]
#[test_case(r#####"tw`border-t-transparent`"#####, r#####"({
  borderTopColor: "transparent",
})
;"##### ; "5")]
#[test_case(r#####"tw`border-t-current`"#####, r#####"({
  borderTopColor: "currentColor",
})
;"##### ; "6")]
#[test_case(r#####"tw`border-t-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderTopColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "7")]
#[test_case(r#####"tw`border-r-transparent`"#####, r#####"({
  borderRightColor: "transparent",
})
;"##### ; "8")]
#[test_case(r#####"tw`border-r-current`"#####, r#####"({
  borderRightColor: "currentColor",
})
;"##### ; "9")]
#[test_case(r#####"tw`border-r-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderRightColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "10")]
#[test_case(r#####"tw`border-b-transparent`"#####, r#####"({
  borderBottomColor: "transparent",
})
;"##### ; "11")]
#[test_case(r#####"tw`border-b-current`"#####, r#####"({
  borderBottomColor: "currentColor",
})
;"##### ; "12")]
#[test_case(r#####"tw`border-b-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderBottomColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "13")]
#[test_case(r#####"tw`border-l-transparent`"#####, r#####"({
  borderLeftColor: "transparent",
})
;"##### ; "14")]
#[test_case(r#####"tw`border-l-current`"#####, r#####"({
  borderLeftColor: "currentColor",
})
;"##### ; "15")]
#[test_case(r#####"tw`border-l-gray-50`"#####, r#####"({
  '--tw-border-opacity': "1",
  borderLeftColor: "rgb(249 250 251 / var(--tw-border-opacity))",
})
;"##### ; "16")]
#[test_case(r#####"tw`border-l-gray-50/20`"#####, r#####"({
  borderLeftColor: "rgb(249 250 251 / 0.2)",
})
;"##### ; "17")]
#[test_case(r#####"tw`border-t-4`"#####, r#####"({
  borderTopWidth: "4px",
})
;"##### ; "18")]
#[test_case(r#####"tw`border-r-4`"#####, r#####"({
  borderRightWidth: "4px",
})
;"##### ; "19")]
#[test_case(r#####"tw`border-b-4`"#####, r#####"({
  borderBottomWidth: "4px",
})
;"##### ; "20")]
#[test_case(r#####"tw`border-l-4`"#####, r#####"({
  borderLeftWidth: "4px",
})"##### ; "21")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
