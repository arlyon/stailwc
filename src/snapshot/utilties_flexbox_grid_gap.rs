use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`gap`"#####, r#####"({
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
#[test_case(r#####"tw`gap-0`"#####, r#####"({
  gap: "0px",
})
;"##### ; "2")]
#[test_case(r#####"tw`gap-0.5`"#####, r#####"({
  gap: "0.125rem",
})
;"##### ; "3")]
#[test_case(r#####"tw`gap-1`"#####, r#####"({
  gap: "0.25rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`gap-1.5`"#####, r#####"({
  gap: "0.375rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`gap-2`"#####, r#####"({
  gap: "0.5rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`gap-2.5`"#####, r#####"({
  gap: "0.625rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`gap-3`"#####, r#####"({
  gap: "0.75rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`gap-3.5`"#####, r#####"({
  gap: "0.875rem",
})
;"##### ; "9")]
#[test_case(r#####"tw`gap-4`"#####, r#####"({
  gap: "1rem",
})
;"##### ; "10")]
#[test_case(r#####"tw`gap-5`"#####, r#####"({
  gap: "1.25rem",
})
;"##### ; "11")]
#[test_case(r#####"tw`gap-6`"#####, r#####"({
  gap: "1.5rem",
})
;"##### ; "12")]
#[test_case(r#####"tw`gap-7`"#####, r#####"({
  gap: "1.75rem",
})
;"##### ; "13")]
#[test_case(r#####"tw`gap-8`"#####, r#####"({
  gap: "2rem",
})
;"##### ; "14")]
#[test_case(r#####"tw`gap-9`"#####, r#####"({
  gap: "2.25rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`gap-10`"#####, r#####"({
  gap: "2.5rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`gap-12`"#####, r#####"({
  gap: "3rem",
})
;"##### ; "17")]
#[test_case(r#####"tw`gap-14`"#####, r#####"({
  gap: "3.5rem",
})
;"##### ; "18")]
#[test_case(r#####"tw`gap-16`"#####, r#####"({
  gap: "4rem",
})
;"##### ; "19")]
#[test_case(r#####"tw`gap-20`"#####, r#####"({
  gap: "5rem",
})
;"##### ; "20")]
#[test_case(r#####"tw`gap-24`"#####, r#####"({
  gap: "6rem",
})
;"##### ; "21")]
#[test_case(r#####"tw`gap-28`"#####, r#####"({
  gap: "7rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`gap-32`"#####, r#####"({
  gap: "8rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`gap-36`"#####, r#####"({
  gap: "9rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`gap-40`"#####, r#####"({
  gap: "10rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`gap-44`"#####, r#####"({
  gap: "11rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`gap-48`"#####, r#####"({
  gap: "12rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`gap-52`"#####, r#####"({
  gap: "13rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`gap-56`"#####, r#####"({
  gap: "14rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`gap-60`"#####, r#####"({
  gap: "15rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`gap-64`"#####, r#####"({
  gap: "16rem",
})
;"##### ; "31")]
#[test_case(r#####"tw`gap-72`"#####, r#####"({
  gap: "18rem",
})
;"##### ; "32")]
#[test_case(r#####"tw`gap-80`"#####, r#####"({
  gap: "20rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`gap-96`"#####, r#####"({
  gap: "24rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`gap-px`"#####, r#####"({
  gap: "1px",
})
;"##### ; "35")]
#[test_case(r#####"tw`gap-[2.75rem]`"#####, r#####"({
  gap: "2.75rem",
}) // https://tailwindcss.com/docs/gap

