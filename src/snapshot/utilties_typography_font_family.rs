use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`font-sans`"#####, r#####"({
  fontFamily:
    "ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"",
})
;"##### ; "1")]
#[test_case(r#####"tw`font-serif`"#####, r#####"({
  fontFamily: "ui-serif, Georgia, Cambria, "Times New Roman", Times, serif",
})
;"##### ; "2")]
#[test_case(r#####"tw`font-mono`"#####, r#####"({
  fontFamily:
    "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace",
})
;"##### ; "3")]
#[test_case(r#####"tw`font-custom`"#####, r#####"({
  fontFamily: "Inter var, sans-serif",
  fontFeatureSettings: ""cv11", "ss01"",
  fontVariationSettings: ""opsz" 32",
})
;"##### ; "4")]
#[test_case(r#####"tw`font-['Open Sans']`"#####, r#####"({
  fontFamily: "'Open Sans'",
})
;"##### ; "5")]
#[test_case(r#####"tw`font-[generic-name:fantasy]`"#####, r#####"({
  fontFamily: "fantasy",
})
;"##### ; "6")]
#[test_case(r#####"tw`font-[family-name:'this and that', this]`"#####, r#####"({
  fontFamily: "'this and that', this",
})"##### ; "7")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
