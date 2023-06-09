use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`width`"#####, r#####"({
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
  "1/2": "50%",
  "1/3": "33.333333%",
  "2/3": "66.666667%",
  "1/4": "25%",
  "2/4": "50%",
  "3/4": "75%",
  "1/5": "20%",
  "2/5": "40%",
  "3/5": "60%",
  "4/5": "80%",
  "1/6": "16.666667%",
  "2/6": "33.333333%",
  "3/6": "50%",
  "4/6": "66.666667%",
  "5/6": "83.333333%",
  "1/12": "8.333333%",
  "2/12": "16.666667%",
  "3/12": "25%",
  "4/12": "33.333333%",
  "5/12": "41.666667%",
  "6/12": "50%",
  "7/12": "58.333333%",
  "8/12": "66.666667%",
  "9/12": "75%",
  "10/12": "83.333333%",
  "11/12": "91.666667%",
  full: "100%",
  screen: "100vw",
  min: "min-content",
  max: "max-content",
  fit: "fit-content",
})
;"##### ; "0")]
#[test_case(r#####"tw`w-0`"#####, r#####"({
  width: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`w-px`"#####, r#####"({
  width: "1px",
})
;"##### ; "2")]
#[test_case(r#####"tw`w-0.5`"#####, r#####"({
  width: "0.125rem",
})
;"##### ; "3")]
#[test_case(r#####"tw`w-1`"#####, r#####"({
  width: "0.25rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`w-1.5`"#####, r#####"({
  width: "0.375rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`w-2`"#####, r#####"({
  width: "0.5rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`w-2.5`"#####, r#####"({
  width: "0.625rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`w-3`"#####, r#####"({
  width: "0.75rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`w-3.5`"#####, r#####"({
  width: "0.875rem",
})
;"##### ; "9")]
#[test_case(r#####"tw`w-4`"#####, r#####"({
  width: "1rem",
})
;"##### ; "10")]
#[test_case(r#####"tw`w-5`"#####, r#####"({
  width: "1.25rem",
})
;"##### ; "11")]
#[test_case(r#####"tw`w-6`"#####, r#####"({
  width: "1.5rem",
})
;"##### ; "12")]
#[test_case(r#####"tw`w-7`"#####, r#####"({
  width: "1.75rem",
})
;"##### ; "13")]
#[test_case(r#####"tw`w-8`"#####, r#####"({
  width: "2rem",
})
;"##### ; "14")]
#[test_case(r#####"tw`w-9`"#####, r#####"({
  width: "2.25rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`w-10`"#####, r#####"({
  width: "2.5rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`w-11`"#####, r#####"({
  width: "2.75rem",
})
;"##### ; "17")]
#[test_case(r#####"tw`w-12`"#####, r#####"({
  width: "3rem",
})
;"##### ; "18")]
#[test_case(r#####"tw`w-14`"#####, r#####"({
  width: "3.5rem",
})
;"##### ; "19")]
#[test_case(r#####"tw`w-16`"#####, r#####"({
  width: "4rem",
})
;"##### ; "20")]
#[test_case(r#####"tw`w-20`"#####, r#####"({
  width: "5rem",
})
;"##### ; "21")]
#[test_case(r#####"tw`w-24`"#####, r#####"({
  width: "6rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`w-28`"#####, r#####"({
  width: "7rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`w-32`"#####, r#####"({
  width: "8rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`w-36`"#####, r#####"({
  width: "9rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`w-40`"#####, r#####"({
  width: "10rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`w-44`"#####, r#####"({
  width: "11rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`w-48`"#####, r#####"({
  width: "12rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`w-52`"#####, r#####"({
  width: "13rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`w-56`"#####, r#####"({
  width: "14rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`w-60`"#####, r#####"({
  width: "15rem",
})
;"##### ; "31")]
#[test_case(r#####"tw`w-64`"#####, r#####"({
  width: "16rem",
})
;"##### ; "32")]
#[test_case(r#####"tw`w-72`"#####, r#####"({
  width: "18rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`w-80`"#####, r#####"({
  width: "20rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`w-96`"#####, r#####"({
  width: "24rem",
})
;"##### ; "35")]
#[test_case(r#####"tw`w-auto`"#####, r#####"({
  width: "auto",
})
;"##### ; "36")]
#[test_case(r#####"tw`w-1/2`"#####, r#####"({
  width: "50%",
})
;"##### ; "37")]
#[test_case(r#####"tw`w-1/3`"#####, r#####"({
  width: "33.333333%",
})
;"##### ; "38")]
#[test_case(r#####"tw`w-2/3`"#####, r#####"({
  width: "66.666667%",
})
;"##### ; "39")]
#[test_case(r#####"tw`w-1/4`"#####, r#####"({
  width: "25%",
})
;"##### ; "40")]
#[test_case(r#####"tw`w-2/4`"#####, r#####"({
  width: "50%",
})
;"##### ; "41")]
#[test_case(r#####"tw`w-3/4`"#####, r#####"({
  width: "75%",
})
;"##### ; "42")]
#[test_case(r#####"tw`w-1/5`"#####, r#####"({
  width: "20%",
})
;"##### ; "43")]
#[test_case(r#####"tw`w-2/5`"#####, r#####"({
  width: "40%",
})
;"##### ; "44")]
#[test_case(r#####"tw`w-3/5`"#####, r#####"({
  width: "60%",
})
;"##### ; "45")]
#[test_case(r#####"tw`w-4/5`"#####, r#####"({
  width: "80%",
})
;"##### ; "46")]
#[test_case(r#####"tw`w-1/6`"#####, r#####"({
  width: "16.666667%",
})
;"##### ; "47")]
#[test_case(r#####"tw`w-2/6`"#####, r#####"({
  width: "33.333333%",
})
;"##### ; "48")]
#[test_case(r#####"tw`w-3/6`"#####, r#####"({
  width: "50%",
})
;"##### ; "49")]
#[test_case(r#####"tw`w-4/6`"#####, r#####"({
  width: "66.666667%",
})
;"##### ; "50")]
#[test_case(r#####"tw`w-5/6`"#####, r#####"({
  width: "83.333333%",
})
;"##### ; "51")]
#[test_case(r#####"tw`w-1/12`"#####, r#####"({
  width: "8.333333%",
})
;"##### ; "52")]
#[test_case(r#####"tw`w-2/12`"#####, r#####"({
  width: "16.666667%",
})
;"##### ; "53")]
#[test_case(r#####"tw`w-3/12`"#####, r#####"({
  width: "25%",
})
;"##### ; "54")]
#[test_case(r#####"tw`w-4/12`"#####, r#####"({
  width: "33.333333%",
})
;"##### ; "55")]
#[test_case(r#####"tw`w-5/12`"#####, r#####"({
  width: "41.666667%",
})
;"##### ; "56")]
#[test_case(r#####"tw`w-6/12`"#####, r#####"({
  width: "50%",
})
;"##### ; "57")]
#[test_case(r#####"tw`w-7/12`"#####, r#####"({
  width: "58.333333%",
})
;"##### ; "58")]
#[test_case(r#####"tw`w-8/12`"#####, r#####"({
  width: "66.666667%",
})
;"##### ; "59")]
#[test_case(r#####"tw`w-9/12`"#####, r#####"({
  width: "75%",
})
;"##### ; "60")]
#[test_case(r#####"tw`w-10/12`"#####, r#####"({
  width: "83.333333%",
})
;"##### ; "61")]
#[test_case(r#####"tw`w-11/12`"#####, r#####"({
  width: "91.666667%",
})
;"##### ; "62")]
#[test_case(r#####"tw`w-full`"#####, r#####"({
  width: "100%",
})
;"##### ; "63")]
#[test_case(r#####"tw`w-screen`"#####, r#####"({
  width: "100vw",
})
;"##### ; "64")]
#[test_case(r#####"tw`w-min`"#####, r#####"({
  width: "min-content",
})
;"##### ; "65")]
#[test_case(r#####"tw`w-max`"#####, r#####"({
  width: "max-content",
})
;"##### ; "66")]
#[test_case(r#####"tw`w-fit`"#####, r#####"({
  width: "fit-content",
})
;"##### ; "67")]
#[test_case(r#####"tw`w-[3.23rem]`"#####, r#####"({
  width: "3.23rem",
})
;"##### ; "68")]
#[test_case(r#####"tw`w-[calc(100%+1rem)]`"#####, r#####"({
  width: "calc(100% + 1rem)",
})
;"##### ; "69")]
#[test_case(r#####"tw`w-[calc(var(--10-10px,calc(-20px-(-30px--40px)))-50px)]`"#####, r#####"({
  width: "calc(var(--10-10px,calc(-20px - (-30px - -40px))) - 50px)",
})
;"##### ; "70")]
#[test_case(r#####"tw`w-[var(--width)]`"#####, r#####"({
  width: "var(--width)",
})
;"##### ; "71")]
#[test_case(r#####"tw`w-[var(--width,calc(100%+1rem))]`"#####, r#####"({
  width: "var(--width,calc(100% + 1rem))",
})
;"##### ; "72")]
#[test_case(r#####"tw`w-[calc(100%/3-1rem*2)]`"#####, r#####"({
  width: "calc(100% / 3 - 1rem * 2)",
})"##### ; "73")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}