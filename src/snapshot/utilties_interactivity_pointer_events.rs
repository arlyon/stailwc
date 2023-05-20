use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`pointer-events-none`"#####, r#####"({
  pointerEvents: "none",
})
;"##### ; "1")]
#[test_case(r#####"tw`pointer-events-auto`"#####, r#####"({
  pointerEvents: "auto",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
