use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`space`"#####, r#####"({
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
#[test_case(r#####"tw`space-x-0`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(0px * var(--tw-space-x-reverse))",
    marginLeft: "calc(0px * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`space-x-0.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(0.125rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(0.125rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`space-x-1`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(0.25rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(0.25rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`space-x-1.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(0.375rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(0.375rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`space-x-2`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(0.5rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(0.5rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "5")]
#[test_case(r#####"tw`space-x-2.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(0.625rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(0.625rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "6")]
#[test_case(r#####"tw`space-x-3`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(0.75rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(0.75rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "7")]
#[test_case(r#####"tw`space-x-3.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(0.875rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(0.875rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "8")]
#[test_case(r#####"tw`space-x-4`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(1rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(1rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "9")]
#[test_case(r#####"tw`space-x-5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(1.25rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(1.25rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "10")]
#[test_case(r#####"tw`space-x-6`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(1.5rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(1.5rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "11")]
#[test_case(r#####"tw`space-x-7`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(1.75rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(1.75rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`space-x-8`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(2rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(2rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "13")]
#[test_case(r#####"tw`space-x-9`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(2.25rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(2.25rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "14")]
#[test_case(r#####"tw`space-x-10`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(2.5rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(2.5rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "15")]
#[test_case(r#####"tw`space-x-11`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(2.75rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(2.75rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "16")]
#[test_case(r#####"tw`space-x-12`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(3rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(3rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "17")]
#[test_case(r#####"tw`space-x-14`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(3.5rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(3.5rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "18")]
#[test_case(r#####"tw`space-x-16`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(4rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(4rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "19")]
#[test_case(r#####"tw`space-x-20`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(5rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(5rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "20")]
#[test_case(r#####"tw`space-x-24`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(6rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(6rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "21")]
#[test_case(r#####"tw`space-x-28`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(7rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(7rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "22")]
#[test_case(r#####"tw`space-x-32`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(8rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(8rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "23")]
#[test_case(r#####"tw`space-x-36`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(9rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(9rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "24")]
#[test_case(r#####"tw`space-x-40`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(10rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(10rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "25")]
#[test_case(r#####"tw`space-x-44`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(11rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(11rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "26")]
#[test_case(r#####"tw`space-x-48`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(12rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(12rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "27")]
#[test_case(r#####"tw`space-x-52`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(13rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(13rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "28")]
#[test_case(r#####"tw`space-x-56`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(14rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(14rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "29")]
#[test_case(r#####"tw`space-x-60`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(15rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(15rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "30")]
#[test_case(r#####"tw`space-x-64`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(16rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(16rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "31")]
#[test_case(r#####"tw`space-x-72`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(18rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(18rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "32")]
#[test_case(r#####"tw`space-x-80`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(20rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(20rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "33")]
#[test_case(r#####"tw`space-x-96`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(24rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(24rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "34")]
#[test_case(r#####"tw`space-x-px`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(1px * var(--tw-space-x-reverse))",
    marginLeft: "calc(1px * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "35")]
#[test_case(r#####"tw`space-y-0`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(0px * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(0px * var(--tw-space-y-reverse))",
  },
})
;"##### ; "36")]
#[test_case(r#####"tw`space-y-0.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(0.125rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(0.125rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "37")]
#[test_case(r#####"tw`space-y-1`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(0.25rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(0.25rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "38")]
#[test_case(r#####"tw`space-y-1.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(0.375rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(0.375rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "39")]
#[test_case(r#####"tw`space-y-2`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(0.5rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(0.5rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "40")]
#[test_case(r#####"tw`space-y-2.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(0.625rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(0.625rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "41")]
#[test_case(r#####"tw`space-y-3`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(0.75rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(0.75rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "42")]
#[test_case(r#####"tw`space-y-3.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(0.875rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(0.875rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "43")]
#[test_case(r#####"tw`space-y-4`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(1rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(1rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "44")]
#[test_case(r#####"tw`space-y-5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(1.25rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(1.25rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "45")]
#[test_case(r#####"tw`space-y-6`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(1.5rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(1.5rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "46")]
#[test_case(r#####"tw`space-y-7`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(1.75rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(1.75rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "47")]
#[test_case(r#####"tw`space-y-8`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(2rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(2rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "48")]
#[test_case(r#####"tw`space-y-9`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(2.25rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(2.25rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "49")]
#[test_case(r#####"tw`space-y-10`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(2.5rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(2.5rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "50")]
#[test_case(r#####"tw`space-y-12`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(3rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(3rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "51")]
#[test_case(r#####"tw`space-y-14`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(3.5rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(3.5rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "52")]
#[test_case(r#####"tw`space-y-16`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(4rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(4rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "53")]
#[test_case(r#####"tw`space-y-20`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(5rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(5rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "54")]
#[test_case(r#####"tw`space-y-24`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(6rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(6rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "55")]
#[test_case(r#####"tw`space-y-28`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(7rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(7rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "56")]
#[test_case(r#####"tw`space-y-32`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(8rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(8rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "57")]
#[test_case(r#####"tw`space-y-36`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(9rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(9rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "58")]
#[test_case(r#####"tw`space-y-40`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(10rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(10rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "59")]
#[test_case(r#####"tw`space-y-44`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(11rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(11rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "60")]
#[test_case(r#####"tw`space-y-48`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(12rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(12rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "61")]
#[test_case(r#####"tw`space-y-52`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(13rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(13rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "62")]
#[test_case(r#####"tw`space-y-56`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(14rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(14rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "63")]
#[test_case(r#####"tw`space-y-60`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(15rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(15rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "64")]
#[test_case(r#####"tw`space-y-64`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(16rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(16rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "65")]
#[test_case(r#####"tw`space-y-72`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(18rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(18rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "66")]
#[test_case(r#####"tw`space-y-80`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(20rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(20rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "67")]
#[test_case(r#####"tw`space-y-px`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(1px * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(1px * var(--tw-space-y-reverse))",
  },
})
;"##### ; "68")]
#[test_case(r#####"tw`-space-x-0`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-0px * var(--tw-space-x-reverse))",
    marginLeft: "calc(-0px * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "69")]
