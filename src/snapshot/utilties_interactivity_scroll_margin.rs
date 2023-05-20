use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`scrollMargin`"#####, r#####"({
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
#[test_case(r#####"tw`scroll-m-0`"#####, r#####"({
  scrollMargin: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`scroll-mx-0`"#####, r#####"({
  scrollMarginLeft: "0px",
  scrollMarginRight: "0px",
})
;"##### ; "2")]
#[test_case(r#####"tw`scroll-my-0`"#####, r#####"({
  scrollMarginTop: "0px",
  scrollMarginBottom: "0px",
})
;"##### ; "3")]
#[test_case(r#####"tw`scroll-mt-0`"#####, r#####"({
  scrollMarginTop: "0px",
})
;"##### ; "4")]
#[test_case(r#####"tw`scroll-mr-0`"#####, r#####"({
  scrollMarginRight: "0px",
})
;"##### ; "5")]
#[test_case(r#####"tw`scroll-mb-0`"#####, r#####"({
  scrollMarginBottom: "0px",
})
;"##### ; "6")]
#[test_case(r#####"tw`scroll-ml-0`"#####, r#####"({
  scrollMarginLeft: "0px",
})
;"##### ; "7")]
#[test_case(r#####"tw`scroll-m-px`"#####, r#####"({
  scrollMargin: "1px",
})
;"##### ; "8")]
#[test_case(r#####"tw`scroll-mx-px`"#####, r#####"({
  scrollMarginLeft: "1px",
  scrollMarginRight: "1px",
})
;"##### ; "9")]
#[test_case(r#####"tw`scroll-my-px`"#####, r#####"({
  scrollMarginTop: "1px",
  scrollMarginBottom: "1px",
})
;"##### ; "10")]
#[test_case(r#####"tw`scroll-mt-px`"#####, r#####"({
  scrollMarginTop: "1px",
})
;"##### ; "11")]
#[test_case(r#####"tw`scroll-mr-px`"#####, r#####"({
  scrollMarginRight: "1px",
})
;"##### ; "12")]
#[test_case(r#####"tw`scroll-mb-px`"#####, r#####"({
  scrollMarginBottom: "1px",
})
;"##### ; "13")]
#[test_case(r#####"tw`scroll-ml-px`"#####, r#####"({
  scrollMarginLeft: "1px",
})
;"##### ; "14")]
#[test_case(r#####"tw`scroll-m-0`"#####, r#####"({
  scrollMargin: "0px",
})
;"##### ; "15")]
#[test_case(r#####"tw`scroll-mx-0`"#####, r#####"({
  scrollMarginLeft: "0px",
  scrollMarginRight: "0px",
})
;"##### ; "16")]
#[test_case(r#####"tw`scroll-my-0`"#####, r#####"({
  scrollMarginTop: "0px",
  scrollMarginBottom: "0px",
})
;"##### ; "17")]
#[test_case(r#####"tw`scroll-mt-0`"#####, r#####"({
  scrollMarginTop: "0px",
})
;"##### ; "18")]
#[test_case(r#####"tw`scroll-mr-0`"#####, r#####"({
  scrollMarginRight: "0px",
})
;"##### ; "19")]
#[test_case(r#####"tw`scroll-mb-0`"#####, r#####"({
  scrollMarginBottom: "0px",
})
;"##### ; "20")]
#[test_case(r#####"tw`scroll-ml-0`"#####, r#####"({
  scrollMarginLeft: "0px",
})
;"##### ; "21")]
#[test_case(r#####"tw`scroll-m-1`"#####, r#####"({
  scrollMargin: "0.25rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`scroll-mx-1`"#####, r#####"({
  scrollMarginLeft: "0.25rem",
  scrollMarginRight: "0.25rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`scroll-my-1`"#####, r#####"({
  scrollMarginTop: "0.25rem",
  scrollMarginBottom: "0.25rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`scroll-mt-1`"#####, r#####"({
  scrollMarginTop: "0.25rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`scroll-mr-1`"#####, r#####"({
  scrollMarginRight: "0.25rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`scroll-mb-1`"#####, r#####"({
  scrollMarginBottom: "0.25rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`scroll-ml-1`"#####, r#####"({
  scrollMarginLeft: "0.25rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`scroll-m-1`"#####, r#####"({
  scrollMargin: "0.25rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`scroll-mx-1`"#####, r#####"({
  scrollMarginLeft: "0.25rem",
  scrollMarginRight: "0.25rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`scroll-my-1`"#####, r#####"({
  scrollMarginTop: "0.25rem",
  scrollMarginBottom: "0.25rem",
})
;"##### ; "31")]
#[test_case(r#####"tw`scroll-mt-1`"#####, r#####"({
  scrollMarginTop: "0.25rem",
})
;"##### ; "32")]
#[test_case(r#####"tw`scroll-mr-1`"#####, r#####"({
  scrollMarginRight: "0.25rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`scroll-mb-1`"#####, r#####"({
  scrollMarginBottom: "0.25rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`scroll-ml-1`"#####, r#####"({
  scrollMarginLeft: "0.25rem",
})
;"##### ; "35")]
#[test_case(r#####"tw`scroll-m-2`"#####, r#####"({
  scrollMargin: "0.5rem",
})
;"##### ; "36")]
#[test_case(r#####"tw`scroll-mx-2`"#####, r#####"({
  scrollMarginLeft: "0.5rem",
  scrollMarginRight: "0.5rem",
})
;"##### ; "37")]
#[test_case(r#####"tw`scroll-my-2`"#####, r#####"({
  scrollMarginTop: "0.5rem",
  scrollMarginBottom: "0.5rem",
})
;"##### ; "38")]
#[test_case(r#####"tw`scroll-mt-2`"#####, r#####"({
  scrollMarginTop: "0.5rem",
})
;"##### ; "39")]
#[test_case(r#####"tw`scroll-mr-2`"#####, r#####"({
  scrollMarginRight: "0.5rem",
})
;"##### ; "40")]
#[test_case(r#####"tw`scroll-mb-2`"#####, r#####"({
  scrollMarginBottom: "0.5rem",
})
;"##### ; "41")]
#[test_case(r#####"tw`scroll-ml-2`"#####, r#####"({
  scrollMarginLeft: "0.5rem",
})
;"##### ; "42")]
#[test_case(r#####"tw`scroll-m-2`"#####, r#####"({
  scrollMargin: "0.5rem",
})
;"##### ; "43")]
#[test_case(r#####"tw`scroll-mx-2`"#####, r#####"({
  scrollMarginLeft: "0.5rem",
  scrollMarginRight: "0.5rem",
})
;"##### ; "44")]
#[test_case(r#####"tw`scroll-my-2`"#####, r#####"({
  scrollMarginTop: "0.5rem",
  scrollMarginBottom: "0.5rem",
})
;"##### ; "45")]
#[test_case(r#####"tw`scroll-mt-2`"#####, r#####"({
  scrollMarginTop: "0.5rem",
})
;"##### ; "46")]
#[test_case(r#####"tw`scroll-mr-2`"#####, r#####"({
  scrollMarginRight: "0.5rem",
})
;"##### ; "47")]
#[test_case(r#####"tw`scroll-mb-2`"#####, r#####"({
  scrollMarginBottom: "0.5rem",
})
;"##### ; "48")]
#[test_case(r#####"tw`scroll-ml-2`"#####, r#####"({
  scrollMarginLeft: "0.5rem",
})
;"##### ; "49")]
#[test_case(r#####"tw`scroll-m-3`"#####, r#####"({
  scrollMargin: "0.75rem",
})
;"##### ; "50")]
#[test_case(r#####"tw`scroll-mx-3`"#####, r#####"({
  scrollMarginLeft: "0.75rem",
  scrollMarginRight: "0.75rem",
})
;"##### ; "51")]
#[test_case(r#####"tw`scroll-my-3`"#####, r#####"({
  scrollMarginTop: "0.75rem",
  scrollMarginBottom: "0.75rem",
})
;"##### ; "52")]
#[test_case(r#####"tw`scroll-mt-3`"#####, r#####"({
  scrollMarginTop: "0.75rem",
})
;"##### ; "53")]
#[test_case(r#####"tw`scroll-mr-3`"#####, r#####"({
  scrollMarginRight: "0.75rem",
})
;"##### ; "54")]
#[test_case(r#####"tw`scroll-mb-3`"#####, r#####"({
  scrollMarginBottom: "0.75rem",
})
;"##### ; "55")]
#[test_case(r#####"tw`scroll-ml-3`"#####, r#####"({
  scrollMarginLeft: "0.75rem",
})
;"##### ; "56")]
#[test_case(r#####"tw`scroll-m-3`"#####, r#####"({
  scrollMargin: "0.75rem",
})
;"##### ; "57")]
#[test_case(r#####"tw`scroll-mx-3`"#####, r#####"({
  scrollMarginLeft: "0.75rem",
  scrollMarginRight: "0.75rem",
})
;"##### ; "58")]
#[test_case(r#####"tw`scroll-my-3`"#####, r#####"({
  scrollMarginTop: "0.75rem",
  scrollMarginBottom: "0.75rem",
})
;"##### ; "59")]
#[test_case(r#####"tw`scroll-mt-3`"#####, r#####"({
  scrollMarginTop: "0.75rem",
})
;"##### ; "60")]
#[test_case(r#####"tw`scroll-mr-3`"#####, r#####"({
  scrollMarginRight: "0.75rem",
})
;"##### ; "61")]
#[test_case(r#####"tw`scroll-mb-3`"#####, r#####"({
  scrollMarginBottom: "0.75rem",
})
;"##### ; "62")]
#[test_case(r#####"tw`scroll-ml-3`"#####, r#####"({
  scrollMarginLeft: "0.75rem",
})
;"##### ; "63")]
#[test_case(r#####"tw`scroll-m-4`"#####, r#####"({
  scrollMargin: "1rem",
})
;"##### ; "64")]
#[test_case(r#####"tw`scroll-mx-4`"#####, r#####"({
  scrollMarginLeft: "1rem",
  scrollMarginRight: "1rem",
})
;"##### ; "65")]
#[test_case(r#####"tw`scroll-my-4`"#####, r#####"({
  scrollMarginTop: "1rem",
  scrollMarginBottom: "1rem",
})
;"##### ; "66")]
#[test_case(r#####"tw`scroll-mt-4`"#####, r#####"({
  scrollMarginTop: "1rem",
})
;"##### ; "67")]
#[test_case(r#####"tw`scroll-mr-4`"#####, r#####"({
  scrollMarginRight: "1rem",
})
;"##### ; "68")]
#[test_case(r#####"tw`scroll-mb-4`"#####, r#####"({
  scrollMarginBottom: "1rem",
})
;"##### ; "69")]
#[test_case(r#####"tw`scroll-ml-4`"#####, r#####"({
  scrollMarginLeft: "1rem",
})
;"##### ; "70")]
#[test_case(r#####"tw`scroll-m-5`"#####, r#####"({
  scrollMargin: "1.25rem",
})
;"##### ; "71")]
#[test_case(r#####"tw`scroll-mx-5`"#####, r#####"({
  scrollMarginLeft: "1.25rem",
  scrollMarginRight: "1.25rem",
})
;"##### ; "72")]
#[test_case(r#####"tw`scroll-my-5`"#####, r#####"({
  scrollMarginTop: "1.25rem",
  scrollMarginBottom: "1.25rem",
})
;"##### ; "73")]
#[test_case(r#####"tw`scroll-mt-5`"#####, r#####"({
  scrollMarginTop: "1.25rem",
})
;"##### ; "74")]
#[test_case(r#####"tw`scroll-mr-5`"#####, r#####"({
  scrollMarginRight: "1.25rem",
})
;"##### ; "75")]
#[test_case(r#####"tw`scroll-mb-5`"#####, r#####"({
  scrollMarginBottom: "1.25rem",
})
;"##### ; "76")]
#[test_case(r#####"tw`scroll-ml-5`"#####, r#####"({
  scrollMarginLeft: "1.25rem",
})
;"##### ; "77")]
#[test_case(r#####"tw`scroll-m-6`"#####, r#####"({
  scrollMargin: "1.5rem",
})
;"##### ; "78")]
#[test_case(r#####"tw`scroll-mx-6`"#####, r#####"({
  scrollMarginLeft: "1.5rem",
  scrollMarginRight: "1.5rem",
})
;"##### ; "79")]
#[test_case(r#####"tw`scroll-my-6`"#####, r#####"({
  scrollMarginTop: "1.5rem",
  scrollMarginBottom: "1.5rem",
})
;"##### ; "80")]
#[test_case(r#####"tw`scroll-mt-6`"#####, r#####"({
  scrollMarginTop: "1.5rem",
})
;"##### ; "81")]
#[test_case(r#####"tw`scroll-mr-6`"#####, r#####"({
  scrollMarginRight: "1.5rem",
})
;"##### ; "82")]
#[test_case(r#####"tw`scroll-mb-6`"#####, r#####"({
  scrollMarginBottom: "1.5rem",
})
;"##### ; "83")]
#[test_case(r#####"tw`scroll-ml-6`"#####, r#####"({
  scrollMarginLeft: "1.5rem",
})
;"##### ; "84")]
#[test_case(r#####"tw`scroll-m-7`"#####, r#####"({
  scrollMargin: "1.75rem",
})
;"##### ; "85")]
#[test_case(r#####"tw`scroll-mx-7`"#####, r#####"({
  scrollMarginLeft: "1.75rem",
  scrollMarginRight: "1.75rem",
})
;"##### ; "86")]
#[test_case(r#####"tw`scroll-my-7`"#####, r#####"({
  scrollMarginTop: "1.75rem",
  scrollMarginBottom: "1.75rem",
})
;"##### ; "87")]
#[test_case(r#####"tw`scroll-mt-7`"#####, r#####"({
  scrollMarginTop: "1.75rem",
})
;"##### ; "88")]
#[test_case(r#####"tw`scroll-mr-7`"#####, r#####"({
  scrollMarginRight: "1.75rem",
})
;"##### ; "89")]
#[test_case(r#####"tw`scroll-mb-7`"#####, r#####"({
  scrollMarginBottom: "1.75rem",
})
;"##### ; "90")]
#[test_case(r#####"tw`scroll-ml-7`"#####, r#####"({
  scrollMarginLeft: "1.75rem",
})
;"##### ; "91")]
#[test_case(r#####"tw`scroll-m-8`"#####, r#####"({
  scrollMargin: "2rem",
})
;"##### ; "92")]
#[test_case(r#####"tw`scroll-mx-8`"#####, r#####"({
  scrollMarginLeft: "2rem",
  scrollMarginRight: "2rem",
})
;"##### ; "93")]
#[test_case(r#####"tw`scroll-my-8`"#####, r#####"({
  scrollMarginTop: "2rem",
  scrollMarginBottom: "2rem",
})
;"##### ; "94")]
#[test_case(r#####"tw`scroll-mt-8`"#####, r#####"({
  scrollMarginTop: "2rem",
})
;"##### ; "95")]
#[test_case(r#####"tw`scroll-mr-8`"#####, r#####"({
  scrollMarginRight: "2rem",
})
;"##### ; "96")]
#[test_case(r#####"tw`scroll-mb-8`"#####, r#####"({
  scrollMarginBottom: "2rem",
})
;"##### ; "97")]
#[test_case(r#####"tw`scroll-ml-8`"#####, r#####"({
  scrollMarginLeft: "2rem",
})
;"##### ; "98")]
#[test_case(r#####"tw`scroll-m-9`"#####, r#####"({
  scrollMargin: "2.25rem",
})
;"##### ; "99")]
#[test_case(r#####"tw`scroll-mx-9`"#####, r#####"({
  scrollMarginLeft: "2.25rem",
  scrollMarginRight: "2.25rem",
})
;"##### ; "100")]
#[test_case(r#####"tw`scroll-my-9`"#####, r#####"({
  scrollMarginTop: "2.25rem",
  scrollMarginBottom: "2.25rem",
})
;"##### ; "101")]
#[test_case(r#####"tw`scroll-mt-9`"#####, r#####"({
  scrollMarginTop: "2.25rem",
})
;"##### ; "102")]
#[test_case(r#####"tw`scroll-mr-9`"#####, r#####"({
  scrollMarginRight: "2.25rem",
})
;"##### ; "103")]
#[test_case(r#####"tw`scroll-mb-9`"#####, r#####"({
  scrollMarginBottom: "2.25rem",
})
;"##### ; "104")]
#[test_case(r#####"tw`scroll-ml-9`"#####, r#####"({
  scrollMarginLeft: "2.25rem",
})
;"##### ; "105")]
#[test_case(r#####"tw`scroll-m-10`"#####, r#####"({
  scrollMargin: "2.5rem",
})
;"##### ; "106")]
#[test_case(r#####"tw`scroll-mx-10`"#####, r#####"({
  scrollMarginLeft: "2.5rem",
  scrollMarginRight: "2.5rem",
})
;"##### ; "107")]
#[test_case(r#####"tw`scroll-my-10`"#####, r#####"({
  scrollMarginTop: "2.5rem",
  scrollMarginBottom: "2.5rem",
})
;"##### ; "108")]
#[test_case(r#####"tw`scroll-mt-10`"#####, r#####"({
  scrollMarginTop: "2.5rem",
})
;"##### ; "109")]
#[test_case(r#####"tw`scroll-mr-10`"#####, r#####"({
  scrollMarginRight: "2.5rem",
})
;"##### ; "110")]
#[test_case(r#####"tw`scroll-mb-10`"#####, r#####"({
  scrollMarginBottom: "2.5rem",
})
;"##### ; "111")]
#[test_case(r#####"tw`scroll-ml-10`"#####, r#####"({
  scrollMarginLeft: "2.5rem",
})
;"##### ; "112")]
#[test_case(r#####"tw`scroll-m-11`"#####, r#####"({
  scrollMargin: "2.75rem",
})
;"##### ; "113")]
#[test_case(r#####"tw`scroll-mx-11`"#####, r#####"({
  scrollMarginLeft: "2.75rem",
  scrollMarginRight: "2.75rem",
})
;"##### ; "114")]
#[test_case(r#####"tw`scroll-my-11`"#####, r#####"({
  scrollMarginTop: "2.75rem",
  scrollMarginBottom: "2.75rem",
})
;"##### ; "115")]
#[test_case(r#####"tw`scroll-mt-11`"#####, r#####"({
  scrollMarginTop: "2.75rem",
})
;"##### ; "116")]
#[test_case(r#####"tw`scroll-mr-11`"#####, r#####"({
  scrollMarginRight: "2.75rem",
})
;"##### ; "117")]
#[test_case(r#####"tw`scroll-mb-11`"#####, r#####"({
  scrollMarginBottom: "2.75rem",
})
;"##### ; "118")]
#[test_case(r#####"tw`scroll-ml-11`"#####, r#####"({
  scrollMarginLeft: "2.75rem",
})
;"##### ; "119")]
#[test_case(r#####"tw`scroll-m-12`"#####, r#####"({
  scrollMargin: "3rem",
})
;"##### ; "120")]
#[test_case(r#####"tw`scroll-mx-12`"#####, r#####"({
  scrollMarginLeft: "3rem",
  scrollMarginRight: "3rem",
})
;"##### ; "121")]
#[test_case(r#####"tw`scroll-my-12`"#####, r#####"({
  scrollMarginTop: "3rem",
  scrollMarginBottom: "3rem",
})
;"##### ; "122")]
#[test_case(r#####"tw`scroll-mt-12`"#####, r#####"({
  scrollMarginTop: "3rem",
})
;"##### ; "123")]
#[test_case(r#####"tw`scroll-mr-12`"#####, r#####"({
  scrollMarginRight: "3rem",
})
;"##### ; "124")]
#[test_case(r#####"tw`scroll-mb-12`"#####, r#####"({
  scrollMarginBottom: "3rem",
})
;"##### ; "125")]
#[test_case(r#####"tw`scroll-ml-12`"#####, r#####"({
  scrollMarginLeft: "3rem",
})
;"##### ; "126")]
#[test_case(r#####"tw`scroll-m-14`"#####, r#####"({
  scrollMargin: "3.5rem",
})
;"##### ; "127")]
#[test_case(r#####"tw`scroll-mx-14`"#####, r#####"({
  scrollMarginLeft: "3.5rem",
  scrollMarginRight: "3.5rem",
})
;"##### ; "128")]
#[test_case(r#####"tw`scroll-my-14`"#####, r#####"({
  scrollMarginTop: "3.5rem",
  scrollMarginBottom: "3.5rem",
})
;"##### ; "129")]
#[test_case(r#####"tw`scroll-mt-14`"#####, r#####"({
  scrollMarginTop: "3.5rem",
})
;"##### ; "130")]
#[test_case(r#####"tw`scroll-mr-14`"#####, r#####"({
  scrollMarginRight: "3.5rem",
})
;"##### ; "131")]
#[test_case(r#####"tw`scroll-mb-14`"#####, r#####"({
  scrollMarginBottom: "3.5rem",
})
;"##### ; "132")]
#[test_case(r#####"tw`scroll-ml-14`"#####, r#####"({
  scrollMarginLeft: "3.5rem",
})
;"##### ; "133")]
#[test_case(r#####"tw`scroll-m-16`"#####, r#####"({
  scrollMargin: "4rem",
})
;"##### ; "134")]
#[test_case(r#####"tw`scroll-mx-16`"#####, r#####"({
  scrollMarginLeft: "4rem",
  scrollMarginRight: "4rem",
})
;"##### ; "135")]
#[test_case(r#####"tw`scroll-my-16`"#####, r#####"({
  scrollMarginTop: "4rem",
  scrollMarginBottom: "4rem",
})
;"##### ; "136")]
#[test_case(r#####"tw`scroll-mt-16`"#####, r#####"({
  scrollMarginTop: "4rem",
})
;"##### ; "137")]
#[test_case(r#####"tw`scroll-mr-16`"#####, r#####"({
  scrollMarginRight: "4rem",
})
;"##### ; "138")]
#[test_case(r#####"tw`scroll-mb-16`"#####, r#####"({
  scrollMarginBottom: "4rem",
})
;"##### ; "139")]
#[test_case(r#####"tw`scroll-ml-16`"#####, r#####"({
  scrollMarginLeft: "4rem",
})
;"##### ; "140")]
#[test_case(r#####"tw`scroll-m-20`"#####, r#####"({
  scrollMargin: "5rem",
})
;"##### ; "141")]
#[test_case(r#####"tw`scroll-mx-20`"#####, r#####"({
  scrollMarginLeft: "5rem",
  scrollMarginRight: "5rem",
})
;"##### ; "142")]
#[test_case(r#####"tw`scroll-my-20`"#####, r#####"({
  scrollMarginTop: "5rem",
  scrollMarginBottom: "5rem",
})
;"##### ; "143")]
#[test_case(r#####"tw`scroll-mt-20`"#####, r#####"({
  scrollMarginTop: "5rem",
})
;"##### ; "144")]
#[test_case(r#####"tw`scroll-mr-20`"#####, r#####"({
  scrollMarginRight: "5rem",
})
;"##### ; "145")]
#[test_case(r#####"tw`scroll-mb-20`"#####, r#####"({
  scrollMarginBottom: "5rem",
})
;"##### ; "146")]
#[test_case(r#####"tw`scroll-ml-20`"#####, r#####"({
  scrollMarginLeft: "5rem",
})
;"##### ; "147")]
#[test_case(r#####"tw`scroll-m-24`"#####, r#####"({
  scrollMargin: "6rem",
})
;"##### ; "148")]
#[test_case(r#####"tw`scroll-mx-24`"#####, r#####"({
  scrollMarginLeft: "6rem",
  scrollMarginRight: "6rem",
})
;"##### ; "149")]
#[test_case(r#####"tw`scroll-my-24`"#####, r#####"({
  scrollMarginTop: "6rem",
  scrollMarginBottom: "6rem",
})
;"##### ; "150")]
#[test_case(r#####"tw`scroll-mt-24`"#####, r#####"({
  scrollMarginTop: "6rem",
})
;"##### ; "151")]
#[test_case(r#####"tw`scroll-mr-24`"#####, r#####"({
  scrollMarginRight: "6rem",
})
;"##### ; "152")]
#[test_case(r#####"tw`scroll-mb-24`"#####, r#####"({
  scrollMarginBottom: "6rem",
})
;"##### ; "153")]
#[test_case(r#####"tw`scroll-ml-24`"#####, r#####"({
  scrollMarginLeft: "6rem",
})
;"##### ; "154")]
#[test_case(r#####"tw`scroll-m-28`"#####, r#####"({
  scrollMargin: "7rem",
})
;"##### ; "155")]
#[test_case(r#####"tw`scroll-mx-28`"#####, r#####"({
  scrollMarginLeft: "7rem",
  scrollMarginRight: "7rem",
})
;"##### ; "156")]
#[test_case(r#####"tw`scroll-my-28`"#####, r#####"({
  scrollMarginTop: "7rem",
  scrollMarginBottom: "7rem",
})
;"##### ; "157")]
#[test_case(r#####"tw`scroll-mt-28`"#####, r#####"({
  scrollMarginTop: "7rem",
})
;"##### ; "158")]
#[test_case(r#####"tw`scroll-mr-28`"#####, r#####"({
  scrollMarginRight: "7rem",
})
;"##### ; "159")]
#[test_case(r#####"tw`scroll-mb-28`"#####, r#####"({
  scrollMarginBottom: "7rem",
})
;"##### ; "160")]
#[test_case(r#####"tw`scroll-ml-28`"#####, r#####"({
  scrollMarginLeft: "7rem",
})
;"##### ; "161")]
#[test_case(r#####"tw`scroll-m-32`"#####, r#####"({
  scrollMargin: "8rem",
})
;"##### ; "162")]
#[test_case(r#####"tw`scroll-mx-32`"#####, r#####"({
  scrollMarginLeft: "8rem",
  scrollMarginRight: "8rem",
})
;"##### ; "163")]
#[test_case(r#####"tw`scroll-my-32`"#####, r#####"({
  scrollMarginTop: "8rem",
  scrollMarginBottom: "8rem",
})
;"##### ; "164")]
#[test_case(r#####"tw`scroll-mt-32`"#####, r#####"({
  scrollMarginTop: "8rem",
})
;"##### ; "165")]
#[test_case(r#####"tw`scroll-mr-32`"#####, r#####"({
  scrollMarginRight: "8rem",
})
;"##### ; "166")]
#[test_case(r#####"tw`scroll-mb-32`"#####, r#####"({
  scrollMarginBottom: "8rem",
})
;"##### ; "167")]
#[test_case(r#####"tw`scroll-ml-32`"#####, r#####"({
  scrollMarginLeft: "8rem",
})
;"##### ; "168")]
#[test_case(r#####"tw`scroll-m-36`"#####, r#####"({
  scrollMargin: "9rem",
})
;"##### ; "169")]
#[test_case(r#####"tw`scroll-mx-36`"#####, r#####"({
  scrollMarginLeft: "9rem",
  scrollMarginRight: "9rem",
})
;"##### ; "170")]
#[test_case(r#####"tw`scroll-my-36`"#####, r#####"({
  scrollMarginTop: "9rem",
  scrollMarginBottom: "9rem",
})
;"##### ; "171")]
#[test_case(r#####"tw`scroll-mt-36`"#####, r#####"({
  scrollMarginTop: "9rem",
})
;"##### ; "172")]
#[test_case(r#####"tw`scroll-mr-36`"#####, r#####"({
  scrollMarginRight: "9rem",
})
;"##### ; "173")]
#[test_case(r#####"tw`scroll-mb-36`"#####, r#####"({
  scrollMarginBottom: "9rem",
})
;"##### ; "174")]
#[test_case(r#####"tw`scroll-ml-36`"#####, r#####"({
  scrollMarginLeft: "9rem",
})
;"##### ; "175")]
#[test_case(r#####"tw`scroll-m-40`"#####, r#####"({
  scrollMargin: "10rem",
})
;"##### ; "176")]
#[test_case(r#####"tw`scroll-mx-40`"#####, r#####"({
  scrollMarginLeft: "10rem",
  scrollMarginRight: "10rem",
})
;"##### ; "177")]
#[test_case(r#####"tw`scroll-my-40`"#####, r#####"({
  scrollMarginTop: "10rem",
  scrollMarginBottom: "10rem",
})
;"##### ; "178")]
#[test_case(r#####"tw`scroll-mt-40`"#####, r#####"({
  scrollMarginTop: "10rem",
})
;"##### ; "179")]
#[test_case(r#####"tw`scroll-mr-40`"#####, r#####"({
  scrollMarginRight: "10rem",
})
;"##### ; "180")]
#[test_case(r#####"tw`scroll-mb-40`"#####, r#####"({
  scrollMarginBottom: "10rem",
})
;"##### ; "181")]
#[test_case(r#####"tw`scroll-ml-40`"#####, r#####"({
  scrollMarginLeft: "10rem",
})
;"##### ; "182")]
#[test_case(r#####"tw`scroll-m-44`"#####, r#####"({
  scrollMargin: "11rem",
})
;"##### ; "183")]
#[test_case(r#####"tw`scroll-mx-44`"#####, r#####"({
  scrollMarginLeft: "11rem",
  scrollMarginRight: "11rem",
})
;"##### ; "184")]
#[test_case(r#####"tw`scroll-my-44`"#####, r#####"({
  scrollMarginTop: "11rem",
  scrollMarginBottom: "11rem",
})
;"##### ; "185")]
#[test_case(r#####"tw`scroll-mt-44`"#####, r#####"({
  scrollMarginTop: "11rem",
})
;"##### ; "186")]
#[test_case(r#####"tw`scroll-mr-44`"#####, r#####"({
  scrollMarginRight: "11rem",
})
;"##### ; "187")]
#[test_case(r#####"tw`scroll-mb-44`"#####, r#####"({
  scrollMarginBottom: "11rem",
})
;"##### ; "188")]
#[test_case(r#####"tw`scroll-ml-44`"#####, r#####"({
  scrollMarginLeft: "11rem",
})
;"##### ; "189")]
#[test_case(r#####"tw`scroll-m-48`"#####, r#####"({
  scrollMargin: "12rem",
})
;"##### ; "190")]
#[test_case(r#####"tw`scroll-mx-48`"#####, r#####"({
  scrollMarginLeft: "12rem",
  scrollMarginRight: "12rem",
})
;"##### ; "191")]
#[test_case(r#####"tw`scroll-my-48`"#####, r#####"({
  scrollMarginTop: "12rem",
  scrollMarginBottom: "12rem",
})
;"##### ; "192")]
#[test_case(r#####"tw`scroll-mt-48`"#####, r#####"({
  scrollMarginTop: "12rem",
})
;"##### ; "193")]
#[test_case(r#####"tw`scroll-mr-48`"#####, r#####"({
  scrollMarginRight: "12rem",
})
;"##### ; "194")]
#[test_case(r#####"tw`scroll-mb-48`"#####, r#####"({
  scrollMarginBottom: "12rem",
})
;"##### ; "195")]
#[test_case(r#####"tw`scroll-ml-48`"#####, r#####"({
  scrollMarginLeft: "12rem",
})
;"##### ; "196")]
#[test_case(r#####"tw`scroll-m-52`"#####, r#####"({
  scrollMargin: "13rem",
})
;"##### ; "197")]
#[test_case(r#####"tw`scroll-mx-52`"#####, r#####"({
  scrollMarginLeft: "13rem",
  scrollMarginRight: "13rem",
})
;"##### ; "198")]
#[test_case(r#####"tw`scroll-my-52`"#####, r#####"({
  scrollMarginTop: "13rem",
  scrollMarginBottom: "13rem",
})
;"##### ; "199")]
#[test_case(r#####"tw`scroll-mt-52`"#####, r#####"({
  scrollMarginTop: "13rem",
})
;"##### ; "200")]
#[test_case(r#####"tw`scroll-mr-52`"#####, r#####"({
  scrollMarginRight: "13rem",
})
;"##### ; "201")]
#[test_case(r#####"tw`scroll-mb-52`"#####, r#####"({
  scrollMarginBottom: "13rem",
})
;"##### ; "202")]
#[test_case(r#####"tw`scroll-ml-52`"#####, r#####"({
  scrollMarginLeft: "13rem",
})
;"##### ; "203")]
#[test_case(r#####"tw`scroll-m-56`"#####, r#####"({
  scrollMargin: "14rem",
})
;"##### ; "204")]
#[test_case(r#####"tw`scroll-mx-56`"#####, r#####"({
  scrollMarginLeft: "14rem",
  scrollMarginRight: "14rem",
})
;"##### ; "205")]
#[test_case(r#####"tw`scroll-my-56`"#####, r#####"({
  scrollMarginTop: "14rem",
  scrollMarginBottom: "14rem",
})
;"##### ; "206")]
#[test_case(r#####"tw`scroll-mt-56`"#####, r#####"({
  scrollMarginTop: "14rem",
})
;"##### ; "207")]
#[test_case(r#####"tw`scroll-mr-56`"#####, r#####"({
  scrollMarginRight: "14rem",
})
;"##### ; "208")]
#[test_case(r#####"tw`scroll-mb-56`"#####, r#####"({
  scrollMarginBottom: "14rem",
})
;"##### ; "209")]
#[test_case(r#####"tw`scroll-ml-56`"#####, r#####"({
  scrollMarginLeft: "14rem",
})
;"##### ; "210")]
#[test_case(r#####"tw`scroll-m-60`"#####, r#####"({
  scrollMargin: "15rem",
})
;"##### ; "211")]
#[test_case(r#####"tw`scroll-mx-60`"#####, r#####"({
  scrollMarginLeft: "15rem",
  scrollMarginRight: "15rem",
})
;"##### ; "212")]
#[test_case(r#####"tw`scroll-my-60`"#####, r#####"({
  scrollMarginTop: "15rem",
  scrollMarginBottom: "15rem",
})
;"##### ; "213")]
#[test_case(r#####"tw`scroll-mt-60`"#####, r#####"({
  scrollMarginTop: "15rem",
})
;"##### ; "214")]
#[test_case(r#####"tw`scroll-mr-60`"#####, r#####"({
  scrollMarginRight: "15rem",
})
;"##### ; "215")]
#[test_case(r#####"tw`scroll-mb-60`"#####, r#####"({
  scrollMarginBottom: "15rem",
})
;"##### ; "216")]
#[test_case(r#####"tw`scroll-ml-60`"#####, r#####"({
  scrollMarginLeft: "15rem",
})
;"##### ; "217")]
#[test_case(r#####"tw`scroll-m-64`"#####, r#####"({
  scrollMargin: "16rem",
})
;"##### ; "218")]
#[test_case(r#####"tw`scroll-mx-64`"#####, r#####"({
  scrollMarginLeft: "16rem",
  scrollMarginRight: "16rem",
})
;"##### ; "219")]
#[test_case(r#####"tw`scroll-my-64`"#####, r#####"({
  scrollMarginTop: "16rem",
  scrollMarginBottom: "16rem",
})
;"##### ; "220")]
#[test_case(r#####"tw`scroll-mt-64`"#####, r#####"({
  scrollMarginTop: "16rem",
})
;"##### ; "221")]
#[test_case(r#####"tw`scroll-mr-64`"#####, r#####"({
  scrollMarginRight: "16rem",
})
;"##### ; "222")]
#[test_case(r#####"tw`scroll-mb-64`"#####, r#####"({
  scrollMarginBottom: "16rem",
})
;"##### ; "223")]
#[test_case(r#####"tw`scroll-ml-64`"#####, r#####"({
  scrollMarginLeft: "16rem",
})
;"##### ; "224")]
#[test_case(r#####"tw`scroll-m-72`"#####, r#####"({
  scrollMargin: "18rem",
})
;"##### ; "225")]
#[test_case(r#####"tw`scroll-mx-72`"#####, r#####"({
  scrollMarginLeft: "18rem",
  scrollMarginRight: "18rem",
})
;"##### ; "226")]
#[test_case(r#####"tw`scroll-my-72`"#####, r#####"({
  scrollMarginTop: "18rem",
  scrollMarginBottom: "18rem",
})
;"##### ; "227")]
#[test_case(r#####"tw`scroll-mt-72`"#####, r#####"({
  scrollMarginTop: "18rem",
})
;"##### ; "228")]
#[test_case(r#####"tw`scroll-mr-72`"#####, r#####"({
  scrollMarginRight: "18rem",
})
;"##### ; "229")]
#[test_case(r#####"tw`scroll-mb-72`"#####, r#####"({
  scrollMarginBottom: "18rem",
})
;"##### ; "230")]
#[test_case(r#####"tw`scroll-ml-72`"#####, r#####"({
  scrollMarginLeft: "18rem",
})
;"##### ; "231")]
#[test_case(r#####"tw`scroll-m-80`"#####, r#####"({
  scrollMargin: "20rem",
})
;"##### ; "232")]
#[test_case(r#####"tw`scroll-mx-80`"#####, r#####"({
  scrollMarginLeft: "20rem",
  scrollMarginRight: "20rem",
})
;"##### ; "233")]
#[test_case(r#####"tw`scroll-my-80`"#####, r#####"({
  scrollMarginTop: "20rem",
  scrollMarginBottom: "20rem",
})
;"##### ; "234")]
#[test_case(r#####"tw`scroll-mt-80`"#####, r#####"({
  scrollMarginTop: "20rem",
})
;"##### ; "235")]
#[test_case(r#####"tw`scroll-mr-80`"#####, r#####"({
  scrollMarginRight: "20rem",
})
;"##### ; "236")]
#[test_case(r#####"tw`scroll-mb-80`"#####, r#####"({
  scrollMarginBottom: "20rem",
})
;"##### ; "237")]
#[test_case(r#####"tw`scroll-ml-80`"#####, r#####"({
  scrollMarginLeft: "20rem",
})
;"##### ; "238")]
#[test_case(r#####"tw`scroll-m-96`"#####, r#####"({
  scrollMargin: "24rem",
})
;"##### ; "239")]
#[test_case(r#####"tw`scroll-mx-96`"#####, r#####"({
  scrollMarginLeft: "24rem",
  scrollMarginRight: "24rem",
})
;"##### ; "240")]
#[test_case(r#####"tw`scroll-my-96`"#####, r#####"({
  scrollMarginTop: "24rem",
  scrollMarginBottom: "24rem",
})
;"##### ; "241")]
#[test_case(r#####"tw`scroll-mt-96`"#####, r#####"({
  scrollMarginTop: "24rem",
})
;"##### ; "242")]
#[test_case(r#####"tw`scroll-mr-96`"#####, r#####"({
  scrollMarginRight: "24rem",
})
;"##### ; "243")]
#[test_case(r#####"tw`scroll-mb-96`"#####, r#####"({
  scrollMarginBottom: "24rem",
})
;"##### ; "244")]
#[test_case(r#####"tw`scroll-ml-96`"#####, r#####"({
  scrollMarginLeft: "24rem",
})
;"##### ; "245")]
#[test_case(r#####"tw`-scroll-m-96`"#####, r#####"({
  scrollMargin: "-24rem",
})
;"##### ; "246")]
#[test_case(r#####"tw`-scroll-mx-96`"#####, r#####"({
  scrollMarginLeft: "-24rem",
  scrollMarginRight: "-24rem",
})
;"##### ; "247")]
#[test_case(r#####"tw`-scroll-my-96`"#####, r#####"({
  scrollMarginTop: "-24rem",
  scrollMarginBottom: "-24rem",
})
;"##### ; "248")]
#[test_case(r#####"tw`-scroll-mt-96`"#####, r#####"({
  scrollMarginTop: "-24rem",
})
;"##### ; "249")]
#[test_case(r#####"tw`-scroll-mr-96`"#####, r#####"({
  scrollMarginRight: "-24rem",
})
;"##### ; "250")]
#[test_case(r#####"tw`-scroll-mb-96`"#####, r#####"({
  scrollMarginBottom: "-24rem",
})
;"##### ; "251")]
#[test_case(r#####"tw`-scroll-ml-96`"#####, r#####"({
  scrollMarginLeft: "-24rem",
})
;"##### ; "252")]
#[test_case(r#####"tw`-scroll-m-[24rem]`"#####, r#####"({
  scrollMargin: "-24rem",
})
;"##### ; "253")]
#[test_case(r#####"tw`scroll-m-[24rem]`"#####, r#####"({
  scrollMargin: "24rem",
})"##### ; "254")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
