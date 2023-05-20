use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`antialiased`"#####, r#####"({
  WebkitFontSmoothing: "antialiased",
  MozOsxFontSmoothing: "grayscale",
})
;"##### ; "1")]
#[test_case(r#####"tw`subpixel-antialiased`"#####, r#####"({
  WebkitFontSmoothing: "auto",
  MozOsxFontSmoothing: "auto",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
