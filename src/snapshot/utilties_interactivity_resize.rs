use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`resize-none`"#####, r#####"({
  resize: "none",
})
;"##### ; "0")]
#[test_case(r#####"tw`resize-y`"#####, r#####"({
  resize: "vertical",
})
;"##### ; "1")]
#[test_case(r#####"tw`resize-x`"#####, r#####"({
  resize: "horizontal",
})
;"##### ; "2")]
#[test_case(r#####"tw`resize`"#####, r#####"({
  resize: "both",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
