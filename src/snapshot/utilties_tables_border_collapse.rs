use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`border-collapse`"#####, r#####"({
  borderCollapse: "collapse",
})
;"##### ; "1")]
#[test_case(r#####"tw`border-separate`"#####, r#####"({
  borderCollapse: "separate",
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
