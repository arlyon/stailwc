use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`borderSpacing`"#####, r#####"({
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
})
;"##### ; "1")]
#[test_case(r#####"tw`border-spacing-0`"#####, r#####"({
  '--tw-border-spacing-x': "0px",
  '--tw-border-spacing-y': "0px",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "2")]
#[test_case(r#####"tw`border-spacing-x-0`"#####, r#####"({
  '--tw-border-spacing-x': "0px",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "3")]
#[test_case(r#####"tw`border-spacing-y-0`"#####, r#####"({
  '--tw-border-spacing-y': "0px",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "4")]
#[test_case(r#####"tw`border-spacing-px`"#####, r#####"({
  '--tw-border-spacing-x': "1px",
  '--tw-border-spacing-y': "1px",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "5")]
#[test_case(r#####"tw`border-spacing-x-px`"#####, r#####"({
  '--tw-border-spacing-x': "1px",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "6")]
#[test_case(r#####"tw`border-spacing-y-px`"#####, r#####"({
  '--tw-border-spacing-y': "1px",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "7")]
#[test_case(r#####"tw`border-spacing-0.5`"#####, r#####"({
  '--tw-border-spacing-x': "0.125rem",
  '--tw-border-spacing-y': "0.125rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "8")]
#[test_case(r#####"tw`border-spacing-x-0.5`"#####, r#####"({
  '--tw-border-spacing-x': "0.125rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "9")]
#[test_case(r#####"tw`border-spacing-y-0.5`"#####, r#####"({
  '--tw-border-spacing-y': "0.125rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "10")]
#[test_case(r#####"tw`border-spacing-1`"#####, r#####"({
  '--tw-border-spacing-x': "0.25rem",
  '--tw-border-spacing-y': "0.25rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "11")]
#[test_case(r#####"tw`border-spacing-x-1`"#####, r#####"({
  '--tw-border-spacing-x': "0.25rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "12")]
#[test_case(r#####"tw`border-spacing-y-1`"#####, r#####"({
  '--tw-border-spacing-y': "0.25rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "13")]
#[test_case(r#####"tw`border-spacing-1.5`"#####, r#####"({
  '--tw-border-spacing-x': "0.375rem",
  '--tw-border-spacing-y': "0.375rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "14")]
#[test_case(r#####"tw`border-spacing-x-1.5`"#####, r#####"({
  '--tw-border-spacing-x': "0.375rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "15")]
#[test_case(r#####"tw`border-spacing-y-1.5`"#####, r#####"({
  '--tw-border-spacing-y': "0.375rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "16")]
#[test_case(r#####"tw`border-spacing-2`"#####, r#####"({
  '--tw-border-spacing-x': "0.5rem",
  '--tw-border-spacing-y': "0.5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "17")]
#[test_case(r#####"tw`border-spacing-x-2`"#####, r#####"({
  '--tw-border-spacing-x': "0.5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "18")]
#[test_case(r#####"tw`border-spacing-y-2`"#####, r#####"({
  '--tw-border-spacing-y': "0.5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "19")]
#[test_case(r#####"tw`border-spacing-2.5`"#####, r#####"({
  '--tw-border-spacing-x': "0.625rem",
  '--tw-border-spacing-y': "0.625rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "20")]
#[test_case(r#####"tw`border-spacing-x-2.5`"#####, r#####"({
  '--tw-border-spacing-x': "0.625rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "21")]
#[test_case(r#####"tw`border-spacing-y-2.5`"#####, r#####"({
  '--tw-border-spacing-y': "0.625rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "22")]
#[test_case(r#####"tw`border-spacing-3`"#####, r#####"({
  '--tw-border-spacing-x': "0.75rem",
  '--tw-border-spacing-y': "0.75rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "23")]
#[test_case(r#####"tw`border-spacing-x-3`"#####, r#####"({
  '--tw-border-spacing-x': "0.75rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "24")]
#[test_case(r#####"tw`border-spacing-y-3`"#####, r#####"({
  '--tw-border-spacing-y': "0.75rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "25")]
#[test_case(r#####"tw`border-spacing-3.5`"#####, r#####"({
  '--tw-border-spacing-x': "0.875rem",
  '--tw-border-spacing-y': "0.875rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "26")]
