use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`maxHeight`"#####, r#####"({
  0: "0px",
  1: "0.25rem",
  2: "0.5rem",
  3: "0.75rem",
  4: "1rem",
  5: "1.25rem",
  6: "1.5rem",
  7: "1.75rem",
  8: "2rem",
  9: "2.25rem",
  10: "2.5rem",
  11: "2.75rem",
  12: "3rem",
  14: "3.5rem",
  16: "4rem",
  20: "5rem",
  24: "6rem",
  28: "7rem",
  32: "8rem",
  36: "9rem",
  40: "10rem",
  44: "11rem",
  48: "12rem",
  52: "13rem",
  56: "14rem",
  60: "15rem",
  64: "16rem",
  72: "18rem",
  80: "20rem",
  96: "24rem",
  px: "1px",
  0.5: "0.125rem",
  1.5: "0.375rem",
  2.5: "0.625rem",
  3.5: "0.875rem",
  none: "none",
  full: "100%",
  screen: "100vh",
  min: "min-content",
  max: "max-content",
  fit: "fit-content",
})
;"##### ; "0")]
#[test_case(r#####"tw`max-h-0`"#####, r#####"({
  maxHeight: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`max-h-0.5`"#####, r#####"({
  maxHeight: "0.125rem",
})
;"##### ; "2")]
#[test_case(r#####"tw`max-h-1`"#####, r#####"({
  maxHeight: "0.25rem",
})
;"##### ; "3")]
#[test_case(r#####"tw`max-h-1.5`"#####, r#####"({
  maxHeight: "0.375rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`max-h-2`"#####, r#####"({
  maxHeight: "0.5rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`max-h-2.5`"#####, r#####"({
  maxHeight: "0.625rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`max-h-3`"#####, r#####"({
  maxHeight: "0.75rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`max-h-3.5`"#####, r#####"({
  maxHeight: "0.875rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`max-h-none`"#####, r#####"({
  maxHeight: "none",
})
;"##### ; "9")]
#[test_case(r#####"tw`max-h-4`"#####, r#####"({
  maxHeight: "1rem",
})
;"##### ; "10")]
#[test_case(r#####"tw`max-h-5`"#####, r#####"({
  maxHeight: "1.25rem",
})
;"##### ; "11")]
#[test_case(r#####"tw`max-h-6`"#####, r#####"({
  maxHeight: "1.5rem",
})
;"##### ; "12")]
#[test_case(r#####"tw`max-h-7`"#####, r#####"({
  maxHeight: "1.75rem",
})
;"##### ; "13")]
#[test_case(r#####"tw`max-h-8`"#####, r#####"({
  maxHeight: "2rem",
})
;"##### ; "14")]
#[test_case(r#####"tw`max-h-9`"#####, r#####"({
  maxHeight: "2.25rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`max-h-10`"#####, r#####"({
  maxHeight: "2.5rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`max-h-11`"#####, r#####"({
  maxHeight: "2.75rem",
})
;"##### ; "17")]
#[test_case(r#####"tw`max-h-12`"#####, r#####"({
  maxHeight: "3rem",
})
;"##### ; "18")]
#[test_case(r#####"tw`max-h-14`"#####, r#####"({
  maxHeight: "3.5rem",
})
;"##### ; "19")]
#[test_case(r#####"tw`max-h-16`"#####, r#####"({
  maxHeight: "4rem",
})
;"##### ; "20")]
#[test_case(r#####"tw`max-h-20`"#####, r#####"({
  maxHeight: "5rem",
})
;"##### ; "21")]
#[test_case(r#####"tw`max-h-24`"#####, r#####"({
  maxHeight: "6rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`max-h-28`"#####, r#####"({
  maxHeight: "7rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`max-h-32`"#####, r#####"({
  maxHeight: "8rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`max-h-36`"#####, r#####"({
  maxHeight: "9rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`max-h-40`"#####, r#####"({
  maxHeight: "10rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`max-h-44`"#####, r#####"({
  maxHeight: "11rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`max-h-48`"#####, r#####"({
  maxHeight: "12rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`max-h-52`"#####, r#####"({
  maxHeight: "13rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`max-h-56`"#####, r#####"({
  maxHeight: "14rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`max-h-60`"#####, r#####"({
  maxHeight: "15rem",
})
;"##### ; "31")]
#[test_case(r#####"tw`max-h-64`"#####, r#####"({
  maxHeight: "16rem",
})
;"##### ; "32")]
#[test_case(r#####"tw`max-h-72`"#####, r#####"({
  maxHeight: "18rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`max-h-80`"#####, r#####"({
  maxHeight: "20rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`max-h-96`"#####, r#####"({
  maxHeight: "24rem",
})
;"##### ; "35")]
#[test_case(r#####"tw`max-h-px`"#####, r#####"({
  maxHeight: "1px",
})
;"##### ; "36")]
#[test_case(r#####"tw`max-h-full`"#####, r#####"({
  maxHeight: "100%",
})
;"##### ; "37")]
#[test_case(r#####"tw`max-h-screen`"#####, r#####"({
  maxHeight: "100vh",
})
;"##### ; "38")]
#[test_case(r#####"tw`max-h-min`"#####, r#####"({
  maxHeight: "min-content",
})
;"##### ; "39")]
#[test_case(r#####"tw`max-h-max`"#####, r#####"({
  maxHeight: "max-content",
})
;"##### ; "40")]
#[test_case(r#####"tw`max-h-fit`"#####, r#####"({
  maxHeight: "fit-content",
})
;"##### ; "41")]
#[test_case(r#####"tw`max-h-[32rem]`"#####, r#####"({
  maxHeight: "32rem",
})"##### ; "42")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
