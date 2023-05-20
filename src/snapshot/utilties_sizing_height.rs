use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`height`"#####, r#####"({
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
  auto: "auto",
  px: "1px",
  0.5: "0.125rem",
  1.5: "0.375rem",
  2.5: "0.625rem",
  3.5: "0.875rem",
  '1/2': "50%",
  '1/3': "33.333333%",
  '2/3': "66.666667%",
  '1/4': "25%",
  '2/4': "50%",
  '3/4': "75%",
  '1/5': "20%",
  '2/5': "40%",
  '3/5': "60%",
  '4/5': "80%",
  '1/6': "16.666667%",
  '2/6': "33.333333%",
  '3/6': "50%",
  '4/6': "66.666667%",
  '5/6': "83.333333%",
  full: "100%",
  screen: "100vh",
  min: "min-content",
  max: "max-content",
  fit: "fit-content",
})
;"##### ; "1")]
#[test_case(r#####"tw`h-0`"#####, r#####"({
  height: "0px",
})
;"##### ; "2")]
#[test_case(r#####"tw`h-px`"#####, r#####"({
  height: "1px",
})
;"##### ; "3")]
#[test_case(r#####"tw`h-0.5`"#####, r#####"({
  height: "0.125rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`h-1`"#####, r#####"({
  height: "0.25rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`h-1.5`"#####, r#####"({
  height: "0.375rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`h-2`"#####, r#####"({
  height: "0.5rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`h-2.5`"#####, r#####"({
  height: "0.625rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`h-3`"#####, r#####"({
  height: "0.75rem",
})
;"##### ; "9")]
#[test_case(r#####"tw`h-3.5`"#####, r#####"({
  height: "0.875rem",
})
;"##### ; "10")]
#[test_case(r#####"tw`h-4`"#####, r#####"({
  height: "1rem",
})
;"##### ; "11")]
#[test_case(r#####"tw`h-5`"#####, r#####"({
  height: "1.25rem",
})
;"##### ; "12")]
#[test_case(r#####"tw`h-6`"#####, r#####"({
  height: "1.5rem",
})
;"##### ; "13")]
#[test_case(r#####"tw`h-7`"#####, r#####"({
  height: "1.75rem",
})
;"##### ; "14")]
#[test_case(r#####"tw`h-8`"#####, r#####"({
  height: "2rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`h-9`"#####, r#####"({
  height: "2.25rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`h-10`"#####, r#####"({
  height: "2.5rem",
})
;"##### ; "17")]
#[test_case(r#####"tw`h-11`"#####, r#####"({
  height: "2.75rem",
})
;"##### ; "18")]
#[test_case(r#####"tw`h-12`"#####, r#####"({
  height: "3rem",
})
;"##### ; "19")]
#[test_case(r#####"tw`h-14`"#####, r#####"({
  height: "3.5rem",
})
;"##### ; "20")]
#[test_case(r#####"tw`h-16`"#####, r#####"({
  height: "4rem",
})
;"##### ; "21")]
#[test_case(r#####"tw`h-20`"#####, r#####"({
  height: "5rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`h-24`"#####, r#####"({
  height: "6rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`h-28`"#####, r#####"({
  height: "7rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`h-32`"#####, r#####"({
  height: "8rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`h-36`"#####, r#####"({
  height: "9rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`h-40`"#####, r#####"({
  height: "10rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`h-44`"#####, r#####"({
  height: "11rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`h-48`"#####, r#####"({
  height: "12rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`h-52`"#####, r#####"({
  height: "13rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`h-56`"#####, r#####"({
  height: "14rem",
})
;"##### ; "31")]
#[test_case(r#####"tw`h-60`"#####, r#####"({
  height: "15rem",
})
;"##### ; "32")]
#[test_case(r#####"tw`h-64`"#####, r#####"({
  height: "16rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`h-72`"#####, r#####"({
  height: "18rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`h-80`"#####, r#####"({
  height: "20rem",
})
;"##### ; "35")]
#[test_case(r#####"tw`h-96`"#####, r#####"({
  height: "24rem",
})
;"##### ; "36")]
#[test_case(r#####"tw`h-auto`"#####, r#####"({
  height: "auto",
})
;"##### ; "37")]
#[test_case(r#####"tw`h-1/2`"#####, r#####"({
  height: "50%",
})
;"##### ; "38")]
#[test_case(r#####"tw`h-1/3`"#####, r#####"({
  height: "33.333333%",
})
;"##### ; "39")]
#[test_case(r#####"tw`h-2/3`"#####, r#####"({
  height: "66.666667%",
})
;"##### ; "40")]
#[test_case(r#####"tw`h-1/4`"#####, r#####"({
  height: "25%",
})
;"##### ; "41")]
#[test_case(r#####"tw`h-2/4`"#####, r#####"({
  height: "50%",
})
;"##### ; "42")]
#[test_case(r#####"tw`h-3/4`"#####, r#####"({
  height: "75%",
})
;"##### ; "43")]
#[test_case(r#####"tw`h-1/5`"#####, r#####"({
  height: "20%",
})
;"##### ; "44")]
#[test_case(r#####"tw`h-2/5`"#####, r#####"({
  height: "40%",
})
;"##### ; "45")]
#[test_case(r#####"tw`h-3/5`"#####, r#####"({
  height: "60%",
})
;"##### ; "46")]
#[test_case(r#####"tw`h-4/5`"#####, r#####"({
  height: "80%",
})
;"##### ; "47")]
#[test_case(r#####"tw`h-1/6`"#####, r#####"({
  height: "16.666667%",
})
;"##### ; "48")]
#[test_case(r#####"tw`h-2/6`"#####, r#####"({
  height: "33.333333%",
})
;"##### ; "49")]
#[test_case(r#####"tw`h-3/6`"#####, r#####"({
  height: "50%",
})
;"##### ; "50")]
#[test_case(r#####"tw`h-4/6`"#####, r#####"({
  height: "66.666667%",
})
;"##### ; "51")]
#[test_case(r#####"tw`h-5/6`"#####, r#####"({
  height: "83.333333%",
})
;"##### ; "52")]
#[test_case(r#####"tw`h-full`"#####, r#####"({
  height: "100%",
})
;"##### ; "53")]
#[test_case(r#####"tw`h-screen`"#####, r#####"({
  height: "100vh",
})
;"##### ; "54")]
#[test_case(r#####"tw`h-min`"#####, r#####"({
  height: "min-content",
})
;"##### ; "55")]
#[test_case(r#####"tw`h-max`"#####, r#####"({
  height: "max-content",
})
;"##### ; "56")]
#[test_case(r#####"tw`h-fit`"#####, r#####"({
  height: "fit-content",
})
;"##### ; "57")]
#[test_case(r#####"tw`h-[32rem]`"#####, r#####"({
  height: "32rem",
})
;"##### ; "58")]
#[test_case(r#####"tw`h-[3.23rem]`"#####, r#####"({
  height: "3.23rem",
})
;"##### ; "59")]
#[test_case(r#####"tw`h-[calc(100%+1rem)]`"#####, r#####"({
  height: "calc(100% + 1rem)",
})
;"##### ; "60")]
#[test_case(r#####"tw`h-[var(--height)]`"#####, r#####"({
  height: "var(--height)",
})
;"##### ; "61")]
#[test_case(r#####"tw`h-[calc(100%-theme('spacing.16'))]`"#####, r#####"({
  height: "calc(100% - 4rem)",
})
;"##### ; "62")]
#[test_case(r#####"tw`h-[calc(100%-theme("spacing.16"))]`"#####, r#####"({
  height: "calc(100% - 4rem)",
})"##### ; "63")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
