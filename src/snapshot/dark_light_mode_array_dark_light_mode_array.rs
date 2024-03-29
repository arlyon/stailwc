use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`dark:block`"#####, r#####"({
  ":is(.test-dark &)": {
    display: "block",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`light:block`"#####, r#####"({
  ".test-light &": {
    display: "block",
  },
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