#[test_case(r#####"tw`border-spacing-x-3.5`"#####, r#####"({
  '--tw-border-spacing-x': "0.875rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "27")]
#[test_case(r#####"tw`border-spacing-y-3.5`"#####, r#####"({
  '--tw-border-spacing-y': "0.875rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "28")]
#[test_case(r#####"tw`border-spacing-4`"#####, r#####"({
  '--tw-border-spacing-x': "1rem",
  '--tw-border-spacing-y': "1rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "29")]
#[test_case(r#####"tw`border-spacing-x-4`"#####, r#####"({
  '--tw-border-spacing-x': "1rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "30")]
#[test_case(r#####"tw`border-spacing-y-4`"#####, r#####"({
  '--tw-border-spacing-y': "1rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "31")]
#[test_case(r#####"tw`border-spacing-5`"#####, r#####"({
  '--tw-border-spacing-x': "1.25rem",
  '--tw-border-spacing-y': "1.25rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "32")]
#[test_case(r#####"tw`border-spacing-x-5`"#####, r#####"({
  '--tw-border-spacing-x': "1.25rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "33")]
#[test_case(r#####"tw`border-spacing-y-5`"#####, r#####"({
  '--tw-border-spacing-y': "1.25rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "34")]
#[test_case(r#####"tw`border-spacing-6`"#####, r#####"({
  '--tw-border-spacing-x': "1.5rem",
  '--tw-border-spacing-y': "1.5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "35")]
#[test_case(r#####"tw`border-spacing-x-6`"#####, r#####"({
  '--tw-border-spacing-x': "1.5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "36")]
#[test_case(r#####"tw`border-spacing-y-6`"#####, r#####"({
  '--tw-border-spacing-y': "1.5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "37")]
#[test_case(r#####"tw`border-spacing-7`"#####, r#####"({
  '--tw-border-spacing-x': "1.75rem",
  '--tw-border-spacing-y': "1.75rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "38")]
#[test_case(r#####"tw`border-spacing-x-7`"#####, r#####"({
  '--tw-border-spacing-x': "1.75rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "39")]
#[test_case(r#####"tw`border-spacing-y-7`"#####, r#####"({
  '--tw-border-spacing-y': "1.75rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "40")]
#[test_case(r#####"tw`border-spacing-8`"#####, r#####"({
  '--tw-border-spacing-x': "2rem",
  '--tw-border-spacing-y': "2rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "41")]
#[test_case(r#####"tw`border-spacing-x-8`"#####, r#####"({
  '--tw-border-spacing-x': "2rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "42")]
#[test_case(r#####"tw`border-spacing-y-8`"#####, r#####"({
  '--tw-border-spacing-y': "2rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "43")]
#[test_case(r#####"tw`border-spacing-9`"#####, r#####"({
  '--tw-border-spacing-x': "2.25rem",
  '--tw-border-spacing-y': "2.25rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "44")]
#[test_case(r#####"tw`border-spacing-x-9`"#####, r#####"({
  '--tw-border-spacing-x': "2.25rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "45")]
#[test_case(r#####"tw`border-spacing-y-9`"#####, r#####"({
  '--tw-border-spacing-y': "2.25rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "46")]
#[test_case(r#####"tw`border-spacing-10`"#####, r#####"({
  '--tw-border-spacing-x': "2.5rem",
  '--tw-border-spacing-y': "2.5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "47")]
#[test_case(r#####"tw`border-spacing-x-10`"#####, r#####"({
  '--tw-border-spacing-x': "2.5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "48")]
#[test_case(r#####"tw`border-spacing-y-10`"#####, r#####"({
  '--tw-border-spacing-y': "2.5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "49")]
#[test_case(r#####"tw`border-spacing-11`"#####, r#####"({
  '--tw-border-spacing-x': "2.75rem",
  '--tw-border-spacing-y': "2.75rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "50")]
#[test_case(r#####"tw`border-spacing-x-11`"#####, r#####"({
  '--tw-border-spacing-x': "2.75rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "51")]
#[test_case(r#####"tw`border-spacing-y-11`"#####, r#####"({
  '--tw-border-spacing-y': "2.75rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "52")]