#[test_case(r#####"tw`-space-x-0.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-0.125rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-0.125rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "70")]
#[test_case(r#####"tw`-space-x-1`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-0.25rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-0.25rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "71")]
#[test_case(r#####"tw`-space-x-1.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-0.375rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-0.375rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "72")]
#[test_case(r#####"tw`-space-x-2`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-0.5rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-0.5rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "73")]
#[test_case(r#####"tw`-space-x-2.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-0.625rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-0.625rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "74")]
#[test_case(r#####"tw`-space-x-3`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-0.75rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-0.75rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "75")]
#[test_case(r#####"tw`-space-x-3.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-0.875rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-0.875rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "76")]
#[test_case(r#####"tw`-space-x-4`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-1rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-1rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "77")]
#[test_case(r#####"tw`-space-x-5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-1.25rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-1.25rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "78")]
#[test_case(r#####"tw`-space-x-6`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-1.5rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-1.5rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "79")]
#[test_case(r#####"tw`-space-x-7`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-1.75rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-1.75rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "80")]
#[test_case(r#####"tw`-space-x-8`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-2rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-2rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "81")]
#[test_case(r#####"tw`-space-x-9`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-2.25rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-2.25rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "82")]
#[test_case(r#####"tw`-space-x-10`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-2.5rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-2.5rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "83")]
#[test_case(r#####"tw`-space-x-12`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-3rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-3rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "84")]
#[test_case(r#####"tw`-space-x-14`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-3.5rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-3.5rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "85")]
#[test_case(r#####"tw`-space-x-16`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-4rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-4rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "86")]
#[test_case(r#####"tw`-space-x-20`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-5rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-5rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "87")]
#[test_case(r#####"tw`-space-x-24`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-6rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-6rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "88")]
#[test_case(r#####"tw`-space-x-28`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-7rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-7rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "89")]
#[test_case(r#####"tw`-space-x-32`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-8rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-8rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "90")]
#[test_case(r#####"tw`-space-x-36`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-9rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-9rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "91")]
#[test_case(r#####"tw`-space-x-40`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-10rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-10rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "92")]
#[test_case(r#####"tw`-space-x-44`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-11rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-11rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "93")]
#[test_case(r#####"tw`-space-x-48`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-12rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-12rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "94")]
#[test_case(r#####"tw`-space-x-52`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-13rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-13rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "95")]
#[test_case(r#####"tw`-space-x-56`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-14rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-14rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "96")]
#[test_case(r#####"tw`-space-x-60`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-15rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-15rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "97")]
#[test_case(r#####"tw`-space-x-64`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-16rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-16rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "98")]
#[test_case(r#####"tw`-space-x-72`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-18rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-18rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "99")]
#[test_case(r#####"tw`-space-x-80`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-20rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-20rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "100")]
#[test_case(r#####"tw`-space-x-96`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-24rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(-24rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "101")]
#[test_case(r#####"tw`-space-x-px`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-1px * var(--tw-space-x-reverse))",
    marginLeft: "calc(-1px * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "102")]
