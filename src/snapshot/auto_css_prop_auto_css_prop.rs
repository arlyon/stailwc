use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"<div css="" />
;"#####, r#####"<div css="" />
;"##### ; "0")]
#[test_case(r#####"<div className="" />"#####, r#####"<div className="" />"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