#[test_case(r#####"tw`border-spacing-12`"#####, r#####"({
  '--tw-border-spacing-x': "3rem",
  '--tw-border-spacing-y': "3rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "53")]
#[test_case(r#####"tw`border-spacing-x-12`"#####, r#####"({
  '--tw-border-spacing-x': "3rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "54")]
#[test_case(r#####"tw`border-spacing-y-12`"#####, r#####"({
  '--tw-border-spacing-y': "3rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "55")]
#[test_case(r#####"tw`border-spacing-14`"#####, r#####"({
  '--tw-border-spacing-x': "3.5rem",
  '--tw-border-spacing-y': "3.5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "56")]
#[test_case(r#####"tw`border-spacing-x-14`"#####, r#####"({
  '--tw-border-spacing-x': "3.5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "57")]
#[test_case(r#####"tw`border-spacing-y-14`"#####, r#####"({
  '--tw-border-spacing-y': "3.5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "58")]
#[test_case(r#####"tw`border-spacing-16`"#####, r#####"({
  '--tw-border-spacing-x': "4rem",
  '--tw-border-spacing-y': "4rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "59")]
#[test_case(r#####"tw`border-spacing-x-16`"#####, r#####"({
  '--tw-border-spacing-x': "4rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "60")]
#[test_case(r#####"tw`border-spacing-y-16`"#####, r#####"({
  '--tw-border-spacing-y': "4rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "61")]
#[test_case(r#####"tw`border-spacing-20`"#####, r#####"({
  '--tw-border-spacing-x': "5rem",
  '--tw-border-spacing-y': "5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "62")]
#[test_case(r#####"tw`border-spacing-x-20`"#####, r#####"({
  '--tw-border-spacing-x': "5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "63")]
#[test_case(r#####"tw`border-spacing-y-20`"#####, r#####"({
  '--tw-border-spacing-y': "5rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "64")]
#[test_case(r#####"tw`border-spacing-24`"#####, r#####"({
  '--tw-border-spacing-x': "6rem",
  '--tw-border-spacing-y': "6rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "65")]
#[test_case(r#####"tw`border-spacing-x-24`"#####, r#####"({
  '--tw-border-spacing-x': "6rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "66")]
#[test_case(r#####"tw`border-spacing-y-24`"#####, r#####"({
  '--tw-border-spacing-y': "6rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "67")]
#[test_case(r#####"tw`border-spacing-28`"#####, r#####"({
  '--tw-border-spacing-x': "7rem",
  '--tw-border-spacing-y': "7rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "68")]
#[test_case(r#####"tw`border-spacing-x-28`"#####, r#####"({
  '--tw-border-spacing-x': "7rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "69")]
#[test_case(r#####"tw`border-spacing-y-28`"#####, r#####"({
  '--tw-border-spacing-y': "7rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "70")]
#[test_case(r#####"tw`border-spacing-32`"#####, r#####"({
  '--tw-border-spacing-x': "8rem",
  '--tw-border-spacing-y': "8rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "71")]
#[test_case(r#####"tw`border-spacing-x-32`"#####, r#####"({
  '--tw-border-spacing-x': "8rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "72")]
#[test_case(r#####"tw`border-spacing-y-32`"#####, r#####"({
  '--tw-border-spacing-y': "8rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "73")]
#[test_case(r#####"tw`border-spacing-36`"#####, r#####"({
  '--tw-border-spacing-x': "9rem",
  '--tw-border-spacing-y': "9rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "74")]
#[test_case(r#####"tw`border-spacing-x-36`"#####, r#####"({
  '--tw-border-spacing-x': "9rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "75")]
#[test_case(r#####"tw`border-spacing-y-36`"#####, r#####"({
  '--tw-border-spacing-y': "9rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "76")]
#[test_case(r#####"tw`border-spacing-40`"#####, r#####"({
  '--tw-border-spacing-x': "10rem",
  '--tw-border-spacing-y': "10rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "77")]
#[test_case(r#####"tw`border-spacing-x-40`"#####, r#####"({
  '--tw-border-spacing-x': "10rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "78")]
#[test_case(r#####"tw`border-spacing-y-40`"#####, r#####"({
  '--tw-border-spacing-y': "10rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "79")]
