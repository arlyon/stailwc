use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`fontSize`"#####, r#####"({
  xs: [
    "0.75rem",
    {
      lineHeight: "1rem",
    },
  ],
  sm: [
    "0.875rem",
    {
      lineHeight: "1.25rem",
    },
  ],
  base: [
    "1rem",
    {
      lineHeight: "1.5rem",
    },
  ],
  lg: [
    "1.125rem",
    {
      lineHeight: "1.75rem",
    },
  ],
  xl: [
    "1.25rem",
    {
      lineHeight: "1.75rem",
    },
  ],
  "2xl": [
    "24px",
    {
      letterSpacing: "-0.01em",
    },
  ],
  "3xl": [
    "32px",
    {
      letterSpacing: "-0.02em",
      lineHeight: "40px",
    },
  ],
  "4xl": [
    "2.25rem",
    {
      lineHeight: "2.5rem",
    },
  ],
  "5xl": [
    "3rem",
    {
      lineHeight: "1",
    },
  ],
  "6xl": [
    "3.75rem",
    {
      lineHeight: "1",
    },
  ],
  "7xl": [
    "4.5rem",
    {
      lineHeight: "1",
    },
  ],
  "8xl": [
    "6rem",
    {
      lineHeight: "1",
    },
  ],
  "9xl": [
    "8rem",
    {
      lineHeight: "1",
    },
  ],
})
;"##### ; "0")]
#[test_case(r#####"tw`text-xs`"#####, r#####"({
  fontSize: "0.75rem",
  lineHeight: "1rem",
})
;"##### ; "1")]
#[test_case(r#####"tw`text-sm`"#####, r#####"({
  fontSize: "0.875rem",
  lineHeight: "1.25rem",
})
;"##### ; "2")]
#[test_case(r#####"tw`text-base`"#####, r#####"({
  fontSize: "1rem",
  lineHeight: "1.5rem",
})
;"##### ; "3")]
#[test_case(r#####"tw`text-lg`"#####, r#####"({
  fontSize: "1.125rem",
  lineHeight: "1.75rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`text-xl`"#####, r#####"({
  fontSize: "1.25rem",
  lineHeight: "1.75rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`text-2xl`"#####, r#####"({
  fontSize: "24px",
  letterSpacing: "-0.01em",
})
;"##### ; "6")]
#[test_case(r#####"tw`text-3xl`"#####, r#####"({
  fontSize: "32px",
  lineHeight: "40px",
  letterSpacing: "-0.02em",
})
;"##### ; "7")]
#[test_case(r#####"tw`text-4xl`"#####, r#####"({
  fontSize: "2.25rem",
  lineHeight: "2.5rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`text-5xl`"#####, r#####"({
  fontSize: "3rem",
  lineHeight: "1",
})
;"##### ; "9")]
#[test_case(r#####"tw`text-6xl`"#####, r#####"({
  fontSize: "3.75rem",
  lineHeight: "1",
})
;"##### ; "10")]
#[test_case(r#####"tw`text-7xl`"#####, r#####"({
  fontSize: "4.5rem",
  lineHeight: "1",
})
;"##### ; "11")]
#[test_case(r#####"tw`text-8xl`"#####, r#####"({
  fontSize: "6rem",
  lineHeight: "1",
})
;"##### ; "12")]
#[test_case(r#####"tw`text-9xl`"#####, r#####"({
  fontSize: "8rem",
  lineHeight: "1",
})
;"##### ; "13")]
#[test_case(r#####"tw`text-[2.23rem]`"#####, r#####"({
  fontSize: "2.23rem",
})
;"##### ; "14")]
#[test_case(r#####"tw`text-[length:var(--font-size)]`"#####, r#####"({
  fontSize: "var(--font-size)",
})
;"##### ; "15")]
#[test_case(r#####"tw`text-2xl`"#####, r#####"({
  fontSize: "24px",
  letterSpacing: "-0.01em",
})
;"##### ; "16")]
#[test_case(r#####"tw`text-3xl`"#####, r#####"({
  fontSize: "32px",
  lineHeight: "40px",
  letterSpacing: "-0.02em",
})"##### ; "17")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
