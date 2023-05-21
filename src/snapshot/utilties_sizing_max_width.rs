use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`maxWidth`"#####, r#####"({
  0: "0rem",
  none: "none",
  xs: "20rem",
  sm: "24rem",
  md: "28rem",
  lg: "32rem",
  xl: "36rem",
  "2xl": "42rem",
  "3xl": "48rem",
  "4xl": "56rem",
  "5xl": "64rem",
  "6xl": "72rem",
  "7xl": "80rem",
  full: "100%",
  min: "min-content",
  max: "max-content",
  fit: "fit-content",
  prose: "65ch",
  "screen-sm": "640px",
  "screen-md": "768px",
  "screen-lg": "1024px",
  "screen-xl": "1280px",
  "screen-2xl": "1536px",
})
;"##### ; "0")]
#[test_case(r#####"tw`max-w-0`"#####, r#####"({
  maxWidth: "0rem",
})
;"##### ; "1")]
#[test_case(r#####"tw`max-w-none`"#####, r#####"({
  maxWidth: "none",
})
;"##### ; "2")]
#[test_case(r#####"tw`max-w-xs`"#####, r#####"({
  maxWidth: "20rem",
})
;"##### ; "3")]
#[test_case(r#####"tw`max-w-sm`"#####, r#####"({
  maxWidth: "24rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`max-w-md`"#####, r#####"({
  maxWidth: "28rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`max-w-lg`"#####, r#####"({
  maxWidth: "32rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`max-w-xl`"#####, r#####"({
  maxWidth: "36rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`max-w-2xl`"#####, r#####"({
  maxWidth: "42rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`max-w-3xl`"#####, r#####"({
  maxWidth: "48rem",
})
;"##### ; "9")]
#[test_case(r#####"tw`max-w-4xl`"#####, r#####"({
  maxWidth: "56rem",
})
;"##### ; "10")]
#[test_case(r#####"tw`max-w-5xl`"#####, r#####"({
  maxWidth: "64rem",
})
;"##### ; "11")]
#[test_case(r#####"tw`max-w-6xl`"#####, r#####"({
  maxWidth: "72rem",
})
;"##### ; "12")]
#[test_case(r#####"tw`max-w-7xl`"#####, r#####"({
  maxWidth: "80rem",
})
;"##### ; "13")]
#[test_case(r#####"tw`max-w-full`"#####, r#####"({
  maxWidth: "100%",
})
;"##### ; "14")]
#[test_case(r#####"tw`max-w-min`"#####, r#####"({
  maxWidth: "min-content",
})
;"##### ; "15")]
#[test_case(r#####"tw`max-w-max`"#####, r#####"({
  maxWidth: "max-content",
})
;"##### ; "16")]
#[test_case(r#####"tw`max-w-fit`"#####, r#####"({
  maxWidth: "fit-content",
})
;"##### ; "17")]
#[test_case(r#####"tw`max-w-prose`"#####, r#####"({
  maxWidth: "65ch",
})
;"##### ; "18")]
#[test_case(r#####"tw`max-w-screen-sm`"#####, r#####"({
  maxWidth: "640px",
})
;"##### ; "19")]
#[test_case(r#####"tw`max-w-screen-md`"#####, r#####"({
  maxWidth: "768px",
})
;"##### ; "20")]
#[test_case(r#####"tw`max-w-screen-lg`"#####, r#####"({
  maxWidth: "1024px",
})
;"##### ; "21")]
#[test_case(r#####"tw`max-w-screen-xl`"#####, r#####"({
  maxWidth: "1280px",
})
;"##### ; "22")]
#[test_case(r#####"tw`max-w-screen-2xl`"#####, r#####"({
  maxWidth: "1536px",
})
;"##### ; "23")]
#[test_case(r#####"tw`max-w-[50%]`"#####, r#####"({
  maxWidth: "50%",
})"##### ; "24")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
