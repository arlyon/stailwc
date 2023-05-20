use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`caption-top`"#####, r#####"({
  captionSide: "top",
})
;"##### ; "1")]
#[test_case(r#####"tw`caption-bottom`"#####, r#####"({
  captionSide: "bottom",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
