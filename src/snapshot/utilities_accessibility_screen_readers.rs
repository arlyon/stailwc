use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`sr-only`"#####, r#####"({
  position: "absolute",
  width: "1px",
  height: "1px",
  padding: "0",
  margin: "-1px",
  overflow: "hidden",
  clip: "rect(0, 0, 0, 0)",
  whiteSpace: "nowrap",
  borderWidth: "0",
})
;"##### ; "0")]
#[test_case(r#####"tw`not-sr-only`"#####, r#####"({
  position: "static",
  width: "auto",
  height: "auto",
  padding: "0",
  margin: "0",
  overflow: "visible",
  clip: "auto",
  whiteSpace: "normal",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