#[test_case(r#####"tw`border-spacing-44`"#####, r#####"({
  '--tw-border-spacing-x': "11rem",
  '--tw-border-spacing-y': "11rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "80")]
#[test_case(r#####"tw`border-spacing-x-44`"#####, r#####"({
  '--tw-border-spacing-x': "11rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "81")]
#[test_case(r#####"tw`border-spacing-y-44`"#####, r#####"({
  '--tw-border-spacing-y': "11rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "82")]
#[test_case(r#####"tw`border-spacing-48`"#####, r#####"({
  '--tw-border-spacing-x': "12rem",
  '--tw-border-spacing-y': "12rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "83")]
#[test_case(r#####"tw`border-spacing-x-48`"#####, r#####"({
  '--tw-border-spacing-x': "12rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "84")]
#[test_case(r#####"tw`border-spacing-y-48`"#####, r#####"({
  '--tw-border-spacing-y': "12rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "85")]
#[test_case(r#####"tw`border-spacing-52`"#####, r#####"({
  '--tw-border-spacing-x': "13rem",
  '--tw-border-spacing-y': "13rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "86")]
#[test_case(r#####"tw`border-spacing-x-52`"#####, r#####"({
  '--tw-border-spacing-x': "13rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "87")]
#[test_case(r#####"tw`border-spacing-y-52`"#####, r#####"({
  '--tw-border-spacing-y': "13rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "88")]
#[test_case(r#####"tw`border-spacing-56`"#####, r#####"({
  '--tw-border-spacing-x': "14rem",
  '--tw-border-spacing-y': "14rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "89")]
#[test_case(r#####"tw`border-spacing-x-56`"#####, r#####"({
  '--tw-border-spacing-x': "14rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "90")]
#[test_case(r#####"tw`border-spacing-y-56`"#####, r#####"({
  '--tw-border-spacing-y': "14rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "91")]
#[test_case(r#####"tw`border-spacing-60`"#####, r#####"({
  '--tw-border-spacing-x': "15rem",
  '--tw-border-spacing-y': "15rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "92")]
#[test_case(r#####"tw`border-spacing-x-60`"#####, r#####"({
  '--tw-border-spacing-x': "15rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "93")]
#[test_case(r#####"tw`border-spacing-y-60`"#####, r#####"({
  '--tw-border-spacing-y': "15rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "94")]
#[test_case(r#####"tw`border-spacing-64`"#####, r#####"({
  '--tw-border-spacing-x': "16rem",
  '--tw-border-spacing-y': "16rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "95")]
#[test_case(r#####"tw`border-spacing-x-64`"#####, r#####"({
  '--tw-border-spacing-x': "16rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "96")]
#[test_case(r#####"tw`border-spacing-y-64`"#####, r#####"({
  '--tw-border-spacing-y': "16rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "97")]
#[test_case(r#####"tw`border-spacing-72`"#####, r#####"({
  '--tw-border-spacing-x': "18rem",
  '--tw-border-spacing-y': "18rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "98")]
#[test_case(r#####"tw`border-spacing-x-72`"#####, r#####"({
  '--tw-border-spacing-x': "18rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "99")]
#[test_case(r#####"tw`border-spacing-y-72`"#####, r#####"({
  '--tw-border-spacing-y': "18rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "100")]
#[test_case(r#####"tw`border-spacing-80`"#####, r#####"({
  '--tw-border-spacing-x': "20rem",
  '--tw-border-spacing-y': "20rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "101")]
#[test_case(r#####"tw`border-spacing-x-80`"#####, r#####"({
  '--tw-border-spacing-x': "20rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "102")]
#[test_case(r#####"tw`border-spacing-y-80`"#####, r#####"({
  '--tw-border-spacing-y': "20rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "103")]
#[test_case(r#####"tw`border-spacing-96`"#####, r#####"({
  '--tw-border-spacing-x': "24rem",
  '--tw-border-spacing-y': "24rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "104")]
#[test_case(r#####"tw`border-spacing-x-96`"#####, r#####"({
  '--tw-border-spacing-x': "24rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})
;"##### ; "105")]
#[test_case(r#####"tw`border-spacing-y-96`"#####, r#####"({
  '--tw-border-spacing-y': "24rem",
  borderSpacing: "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
})"##### ; "106")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
