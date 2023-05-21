use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`divide-solid`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    borderStyle: "solid",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`divide-dashed`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    borderStyle: "dashed",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`divide-dotted`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    borderStyle: "dotted",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`divide-double`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    borderStyle: "double",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`divide-none`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    borderStyle: "none",
  },
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
