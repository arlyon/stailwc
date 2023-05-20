use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw.div`block`
;"#####, r#####"_styled.div({
  display: "block",
})

;"##### ; "0")]
#[test_case(r#####"<div tw="block" />"#####, r#####"<div
  css={{
    display: "block",
  }}
/>"##### ; "1")]
#[test_case(r#####"const Test = tw.div``
;"#####, r#####"const Test = _styled.div({})

;"##### ; "2")]
#[test_case(r#####"<Test tw="block" />"#####, r#####"<Test
  css={{
    display: "block",
  }}
/>"##### ; "3")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
