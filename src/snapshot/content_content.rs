use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`content`"#####, r#####"({
  none: "none",
  DEFAULT: ""default"",
  test: ""hi"",
})
;"##### ; "0")]
#[test_case(r#####"tw`content`"#####, r#####"({
  "--tw-content": ""default"",
  content: "var(--tw-content)",
})
;"##### ; "1")]
#[test_case(r#####"tw`content-test`"#####, r#####"({
  "--tw-content": ""hi"",
  content: "var(--tw-content)",
})
;"##### ; "2")]
#[test_case(r#####"tw`content-['hello']`"#####, r#####"({
  "--tw-content": "'hello'",
  content: "var(--tw-content)",
})
;"##### ; "3")]
#[test_case(r#####"tw`content-[attr(content-before)]`"#####, r#####"({
  "--tw-content": "attr(content-before)",
  content: "var(--tw-content)",
})
;"##### ; "4")]
#[test_case(r#####"tw`content-['>']`"#####, r#####"({
  "--tw-content": "'>'",
  content: "var(--tw-content)",
})
;"##### ; "5")]
#[test_case(r#####"tw`content-['—']`"#####, r#####"({
  "--tw-content": "'\\u2014'",
  content: "var(--tw-content)",
})
;"##### ; "6")]
#[test_case(r#####"tw`before:content-['—']`"#####, r#####"({
  "::before": {
    "--tw-content": "'\\u2014'",
    content: "var(--tw-content)",
  },
})
;"##### ; "7")]
#[test_case(r#####"tw`before:(content-['—'] block)`"#####, r#####"({
  "::before": {
    content: "var(--tw-content)",
    display: "block",
    "--tw-content": "'\\u2014'",
  },
})
;"##### ; "8")]
#[test_case(r#####"tw`content-none`"#####, r#####"({
  "--tw-content": "none",
  content: "var(--tw-content)",
})
;"##### ; "9")]
#[test_case(r#####"tw`before:block`"#####, r#####"({
  "::before": {
    content: "var(--tw-content)",
    display: "block",
  },
})
;"##### ; "10")]
#[test_case(r#####"tw`peer-focus:before:block`"#####, r#####"({
  ".peer:focus ~ &::before": {
    content: "var(--tw-content)",
    display: "block",
  },
})"##### ; "11")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