#[test_case(r#####"tw`-space-y-0`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-0px * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-0px * var(--tw-space-y-reverse))",
  },
})
;"##### ; "103")]
#[test_case(r#####"tw`-space-y-0.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-0.125rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-0.125rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "104")]
#[test_case(r#####"tw`-space-y-1`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-0.25rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-0.25rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "105")]
#[test_case(r#####"tw`-space-y-1.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-0.375rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-0.375rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "106")]
#[test_case(r#####"tw`-space-y-2`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-0.5rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-0.5rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "107")]
#[test_case(r#####"tw`-space-y-2.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-0.625rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-0.625rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "108")]
#[test_case(r#####"tw`-space-y-3`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-0.75rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-0.75rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "109")]
#[test_case(r#####"tw`-space-y-3.5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-0.875rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-0.875rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "110")]
#[test_case(r#####"tw`-space-y-4`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-1rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-1rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "111")]
#[test_case(r#####"tw`-space-y-5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-1.25rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-1.25rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "112")]
#[test_case(r#####"tw`-space-y-6`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-1.5rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-1.5rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "113")]
#[test_case(r#####"tw`-space-y-7`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-1.75rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-1.75rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "114")]
#[test_case(r#####"tw`-space-y-8`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-2rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-2rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "115")]
#[test_case(r#####"tw`-space-y-9`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-2.25rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-2.25rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "116")]
#[test_case(r#####"tw`-space-y-10`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-2.5rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-2.5rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "117")]
#[test_case(r#####"tw`-space-y-12`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-3rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-3rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "118")]
#[test_case(r#####"tw`-space-y-14`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-3.5rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-3.5rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "119")]
#[test_case(r#####"tw`-space-y-16`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-4rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-4rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "120")]
#[test_case(r#####"tw`-space-y-20`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-5rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-5rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "121")]
#[test_case(r#####"tw`-space-y-24`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-6rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-6rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "122")]
#[test_case(r#####"tw`-space-y-28`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-7rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-7rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "123")]
#[test_case(r#####"tw`-space-y-32`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-8rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-8rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "124")]
#[test_case(r#####"tw`-space-y-36`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-9rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-9rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "125")]
#[test_case(r#####"tw`-space-y-40`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-10rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-10rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "126")]
#[test_case(r#####"tw`-space-y-44`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-11rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-11rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "127")]
#[test_case(r#####"tw`-space-y-48`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-12rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-12rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "128")]
#[test_case(r#####"tw`-space-y-52`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-13rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-13rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "129")]
#[test_case(r#####"tw`-space-y-56`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-14rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-14rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "130")]
#[test_case(r#####"tw`-space-y-60`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-15rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-15rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "131")]
#[test_case(r#####"tw`-space-y-64`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-16rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-16rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "132")]
#[test_case(r#####"tw`-space-y-72`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-18rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-18rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "133")]
#[test_case(r#####"tw`-space-y-80`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-20rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-20rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "134")]
#[test_case(r#####"tw`-space-y-96`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-24rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-24rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "135")]
#[test_case(r#####"tw`-space-y-px`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-1px * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-1px * var(--tw-space-y-reverse))",
  },
})
;"##### ; "136")]
#[test_case(r#####"tw`space-x-reverse`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "1",
  },
})
;"##### ; "137")]
#[test_case(r#####"tw`space-y-reverse`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "1",
  },
})
;"##### ; "138")]
#[test_case(r#####"tw`space-x-[5px]`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(5px * var(--tw-space-x-reverse))",
    marginLeft: "calc(5px * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "139")]
