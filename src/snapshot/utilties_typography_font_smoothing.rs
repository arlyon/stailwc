use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`antialiased`"#####, r#####"({
  WebkitFontSmoothing: "antialiased",
  MozOsxFontSmoothing: "grayscale",
})
;"##### ; "0")]
#[test_case(r#####"tw`subpixel-antialiased`"#####, r#####"({
  WebkitFontSmoothing: "auto",
  MozOsxFontSmoothing: "auto",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
