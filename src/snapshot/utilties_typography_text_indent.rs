use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`textIndent`"#####, r#####"({
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
;"##### ; "0")]
#[test_case(r#####"tw`indent-0`"#####, r#####"({
  textIndent: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`indent-px`"#####, r#####"({
  textIndent: "1px",
})
;"##### ; "2")]
#[test_case(r#####"tw`indent-0.5`"#####, r#####"({
  textIndent: "0.125rem",
})
;"##### ; "3")]
#[test_case(r#####"tw`indent-1`"#####, r#####"({
  textIndent: "0.25rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`indent-1.5`"#####, r#####"({
  textIndent: "0.375rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`indent-2`"#####, r#####"({
  textIndent: "0.5rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`indent-2.5`"#####, r#####"({
  textIndent: "0.625rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`indent-3`"#####, r#####"({
  textIndent: "0.75rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`indent-3.5`"#####, r#####"({
  textIndent: "0.875rem",
})
;"##### ; "9")]
#[test_case(r#####"tw`indent-4`"#####, r#####"({
  textIndent: "1rem",
})
;"##### ; "10")]
#[test_case(r#####"tw`indent-5`"#####, r#####"({
  textIndent: "1.25rem",
})
;"##### ; "11")]
#[test_case(r#####"tw`indent-6`"#####, r#####"({
  textIndent: "1.5rem",
})
;"##### ; "12")]
#[test_case(r#####"tw`indent-7`"#####, r#####"({
  textIndent: "1.75rem",
})
;"##### ; "13")]
#[test_case(r#####"tw`indent-8`"#####, r#####"({
  textIndent: "2rem",
})
;"##### ; "14")]
#[test_case(r#####"tw`indent-9`"#####, r#####"({
  textIndent: "2.25rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`indent-10`"#####, r#####"({
  textIndent: "2.5rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`indent-11`"#####, r#####"({
  textIndent: "2.75rem",
})
;"##### ; "17")]
#[test_case(r#####"tw`indent-12`"#####, r#####"({
  textIndent: "3rem",
})
;"##### ; "18")]
#[test_case(r#####"tw`indent-14`"#####, r#####"({
  textIndent: "3.5rem",
})
;"##### ; "19")]
#[test_case(r#####"tw`indent-16`"#####, r#####"({
  textIndent: "4rem",
})
;"##### ; "20")]
#[test_case(r#####"tw`indent-20`"#####, r#####"({
  textIndent: "5rem",
})
;"##### ; "21")]
#[test_case(r#####"tw`indent-24`"#####, r#####"({
  textIndent: "6rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`indent-28`"#####, r#####"({
  textIndent: "7rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`indent-32`"#####, r#####"({
  textIndent: "8rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`indent-36`"#####, r#####"({
  textIndent: "9rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`indent-40`"#####, r#####"({
  textIndent: "10rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`indent-44`"#####, r#####"({
  textIndent: "11rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`indent-48`"#####, r#####"({
  textIndent: "12rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`indent-52`"#####, r#####"({
  textIndent: "13rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`indent-56`"#####, r#####"({
  textIndent: "14rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`indent-60`"#####, r#####"({
  textIndent: "15rem",
})
;"##### ; "31")]
#[test_case(r#####"tw`indent-64`"#####, r#####"({
  textIndent: "16rem",
})
;"##### ; "32")]
#[test_case(r#####"tw`indent-72`"#####, r#####"({
  textIndent: "18rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`indent-80`"#####, r#####"({
  textIndent: "20rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`indent-96`"#####, r#####"({
  textIndent: "24rem",
})
;"##### ; "35")]
#[test_case(r#####"tw`-indent-0`"#####, r#####"({
  textIndent: "-0px",
})
;"##### ; "36")]
#[test_case(r#####"tw`-indent-px`"#####, r#####"({
  textIndent: "-1px",
})
;"##### ; "37")]
#[test_case(r#####"tw`-indent-0.5`"#####, r#####"({
  textIndent: "-0.125rem",
})
;"##### ; "38")]
#[test_case(r#####"tw`-indent-1`"#####, r#####"({
  textIndent: "-0.25rem",
})
;"##### ; "39")]
#[test_case(r#####"tw`-indent-1.5`"#####, r#####"({
  textIndent: "-0.375rem",
})
;"##### ; "40")]
#[test_case(r#####"tw`-indent-2`"#####, r#####"({
  textIndent: "-0.5rem",
})
;"##### ; "41")]
#[test_case(r#####"tw`-indent-2.5`"#####, r#####"({
  textIndent: "-0.625rem",
})
;"##### ; "42")]
#[test_case(r#####"tw`-indent-3`"#####, r#####"({
  textIndent: "-0.75rem",
})
;"##### ; "43")]
#[test_case(r#####"tw`-indent-3.5`"#####, r#####"({
  textIndent: "-0.875rem",
})
;"##### ; "44")]
#[test_case(r#####"tw`-indent-4`"#####, r#####"({
  textIndent: "-1rem",
})
;"##### ; "45")]
#[test_case(r#####"tw`-indent-5`"#####, r#####"({
  textIndent: "-1.25rem",
})
;"##### ; "46")]
#[test_case(r#####"tw`-indent-6`"#####, r#####"({
  textIndent: "-1.5rem",
})
;"##### ; "47")]
#[test_case(r#####"tw`-indent-7`"#####, r#####"({
  textIndent: "-1.75rem",
})
;"##### ; "48")]
#[test_case(r#####"tw`-indent-8`"#####, r#####"({
  textIndent: "-2rem",
})
;"##### ; "49")]
#[test_case(r#####"tw`-indent-9`"#####, r#####"({
  textIndent: "-2.25rem",
})
;"##### ; "50")]
#[test_case(r#####"tw`-indent-10`"#####, r#####"({
  textIndent: "-2.5rem",
})
;"##### ; "51")]
#[test_case(r#####"tw`-indent-11`"#####, r#####"({
  textIndent: "-2.75rem",
})
;"##### ; "52")]
#[test_case(r#####"tw`-indent-12`"#####, r#####"({
  textIndent: "-3rem",
})
;"##### ; "53")]
#[test_case(r#####"tw`-indent-14`"#####, r#####"({
  textIndent: "-3.5rem",
})
;"##### ; "54")]
#[test_case(r#####"tw`-indent-16`"#####, r#####"({
  textIndent: "-4rem",
})
;"##### ; "55")]
#[test_case(r#####"tw`-indent-20`"#####, r#####"({
  textIndent: "-5rem",
})
;"##### ; "56")]
#[test_case(r#####"tw`-indent-24`"#####, r#####"({
  textIndent: "-6rem",
})
;"##### ; "57")]
#[test_case(r#####"tw`-indent-28`"#####, r#####"({
  textIndent: "-7rem",
})
;"##### ; "58")]
#[test_case(r#####"tw`-indent-32`"#####, r#####"({
  textIndent: "-8rem",
})
;"##### ; "59")]
#[test_case(r#####"tw`-indent-36`"#####, r#####"({
  textIndent: "-9rem",
})
;"##### ; "60")]
#[test_case(r#####"tw`-indent-40`"#####, r#####"({
  textIndent: "-10rem",
})
;"##### ; "61")]
#[test_case(r#####"tw`-indent-44`"#####, r#####"({
  textIndent: "-11rem",
})
;"##### ; "62")]
#[test_case(r#####"tw`-indent-48`"#####, r#####"({
  textIndent: "-12rem",
})
;"##### ; "63")]
#[test_case(r#####"tw`-indent-52`"#####, r#####"({
  textIndent: "-13rem",
})
;"##### ; "64")]
#[test_case(r#####"tw`-indent-56`"#####, r#####"({
  textIndent: "-14rem",
})
;"##### ; "65")]
#[test_case(r#####"tw`-indent-60`"#####, r#####"({
  textIndent: "-15rem",
})
;"##### ; "66")]
#[test_case(r#####"tw`-indent-64`"#####, r#####"({
  textIndent: "-16rem",
})
;"##### ; "67")]
#[test_case(r#####"tw`-indent-72`"#####, r#####"({
  textIndent: "-18rem",
})
;"##### ; "68")]
#[test_case(r#####"tw`-indent-80`"#####, r#####"({
  textIndent: "-20rem",
})
;"##### ; "69")]
#[test_case(r#####"tw`-indent-96`"#####, r#####"({
  textIndent: "-24rem",
})
;"##### ; "70")]
#[test_case(r#####"tw`indent-[50%]`"#####, r#####"({
  textIndent: "50%",
})
;"##### ; "71")]
#[test_case(r#####"tw`indent-[length:10px]`"#####, r#####"({
  textIndent: "10px",
})
;"##### ; "72")]
#[test_case(r#####"tw`indent-[lookup:10px]`"#####, r#####"({
  textIndent: "10px",
})"##### ; "73")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