;"##### ; "36")]
#[test_case(r#####"tw`gap-x-0`"#####, r#####"({
  columnGap: "0px",
})
;"##### ; "37")]
#[test_case(r#####"tw`gap-x-0.5`"#####, r#####"({
  columnGap: "0.125rem",
})
;"##### ; "38")]
#[test_case(r#####"tw`gap-x-1`"#####, r#####"({
  columnGap: "0.25rem",
})
;"##### ; "39")]
#[test_case(r#####"tw`gap-x-1.5`"#####, r#####"({
  columnGap: "0.375rem",
})
;"##### ; "40")]
#[test_case(r#####"tw`gap-x-2`"#####, r#####"({
  columnGap: "0.5rem",
})
;"##### ; "41")]
#[test_case(r#####"tw`gap-x-2.5`"#####, r#####"({
  columnGap: "0.625rem",
})
;"##### ; "42")]
#[test_case(r#####"tw`gap-x-3`"#####, r#####"({
  columnGap: "0.75rem",
})
;"##### ; "43")]
#[test_case(r#####"tw`gap-x-3.5`"#####, r#####"({
  columnGap: "0.875rem",
})
;"##### ; "44")]
#[test_case(r#####"tw`gap-x-4`"#####, r#####"({
  columnGap: "1rem",
})
;"##### ; "45")]
#[test_case(r#####"tw`gap-x-5`"#####, r#####"({
  columnGap: "1.25rem",
})
;"##### ; "46")]
#[test_case(r#####"tw`gap-x-6`"#####, r#####"({
  columnGap: "1.5rem",
})
;"##### ; "47")]
#[test_case(r#####"tw`gap-x-8`"#####, r#####"({
  columnGap: "2rem",
})
;"##### ; "48")]
#[test_case(r#####"tw`gap-x-10`"#####, r#####"({
  columnGap: "2.5rem",
})
;"##### ; "49")]
#[test_case(r#####"tw`gap-x-12`"#####, r#####"({
  columnGap: "3rem",
})
;"##### ; "50")]
#[test_case(r#####"tw`gap-x-16`"#####, r#####"({
  columnGap: "4rem",
})
;"##### ; "51")]
#[test_case(r#####"tw`gap-x-20`"#####, r#####"({
  columnGap: "5rem",
})
;"##### ; "52")]
#[test_case(r#####"tw`gap-x-24`"#####, r#####"({
  columnGap: "6rem",
})
;"##### ; "53")]
#[test_case(r#####"tw`gap-x-32`"#####, r#####"({
  columnGap: "8rem",
})
;"##### ; "54")]
#[test_case(r#####"tw`gap-x-40`"#####, r#####"({
  columnGap: "10rem",
})
;"##### ; "55")]
#[test_case(r#####"tw`gap-x-48`"#####, r#####"({
  columnGap: "12rem",
})
;"##### ; "56")]
#[test_case(r#####"tw`gap-x-56`"#####, r#####"({
  columnGap: "14rem",
})
;"##### ; "57")]
#[test_case(r#####"tw`gap-x-64`"#####, r#####"({
  columnGap: "16rem",
})
;"##### ; "58")]
#[test_case(r#####"tw`gap-x-px`"#####, r#####"({
  columnGap: "1px",
})
;"##### ; "59")]
#[test_case(r#####"tw`gap-x-[2.75rem]`"#####, r#####"({
  columnGap: "2.75rem",
}) // https://tailwindcss.com/docs/gap

