use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`css-class-with-variable-as-rule-property`"#####, r#####"({
  '--some-css-variable-as-rule-prop': "blue",
}) // Test negative css variables

;"##### ; "1")]
#[test_case(r#####"tw`-mx-gutter-1/2`"#####, r#####"({
  marginLeft: "calc(var(--gutter-half) * -1)",
  marginRight: "calc(var(--gutter-half) * -1)",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
