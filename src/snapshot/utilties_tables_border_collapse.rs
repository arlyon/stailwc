use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`border-collapse`"#####, r#####"({
  borderCollapse: "collapse",
})
;"##### ; "0")]
#[test_case(r#####"tw`border-separate`"#####, r#####"({
  borderCollapse: "separate",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
