use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import '../macro' // twinImport

// Css prop isn't handled by twin
;"#####, r#####";"##### ; "0")]
#[test_case(r#####"<div css="" />
;"#####, r#####"<div css="" />
;"##### ; "1")]
#[test_case(r#####"<div className="" />"#####, r#####"<div className="" />"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