;"##### ; "60")]
#[test_case(r#####"tw`gap-y-0`"#####, r#####"({
  rowGap: "0px",
})
;"##### ; "61")]
#[test_case(r#####"tw`gap-y-0.5`"#####, r#####"({
  rowGap: "0.125rem",
})
;"##### ; "62")]
#[test_case(r#####"tw`gap-y-1`"#####, r#####"({
  rowGap: "0.25rem",
})
;"##### ; "63")]
#[test_case(r#####"tw`gap-y-1.5`"#####, r#####"({
  rowGap: "0.375rem",
})
;"##### ; "64")]
#[test_case(r#####"tw`gap-y-2`"#####, r#####"({
  rowGap: "0.5rem",
})
;"##### ; "65")]
#[test_case(r#####"tw`gap-y-2.5`"#####, r#####"({
  rowGap: "0.625rem",
})
;"##### ; "66")]
#[test_case(r#####"tw`gap-y-3`"#####, r#####"({
  rowGap: "0.75rem",
})
;"##### ; "67")]
#[test_case(r#####"tw`gap-y-3.5`"#####, r#####"({
  rowGap: "0.875rem",
})
;"##### ; "68")]
#[test_case(r#####"tw`gap-y-4`"#####, r#####"({
  rowGap: "1rem",
})
;"##### ; "69")]
#[test_case(r#####"tw`gap-y-5`"#####, r#####"({
  rowGap: "1.25rem",
})
;"##### ; "70")]
#[test_case(r#####"tw`gap-y-6`"#####, r#####"({
  rowGap: "1.5rem",
})
;"##### ; "71")]
#[test_case(r#####"tw`gap-y-7`"#####, r#####"({
  rowGap: "1.75rem",
})
;"##### ; "72")]
#[test_case(r#####"tw`gap-y-8`"#####, r#####"({
  rowGap: "2rem",
})
;"##### ; "73")]
#[test_case(r#####"tw`gap-y-9`"#####, r#####"({
  rowGap: "2.25rem",
})
;"##### ; "74")]
#[test_case(r#####"tw`gap-y-10`"#####, r#####"({
  rowGap: "2.5rem",
})
;"##### ; "75")]
#[test_case(r#####"tw`gap-y-11`"#####, r#####"({
  rowGap: "2.75rem",
})
;"##### ; "76")]
#[test_case(r#####"tw`gap-y-12`"#####, r#####"({
  rowGap: "3rem",
})
;"##### ; "77")]
#[test_case(r#####"tw`gap-y-16`"#####, r#####"({
  rowGap: "4rem",
})
;"##### ; "78")]
#[test_case(r#####"tw`gap-y-20`"#####, r#####"({
  rowGap: "5rem",
})
;"##### ; "79")]
#[test_case(r#####"tw`gap-y-24`"#####, r#####"({
  rowGap: "6rem",
})
;"##### ; "80")]
#[test_case(r#####"tw`gap-y-28`"#####, r#####"({
  rowGap: "7rem",
})
;"##### ; "81")]
#[test_case(r#####"tw`gap-y-32`"#####, r#####"({
  rowGap: "8rem",
})
;"##### ; "82")]
#[test_case(r#####"tw`gap-y-36`"#####, r#####"({
  rowGap: "9rem",
})
;"##### ; "83")]
#[test_case(r#####"tw`gap-y-40`"#####, r#####"({
  rowGap: "10rem",
})
;"##### ; "84")]
#[test_case(r#####"tw`gap-y-44`"#####, r#####"({
  rowGap: "11rem",
})
;"##### ; "85")]
#[test_case(r#####"tw`gap-y-48`"#####, r#####"({
  rowGap: "12rem",
})
;"##### ; "86")]
#[test_case(r#####"tw`gap-y-52`"#####, r#####"({
  rowGap: "13rem",
})
;"##### ; "87")]
#[test_case(r#####"tw`gap-y-56`"#####, r#####"({
  rowGap: "14rem",
})
;"##### ; "88")]
#[test_case(r#####"tw`gap-y-60`"#####, r#####"({
  rowGap: "15rem",
})
;"##### ; "89")]
#[test_case(r#####"tw`gap-y-64`"#####, r#####"({
  rowGap: "16rem",
})
;"##### ; "90")]
#[test_case(r#####"tw`gap-y-72`"#####, r#####"({
  rowGap: "18rem",
})
;"##### ; "91")]
#[test_case(r#####"tw`gap-y-80`"#####, r#####"({
  rowGap: "20rem",
})
;"##### ; "92")]
#[test_case(r#####"tw`gap-y-96`"#####, r#####"({
  rowGap: "24rem",
})
;"##### ; "93")]
#[test_case(r#####"tw`gap-y-px`"#####, r#####"({
  rowGap: "1px",
})
;"##### ; "94")]
#[test_case(r#####"tw`gap-y-[2.75rem]`"#####, r#####"({
  rowGap: "2.75rem",
})"##### ; "95")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
