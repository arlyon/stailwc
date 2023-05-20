use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`snap-start`"#####, r#####"({
  scrollSnapAlign: "start",
})
;"##### ; "0")]
#[test_case(r#####"tw`snap-end`"#####, r#####"({
  scrollSnapAlign: "end",
})
;"##### ; "1")]
#[test_case(r#####"tw`snap-center`"#####, r#####"({
  scrollSnapAlign: "center",
})
;"##### ; "2")]
#[test_case(r#####"tw`snap-align-none`"#####, r#####"({
  scrollSnapAlign: "none",
})"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