#[test_case(r#####"tw`space-y-[5px]`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(5px * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(5px * var(--tw-space-y-reverse))",
  },
})
;"##### ; "140")]
#[test_case(r#####"tw`-space-x-[5px]`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(-5px * var(--tw-space-x-reverse))",
    marginLeft: "calc(-5px * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "141")]
#[test_case(r#####"tw`-space-y-[5px]`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "0",
    marginTop: "calc(-5px * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(-5px * var(--tw-space-y-reverse))",
  },
})
;"##### ; "142")]
#[test_case(r#####"tw`space-x-reverse space-x-0`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "1",
    marginRight: "calc(0px * var(--tw-space-x-reverse))",
    marginLeft: "calc(0px * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "143")]
#[test_case(r#####"tw`space-x-0 space-x-reverse`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "1",
    marginRight: "calc(0px * var(--tw-space-x-reverse))",
    marginLeft: "calc(0px * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "144")]
#[test_case(r#####"tw`space-y-reverse space-y-0`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "1",
    marginTop: "calc(0px * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(0px * var(--tw-space-y-reverse))",
  },
})
;"##### ; "145")]
#[test_case(r#####"tw`space-y-0 space-y-reverse`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "1",
    marginTop: "calc(0px * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(0px * var(--tw-space-y-reverse))",
  },
})
;"##### ; "146")]
#[test_case(r#####"tw`space-x-reverse space-x-32`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "1",
    marginRight: "calc(8rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(8rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "147")]
#[test_case(r#####"tw`space-x-32 space-x-reverse`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "1",
    marginRight: "calc(8rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(8rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "148")]
#[test_case(r#####"tw`space-y-reverse space-y-32`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "1",
    marginTop: "calc(8rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(8rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "149")]
#[test_case(r#####"tw`space-y-32 space-y-reverse`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "1",
    marginTop: "calc(8rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(8rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "150")]
#[test_case(r#####"tw`space-x-reverse space-x-px`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "1",
    marginRight: "calc(1px * var(--tw-space-x-reverse))",
    marginLeft: "calc(1px * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "151")]
#[test_case(r#####"tw`space-x-px space-x-reverse`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "1",
    marginRight: "calc(1px * var(--tw-space-x-reverse))",
    marginLeft: "calc(1px * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "152")]
#[test_case(r#####"tw`space-y-reverse space-y-px`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "1",
    marginTop: "calc(1px * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(1px * var(--tw-space-y-reverse))",
  },
})
;"##### ; "153")]
#[test_case(r#####"tw`space-y-px space-y-reverse`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "1",
    marginTop: "calc(1px * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(1px * var(--tw-space-y-reverse))",
  },
})
;"##### ; "154")]
#[test_case(r#####"tw`space-x-reverse space-x-12`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "1",
    marginRight: "calc(3rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(3rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "155")]
#[test_case(r#####"tw`space-x-12 space-x-reverse`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "1",
    marginRight: "calc(3rem * var(--tw-space-x-reverse))",
    marginLeft: "calc(3rem * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "156")]
#[test_case(r#####"tw`space-y-reverse space-y-12`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "1",
    marginTop: "calc(3rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(3rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "157")]
#[test_case(r#####"tw`space-y-12 space-y-reverse`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-y-reverse": "1",
    marginTop: "calc(3rem * calc(1 - var(--tw-space-y-reverse)))",
    marginBottom: "calc(3rem * var(--tw-space-y-reverse))",
  },
})
;"##### ; "158")]
#[test_case(r#####"tw`space-x-[20cm]`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(20cm * var(--tw-space-x-reverse))",
    marginLeft: "calc(20cm * calc(1 - var(--tw-space-x-reverse)))",
  },
})
;"##### ; "159")]
#[test_case(r#####"tw`space-x-[calc(20%-1cm)]`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-space-x-reverse": "0",
    marginRight: "calc(calc(20% - 1cm) * var(--tw-space-x-reverse))",
    marginLeft: "calc(calc(20% - 1cm) * calc(1 - var(--tw-space-x-reverse)))",
  },
})"##### ; "160")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
