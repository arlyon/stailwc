use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`flexBasis`"#####, r#####"({
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
})
;"##### ; "0")]
#[test_case(r#####"tw`basis-0`"#####, r#####"({
  flexBasis: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`basis-1`"#####, r#####"({
  flexBasis: "0.25rem",
})
;"##### ; "2")]
#[test_case(r#####"tw`basis-2`"#####, r#####"({
  flexBasis: "0.5rem",
})
;"##### ; "3")]
#[test_case(r#####"tw`basis-3`"#####, r#####"({
  flexBasis: "0.75rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`basis-4`"#####, r#####"({
  flexBasis: "1rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`basis-5`"#####, r#####"({
  flexBasis: "1.25rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`basis-6`"#####, r#####"({
  flexBasis: "1.5rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`basis-7`"#####, r#####"({
  flexBasis: "1.75rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`basis-8`"#####, r#####"({
  flexBasis: "2rem",
})
;"##### ; "9")]
#[test_case(r#####"tw`basis-9`"#####, r#####"({
  flexBasis: "2.25rem",
})
;"##### ; "10")]
#[test_case(r#####"tw`basis-10`"#####, r#####"({
  flexBasis: "2.5rem",
})
;"##### ; "11")]
#[test_case(r#####"tw`basis-11`"#####, r#####"({
  flexBasis: "2.75rem",
})
;"##### ; "12")]
#[test_case(r#####"tw`basis-12`"#####, r#####"({
  flexBasis: "3rem",
})
;"##### ; "13")]
#[test_case(r#####"tw`basis-14`"#####, r#####"({
  flexBasis: "3.5rem",
})
;"##### ; "14")]
#[test_case(r#####"tw`basis-16`"#####, r#####"({
  flexBasis: "4rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`basis-20`"#####, r#####"({
  flexBasis: "5rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`basis-24`"#####, r#####"({
  flexBasis: "6rem",
})
;"##### ; "17")]
#[test_case(r#####"tw`basis-28`"#####, r#####"({
  flexBasis: "7rem",
})
;"##### ; "18")]
#[test_case(r#####"tw`basis-32`"#####, r#####"({
  flexBasis: "8rem",
})
;"##### ; "19")]
#[test_case(r#####"tw`basis-36`"#####, r#####"({
  flexBasis: "9rem",
})
;"##### ; "20")]
#[test_case(r#####"tw`basis-40`"#####, r#####"({
  flexBasis: "10rem",
})
;"##### ; "21")]
#[test_case(r#####"tw`basis-44`"#####, r#####"({
  flexBasis: "11rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`basis-48`"#####, r#####"({
  flexBasis: "12rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`basis-52`"#####, r#####"({
  flexBasis: "13rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`basis-56`"#####, r#####"({
  flexBasis: "14rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`basis-60`"#####, r#####"({
  flexBasis: "15rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`basis-64`"#####, r#####"({
  flexBasis: "16rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`basis-72`"#####, r#####"({
  flexBasis: "18rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`basis-80`"#####, r#####"({
  flexBasis: "20rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`basis-96`"#####, r#####"({
  flexBasis: "24rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`basis-auto`"#####, r#####"({
  flexBasis: "auto",
})
;"##### ; "31")]
#[test_case(r#####"tw`basis-px`"#####, r#####"({
  flexBasis: "1px",
})
;"##### ; "32")]
#[test_case(r#####"tw`basis-0.5`"#####, r#####"({
  flexBasis: "0.125rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`basis-1.5`"#####, r#####"({
  flexBasis: "0.375rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`basis-2.5`"#####, r#####"({
  flexBasis: "0.625rem",
})
;"##### ; "35")]
#[test_case(r#####"tw`basis-3.5`"#####, r#####"({
  flexBasis: "0.875rem",
})
;"##### ; "36")]
#[test_case(r#####"tw`basis-1/2`"#####, r#####"({
  flexBasis: "50%",
})
;"##### ; "37")]
#[test_case(r#####"tw`basis-1/3`"#####, r#####"({
  flexBasis: "33.333333%",
})
;"##### ; "38")]
#[test_case(r#####"tw`basis-2/3`"#####, r#####"({
  flexBasis: "66.666667%",
})
;"##### ; "39")]
#[test_case(r#####"tw`basis-1/4`"#####, r#####"({
  flexBasis: "25%",
})
;"##### ; "40")]
#[test_case(r#####"tw`basis-2/4`"#####, r#####"({
  flexBasis: "50%",
})
;"##### ; "41")]
#[test_case(r#####"tw`basis-3/4`"#####, r#####"({
  flexBasis: "75%",
})
;"##### ; "42")]
#[test_case(r#####"tw`basis-1/5`"#####, r#####"({
  flexBasis: "20%",
})
;"##### ; "43")]
#[test_case(r#####"tw`basis-2/5`"#####, r#####"({
  flexBasis: "40%",
})
;"##### ; "44")]
#[test_case(r#####"tw`basis-3/5`"#####, r#####"({
  flexBasis: "60%",
})
;"##### ; "45")]
#[test_case(r#####"tw`basis-4/5`"#####, r#####"({
  flexBasis: "80%",
})
;"##### ; "46")]
#[test_case(r#####"tw`basis-1/6`"#####, r#####"({
  flexBasis: "16.666667%",
})
;"##### ; "47")]
#[test_case(r#####"tw`basis-2/6`"#####, r#####"({
  flexBasis: "33.333333%",
})
;"##### ; "48")]
#[test_case(r#####"tw`basis-3/6`"#####, r#####"({
  flexBasis: "50%",
})
;"##### ; "49")]
#[test_case(r#####"tw`basis-4/6`"#####, r#####"({
  flexBasis: "66.666667%",
})
;"##### ; "50")]
#[test_case(r#####"tw`basis-5/6`"#####, r#####"({
  flexBasis: "83.333333%",
})
;"##### ; "51")]
#[test_case(r#####"tw`basis-1/12`"#####, r#####"({
  flexBasis: "8.333333%",
})
;"##### ; "52")]
#[test_case(r#####"tw`basis-2/12`"#####, r#####"({
  flexBasis: "16.666667%",
})
;"##### ; "53")]
#[test_case(r#####"tw`basis-3/12`"#####, r#####"({
  flexBasis: "25%",
})
;"##### ; "54")]
#[test_case(r#####"tw`basis-4/12`"#####, r#####"({
  flexBasis: "33.333333%",
})
;"##### ; "55")]
#[test_case(r#####"tw`basis-5/12`"#####, r#####"({
  flexBasis: "41.666667%",
})
;"##### ; "56")]
#[test_case(r#####"tw`basis-6/12`"#####, r#####"({
  flexBasis: "50%",
})
;"##### ; "57")]
#[test_case(r#####"tw`basis-7/12`"#####, r#####"({
  flexBasis: "58.333333%",
})
;"##### ; "58")]
#[test_case(r#####"tw`basis-8/12`"#####, r#####"({
  flexBasis: "66.666667%",
})
;"##### ; "59")]
#[test_case(r#####"tw`basis-9/12`"#####, r#####"({
  flexBasis: "75%",
})
;"##### ; "60")]
#[test_case(r#####"tw`basis-10/12`"#####, r#####"({
  flexBasis: "83.333333%",
})
;"##### ; "61")]
#[test_case(r#####"tw`basis-11/12`"#####, r#####"({
  flexBasis: "91.666667%",
})
;"##### ; "62")]
#[test_case(r#####"tw`basis-full`"#####, r#####"({
  flexBasis: "100%",
})
;"##### ; "63")]
#[test_case(r#####"tw`basis-[14.2857143%]`"#####, r#####"({
  flexBasis: "14.2857143%",
})"##### ; "64")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
