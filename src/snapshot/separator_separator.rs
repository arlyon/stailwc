use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`[&[data-foo][data-bar]:not([data-baz])]__underline`"#####, r#####"({
  '&[data-foo][data-bar]:not([data-baz])': {
    textDecorationLine: "underline",
  },
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
