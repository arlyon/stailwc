use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`clear-left`"#####, r#####"({
  clear: "left",
})
;"##### ; "1")]
#[test_case(r#####"tw`clear-right`"#####, r#####"({
  clear: "right",
})
;"##### ; "2")]
#[test_case(r#####"tw`clear-both`"#####, r#####"({
  clear: "both",
})
;"##### ; "3")]
#[test_case(r#####"tw`clear-none`"#####, r#####"({
  clear: "none",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
