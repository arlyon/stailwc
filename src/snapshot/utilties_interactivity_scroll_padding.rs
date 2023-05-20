use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`scrollPadding`"#####, r#####"({
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
#[test_case(r#####"tw`scroll-p-0`"#####, r#####"({
  scrollPadding: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`scroll-px-0`"#####, r#####"({
  scrollPaddingLeft: "0px",
  scrollPaddingRight: "0px",
})
;"##### ; "2")]
#[test_case(r#####"tw`scroll-py-0`"#####, r#####"({
  scrollPaddingTop: "0px",
  scrollPaddingBottom: "0px",
})
;"##### ; "3")]
#[test_case(r#####"tw`scroll-pt-0`"#####, r#####"({
  scrollPaddingTop: "0px",
})
;"##### ; "4")]
#[test_case(r#####"tw`scroll-pr-0`"#####, r#####"({
  scrollPaddingRight: "0px",
})
;"##### ; "5")]
#[test_case(r#####"tw`scroll-pb-0`"#####, r#####"({
  scrollPaddingBottom: "0px",
})
;"##### ; "6")]
#[test_case(r#####"tw`scroll-pl-0`"#####, r#####"({
  scrollPaddingLeft: "0px",
})
;"##### ; "7")]
#[test_case(r#####"tw`scroll-p-px`"#####, r#####"({
  scrollPadding: "1px",
})
;"##### ; "8")]
#[test_case(r#####"tw`scroll-px-px`"#####, r#####"({
  scrollPaddingLeft: "1px",
  scrollPaddingRight: "1px",
})
;"##### ; "9")]
#[test_case(r#####"tw`scroll-py-px`"#####, r#####"({
  scrollPaddingTop: "1px",
  scrollPaddingBottom: "1px",
})
;"##### ; "10")]
#[test_case(r#####"tw`scroll-pt-px`"#####, r#####"({
  scrollPaddingTop: "1px",
})
;"##### ; "11")]
#[test_case(r#####"tw`scroll-pr-px`"#####, r#####"({
  scrollPaddingRight: "1px",
})
;"##### ; "12")]
#[test_case(r#####"tw`scroll-pb-px`"#####, r#####"({
  scrollPaddingBottom: "1px",
})
;"##### ; "13")]
#[test_case(r#####"tw`scroll-pl-px`"#####, r#####"({
  scrollPaddingLeft: "1px",
})
;"##### ; "14")]
#[test_case(r#####"tw`scroll-p-0`"#####, r#####"({
  scrollPadding: "0px",
})
;"##### ; "15")]
#[test_case(r#####"tw`scroll-px-0`"#####, r#####"({
  scrollPaddingLeft: "0px",
  scrollPaddingRight: "0px",
})
;"##### ; "16")]
#[test_case(r#####"tw`scroll-py-0`"#####, r#####"({
  scrollPaddingTop: "0px",
  scrollPaddingBottom: "0px",
})
;"##### ; "17")]
#[test_case(r#####"tw`scroll-pt-0`"#####, r#####"({
  scrollPaddingTop: "0px",
})
;"##### ; "18")]
#[test_case(r#####"tw`scroll-pr-0`"#####, r#####"({
  scrollPaddingRight: "0px",
})
;"##### ; "19")]
#[test_case(r#####"tw`scroll-pb-0`"#####, r#####"({
  scrollPaddingBottom: "0px",
})
;"##### ; "20")]
#[test_case(r#####"tw`scroll-pl-0`"#####, r#####"({
  scrollPaddingLeft: "0px",
})
;"##### ; "21")]
#[test_case(r#####"tw`scroll-p-1`"#####, r#####"({
  scrollPadding: "0.25rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`scroll-px-1`"#####, r#####"({
  scrollPaddingLeft: "0.25rem",
  scrollPaddingRight: "0.25rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`scroll-py-1`"#####, r#####"({
  scrollPaddingTop: "0.25rem",
  scrollPaddingBottom: "0.25rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`scroll-pt-1`"#####, r#####"({
  scrollPaddingTop: "0.25rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`scroll-pr-1`"#####, r#####"({
  scrollPaddingRight: "0.25rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`scroll-pb-1`"#####, r#####"({
  scrollPaddingBottom: "0.25rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`scroll-pl-1`"#####, r#####"({
  scrollPaddingLeft: "0.25rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`scroll-p-1`"#####, r#####"({
  scrollPadding: "0.25rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`scroll-px-1`"#####, r#####"({
  scrollPaddingLeft: "0.25rem",
  scrollPaddingRight: "0.25rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`scroll-py-1`"#####, r#####"({
  scrollPaddingTop: "0.25rem",
  scrollPaddingBottom: "0.25rem",
})
;"##### ; "31")]
#[test_case(r#####"tw`scroll-pt-1`"#####, r#####"({
  scrollPaddingTop: "0.25rem",
})
;"##### ; "32")]
#[test_case(r#####"tw`scroll-pr-1`"#####, r#####"({
  scrollPaddingRight: "0.25rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`scroll-pb-1`"#####, r#####"({
  scrollPaddingBottom: "0.25rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`scroll-pl-1`"#####, r#####"({
  scrollPaddingLeft: "0.25rem",
})
;"##### ; "35")]
#[test_case(r#####"tw`scroll-p-2`"#####, r#####"({
  scrollPadding: "0.5rem",
})
;"##### ; "36")]
#[test_case(r#####"tw`scroll-px-2`"#####, r#####"({
  scrollPaddingLeft: "0.5rem",
  scrollPaddingRight: "0.5rem",
})
;"##### ; "37")]
#[test_case(r#####"tw`scroll-py-2`"#####, r#####"({
  scrollPaddingTop: "0.5rem",
  scrollPaddingBottom: "0.5rem",
})
;"##### ; "38")]
#[test_case(r#####"tw`scroll-pt-2`"#####, r#####"({
  scrollPaddingTop: "0.5rem",
})
;"##### ; "39")]
#[test_case(r#####"tw`scroll-pr-2`"#####, r#####"({
  scrollPaddingRight: "0.5rem",
})
;"##### ; "40")]
#[test_case(r#####"tw`scroll-pb-2`"#####, r#####"({
  scrollPaddingBottom: "0.5rem",
})
;"##### ; "41")]
#[test_case(r#####"tw`scroll-pl-2`"#####, r#####"({
  scrollPaddingLeft: "0.5rem",
})
;"##### ; "42")]
#[test_case(r#####"tw`scroll-p-2`"#####, r#####"({
  scrollPadding: "0.5rem",
})
;"##### ; "43")]
#[test_case(r#####"tw`scroll-px-2`"#####, r#####"({
  scrollPaddingLeft: "0.5rem",
  scrollPaddingRight: "0.5rem",
})
;"##### ; "44")]
#[test_case(r#####"tw`scroll-py-2`"#####, r#####"({
  scrollPaddingTop: "0.5rem",
  scrollPaddingBottom: "0.5rem",
})
;"##### ; "45")]
#[test_case(r#####"tw`scroll-pt-2`"#####, r#####"({
  scrollPaddingTop: "0.5rem",
})
;"##### ; "46")]
#[test_case(r#####"tw`scroll-pr-2`"#####, r#####"({
  scrollPaddingRight: "0.5rem",
})
;"##### ; "47")]
#[test_case(r#####"tw`scroll-pb-2`"#####, r#####"({
  scrollPaddingBottom: "0.5rem",
})
;"##### ; "48")]
#[test_case(r#####"tw`scroll-pl-2`"#####, r#####"({
  scrollPaddingLeft: "0.5rem",
})
;"##### ; "49")]
#[test_case(r#####"tw`scroll-p-3`"#####, r#####"({
  scrollPadding: "0.75rem",
})
;"##### ; "50")]
#[test_case(r#####"tw`scroll-px-3`"#####, r#####"({
  scrollPaddingLeft: "0.75rem",
  scrollPaddingRight: "0.75rem",
})
;"##### ; "51")]
#[test_case(r#####"tw`scroll-py-3`"#####, r#####"({
  scrollPaddingTop: "0.75rem",
  scrollPaddingBottom: "0.75rem",
})
;"##### ; "52")]
#[test_case(r#####"tw`scroll-pt-3`"#####, r#####"({
  scrollPaddingTop: "0.75rem",
})
;"##### ; "53")]
#[test_case(r#####"tw`scroll-pr-3`"#####, r#####"({
  scrollPaddingRight: "0.75rem",
})
;"##### ; "54")]
#[test_case(r#####"tw`scroll-pb-3`"#####, r#####"({
  scrollPaddingBottom: "0.75rem",
})
;"##### ; "55")]
#[test_case(r#####"tw`scroll-pl-3`"#####, r#####"({
  scrollPaddingLeft: "0.75rem",
})
;"##### ; "56")]
#[test_case(r#####"tw`scroll-p-3`"#####, r#####"({
  scrollPadding: "0.75rem",
})
;"##### ; "57")]
#[test_case(r#####"tw`scroll-px-3`"#####, r#####"({
  scrollPaddingLeft: "0.75rem",
  scrollPaddingRight: "0.75rem",
})
;"##### ; "58")]
#[test_case(r#####"tw`scroll-py-3`"#####, r#####"({
  scrollPaddingTop: "0.75rem",
  scrollPaddingBottom: "0.75rem",
})
;"##### ; "59")]
#[test_case(r#####"tw`scroll-pt-3`"#####, r#####"({
  scrollPaddingTop: "0.75rem",
})
;"##### ; "60")]
#[test_case(r#####"tw`scroll-pr-3`"#####, r#####"({
  scrollPaddingRight: "0.75rem",
})
;"##### ; "61")]
#[test_case(r#####"tw`scroll-pb-3`"#####, r#####"({
  scrollPaddingBottom: "0.75rem",
})
;"##### ; "62")]
#[test_case(r#####"tw`scroll-pl-3`"#####, r#####"({
  scrollPaddingLeft: "0.75rem",
})
;"##### ; "63")]
#[test_case(r#####"tw`scroll-p-4`"#####, r#####"({
  scrollPadding: "1rem",
})
;"##### ; "64")]
#[test_case(r#####"tw`scroll-px-4`"#####, r#####"({
  scrollPaddingLeft: "1rem",
  scrollPaddingRight: "1rem",
})
;"##### ; "65")]
#[test_case(r#####"tw`scroll-py-4`"#####, r#####"({
  scrollPaddingTop: "1rem",
  scrollPaddingBottom: "1rem",
})
;"##### ; "66")]
#[test_case(r#####"tw`scroll-pt-4`"#####, r#####"({
  scrollPaddingTop: "1rem",
})
;"##### ; "67")]
#[test_case(r#####"tw`scroll-pr-4`"#####, r#####"({
  scrollPaddingRight: "1rem",
})
;"##### ; "68")]
#[test_case(r#####"tw`scroll-pb-4`"#####, r#####"({
  scrollPaddingBottom: "1rem",
})
;"##### ; "69")]
#[test_case(r#####"tw`scroll-pl-4`"#####, r#####"({
  scrollPaddingLeft: "1rem",
})
;"##### ; "70")]
#[test_case(r#####"tw`scroll-p-5`"#####, r#####"({
  scrollPadding: "1.25rem",
})
;"##### ; "71")]
#[test_case(r#####"tw`scroll-px-5`"#####, r#####"({
  scrollPaddingLeft: "1.25rem",
  scrollPaddingRight: "1.25rem",
})
;"##### ; "72")]
#[test_case(r#####"tw`scroll-py-5`"#####, r#####"({
  scrollPaddingTop: "1.25rem",
  scrollPaddingBottom: "1.25rem",
})
;"##### ; "73")]
#[test_case(r#####"tw`scroll-pt-5`"#####, r#####"({
  scrollPaddingTop: "1.25rem",
})
;"##### ; "74")]
#[test_case(r#####"tw`scroll-pr-5`"#####, r#####"({
  scrollPaddingRight: "1.25rem",
})
;"##### ; "75")]
#[test_case(r#####"tw`scroll-pb-5`"#####, r#####"({
  scrollPaddingBottom: "1.25rem",
})
;"##### ; "76")]
#[test_case(r#####"tw`scroll-pl-5`"#####, r#####"({
  scrollPaddingLeft: "1.25rem",
})
;"##### ; "77")]
#[test_case(r#####"tw`scroll-p-6`"#####, r#####"({
  scrollPadding: "1.5rem",
})
;"##### ; "78")]
#[test_case(r#####"tw`scroll-px-6`"#####, r#####"({
  scrollPaddingLeft: "1.5rem",
  scrollPaddingRight: "1.5rem",
})
;"##### ; "79")]
#[test_case(r#####"tw`scroll-py-6`"#####, r#####"({
  scrollPaddingTop: "1.5rem",
  scrollPaddingBottom: "1.5rem",
})
;"##### ; "80")]
#[test_case(r#####"tw`scroll-pt-6`"#####, r#####"({
  scrollPaddingTop: "1.5rem",
})
;"##### ; "81")]
#[test_case(r#####"tw`scroll-pr-6`"#####, r#####"({
  scrollPaddingRight: "1.5rem",
})
;"##### ; "82")]
#[test_case(r#####"tw`scroll-pb-6`"#####, r#####"({
  scrollPaddingBottom: "1.5rem",
})
;"##### ; "83")]
#[test_case(r#####"tw`scroll-pl-6`"#####, r#####"({
  scrollPaddingLeft: "1.5rem",
})
;"##### ; "84")]
#[test_case(r#####"tw`scroll-p-7`"#####, r#####"({
  scrollPadding: "1.75rem",
})
;"##### ; "85")]
#[test_case(r#####"tw`scroll-px-7`"#####, r#####"({
  scrollPaddingLeft: "1.75rem",
  scrollPaddingRight: "1.75rem",
})
;"##### ; "86")]
#[test_case(r#####"tw`scroll-py-7`"#####, r#####"({
  scrollPaddingTop: "1.75rem",
  scrollPaddingBottom: "1.75rem",
})
;"##### ; "87")]
#[test_case(r#####"tw`scroll-pt-7`"#####, r#####"({
  scrollPaddingTop: "1.75rem",
})
;"##### ; "88")]
#[test_case(r#####"tw`scroll-pr-7`"#####, r#####"({
  scrollPaddingRight: "1.75rem",
})
;"##### ; "89")]
#[test_case(r#####"tw`scroll-pb-7`"#####, r#####"({
  scrollPaddingBottom: "1.75rem",
})
;"##### ; "90")]
#[test_case(r#####"tw`scroll-pl-7`"#####, r#####"({
  scrollPaddingLeft: "1.75rem",
})
;"##### ; "91")]
#[test_case(r#####"tw`scroll-p-8`"#####, r#####"({
  scrollPadding: "2rem",
})
;"##### ; "92")]
#[test_case(r#####"tw`scroll-px-8`"#####, r#####"({
  scrollPaddingLeft: "2rem",
  scrollPaddingRight: "2rem",
})
;"##### ; "93")]
#[test_case(r#####"tw`scroll-py-8`"#####, r#####"({
  scrollPaddingTop: "2rem",
  scrollPaddingBottom: "2rem",
})
;"##### ; "94")]
#[test_case(r#####"tw`scroll-pt-8`"#####, r#####"({
  scrollPaddingTop: "2rem",
})
;"##### ; "95")]
#[test_case(r#####"tw`scroll-pr-8`"#####, r#####"({
  scrollPaddingRight: "2rem",
})
;"##### ; "96")]
#[test_case(r#####"tw`scroll-pb-8`"#####, r#####"({
  scrollPaddingBottom: "2rem",
})
;"##### ; "97")]
#[test_case(r#####"tw`scroll-pl-8`"#####, r#####"({
  scrollPaddingLeft: "2rem",
})
;"##### ; "98")]
#[test_case(r#####"tw`scroll-p-9`"#####, r#####"({
  scrollPadding: "2.25rem",
})
;"##### ; "99")]
#[test_case(r#####"tw`scroll-px-9`"#####, r#####"({
  scrollPaddingLeft: "2.25rem",
  scrollPaddingRight: "2.25rem",
})
;"##### ; "100")]
#[test_case(r#####"tw`scroll-py-9`"#####, r#####"({
  scrollPaddingTop: "2.25rem",
  scrollPaddingBottom: "2.25rem",
})
;"##### ; "101")]
#[test_case(r#####"tw`scroll-pt-9`"#####, r#####"({
  scrollPaddingTop: "2.25rem",
})
;"##### ; "102")]
#[test_case(r#####"tw`scroll-pr-9`"#####, r#####"({
  scrollPaddingRight: "2.25rem",
})
;"##### ; "103")]
#[test_case(r#####"tw`scroll-pb-9`"#####, r#####"({
  scrollPaddingBottom: "2.25rem",
})
;"##### ; "104")]
#[test_case(r#####"tw`scroll-pl-9`"#####, r#####"({
  scrollPaddingLeft: "2.25rem",
})
;"##### ; "105")]
#[test_case(r#####"tw`scroll-p-10`"#####, r#####"({
  scrollPadding: "2.5rem",
})
;"##### ; "106")]
#[test_case(r#####"tw`scroll-px-10`"#####, r#####"({
  scrollPaddingLeft: "2.5rem",
  scrollPaddingRight: "2.5rem",
})
;"##### ; "107")]
#[test_case(r#####"tw`scroll-py-10`"#####, r#####"({
  scrollPaddingTop: "2.5rem",
  scrollPaddingBottom: "2.5rem",
})
;"##### ; "108")]
#[test_case(r#####"tw`scroll-pt-10`"#####, r#####"({
  scrollPaddingTop: "2.5rem",
})
;"##### ; "109")]
#[test_case(r#####"tw`scroll-pr-10`"#####, r#####"({
  scrollPaddingRight: "2.5rem",
})
;"##### ; "110")]
#[test_case(r#####"tw`scroll-pb-10`"#####, r#####"({
  scrollPaddingBottom: "2.5rem",
})
;"##### ; "111")]
#[test_case(r#####"tw`scroll-pl-10`"#####, r#####"({
  scrollPaddingLeft: "2.5rem",
})
;"##### ; "112")]
#[test_case(r#####"tw`scroll-p-11`"#####, r#####"({
  scrollPadding: "2.75rem",
})
;"##### ; "113")]
#[test_case(r#####"tw`scroll-px-11`"#####, r#####"({
  scrollPaddingLeft: "2.75rem",
  scrollPaddingRight: "2.75rem",
})
;"##### ; "114")]
#[test_case(r#####"tw`scroll-py-11`"#####, r#####"({
  scrollPaddingTop: "2.75rem",
  scrollPaddingBottom: "2.75rem",
})
;"##### ; "115")]
#[test_case(r#####"tw`scroll-pt-11`"#####, r#####"({
  scrollPaddingTop: "2.75rem",
})
;"##### ; "116")]
#[test_case(r#####"tw`scroll-pr-11`"#####, r#####"({
  scrollPaddingRight: "2.75rem",
})
;"##### ; "117")]
#[test_case(r#####"tw`scroll-pb-11`"#####, r#####"({
  scrollPaddingBottom: "2.75rem",
})
;"##### ; "118")]
#[test_case(r#####"tw`scroll-pl-11`"#####, r#####"({
  scrollPaddingLeft: "2.75rem",
})
;"##### ; "119")]
#[test_case(r#####"tw`scroll-p-12`"#####, r#####"({
  scrollPadding: "3rem",
})
;"##### ; "120")]
#[test_case(r#####"tw`scroll-px-12`"#####, r#####"({
  scrollPaddingLeft: "3rem",
  scrollPaddingRight: "3rem",
})
;"##### ; "121")]
#[test_case(r#####"tw`scroll-py-12`"#####, r#####"({
  scrollPaddingTop: "3rem",
  scrollPaddingBottom: "3rem",
})
;"##### ; "122")]
#[test_case(r#####"tw`scroll-pt-12`"#####, r#####"({
  scrollPaddingTop: "3rem",
})
;"##### ; "123")]
#[test_case(r#####"tw`scroll-pr-12`"#####, r#####"({
  scrollPaddingRight: "3rem",
})
;"##### ; "124")]
#[test_case(r#####"tw`scroll-pb-12`"#####, r#####"({
  scrollPaddingBottom: "3rem",
})
;"##### ; "125")]
#[test_case(r#####"tw`scroll-pl-12`"#####, r#####"({
  scrollPaddingLeft: "3rem",
})
;"##### ; "126")]
#[test_case(r#####"tw`scroll-p-14`"#####, r#####"({
  scrollPadding: "3.5rem",
})
;"##### ; "127")]
#[test_case(r#####"tw`scroll-px-14`"#####, r#####"({
  scrollPaddingLeft: "3.5rem",
  scrollPaddingRight: "3.5rem",
})
;"##### ; "128")]
#[test_case(r#####"tw`scroll-py-14`"#####, r#####"({
  scrollPaddingTop: "3.5rem",
  scrollPaddingBottom: "3.5rem",
})
;"##### ; "129")]
#[test_case(r#####"tw`scroll-pt-14`"#####, r#####"({
  scrollPaddingTop: "3.5rem",
})
;"##### ; "130")]
#[test_case(r#####"tw`scroll-pr-14`"#####, r#####"({
  scrollPaddingRight: "3.5rem",
})
;"##### ; "131")]
#[test_case(r#####"tw`scroll-pb-14`"#####, r#####"({
  scrollPaddingBottom: "3.5rem",
})
;"##### ; "132")]
#[test_case(r#####"tw`scroll-pl-14`"#####, r#####"({
  scrollPaddingLeft: "3.5rem",
})
;"##### ; "133")]
#[test_case(r#####"tw`scroll-p-16`"#####, r#####"({
  scrollPadding: "4rem",
})
;"##### ; "134")]
#[test_case(r#####"tw`scroll-px-16`"#####, r#####"({
  scrollPaddingLeft: "4rem",
  scrollPaddingRight: "4rem",
})
;"##### ; "135")]
#[test_case(r#####"tw`scroll-py-16`"#####, r#####"({
  scrollPaddingTop: "4rem",
  scrollPaddingBottom: "4rem",
})
;"##### ; "136")]
#[test_case(r#####"tw`scroll-pt-16`"#####, r#####"({
  scrollPaddingTop: "4rem",
})
;"##### ; "137")]
#[test_case(r#####"tw`scroll-pr-16`"#####, r#####"({
  scrollPaddingRight: "4rem",
})
;"##### ; "138")]
#[test_case(r#####"tw`scroll-pb-16`"#####, r#####"({
  scrollPaddingBottom: "4rem",
})
;"##### ; "139")]
#[test_case(r#####"tw`scroll-pl-16`"#####, r#####"({
  scrollPaddingLeft: "4rem",
})
;"##### ; "140")]
#[test_case(r#####"tw`scroll-p-20`"#####, r#####"({
  scrollPadding: "5rem",
})
;"##### ; "141")]
#[test_case(r#####"tw`scroll-px-20`"#####, r#####"({
  scrollPaddingLeft: "5rem",
  scrollPaddingRight: "5rem",
})
;"##### ; "142")]
#[test_case(r#####"tw`scroll-py-20`"#####, r#####"({
  scrollPaddingTop: "5rem",
  scrollPaddingBottom: "5rem",
})
;"##### ; "143")]
#[test_case(r#####"tw`scroll-pt-20`"#####, r#####"({
  scrollPaddingTop: "5rem",
})
;"##### ; "144")]
#[test_case(r#####"tw`scroll-pr-20`"#####, r#####"({
  scrollPaddingRight: "5rem",
})
;"##### ; "145")]
#[test_case(r#####"tw`scroll-pb-20`"#####, r#####"({
  scrollPaddingBottom: "5rem",
})
;"##### ; "146")]
#[test_case(r#####"tw`scroll-pl-20`"#####, r#####"({
  scrollPaddingLeft: "5rem",
})
;"##### ; "147")]
#[test_case(r#####"tw`scroll-p-24`"#####, r#####"({
  scrollPadding: "6rem",
})
;"##### ; "148")]
#[test_case(r#####"tw`scroll-px-24`"#####, r#####"({
  scrollPaddingLeft: "6rem",
  scrollPaddingRight: "6rem",
})
;"##### ; "149")]
#[test_case(r#####"tw`scroll-py-24`"#####, r#####"({
  scrollPaddingTop: "6rem",
  scrollPaddingBottom: "6rem",
})
;"##### ; "150")]
#[test_case(r#####"tw`scroll-pt-24`"#####, r#####"({
  scrollPaddingTop: "6rem",
})
;"##### ; "151")]
#[test_case(r#####"tw`scroll-pr-24`"#####, r#####"({
  scrollPaddingRight: "6rem",
})
;"##### ; "152")]
#[test_case(r#####"tw`scroll-pb-24`"#####, r#####"({
  scrollPaddingBottom: "6rem",
})
;"##### ; "153")]
#[test_case(r#####"tw`scroll-pl-24`"#####, r#####"({
  scrollPaddingLeft: "6rem",
})
;"##### ; "154")]
#[test_case(r#####"tw`scroll-p-28`"#####, r#####"({
  scrollPadding: "7rem",
})
;"##### ; "155")]
#[test_case(r#####"tw`scroll-px-28`"#####, r#####"({
  scrollPaddingLeft: "7rem",
  scrollPaddingRight: "7rem",
})
;"##### ; "156")]
#[test_case(r#####"tw`scroll-py-28`"#####, r#####"({
  scrollPaddingTop: "7rem",
  scrollPaddingBottom: "7rem",
})
;"##### ; "157")]
#[test_case(r#####"tw`scroll-pt-28`"#####, r#####"({
  scrollPaddingTop: "7rem",
})
;"##### ; "158")]
#[test_case(r#####"tw`scroll-pr-28`"#####, r#####"({
  scrollPaddingRight: "7rem",
})
;"##### ; "159")]
#[test_case(r#####"tw`scroll-pb-28`"#####, r#####"({
  scrollPaddingBottom: "7rem",
})
;"##### ; "160")]
#[test_case(r#####"tw`scroll-pl-28`"#####, r#####"({
  scrollPaddingLeft: "7rem",
})
;"##### ; "161")]
#[test_case(r#####"tw`scroll-p-32`"#####, r#####"({
  scrollPadding: "8rem",
})
;"##### ; "162")]
#[test_case(r#####"tw`scroll-px-32`"#####, r#####"({
  scrollPaddingLeft: "8rem",
  scrollPaddingRight: "8rem",
})
;"##### ; "163")]
#[test_case(r#####"tw`scroll-py-32`"#####, r#####"({
  scrollPaddingTop: "8rem",
  scrollPaddingBottom: "8rem",
})
;"##### ; "164")]
#[test_case(r#####"tw`scroll-pt-32`"#####, r#####"({
  scrollPaddingTop: "8rem",
})
;"##### ; "165")]
#[test_case(r#####"tw`scroll-pr-32`"#####, r#####"({
  scrollPaddingRight: "8rem",
})
;"##### ; "166")]
#[test_case(r#####"tw`scroll-pb-32`"#####, r#####"({
  scrollPaddingBottom: "8rem",
})
;"##### ; "167")]
#[test_case(r#####"tw`scroll-pl-32`"#####, r#####"({
  scrollPaddingLeft: "8rem",
})
;"##### ; "168")]
#[test_case(r#####"tw`scroll-p-36`"#####, r#####"({
  scrollPadding: "9rem",
})
;"##### ; "169")]
#[test_case(r#####"tw`scroll-px-36`"#####, r#####"({
  scrollPaddingLeft: "9rem",
  scrollPaddingRight: "9rem",
})
;"##### ; "170")]
#[test_case(r#####"tw`scroll-py-36`"#####, r#####"({
  scrollPaddingTop: "9rem",
  scrollPaddingBottom: "9rem",
})
;"##### ; "171")]
#[test_case(r#####"tw`scroll-pt-36`"#####, r#####"({
  scrollPaddingTop: "9rem",
})
;"##### ; "172")]
#[test_case(r#####"tw`scroll-pr-36`"#####, r#####"({
  scrollPaddingRight: "9rem",
})
;"##### ; "173")]
#[test_case(r#####"tw`scroll-pb-36`"#####, r#####"({
  scrollPaddingBottom: "9rem",
})
;"##### ; "174")]
#[test_case(r#####"tw`scroll-pl-36`"#####, r#####"({
  scrollPaddingLeft: "9rem",
})
;"##### ; "175")]
#[test_case(r#####"tw`scroll-p-40`"#####, r#####"({
  scrollPadding: "10rem",
})
;"##### ; "176")]
#[test_case(r#####"tw`scroll-px-40`"#####, r#####"({
  scrollPaddingLeft: "10rem",
  scrollPaddingRight: "10rem",
})
;"##### ; "177")]
#[test_case(r#####"tw`scroll-py-40`"#####, r#####"({
  scrollPaddingTop: "10rem",
  scrollPaddingBottom: "10rem",
})
;"##### ; "178")]
#[test_case(r#####"tw`scroll-pt-40`"#####, r#####"({
  scrollPaddingTop: "10rem",
})
;"##### ; "179")]
#[test_case(r#####"tw`scroll-pr-40`"#####, r#####"({
  scrollPaddingRight: "10rem",
})
;"##### ; "180")]
#[test_case(r#####"tw`scroll-pb-40`"#####, r#####"({
  scrollPaddingBottom: "10rem",
})
;"##### ; "181")]
#[test_case(r#####"tw`scroll-pl-40`"#####, r#####"({
  scrollPaddingLeft: "10rem",
})
;"##### ; "182")]
#[test_case(r#####"tw`scroll-p-44`"#####, r#####"({
  scrollPadding: "11rem",
})
;"##### ; "183")]
#[test_case(r#####"tw`scroll-px-44`"#####, r#####"({
  scrollPaddingLeft: "11rem",
  scrollPaddingRight: "11rem",
})
;"##### ; "184")]
#[test_case(r#####"tw`scroll-py-44`"#####, r#####"({
  scrollPaddingTop: "11rem",
  scrollPaddingBottom: "11rem",
})
;"##### ; "185")]
#[test_case(r#####"tw`scroll-pt-44`"#####, r#####"({
  scrollPaddingTop: "11rem",
})
;"##### ; "186")]
#[test_case(r#####"tw`scroll-pr-44`"#####, r#####"({
  scrollPaddingRight: "11rem",
})
;"##### ; "187")]
#[test_case(r#####"tw`scroll-pb-44`"#####, r#####"({
  scrollPaddingBottom: "11rem",
})
;"##### ; "188")]
#[test_case(r#####"tw`scroll-pl-44`"#####, r#####"({
  scrollPaddingLeft: "11rem",
})
;"##### ; "189")]
#[test_case(r#####"tw`scroll-p-48`"#####, r#####"({
  scrollPadding: "12rem",
})
;"##### ; "190")]
#[test_case(r#####"tw`scroll-px-48`"#####, r#####"({
  scrollPaddingLeft: "12rem",
  scrollPaddingRight: "12rem",
})
;"##### ; "191")]
#[test_case(r#####"tw`scroll-py-48`"#####, r#####"({
  scrollPaddingTop: "12rem",
  scrollPaddingBottom: "12rem",
})
;"##### ; "192")]
#[test_case(r#####"tw`scroll-pt-48`"#####, r#####"({
  scrollPaddingTop: "12rem",
})
;"##### ; "193")]
#[test_case(r#####"tw`scroll-pr-48`"#####, r#####"({
  scrollPaddingRight: "12rem",
})
;"##### ; "194")]
#[test_case(r#####"tw`scroll-pb-48`"#####, r#####"({
  scrollPaddingBottom: "12rem",
})
;"##### ; "195")]
#[test_case(r#####"tw`scroll-pl-48`"#####, r#####"({
  scrollPaddingLeft: "12rem",
})
;"##### ; "196")]
#[test_case(r#####"tw`scroll-p-52`"#####, r#####"({
  scrollPadding: "13rem",
})
;"##### ; "197")]
#[test_case(r#####"tw`scroll-px-52`"#####, r#####"({
  scrollPaddingLeft: "13rem",
  scrollPaddingRight: "13rem",
})
;"##### ; "198")]
#[test_case(r#####"tw`scroll-py-52`"#####, r#####"({
  scrollPaddingTop: "13rem",
  scrollPaddingBottom: "13rem",
})
;"##### ; "199")]
#[test_case(r#####"tw`scroll-pt-52`"#####, r#####"({
  scrollPaddingTop: "13rem",
})
;"##### ; "200")]
#[test_case(r#####"tw`scroll-pr-52`"#####, r#####"({
  scrollPaddingRight: "13rem",
})
;"##### ; "201")]
#[test_case(r#####"tw`scroll-pb-52`"#####, r#####"({
  scrollPaddingBottom: "13rem",
})
;"##### ; "202")]
#[test_case(r#####"tw`scroll-pl-52`"#####, r#####"({
  scrollPaddingLeft: "13rem",
})
;"##### ; "203")]
#[test_case(r#####"tw`scroll-p-56`"#####, r#####"({
  scrollPadding: "14rem",
})
;"##### ; "204")]
#[test_case(r#####"tw`scroll-px-56`"#####, r#####"({
  scrollPaddingLeft: "14rem",
  scrollPaddingRight: "14rem",
})
;"##### ; "205")]
#[test_case(r#####"tw`scroll-py-56`"#####, r#####"({
  scrollPaddingTop: "14rem",
  scrollPaddingBottom: "14rem",
})
;"##### ; "206")]
#[test_case(r#####"tw`scroll-pt-56`"#####, r#####"({
  scrollPaddingTop: "14rem",
})
;"##### ; "207")]
#[test_case(r#####"tw`scroll-pr-56`"#####, r#####"({
  scrollPaddingRight: "14rem",
})
;"##### ; "208")]
#[test_case(r#####"tw`scroll-pb-56`"#####, r#####"({
  scrollPaddingBottom: "14rem",
})
;"##### ; "209")]
#[test_case(r#####"tw`scroll-pl-56`"#####, r#####"({
  scrollPaddingLeft: "14rem",
})
;"##### ; "210")]
#[test_case(r#####"tw`scroll-p-60`"#####, r#####"({
  scrollPadding: "15rem",
})
;"##### ; "211")]
#[test_case(r#####"tw`scroll-px-60`"#####, r#####"({
  scrollPaddingLeft: "15rem",
  scrollPaddingRight: "15rem",
})
;"##### ; "212")]
#[test_case(r#####"tw`scroll-py-60`"#####, r#####"({
  scrollPaddingTop: "15rem",
  scrollPaddingBottom: "15rem",
})
;"##### ; "213")]
#[test_case(r#####"tw`scroll-pt-60`"#####, r#####"({
  scrollPaddingTop: "15rem",
})
;"##### ; "214")]
#[test_case(r#####"tw`scroll-pr-60`"#####, r#####"({
  scrollPaddingRight: "15rem",
})
;"##### ; "215")]
#[test_case(r#####"tw`scroll-pb-60`"#####, r#####"({
  scrollPaddingBottom: "15rem",
})
;"##### ; "216")]
#[test_case(r#####"tw`scroll-pl-60`"#####, r#####"({
  scrollPaddingLeft: "15rem",
})
;"##### ; "217")]
#[test_case(r#####"tw`scroll-p-64`"#####, r#####"({
  scrollPadding: "16rem",
})
;"##### ; "218")]
#[test_case(r#####"tw`scroll-px-64`"#####, r#####"({
  scrollPaddingLeft: "16rem",
  scrollPaddingRight: "16rem",
})
;"##### ; "219")]
#[test_case(r#####"tw`scroll-py-64`"#####, r#####"({
  scrollPaddingTop: "16rem",
  scrollPaddingBottom: "16rem",
})
;"##### ; "220")]
#[test_case(r#####"tw`scroll-pt-64`"#####, r#####"({
  scrollPaddingTop: "16rem",
})
;"##### ; "221")]
#[test_case(r#####"tw`scroll-pr-64`"#####, r#####"({
  scrollPaddingRight: "16rem",
})
;"##### ; "222")]
#[test_case(r#####"tw`scroll-pb-64`"#####, r#####"({
  scrollPaddingBottom: "16rem",
})
;"##### ; "223")]
#[test_case(r#####"tw`scroll-pl-64`"#####, r#####"({
  scrollPaddingLeft: "16rem",
})
;"##### ; "224")]
#[test_case(r#####"tw`scroll-p-72`"#####, r#####"({
  scrollPadding: "18rem",
})
;"##### ; "225")]
#[test_case(r#####"tw`scroll-px-72`"#####, r#####"({
  scrollPaddingLeft: "18rem",
  scrollPaddingRight: "18rem",
})
;"##### ; "226")]
#[test_case(r#####"tw`scroll-py-72`"#####, r#####"({
  scrollPaddingTop: "18rem",
  scrollPaddingBottom: "18rem",
})
;"##### ; "227")]
#[test_case(r#####"tw`scroll-pt-72`"#####, r#####"({
  scrollPaddingTop: "18rem",
})
;"##### ; "228")]
#[test_case(r#####"tw`scroll-pr-72`"#####, r#####"({
  scrollPaddingRight: "18rem",
})
;"##### ; "229")]
#[test_case(r#####"tw`scroll-pb-72`"#####, r#####"({
  scrollPaddingBottom: "18rem",
})
;"##### ; "230")]
#[test_case(r#####"tw`scroll-pl-72`"#####, r#####"({
  scrollPaddingLeft: "18rem",
})
;"##### ; "231")]
#[test_case(r#####"tw`scroll-p-80`"#####, r#####"({
  scrollPadding: "20rem",
})
;"##### ; "232")]
#[test_case(r#####"tw`scroll-px-80`"#####, r#####"({
  scrollPaddingLeft: "20rem",
  scrollPaddingRight: "20rem",
})
;"##### ; "233")]
#[test_case(r#####"tw`scroll-py-80`"#####, r#####"({
  scrollPaddingTop: "20rem",
  scrollPaddingBottom: "20rem",
})
;"##### ; "234")]
#[test_case(r#####"tw`scroll-pt-80`"#####, r#####"({
  scrollPaddingTop: "20rem",
})
;"##### ; "235")]
#[test_case(r#####"tw`scroll-pr-80`"#####, r#####"({
  scrollPaddingRight: "20rem",
})
;"##### ; "236")]
#[test_case(r#####"tw`scroll-pb-80`"#####, r#####"({
  scrollPaddingBottom: "20rem",
})
;"##### ; "237")]
#[test_case(r#####"tw`scroll-pl-80`"#####, r#####"({
  scrollPaddingLeft: "20rem",
})
;"##### ; "238")]
#[test_case(r#####"tw`scroll-p-96`"#####, r#####"({
  scrollPadding: "24rem",
})
;"##### ; "239")]
#[test_case(r#####"tw`scroll-px-96`"#####, r#####"({
  scrollPaddingLeft: "24rem",
  scrollPaddingRight: "24rem",
})
;"##### ; "240")]
#[test_case(r#####"tw`scroll-py-96`"#####, r#####"({
  scrollPaddingTop: "24rem",
  scrollPaddingBottom: "24rem",
})
;"##### ; "241")]
#[test_case(r#####"tw`scroll-pt-96`"#####, r#####"({
  scrollPaddingTop: "24rem",
})
;"##### ; "242")]
#[test_case(r#####"tw`scroll-pr-96`"#####, r#####"({
  scrollPaddingRight: "24rem",
})
;"##### ; "243")]
#[test_case(r#####"tw`scroll-pb-96`"#####, r#####"({
  scrollPaddingBottom: "24rem",
})
;"##### ; "244")]
#[test_case(r#####"tw`scroll-pl-96`"#####, r#####"({
  scrollPaddingLeft: "24rem",
})
;"##### ; "245")]
#[test_case(r#####"tw`scroll-p-[24rem]`"#####, r#####"({
  scrollPadding: "24rem",
})"##### ; "246")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
