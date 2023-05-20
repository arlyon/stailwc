use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`lineHeight`"#####, r#####"({
  3: ".75rem",
  4: "1rem",
  5: "1.25rem",
  6: "1.5rem",
  7: "1.75rem",
  8: "2rem",
  9: "2.25rem",
  10: "2.5rem",
  none: "1",
  tight: "1.25",
  snug: "1.375",
  normal: "1.5",
  relaxed: "1.625",
  loose: "2",
})
;"##### ; "0")]
#[test_case(r#####"tw`leading-3`"#####, r#####"({
  lineHeight: ".75rem",
})
;"##### ; "1")]
#[test_case(r#####"tw`leading-4`"#####, r#####"({
  lineHeight: "1rem",
})
;"##### ; "2")]
#[test_case(r#####"tw`leading-5`"#####, r#####"({
  lineHeight: "1.25rem",
})
;"##### ; "3")]
#[test_case(r#####"tw`leading-6`"#####, r#####"({
  lineHeight: "1.5rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`leading-7`"#####, r#####"({
  lineHeight: "1.75rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`leading-8`"#####, r#####"({
  lineHeight: "2rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`leading-9`"#####, r#####"({
  lineHeight: "2.25rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`leading-10`"#####, r#####"({
  lineHeight: "2.5rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`leading-none`"#####, r#####"({
  lineHeight: "1",
})
;"##### ; "9")]
#[test_case(r#####"tw`leading-tight`"#####, r#####"({
  lineHeight: "1.25",
})
;"##### ; "10")]
#[test_case(r#####"tw`leading-snug`"#####, r#####"({
  lineHeight: "1.375",
})
;"##### ; "11")]
#[test_case(r#####"tw`leading-normal`"#####, r#####"({
  lineHeight: "1.5",
})
;"##### ; "12")]
#[test_case(r#####"tw`leading-relaxed`"#####, r#####"({
  lineHeight: "1.625",
})
;"##### ; "13")]
#[test_case(r#####"tw`leading-loose`"#####, r#####"({
  lineHeight: "2",
})
;"##### ; "14")]
#[test_case(r#####"tw`leading-[3rem]`"#####, r#####"({
  lineHeight: "3rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`leading-[var(--leading)]`"#####, r#####"({
  lineHeight: "var(--leading)",
})"##### ; "16")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
