use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`resize-none`"#####, r#####"({
  resize: "none",
})
;"##### ; "1")]
#[test_case(r#####"tw`resize-y`"#####, r#####"({
  resize: "vertical",
})
;"##### ; "2")]
#[test_case(r#####"tw`resize-x`"#####, r#####"({
  resize: "horizontal",
})
;"##### ; "3")]
#[test_case(r#####"tw`resize`"#####, r#####"({
  resize: "both",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
