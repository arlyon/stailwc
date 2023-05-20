use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`inset`"#####, r#####"({
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
  full: "100%",
})
;"##### ; "0")]
#[test_case(r#####"tw`inset-0`"#####, r#####"({
  inset: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`-inset-0`"#####, r#####"({
  inset: "-0px",
})
;"##### ; "2")]
#[test_case(r#####"tw`inset-y-0`"#####, r#####"({
  top: "0px",
  bottom: "0px",
})
;"##### ; "3")]
#[test_case(r#####"tw`inset-x-0`"#####, r#####"({
  left: "0px",
  right: "0px",
})
;"##### ; "4")]
#[test_case(r#####"tw`-inset-y-0`"#####, r#####"({
  top: "-0px",
  bottom: "-0px",
})
;"##### ; "5")]
#[test_case(r#####"tw`-inset-x-0`"#####, r#####"({
  left: "-0px",
  right: "-0px",
})
;"##### ; "6")]
#[test_case(r#####"tw`top-0`"#####, r#####"({
  top: "0px",
})
;"##### ; "7")]
#[test_case(r#####"tw`right-0`"#####, r#####"({
  right: "0px",
})
;"##### ; "8")]
#[test_case(r#####"tw`bottom-0`"#####, r#####"({
  bottom: "0px",
})
;"##### ; "9")]
#[test_case(r#####"tw`left-0`"#####, r#####"({
  left: "0px",
})
;"##### ; "10")]
#[test_case(r#####"tw`-top-0`"#####, r#####"({
  top: "-0px",
})
;"##### ; "11")]
#[test_case(r#####"tw`-right-0`"#####, r#####"({
  right: "-0px",
})
;"##### ; "12")]
#[test_case(r#####"tw`-bottom-0`"#####, r#####"({
  bottom: "-0px",
})
;"##### ; "13")]
#[test_case(r#####"tw`-left-0`"#####, r#####"({
  left: "-0px",
})
;"##### ; "14")]
#[test_case(r#####"tw`inset-0.5`"#####, r#####"({
  inset: "0.125rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`-inset-0.5`"#####, r#####"({
  inset: "-0.125rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`inset-y-0.5`"#####, r#####"({
  top: "0.125rem",
  bottom: "0.125rem",
})
;"##### ; "17")]
#[test_case(r#####"tw`inset-x-0.5`"#####, r#####"({
  left: "0.125rem",
  right: "0.125rem",
})
;"##### ; "18")]
#[test_case(r#####"tw`-inset-y-0.5`"#####, r#####"({
  top: "-0.125rem",
  bottom: "-0.125rem",
})
;"##### ; "19")]
#[test_case(r#####"tw`-inset-x-0.5`"#####, r#####"({
  left: "-0.125rem",
  right: "-0.125rem",
})
;"##### ; "20")]
#[test_case(r#####"tw`top-0.5`"#####, r#####"({
  top: "0.125rem",
})
;"##### ; "21")]
#[test_case(r#####"tw`right-0.5`"#####, r#####"({
  right: "0.125rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`bottom-0.5`"#####, r#####"({
  bottom: "0.125rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`left-0.5`"#####, r#####"({
  left: "0.125rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`-top-0.5`"#####, r#####"({
  top: "-0.125rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`-right-0.5`"#####, r#####"({
  right: "-0.125rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`-bottom-0.5`"#####, r#####"({
  bottom: "-0.125rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`-left-0.5`"#####, r#####"({
  left: "-0.125rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`inset-1`"#####, r#####"({
  inset: "0.25rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`-inset-1`"#####, r#####"({
  inset: "-0.25rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`inset-y-1`"#####, r#####"({
  top: "0.25rem",
  bottom: "0.25rem",
})
;"##### ; "31")]
#[test_case(r#####"tw`inset-x-1`"#####, r#####"({
  left: "0.25rem",
  right: "0.25rem",
})
;"##### ; "32")]
#[test_case(r#####"tw`-inset-y-1`"#####, r#####"({
  top: "-0.25rem",
  bottom: "-0.25rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`-inset-x-1`"#####, r#####"({
  left: "-0.25rem",
  right: "-0.25rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`top-1`"#####, r#####"({
  top: "0.25rem",
})
;"##### ; "35")]
#[test_case(r#####"tw`right-1`"#####, r#####"({
  right: "0.25rem",
})
;"##### ; "36")]
#[test_case(r#####"tw`bottom-1`"#####, r#####"({
  bottom: "0.25rem",
})
;"##### ; "37")]
#[test_case(r#####"tw`left-1`"#####, r#####"({
  left: "0.25rem",
})
;"##### ; "38")]
#[test_case(r#####"tw`-top-1`"#####, r#####"({
  top: "-0.25rem",
})
;"##### ; "39")]
#[test_case(r#####"tw`-right-1`"#####, r#####"({
  right: "-0.25rem",
})
;"##### ; "40")]
#[test_case(r#####"tw`-bottom-1`"#####, r#####"({
  bottom: "-0.25rem",
})
;"##### ; "41")]
#[test_case(r#####"tw`-left-1`"#####, r#####"({
  left: "-0.25rem",
})
;"##### ; "42")]
#[test_case(r#####"tw`inset-1.5`"#####, r#####"({
  inset: "0.375rem",
})
;"##### ; "43")]
#[test_case(r#####"tw`-inset-1.5`"#####, r#####"({
  inset: "-0.375rem",
})
;"##### ; "44")]
#[test_case(r#####"tw`inset-y-1.5`"#####, r#####"({
  top: "0.375rem",
  bottom: "0.375rem",
})
;"##### ; "45")]
#[test_case(r#####"tw`inset-x-1.5`"#####, r#####"({
  left: "0.375rem",
  right: "0.375rem",
})
;"##### ; "46")]
#[test_case(r#####"tw`-inset-y-1.5`"#####, r#####"({
  top: "-0.375rem",
  bottom: "-0.375rem",
})
;"##### ; "47")]
#[test_case(r#####"tw`-inset-x-1.5`"#####, r#####"({
  left: "-0.375rem",
  right: "-0.375rem",
})
;"##### ; "48")]
#[test_case(r#####"tw`top-1.5`"#####, r#####"({
  top: "0.375rem",
})
;"##### ; "49")]
#[test_case(r#####"tw`right-1.5`"#####, r#####"({
  right: "0.375rem",
})
;"##### ; "50")]
#[test_case(r#####"tw`bottom-1.5`"#####, r#####"({
  bottom: "0.375rem",
})
;"##### ; "51")]
#[test_case(r#####"tw`left-1.5`"#####, r#####"({
  left: "0.375rem",
})
;"##### ; "52")]
#[test_case(r#####"tw`-top-1.5`"#####, r#####"({
  top: "-0.375rem",
})
;"##### ; "53")]
#[test_case(r#####"tw`-right-1.5`"#####, r#####"({
  right: "-0.375rem",
})
;"##### ; "54")]
#[test_case(r#####"tw`-bottom-1.5`"#####, r#####"({
  bottom: "-0.375rem",
})
;"##### ; "55")]
#[test_case(r#####"tw`-left-1.5`"#####, r#####"({
  left: "-0.375rem",
})
;"##### ; "56")]
#[test_case(r#####"tw`inset-2`"#####, r#####"({
  inset: "0.5rem",
})
;"##### ; "57")]
#[test_case(r#####"tw`-inset-2`"#####, r#####"({
  inset: "-0.5rem",
})
;"##### ; "58")]
#[test_case(r#####"tw`inset-y-2`"#####, r#####"({
  top: "0.5rem",
  bottom: "0.5rem",
})
;"##### ; "59")]
#[test_case(r#####"tw`inset-x-2`"#####, r#####"({
  left: "0.5rem",
  right: "0.5rem",
})
;"##### ; "60")]
#[test_case(r#####"tw`-inset-y-2`"#####, r#####"({
  top: "-0.5rem",
  bottom: "-0.5rem",
})
;"##### ; "61")]
#[test_case(r#####"tw`-inset-x-2`"#####, r#####"({
  left: "-0.5rem",
  right: "-0.5rem",
})
;"##### ; "62")]
#[test_case(r#####"tw`top-2`"#####, r#####"({
  top: "0.5rem",
})
;"##### ; "63")]
#[test_case(r#####"tw`right-2`"#####, r#####"({
  right: "0.5rem",
})
;"##### ; "64")]
#[test_case(r#####"tw`bottom-2`"#####, r#####"({
  bottom: "0.5rem",
})
;"##### ; "65")]
#[test_case(r#####"tw`left-2`"#####, r#####"({
  left: "0.5rem",
})
;"##### ; "66")]
#[test_case(r#####"tw`-top-2`"#####, r#####"({
  top: "-0.5rem",
})
;"##### ; "67")]
#[test_case(r#####"tw`-right-2`"#####, r#####"({
  right: "-0.5rem",
})
;"##### ; "68")]
#[test_case(r#####"tw`-bottom-2`"#####, r#####"({
  bottom: "-0.5rem",
})
;"##### ; "69")]
#[test_case(r#####"tw`-left-2`"#####, r#####"({
  left: "-0.5rem",
})
;"##### ; "70")]
#[test_case(r#####"tw`inset-2.5`"#####, r#####"({
  inset: "0.625rem",
})
;"##### ; "71")]
#[test_case(r#####"tw`-inset-2.5`"#####, r#####"({
  inset: "-0.625rem",
})
;"##### ; "72")]
#[test_case(r#####"tw`inset-y-2.5`"#####, r#####"({
  top: "0.625rem",
  bottom: "0.625rem",
})
;"##### ; "73")]
#[test_case(r#####"tw`inset-x-2.5`"#####, r#####"({
  left: "0.625rem",
  right: "0.625rem",
})
;"##### ; "74")]
#[test_case(r#####"tw`-inset-y-2.5`"#####, r#####"({
  top: "-0.625rem",
  bottom: "-0.625rem",
})
;"##### ; "75")]
#[test_case(r#####"tw`top-2.5`"#####, r#####"({
  top: "0.625rem",
})
;"##### ; "76")]
#[test_case(r#####"tw`right-2.5`"#####, r#####"({
  right: "0.625rem",
})
;"##### ; "77")]
#[test_case(r#####"tw`bottom-2.5`"#####, r#####"({
  bottom: "0.625rem",
})
;"##### ; "78")]
#[test_case(r#####"tw`left-2.5`"#####, r#####"({
  left: "0.625rem",
})
;"##### ; "79")]
#[test_case(r#####"tw`-top-2.5`"#####, r#####"({
  top: "-0.625rem",
})
;"##### ; "80")]
#[test_case(r#####"tw`-bottom-2.5`"#####, r#####"({
  bottom: "-0.625rem",
})
;"##### ; "81")]
#[test_case(r#####"tw`-left-2.5`"#####, r#####"({
  left: "-0.625rem",
})
;"##### ; "82")]
#[test_case(r#####"tw`inset-3`"#####, r#####"({
  inset: "0.75rem",
})
;"##### ; "83")]
#[test_case(r#####"tw`-inset-3`"#####, r#####"({
  inset: "-0.75rem",
})
;"##### ; "84")]
#[test_case(r#####"tw`inset-y-3`"#####, r#####"({
  top: "0.75rem",
  bottom: "0.75rem",
})
;"##### ; "85")]
#[test_case(r#####"tw`inset-x-3`"#####, r#####"({
  left: "0.75rem",
  right: "0.75rem",
})
;"##### ; "86")]
#[test_case(r#####"tw`-inset-y-3`"#####, r#####"({
  top: "-0.75rem",
  bottom: "-0.75rem",
})
;"##### ; "87")]
#[test_case(r#####"tw`-inset-x-3`"#####, r#####"({
  left: "-0.75rem",
  right: "-0.75rem",
})
;"##### ; "88")]
#[test_case(r#####"tw`top-3`"#####, r#####"({
  top: "0.75rem",
})
;"##### ; "89")]
#[test_case(r#####"tw`right-3`"#####, r#####"({
  right: "0.75rem",
})
;"##### ; "90")]
#[test_case(r#####"tw`bottom-3`"#####, r#####"({
  bottom: "0.75rem",
})
;"##### ; "91")]
#[test_case(r#####"tw`left-3`"#####, r#####"({
  left: "0.75rem",
})
;"##### ; "92")]
#[test_case(r#####"tw`-top-3`"#####, r#####"({
  top: "-0.75rem",
})
;"##### ; "93")]
#[test_case(r#####"tw`-right-3`"#####, r#####"({
  right: "-0.75rem",
})
;"##### ; "94")]
#[test_case(r#####"tw`-bottom-3`"#####, r#####"({
  bottom: "-0.75rem",
})
;"##### ; "95")]
#[test_case(r#####"tw`-left-3`"#####, r#####"({
  left: "-0.75rem",
})
;"##### ; "96")]
#[test_case(r#####"tw`inset-3.5`"#####, r#####"({
  inset: "0.875rem",
})
;"##### ; "97")]
#[test_case(r#####"tw`-inset-3.5`"#####, r#####"({
  inset: "-0.875rem",
})
;"##### ; "98")]
#[test_case(r#####"tw`inset-y-3.5`"#####, r#####"({
  top: "0.875rem",
  bottom: "0.875rem",
})
;"##### ; "99")]
#[test_case(r#####"tw`inset-x-3.5`"#####, r#####"({
  left: "0.875rem",
  right: "0.875rem",
})
;"##### ; "100")]
#[test_case(r#####"tw`-inset-y-3.5`"#####, r#####"({
  top: "-0.875rem",
  bottom: "-0.875rem",
})
;"##### ; "101")]
#[test_case(r#####"tw`-inset-x-3.5`"#####, r#####"({
  left: "-0.875rem",
  right: "-0.875rem",
})
;"##### ; "102")]
#[test_case(r#####"tw`top-3.5`"#####, r#####"({
  top: "0.875rem",
})
;"##### ; "103")]
#[test_case(r#####"tw`right-3.5`"#####, r#####"({
  right: "0.875rem",
})
;"##### ; "104")]
#[test_case(r#####"tw`bottom-3.5`"#####, r#####"({
  bottom: "0.875rem",
})
;"##### ; "105")]
#[test_case(r#####"tw`left-3.5`"#####, r#####"({
  left: "0.875rem",
})
;"##### ; "106")]
#[test_case(r#####"tw`-top-3.5`"#####, r#####"({
  top: "-0.875rem",
})
;"##### ; "107")]
#[test_case(r#####"tw`-right-3.5`"#####, r#####"({
  right: "-0.875rem",
})
;"##### ; "108")]
#[test_case(r#####"tw`-bottom-3.5`"#####, r#####"({
  bottom: "-0.875rem",
})
;"##### ; "109")]
#[test_case(r#####"tw`-left-3.5`"#####, r#####"({
  left: "-0.875rem",
})
;"##### ; "110")]
#[test_case(r#####"tw`inset-4`"#####, r#####"({
  inset: "1rem",
})
;"##### ; "111")]
#[test_case(r#####"tw`-inset-4`"#####, r#####"({
  inset: "-1rem",
})
;"##### ; "112")]
#[test_case(r#####"tw`inset-y-4`"#####, r#####"({
  top: "1rem",
  bottom: "1rem",
})
;"##### ; "113")]
#[test_case(r#####"tw`inset-x-4`"#####, r#####"({
  left: "1rem",
  right: "1rem",
})
;"##### ; "114")]
#[test_case(r#####"tw`-inset-y-4`"#####, r#####"({
  top: "-1rem",
  bottom: "-1rem",
})
;"##### ; "115")]
#[test_case(r#####"tw`-inset-x-4`"#####, r#####"({
  left: "-1rem",
  right: "-1rem",
})
;"##### ; "116")]
#[test_case(r#####"tw`top-4`"#####, r#####"({
  top: "1rem",
})
;"##### ; "117")]
#[test_case(r#####"tw`right-4`"#####, r#####"({
  right: "1rem",
})
;"##### ; "118")]
#[test_case(r#####"tw`bottom-4`"#####, r#####"({
  bottom: "1rem",
})
;"##### ; "119")]
#[test_case(r#####"tw`left-4`"#####, r#####"({
  left: "1rem",
})
;"##### ; "120")]
#[test_case(r#####"tw`-top-4`"#####, r#####"({
  top: "-1rem",
})
;"##### ; "121")]
#[test_case(r#####"tw`-right-4`"#####, r#####"({
  right: "-1rem",
})
;"##### ; "122")]
#[test_case(r#####"tw`-bottom-4`"#####, r#####"({
  bottom: "-1rem",
})
;"##### ; "123")]
#[test_case(r#####"tw`-left-4`"#####, r#####"({
  left: "-1rem",
})
;"##### ; "124")]
#[test_case(r#####"tw`inset-5`"#####, r#####"({
  inset: "1.25rem",
})
;"##### ; "125")]
#[test_case(r#####"tw`-inset-5`"#####, r#####"({
  inset: "-1.25rem",
})
;"##### ; "126")]
#[test_case(r#####"tw`inset-y-5`"#####, r#####"({
  top: "1.25rem",
  bottom: "1.25rem",
})
;"##### ; "127")]
#[test_case(r#####"tw`inset-x-5`"#####, r#####"({
  left: "1.25rem",
  right: "1.25rem",
})
;"##### ; "128")]
#[test_case(r#####"tw`-inset-y-5`"#####, r#####"({
  top: "-1.25rem",
  bottom: "-1.25rem",
})
;"##### ; "129")]
#[test_case(r#####"tw`-inset-x-5`"#####, r#####"({
  left: "-1.25rem",
  right: "-1.25rem",
})
;"##### ; "130")]
#[test_case(r#####"tw`top-5`"#####, r#####"({
  top: "1.25rem",
})
;"##### ; "131")]
#[test_case(r#####"tw`right-5`"#####, r#####"({
  right: "1.25rem",
})
;"##### ; "132")]
#[test_case(r#####"tw`bottom-5`"#####, r#####"({
  bottom: "1.25rem",
})
;"##### ; "133")]
#[test_case(r#####"tw`left-5`"#####, r#####"({
  left: "1.25rem",
})
;"##### ; "134")]
#[test_case(r#####"tw`-top-5`"#####, r#####"({
  top: "-1.25rem",
})
;"##### ; "135")]
#[test_case(r#####"tw`-right-5`"#####, r#####"({
  right: "-1.25rem",
})
;"##### ; "136")]
#[test_case(r#####"tw`-bottom-5`"#####, r#####"({
  bottom: "-1.25rem",
})
;"##### ; "137")]
#[test_case(r#####"tw`-left-5`"#####, r#####"({
  left: "-1.25rem",
})
;"##### ; "138")]
#[test_case(r#####"tw`inset-6`"#####, r#####"({
  inset: "1.5rem",
})
;"##### ; "139")]
#[test_case(r#####"tw`-inset-6`"#####, r#####"({
  inset: "-1.5rem",
})
;"##### ; "140")]
#[test_case(r#####"tw`inset-y-6`"#####, r#####"({
  top: "1.5rem",
  bottom: "1.5rem",
})
;"##### ; "141")]
#[test_case(r#####"tw`inset-x-6`"#####, r#####"({
  left: "1.5rem",
  right: "1.5rem",
})
;"##### ; "142")]
#[test_case(r#####"tw`-inset-y-6`"#####, r#####"({
  top: "-1.5rem",
  bottom: "-1.5rem",
})
;"##### ; "143")]
#[test_case(r#####"tw`-inset-x-6`"#####, r#####"({
  left: "-1.5rem",
  right: "-1.5rem",
})
;"##### ; "144")]
#[test_case(r#####"tw`top-6`"#####, r#####"({
  top: "1.5rem",
})
;"##### ; "145")]
#[test_case(r#####"tw`right-6`"#####, r#####"({
  right: "1.5rem",
})
;"##### ; "146")]
#[test_case(r#####"tw`bottom-6`"#####, r#####"({
  bottom: "1.5rem",
})
;"##### ; "147")]
#[test_case(r#####"tw`left-6`"#####, r#####"({
  left: "1.5rem",
})
;"##### ; "148")]
#[test_case(r#####"tw`-top-6`"#####, r#####"({
  top: "-1.5rem",
})
;"##### ; "149")]
#[test_case(r#####"tw`-right-6`"#####, r#####"({
  right: "-1.5rem",
})
;"##### ; "150")]
#[test_case(r#####"tw`-bottom-6`"#####, r#####"({
  bottom: "-1.5rem",
})
;"##### ; "151")]
#[test_case(r#####"tw`-left-6`"#####, r#####"({
  left: "-1.5rem",
})
;"##### ; "152")]
#[test_case(r#####"tw`inset-7`"#####, r#####"({
  inset: "1.75rem",
})
;"##### ; "153")]
#[test_case(r#####"tw`-inset-7`"#####, r#####"({
  inset: "-1.75rem",
})
;"##### ; "154")]
#[test_case(r#####"tw`inset-y-7`"#####, r#####"({
  top: "1.75rem",
  bottom: "1.75rem",
})
;"##### ; "155")]
#[test_case(r#####"tw`inset-x-7`"#####, r#####"({
  left: "1.75rem",
  right: "1.75rem",
})
;"##### ; "156")]
#[test_case(r#####"tw`-inset-y-7`"#####, r#####"({
  top: "-1.75rem",
  bottom: "-1.75rem",
})
;"##### ; "157")]
#[test_case(r#####"tw`-inset-x-7`"#####, r#####"({
  left: "-1.75rem",
  right: "-1.75rem",
})
;"##### ; "158")]
#[test_case(r#####"tw`top-7`"#####, r#####"({
  top: "1.75rem",
})
;"##### ; "159")]
#[test_case(r#####"tw`right-7`"#####, r#####"({
  right: "1.75rem",
})
;"##### ; "160")]
#[test_case(r#####"tw`bottom-7`"#####, r#####"({
  bottom: "1.75rem",
})
;"##### ; "161")]
#[test_case(r#####"tw`left-7`"#####, r#####"({
  left: "1.75rem",
})
;"##### ; "162")]
#[test_case(r#####"tw`-top-7`"#####, r#####"({
  top: "-1.75rem",
})
;"##### ; "163")]
#[test_case(r#####"tw`-right-7`"#####, r#####"({
  right: "-1.75rem",
})
;"##### ; "164")]
#[test_case(r#####"tw`-bottom-7`"#####, r#####"({
  bottom: "-1.75rem",
})
;"##### ; "165")]
#[test_case(r#####"tw`-left-7`"#####, r#####"({
  left: "-1.75rem",
})
;"##### ; "166")]
#[test_case(r#####"tw`inset-8`"#####, r#####"({
  inset: "2rem",
})
;"##### ; "167")]
#[test_case(r#####"tw`-inset-8`"#####, r#####"({
  inset: "-2rem",
})
;"##### ; "168")]
#[test_case(r#####"tw`inset-y-8`"#####, r#####"({
  top: "2rem",
  bottom: "2rem",
})
;"##### ; "169")]
#[test_case(r#####"tw`inset-x-8`"#####, r#####"({
  left: "2rem",
  right: "2rem",
})
;"##### ; "170")]
#[test_case(r#####"tw`-inset-y-8`"#####, r#####"({
  top: "-2rem",
  bottom: "-2rem",
})
;"##### ; "171")]
#[test_case(r#####"tw`-inset-x-8`"#####, r#####"({
  left: "-2rem",
  right: "-2rem",
})
;"##### ; "172")]
#[test_case(r#####"tw`top-8`"#####, r#####"({
  top: "2rem",
})
;"##### ; "173")]
#[test_case(r#####"tw`right-8`"#####, r#####"({
  right: "2rem",
})
;"##### ; "174")]
#[test_case(r#####"tw`bottom-8`"#####, r#####"({
  bottom: "2rem",
})
;"##### ; "175")]
#[test_case(r#####"tw`left-8`"#####, r#####"({
  left: "2rem",
})
;"##### ; "176")]
#[test_case(r#####"tw`-top-8`"#####, r#####"({
  top: "-2rem",
})
;"##### ; "177")]
#[test_case(r#####"tw`-right-8`"#####, r#####"({
  right: "-2rem",
})
;"##### ; "178")]
#[test_case(r#####"tw`-bottom-8`"#####, r#####"({
  bottom: "-2rem",
})
;"##### ; "179")]
#[test_case(r#####"tw`-left-8`"#####, r#####"({
  left: "-2rem",
})
;"##### ; "180")]
#[test_case(r#####"tw`inset-9`"#####, r#####"({
  inset: "2.25rem",
})
;"##### ; "181")]
#[test_case(r#####"tw`-inset-9`"#####, r#####"({
  inset: "-2.25rem",
})
;"##### ; "182")]
#[test_case(r#####"tw`inset-y-9`"#####, r#####"({
  top: "2.25rem",
  bottom: "2.25rem",
})
;"##### ; "183")]
#[test_case(r#####"tw`inset-x-9`"#####, r#####"({
  left: "2.25rem",
  right: "2.25rem",
})
;"##### ; "184")]
#[test_case(r#####"tw`-inset-y-9`"#####, r#####"({
  top: "-2.25rem",
  bottom: "-2.25rem",
})
;"##### ; "185")]
#[test_case(r#####"tw`-inset-x-9`"#####, r#####"({
  left: "-2.25rem",
  right: "-2.25rem",
})
;"##### ; "186")]
#[test_case(r#####"tw`top-9`"#####, r#####"({
  top: "2.25rem",
})
;"##### ; "187")]
#[test_case(r#####"tw`right-9`"#####, r#####"({
  right: "2.25rem",
})
;"##### ; "188")]
#[test_case(r#####"tw`bottom-9`"#####, r#####"({
  bottom: "2.25rem",
})
;"##### ; "189")]
#[test_case(r#####"tw`left-9`"#####, r#####"({
  left: "2.25rem",
})
;"##### ; "190")]
#[test_case(r#####"tw`-top-9`"#####, r#####"({
  top: "-2.25rem",
})
;"##### ; "191")]
#[test_case(r#####"tw`-right-9`"#####, r#####"({
  right: "-2.25rem",
})
;"##### ; "192")]
#[test_case(r#####"tw`-bottom-9`"#####, r#####"({
  bottom: "-2.25rem",
})
;"##### ; "193")]
#[test_case(r#####"tw`-left-9`"#####, r#####"({
  left: "-2.25rem",
})
;"##### ; "194")]
#[test_case(r#####"tw`inset-10`"#####, r#####"({
  inset: "2.5rem",
})
;"##### ; "195")]
#[test_case(r#####"tw`-inset-10`"#####, r#####"({
  inset: "-2.5rem",
})
;"##### ; "196")]
#[test_case(r#####"tw`inset-y-10`"#####, r#####"({
  top: "2.5rem",
  bottom: "2.5rem",
})
;"##### ; "197")]
#[test_case(r#####"tw`inset-x-10`"#####, r#####"({
  left: "2.5rem",
  right: "2.5rem",
})
;"##### ; "198")]
#[test_case(r#####"tw`-inset-y-10`"#####, r#####"({
  top: "-2.5rem",
  bottom: "-2.5rem",
})
;"##### ; "199")]
#[test_case(r#####"tw`-inset-x-10`"#####, r#####"({
  left: "-2.5rem",
  right: "-2.5rem",
})
;"##### ; "200")]
#[test_case(r#####"tw`top-10`"#####, r#####"({
  top: "2.5rem",
})
;"##### ; "201")]
#[test_case(r#####"tw`right-10`"#####, r#####"({
  right: "2.5rem",
})
;"##### ; "202")]
#[test_case(r#####"tw`bottom-10`"#####, r#####"({
  bottom: "2.5rem",
})
;"##### ; "203")]
#[test_case(r#####"tw`left-10`"#####, r#####"({
  left: "2.5rem",
})
;"##### ; "204")]
#[test_case(r#####"tw`-top-10`"#####, r#####"({
  top: "-2.5rem",
})
;"##### ; "205")]
#[test_case(r#####"tw`-right-10`"#####, r#####"({
  right: "-2.5rem",
})
;"##### ; "206")]
#[test_case(r#####"tw`-bottom-10`"#####, r#####"({
  bottom: "-2.5rem",
})
;"##### ; "207")]
#[test_case(r#####"tw`-left-10`"#####, r#####"({
  left: "-2.5rem",
})
;"##### ; "208")]
#[test_case(r#####"tw`inset-11`"#####, r#####"({
  inset: "2.75rem",
})
;"##### ; "209")]
#[test_case(r#####"tw`-inset-11`"#####, r#####"({
  inset: "-2.75rem",
})
;"##### ; "210")]
#[test_case(r#####"tw`inset-y-11`"#####, r#####"({
  top: "2.75rem",
  bottom: "2.75rem",
})
;"##### ; "211")]
#[test_case(r#####"tw`inset-x-11`"#####, r#####"({
  left: "2.75rem",
  right: "2.75rem",
})
;"##### ; "212")]
#[test_case(r#####"tw`-inset-y-11`"#####, r#####"({
  top: "-2.75rem",
  bottom: "-2.75rem",
})
;"##### ; "213")]
#[test_case(r#####"tw`-inset-x-11`"#####, r#####"({
  left: "-2.75rem",
  right: "-2.75rem",
})
;"##### ; "214")]
#[test_case(r#####"tw`top-11`"#####, r#####"({
  top: "2.75rem",
})
;"##### ; "215")]
#[test_case(r#####"tw`right-11`"#####, r#####"({
  right: "2.75rem",
})
;"##### ; "216")]
#[test_case(r#####"tw`bottom-11`"#####, r#####"({
  bottom: "2.75rem",
})
;"##### ; "217")]
#[test_case(r#####"tw`left-11`"#####, r#####"({
  left: "2.75rem",
})
;"##### ; "218")]
#[test_case(r#####"tw`-top-11`"#####, r#####"({
  top: "-2.75rem",
})
;"##### ; "219")]
#[test_case(r#####"tw`-right-11`"#####, r#####"({
  right: "-2.75rem",
})
;"##### ; "220")]
#[test_case(r#####"tw`-bottom-11`"#####, r#####"({
  bottom: "-2.75rem",
})
;"##### ; "221")]
#[test_case(r#####"tw`-left-11`"#####, r#####"({
  left: "-2.75rem",
})
;"##### ; "222")]
#[test_case(r#####"tw`inset-12`"#####, r#####"({
  inset: "3rem",
})
;"##### ; "223")]
#[test_case(r#####"tw`-inset-12`"#####, r#####"({
  inset: "-3rem",
})
;"##### ; "224")]
#[test_case(r#####"tw`inset-y-12`"#####, r#####"({
  top: "3rem",
  bottom: "3rem",
})
;"##### ; "225")]
#[test_case(r#####"tw`inset-x-12`"#####, r#####"({
  left: "3rem",
  right: "3rem",
})
;"##### ; "226")]
#[test_case(r#####"tw`-inset-y-12`"#####, r#####"({
  top: "-3rem",
  bottom: "-3rem",
})
;"##### ; "227")]
#[test_case(r#####"tw`-inset-x-12`"#####, r#####"({
  left: "-3rem",
  right: "-3rem",
})
;"##### ; "228")]
#[test_case(r#####"tw`top-12`"#####, r#####"({
  top: "3rem",
})
;"##### ; "229")]
#[test_case(r#####"tw`right-12`"#####, r#####"({
  right: "3rem",
})
;"##### ; "230")]
#[test_case(r#####"tw`bottom-12`"#####, r#####"({
  bottom: "3rem",
})
;"##### ; "231")]
#[test_case(r#####"tw`left-12`"#####, r#####"({
  left: "3rem",
})
;"##### ; "232")]
#[test_case(r#####"tw`-top-12`"#####, r#####"({
  top: "-3rem",
})
;"##### ; "233")]
#[test_case(r#####"tw`-right-12`"#####, r#####"({
  right: "-3rem",
})
;"##### ; "234")]
#[test_case(r#####"tw`-bottom-12`"#####, r#####"({
  bottom: "-3rem",
})
;"##### ; "235")]
#[test_case(r#####"tw`-left-12`"#####, r#####"({
  left: "-3rem",
})
;"##### ; "236")]
#[test_case(r#####"tw`inset-14`"#####, r#####"({
  inset: "3.5rem",
})
;"##### ; "237")]
#[test_case(r#####"tw`-inset-14`"#####, r#####"({
  inset: "-3.5rem",
})
;"##### ; "238")]
#[test_case(r#####"tw`inset-y-14`"#####, r#####"({
  top: "3.5rem",
  bottom: "3.5rem",
})
;"##### ; "239")]
#[test_case(r#####"tw`inset-x-14`"#####, r#####"({
  left: "3.5rem",
  right: "3.5rem",
})
;"##### ; "240")]
#[test_case(r#####"tw`-inset-y-14`"#####, r#####"({
  top: "-3.5rem",
  bottom: "-3.5rem",
})
;"##### ; "241")]
#[test_case(r#####"tw`-inset-x-14`"#####, r#####"({
  left: "-3.5rem",
  right: "-3.5rem",
})
;"##### ; "242")]
#[test_case(r#####"tw`top-14`"#####, r#####"({
  top: "3.5rem",
})
;"##### ; "243")]
#[test_case(r#####"tw`right-14`"#####, r#####"({
  right: "3.5rem",
})
;"##### ; "244")]
#[test_case(r#####"tw`bottom-14`"#####, r#####"({
  bottom: "3.5rem",
})
;"##### ; "245")]
#[test_case(r#####"tw`left-14`"#####, r#####"({
  left: "3.5rem",
})
;"##### ; "246")]
#[test_case(r#####"tw`-top-14`"#####, r#####"({
  top: "-3.5rem",
})
;"##### ; "247")]
#[test_case(r#####"tw`-right-14`"#####, r#####"({
  right: "-3.5rem",
})
;"##### ; "248")]
#[test_case(r#####"tw`-bottom-14`"#####, r#####"({
  bottom: "-3.5rem",
})
;"##### ; "249")]
#[test_case(r#####"tw`-left-14`"#####, r#####"({
  left: "-3.5rem",
})
;"##### ; "250")]
#[test_case(r#####"tw`inset-16`"#####, r#####"({
  inset: "4rem",
})
;"##### ; "251")]
#[test_case(r#####"tw`-inset-16`"#####, r#####"({
  inset: "-4rem",
})
;"##### ; "252")]
#[test_case(r#####"tw`inset-y-16`"#####, r#####"({
  top: "4rem",
  bottom: "4rem",
})
;"##### ; "253")]
#[test_case(r#####"tw`inset-x-16`"#####, r#####"({
  left: "4rem",
  right: "4rem",
})
;"##### ; "254")]
#[test_case(r#####"tw`-inset-y-16`"#####, r#####"({
  top: "-4rem",
  bottom: "-4rem",
})
;"##### ; "255")]
#[test_case(r#####"tw`-inset-x-16`"#####, r#####"({
  left: "-4rem",
  right: "-4rem",
})
;"##### ; "256")]
#[test_case(r#####"tw`top-16`"#####, r#####"({
  top: "4rem",
})
;"##### ; "257")]
#[test_case(r#####"tw`right-16`"#####, r#####"({
  right: "4rem",
})
;"##### ; "258")]
#[test_case(r#####"tw`bottom-16`"#####, r#####"({
  bottom: "4rem",
})
;"##### ; "259")]
#[test_case(r#####"tw`left-16`"#####, r#####"({
  left: "4rem",
})
;"##### ; "260")]
#[test_case(r#####"tw`-top-16`"#####, r#####"({
  top: "-4rem",
})
;"##### ; "261")]
#[test_case(r#####"tw`-right-16`"#####, r#####"({
  right: "-4rem",
})
;"##### ; "262")]
#[test_case(r#####"tw`-bottom-16`"#####, r#####"({
  bottom: "-4rem",
})
;"##### ; "263")]
#[test_case(r#####"tw`-left-16`"#####, r#####"({
  left: "-4rem",
})
;"##### ; "264")]
#[test_case(r#####"tw`inset-20`"#####, r#####"({
  inset: "5rem",
})
;"##### ; "265")]
#[test_case(r#####"tw`-inset-20`"#####, r#####"({
  inset: "-5rem",
})
;"##### ; "266")]
#[test_case(r#####"tw`inset-y-20`"#####, r#####"({
  top: "5rem",
  bottom: "5rem",
})
;"##### ; "267")]
#[test_case(r#####"tw`inset-x-20`"#####, r#####"({
  left: "5rem",
  right: "5rem",
})
;"##### ; "268")]
#[test_case(r#####"tw`-inset-y-20`"#####, r#####"({
  top: "-5rem",
  bottom: "-5rem",
})
;"##### ; "269")]
#[test_case(r#####"tw`-inset-x-20`"#####, r#####"({
  left: "-5rem",
  right: "-5rem",
})
;"##### ; "270")]
#[test_case(r#####"tw`top-20`"#####, r#####"({
  top: "5rem",
})
;"##### ; "271")]
#[test_case(r#####"tw`right-20`"#####, r#####"({
  right: "5rem",
})
;"##### ; "272")]
#[test_case(r#####"tw`bottom-20`"#####, r#####"({
  bottom: "5rem",
})
;"##### ; "273")]
#[test_case(r#####"tw`left-20`"#####, r#####"({
  left: "5rem",
})
;"##### ; "274")]
#[test_case(r#####"tw`-top-20`"#####, r#####"({
  top: "-5rem",
})
;"##### ; "275")]
#[test_case(r#####"tw`-right-20`"#####, r#####"({
  right: "-5rem",
})
;"##### ; "276")]
#[test_case(r#####"tw`-bottom-20`"#####, r#####"({
  bottom: "-5rem",
})
;"##### ; "277")]
#[test_case(r#####"tw`-left-20`"#####, r#####"({
  left: "-5rem",
})
;"##### ; "278")]
#[test_case(r#####"tw`inset-24`"#####, r#####"({
  inset: "6rem",
})
;"##### ; "279")]
#[test_case(r#####"tw`-inset-24`"#####, r#####"({
  inset: "-6rem",
})
;"##### ; "280")]
#[test_case(r#####"tw`inset-y-24`"#####, r#####"({
  top: "6rem",
  bottom: "6rem",
})
;"##### ; "281")]
#[test_case(r#####"tw`inset-x-24`"#####, r#####"({
  left: "6rem",
  right: "6rem",
})
;"##### ; "282")]
#[test_case(r#####"tw`-inset-y-24`"#####, r#####"({
  top: "-6rem",
  bottom: "-6rem",
})
;"##### ; "283")]
#[test_case(r#####"tw`-inset-x-24`"#####, r#####"({
  left: "-6rem",
  right: "-6rem",
})
;"##### ; "284")]
#[test_case(r#####"tw`top-24`"#####, r#####"({
  top: "6rem",
})
;"##### ; "285")]
#[test_case(r#####"tw`right-24`"#####, r#####"({
  right: "6rem",
})
;"##### ; "286")]
#[test_case(r#####"tw`bottom-24`"#####, r#####"({
  bottom: "6rem",
})
;"##### ; "287")]
#[test_case(r#####"tw`left-24`"#####, r#####"({
  left: "6rem",
})
;"##### ; "288")]
#[test_case(r#####"tw`-right-24`"#####, r#####"({
  right: "-6rem",
})
;"##### ; "289")]
#[test_case(r#####"tw`-bottom-24`"#####, r#####"({
  bottom: "-6rem",
})
;"##### ; "290")]
#[test_case(r#####"tw`-left-24`"#####, r#####"({
  left: "-6rem",
})
;"##### ; "291")]
#[test_case(r#####"tw`inset-28`"#####, r#####"({
  inset: "7rem",
})
;"##### ; "292")]
#[test_case(r#####"tw`-inset-28`"#####, r#####"({
  inset: "-7rem",
})
;"##### ; "293")]
#[test_case(r#####"tw`inset-y-28`"#####, r#####"({
  top: "7rem",
  bottom: "7rem",
})
;"##### ; "294")]
#[test_case(r#####"tw`inset-x-28`"#####, r#####"({
  left: "7rem",
  right: "7rem",
})
;"##### ; "295")]
#[test_case(r#####"tw`-inset-y-28`"#####, r#####"({
  top: "-7rem",
  bottom: "-7rem",
})
;"##### ; "296")]
#[test_case(r#####"tw`-inset-x-28`"#####, r#####"({
  left: "-7rem",
  right: "-7rem",
})
;"##### ; "297")]
#[test_case(r#####"tw`top-28`"#####, r#####"({
  top: "7rem",
})
;"##### ; "298")]
#[test_case(r#####"tw`right-28`"#####, r#####"({
  right: "7rem",
})
;"##### ; "299")]
#[test_case(r#####"tw`bottom-28`"#####, r#####"({
  bottom: "7rem",
})
;"##### ; "300")]
#[test_case(r#####"tw`left-28`"#####, r#####"({
  left: "7rem",
})
;"##### ; "301")]
#[test_case(r#####"tw`-top-28`"#####, r#####"({
  top: "-7rem",
})
;"##### ; "302")]
#[test_case(r#####"tw`-right-28`"#####, r#####"({
  right: "-7rem",
})
;"##### ; "303")]
#[test_case(r#####"tw`-bottom-28`"#####, r#####"({
  bottom: "-7rem",
})
;"##### ; "304")]
#[test_case(r#####"tw`-left-28`"#####, r#####"({
  left: "-7rem",
})
;"##### ; "305")]
#[test_case(r#####"tw`inset-32`"#####, r#####"({
  inset: "8rem",
})
;"##### ; "306")]
#[test_case(r#####"tw`-inset-32`"#####, r#####"({
  inset: "-8rem",
})
;"##### ; "307")]
#[test_case(r#####"tw`inset-y-32`"#####, r#####"({
  top: "8rem",
  bottom: "8rem",
})
;"##### ; "308")]
#[test_case(r#####"tw`inset-x-32`"#####, r#####"({
  left: "8rem",
  right: "8rem",
})
;"##### ; "309")]
#[test_case(r#####"tw`-inset-y-32`"#####, r#####"({
  top: "-8rem",
  bottom: "-8rem",
})
;"##### ; "310")]
#[test_case(r#####"tw`-inset-x-32`"#####, r#####"({
  left: "-8rem",
  right: "-8rem",
})
;"##### ; "311")]
#[test_case(r#####"tw`right-32`"#####, r#####"({
  right: "8rem",
})
;"##### ; "312")]
#[test_case(r#####"tw`bottom-32`"#####, r#####"({
  bottom: "8rem",
})
;"##### ; "313")]
#[test_case(r#####"tw`left-32`"#####, r#####"({
  left: "8rem",
})
;"##### ; "314")]
#[test_case(r#####"tw`-top-32`"#####, r#####"({
  top: "-8rem",
})
;"##### ; "315")]
#[test_case(r#####"tw`-right-32`"#####, r#####"({
  right: "-8rem",
})
;"##### ; "316")]
#[test_case(r#####"tw`-bottom-32`"#####, r#####"({
  bottom: "-8rem",
})
;"##### ; "317")]
#[test_case(r#####"tw`-left-32`"#####, r#####"({
  left: "-8rem",
})
;"##### ; "318")]
#[test_case(r#####"tw`inset-36`"#####, r#####"({
  inset: "9rem",
})
;"##### ; "319")]
#[test_case(r#####"tw`-inset-36`"#####, r#####"({
  inset: "-9rem",
})
;"##### ; "320")]
#[test_case(r#####"tw`inset-y-36`"#####, r#####"({
  top: "9rem",
  bottom: "9rem",
})
;"##### ; "321")]
#[test_case(r#####"tw`inset-x-36`"#####, r#####"({
  left: "9rem",
  right: "9rem",
})
;"##### ; "322")]
#[test_case(r#####"tw`-inset-y-36`"#####, r#####"({
  top: "-9rem",
  bottom: "-9rem",
})
;"##### ; "323")]
#[test_case(r#####"tw`-inset-x-36`"#####, r#####"({
  left: "-9rem",
  right: "-9rem",
})
;"##### ; "324")]
#[test_case(r#####"tw`top-36`"#####, r#####"({
  top: "9rem",
})
;"##### ; "325")]
#[test_case(r#####"tw`right-36`"#####, r#####"({
  right: "9rem",
})
;"##### ; "326")]
#[test_case(r#####"tw`bottom-36`"#####, r#####"({
  bottom: "9rem",
})
;"##### ; "327")]
#[test_case(r#####"tw`left-36`"#####, r#####"({
  left: "9rem",
})
;"##### ; "328")]
#[test_case(r#####"tw`-top-36`"#####, r#####"({
  top: "-9rem",
})
;"##### ; "329")]
#[test_case(r#####"tw`-right-36`"#####, r#####"({
  right: "-9rem",
})
;"##### ; "330")]
#[test_case(r#####"tw`-bottom-36`"#####, r#####"({
  bottom: "-9rem",
})
;"##### ; "331")]
#[test_case(r#####"tw`-left-36`"#####, r#####"({
  left: "-9rem",
})
;"##### ; "332")]
#[test_case(r#####"tw`inset-40`"#####, r#####"({
  inset: "10rem",
})
;"##### ; "333")]
#[test_case(r#####"tw`-inset-40`"#####, r#####"({
  inset: "-10rem",
})
;"##### ; "334")]
#[test_case(r#####"tw`inset-y-40`"#####, r#####"({
  top: "10rem",
  bottom: "10rem",
})
;"##### ; "335")]
#[test_case(r#####"tw`inset-x-40`"#####, r#####"({
  left: "10rem",
  right: "10rem",
})
;"##### ; "336")]
#[test_case(r#####"tw`-inset-y-40`"#####, r#####"({
  top: "-10rem",
  bottom: "-10rem",
})
;"##### ; "337")]
#[test_case(r#####"tw`-inset-x-40`"#####, r#####"({
  left: "-10rem",
  right: "-10rem",
})
;"##### ; "338")]
#[test_case(r#####"tw`top-40`"#####, r#####"({
  top: "10rem",
})
;"##### ; "339")]
#[test_case(r#####"tw`right-40`"#####, r#####"({
  right: "10rem",
})
;"##### ; "340")]
#[test_case(r#####"tw`bottom-40`"#####, r#####"({
  bottom: "10rem",
})
;"##### ; "341")]
#[test_case(r#####"tw`left-40`"#####, r#####"({
  left: "10rem",
})
;"##### ; "342")]
#[test_case(r#####"tw`-top-40`"#####, r#####"({
  top: "-10rem",
})
;"##### ; "343")]
#[test_case(r#####"tw`-right-40`"#####, r#####"({
  right: "-10rem",
})
;"##### ; "344")]
#[test_case(r#####"tw`-bottom-40`"#####, r#####"({
  bottom: "-10rem",
})
;"##### ; "345")]
#[test_case(r#####"tw`-left-40`"#####, r#####"({
  left: "-10rem",
})
;"##### ; "346")]
#[test_case(r#####"tw`inset-44`"#####, r#####"({
  inset: "11rem",
})
;"##### ; "347")]
#[test_case(r#####"tw`-inset-44`"#####, r#####"({
  inset: "-11rem",
})
;"##### ; "348")]
#[test_case(r#####"tw`inset-y-44`"#####, r#####"({
  top: "11rem",
  bottom: "11rem",
})
;"##### ; "349")]
#[test_case(r#####"tw`inset-x-44`"#####, r#####"({
  left: "11rem",
  right: "11rem",
})
;"##### ; "350")]
#[test_case(r#####"tw`-inset-y-44`"#####, r#####"({
  top: "-11rem",
  bottom: "-11rem",
})
;"##### ; "351")]
#[test_case(r#####"tw`-inset-x-44`"#####, r#####"({
  left: "-11rem",
  right: "-11rem",
})
;"##### ; "352")]
#[test_case(r#####"tw`top-44`"#####, r#####"({
  top: "11rem",
})
;"##### ; "353")]
#[test_case(r#####"tw`right-44`"#####, r#####"({
  right: "11rem",
})
;"##### ; "354")]
#[test_case(r#####"tw`bottom-44`"#####, r#####"({
  bottom: "11rem",
})
;"##### ; "355")]
#[test_case(r#####"tw`left-44`"#####, r#####"({
  left: "11rem",
})
;"##### ; "356")]
#[test_case(r#####"tw`-top-44`"#####, r#####"({
  top: "-11rem",
})
;"##### ; "357")]
#[test_case(r#####"tw`-right-44`"#####, r#####"({
  right: "-11rem",
})
;"##### ; "358")]
#[test_case(r#####"tw`-bottom-44`"#####, r#####"({
  bottom: "-11rem",
})
;"##### ; "359")]
#[test_case(r#####"tw`-left-44`"#####, r#####"({
  left: "-11rem",
})
;"##### ; "360")]
#[test_case(r#####"tw`inset-48`"#####, r#####"({
  inset: "12rem",
})
;"##### ; "361")]
#[test_case(r#####"tw`-inset-48`"#####, r#####"({
  inset: "-12rem",
})
;"##### ; "362")]
#[test_case(r#####"tw`inset-y-48`"#####, r#####"({
  top: "12rem",
  bottom: "12rem",
})
;"##### ; "363")]
#[test_case(r#####"tw`inset-x-48`"#####, r#####"({
  left: "12rem",
  right: "12rem",
})
;"##### ; "364")]
#[test_case(r#####"tw`-inset-y-48`"#####, r#####"({
  top: "-12rem",
  bottom: "-12rem",
})
;"##### ; "365")]
#[test_case(r#####"tw`-inset-x-48`"#####, r#####"({
  left: "-12rem",
  right: "-12rem",
})
;"##### ; "366")]
#[test_case(r#####"tw`top-48`"#####, r#####"({
  top: "12rem",
})
;"##### ; "367")]
#[test_case(r#####"tw`right-48`"#####, r#####"({
  right: "12rem",
})
;"##### ; "368")]
#[test_case(r#####"tw`bottom-48`"#####, r#####"({
  bottom: "12rem",
})
;"##### ; "369")]
#[test_case(r#####"tw`left-48`"#####, r#####"({
  left: "12rem",
})
;"##### ; "370")]
#[test_case(r#####"tw`-right-48`"#####, r#####"({
  right: "-12rem",
})
;"##### ; "371")]
#[test_case(r#####"tw`-bottom-48`"#####, r#####"({
  bottom: "-12rem",
})
;"##### ; "372")]
#[test_case(r#####"tw`-left-48`"#####, r#####"({
  left: "-12rem",
})
;"##### ; "373")]
#[test_case(r#####"tw`inset-52`"#####, r#####"({
  inset: "13rem",
})
;"##### ; "374")]
#[test_case(r#####"tw`-inset-52`"#####, r#####"({
  inset: "-13rem",
})
;"##### ; "375")]
#[test_case(r#####"tw`inset-y-52`"#####, r#####"({
  top: "13rem",
  bottom: "13rem",
})
;"##### ; "376")]
#[test_case(r#####"tw`inset-x-52`"#####, r#####"({
  left: "13rem",
  right: "13rem",
})
;"##### ; "377")]
#[test_case(r#####"tw`-inset-y-52`"#####, r#####"({
  top: "-13rem",
  bottom: "-13rem",
})
;"##### ; "378")]
#[test_case(r#####"tw`-inset-x-52`"#####, r#####"({
  left: "-13rem",
  right: "-13rem",
})
;"##### ; "379")]
#[test_case(r#####"tw`top-52`"#####, r#####"({
  top: "13rem",
})
;"##### ; "380")]
#[test_case(r#####"tw`right-52`"#####, r#####"({
  right: "13rem",
})
;"##### ; "381")]
#[test_case(r#####"tw`bottom-52`"#####, r#####"({
  bottom: "13rem",
})
;"##### ; "382")]
#[test_case(r#####"tw`left-52`"#####, r#####"({
  left: "13rem",
})
;"##### ; "383")]
#[test_case(r#####"tw`-top-52`"#####, r#####"({
  top: "-13rem",
})
;"##### ; "384")]
#[test_case(r#####"tw`-right-52`"#####, r#####"({
  right: "-13rem",
})
;"##### ; "385")]
#[test_case(r#####"tw`-bottom-52`"#####, r#####"({
  bottom: "-13rem",
})
;"##### ; "386")]
#[test_case(r#####"tw`-left-52`"#####, r#####"({
  left: "-13rem",
})
;"##### ; "387")]
#[test_case(r#####"tw`inset-56`"#####, r#####"({
  inset: "14rem",
})
;"##### ; "388")]
#[test_case(r#####"tw`-inset-56`"#####, r#####"({
  inset: "-14rem",
})
;"##### ; "389")]
#[test_case(r#####"tw`inset-y-56`"#####, r#####"({
  top: "14rem",
  bottom: "14rem",
})
;"##### ; "390")]
#[test_case(r#####"tw`inset-x-56`"#####, r#####"({
  left: "14rem",
  right: "14rem",
})
;"##### ; "391")]
#[test_case(r#####"tw`-inset-y-56`"#####, r#####"({
  top: "-14rem",
  bottom: "-14rem",
})
;"##### ; "392")]
#[test_case(r#####"tw`-inset-x-56`"#####, r#####"({
  left: "-14rem",
  right: "-14rem",
})
;"##### ; "393")]
#[test_case(r#####"tw`top-56`"#####, r#####"({
  top: "14rem",
})
;"##### ; "394")]
#[test_case(r#####"tw`right-56`"#####, r#####"({
  right: "14rem",
})
;"##### ; "395")]
#[test_case(r#####"tw`bottom-56`"#####, r#####"({
  bottom: "14rem",
})
;"##### ; "396")]
#[test_case(r#####"tw`left-56`"#####, r#####"({
  left: "14rem",
})
;"##### ; "397")]
#[test_case(r#####"tw`-top-56`"#####, r#####"({
  top: "-14rem",
})
;"##### ; "398")]
#[test_case(r#####"tw`-right-56`"#####, r#####"({
  right: "-14rem",
})
;"##### ; "399")]
#[test_case(r#####"tw`-bottom-56`"#####, r#####"({
  bottom: "-14rem",
})
;"##### ; "400")]
#[test_case(r#####"tw`-left-56`"#####, r#####"({
  left: "-14rem",
})
;"##### ; "401")]
#[test_case(r#####"tw`inset-60`"#####, r#####"({
  inset: "15rem",
})
;"##### ; "402")]
#[test_case(r#####"tw`-inset-60`"#####, r#####"({
  inset: "-15rem",
})
;"##### ; "403")]
#[test_case(r#####"tw`inset-y-60`"#####, r#####"({
  top: "15rem",
  bottom: "15rem",
})
;"##### ; "404")]
#[test_case(r#####"tw`inset-x-60`"#####, r#####"({
  left: "15rem",
  right: "15rem",
})
;"##### ; "405")]
#[test_case(r#####"tw`-inset-y-60`"#####, r#####"({
  top: "-15rem",
  bottom: "-15rem",
})
;"##### ; "406")]
#[test_case(r#####"tw`-inset-x-60`"#####, r#####"({
  left: "-15rem",
  right: "-15rem",
})
;"##### ; "407")]
#[test_case(r#####"tw`top-60`"#####, r#####"({
  top: "15rem",
})
;"##### ; "408")]
#[test_case(r#####"tw`right-60`"#####, r#####"({
  right: "15rem",
})
;"##### ; "409")]
#[test_case(r#####"tw`bottom-60`"#####, r#####"({
  bottom: "15rem",
})
;"##### ; "410")]
#[test_case(r#####"tw`left-60`"#####, r#####"({
  left: "15rem",
})
;"##### ; "411")]
#[test_case(r#####"tw`-top-60`"#####, r#####"({
  top: "-15rem",
})
;"##### ; "412")]
#[test_case(r#####"tw`-right-60`"#####, r#####"({
  right: "-15rem",
})
;"##### ; "413")]
#[test_case(r#####"tw`-bottom-60`"#####, r#####"({
  bottom: "-15rem",
})
;"##### ; "414")]
#[test_case(r#####"tw`-left-60`"#####, r#####"({
  left: "-15rem",
})
;"##### ; "415")]
#[test_case(r#####"tw`inset-64`"#####, r#####"({
  inset: "16rem",
})
;"##### ; "416")]
#[test_case(r#####"tw`-inset-64`"#####, r#####"({
  inset: "-16rem",
})
;"##### ; "417")]
#[test_case(r#####"tw`inset-y-64`"#####, r#####"({
  top: "16rem",
  bottom: "16rem",
})
;"##### ; "418")]
#[test_case(r#####"tw`inset-x-64`"#####, r#####"({
  left: "16rem",
  right: "16rem",
})
;"##### ; "419")]
#[test_case(r#####"tw`-inset-y-64`"#####, r#####"({
  top: "-16rem",
  bottom: "-16rem",
})
;"##### ; "420")]
#[test_case(r#####"tw`-inset-x-64`"#####, r#####"({
  left: "-16rem",
  right: "-16rem",
})
;"##### ; "421")]
#[test_case(r#####"tw`top-64`"#####, r#####"({
  top: "16rem",
})
;"##### ; "422")]
#[test_case(r#####"tw`right-64`"#####, r#####"({
  right: "16rem",
})
;"##### ; "423")]
#[test_case(r#####"tw`bottom-64`"#####, r#####"({
  bottom: "16rem",
})
;"##### ; "424")]
#[test_case(r#####"tw`left-64`"#####, r#####"({
  left: "16rem",
})
;"##### ; "425")]
#[test_case(r#####"tw`-top-64`"#####, r#####"({
  top: "-16rem",
})
;"##### ; "426")]
#[test_case(r#####"tw`-right-64`"#####, r#####"({
  right: "-16rem",
})
;"##### ; "427")]
#[test_case(r#####"tw`-bottom-64`"#####, r#####"({
  bottom: "-16rem",
})
;"##### ; "428")]
#[test_case(r#####"tw`-left-64`"#####, r#####"({
  left: "-16rem",
})
;"##### ; "429")]
#[test_case(r#####"tw`inset-72`"#####, r#####"({
  inset: "18rem",
})
;"##### ; "430")]
#[test_case(r#####"tw`-inset-72`"#####, r#####"({
  inset: "-18rem",
})
;"##### ; "431")]
#[test_case(r#####"tw`inset-y-72`"#####, r#####"({
  top: "18rem",
  bottom: "18rem",
})
;"##### ; "432")]
#[test_case(r#####"tw`inset-x-72`"#####, r#####"({
  left: "18rem",
  right: "18rem",
})
;"##### ; "433")]
#[test_case(r#####"tw`-inset-y-72`"#####, r#####"({
  top: "-18rem",
  bottom: "-18rem",
})
;"##### ; "434")]
#[test_case(r#####"tw`-inset-x-72`"#####, r#####"({
  left: "-18rem",
  right: "-18rem",
})
;"##### ; "435")]
#[test_case(r#####"tw`top-72`"#####, r#####"({
  top: "18rem",
})
;"##### ; "436")]
#[test_case(r#####"tw`right-72`"#####, r#####"({
  right: "18rem",
})
;"##### ; "437")]
#[test_case(r#####"tw`bottom-72`"#####, r#####"({
  bottom: "18rem",
})
;"##### ; "438")]
#[test_case(r#####"tw`left-72`"#####, r#####"({
  left: "18rem",
})
;"##### ; "439")]
#[test_case(r#####"tw`-top-72`"#####, r#####"({
  top: "-18rem",
})
;"##### ; "440")]
#[test_case(r#####"tw`-right-72`"#####, r#####"({
  right: "-18rem",
})
;"##### ; "441")]
#[test_case(r#####"tw`-bottom-72`"#####, r#####"({
  bottom: "-18rem",
})
;"##### ; "442")]
#[test_case(r#####"tw`-left-72`"#####, r#####"({
  left: "-18rem",
})
;"##### ; "443")]
#[test_case(r#####"tw`inset-80`"#####, r#####"({
  inset: "20rem",
})
;"##### ; "444")]
#[test_case(r#####"tw`-inset-80`"#####, r#####"({
  inset: "-20rem",
})
;"##### ; "445")]
#[test_case(r#####"tw`inset-y-80`"#####, r#####"({
  top: "20rem",
  bottom: "20rem",
})
;"##### ; "446")]
#[test_case(r#####"tw`inset-x-80`"#####, r#####"({
  left: "20rem",
  right: "20rem",
})
;"##### ; "447")]
#[test_case(r#####"tw`-inset-y-80`"#####, r#####"({
  top: "-20rem",
  bottom: "-20rem",
})
;"##### ; "448")]
#[test_case(r#####"tw`-inset-x-80`"#####, r#####"({
  left: "-20rem",
  right: "-20rem",
})
;"##### ; "449")]
#[test_case(r#####"tw`top-80`"#####, r#####"({
  top: "20rem",
})
;"##### ; "450")]
#[test_case(r#####"tw`right-80`"#####, r#####"({
  right: "20rem",
})
;"##### ; "451")]
#[test_case(r#####"tw`bottom-80`"#####, r#####"({
  bottom: "20rem",
})
;"##### ; "452")]
#[test_case(r#####"tw`left-80`"#####, r#####"({
  left: "20rem",
})
;"##### ; "453")]
#[test_case(r#####"tw`-top-80`"#####, r#####"({
  top: "-20rem",
})
;"##### ; "454")]
#[test_case(r#####"tw`-right-80`"#####, r#####"({
  right: "-20rem",
})
;"##### ; "455")]
#[test_case(r#####"tw`-bottom-80`"#####, r#####"({
  bottom: "-20rem",
})
;"##### ; "456")]
#[test_case(r#####"tw`-left-80`"#####, r#####"({
  left: "-20rem",
})
;"##### ; "457")]
#[test_case(r#####"tw`inset-96`"#####, r#####"({
  inset: "24rem",
})
;"##### ; "458")]
#[test_case(r#####"tw`-inset-96`"#####, r#####"({
  inset: "-24rem",
})
;"##### ; "459")]
#[test_case(r#####"tw`inset-y-96`"#####, r#####"({
  top: "24rem",
  bottom: "24rem",
})
;"##### ; "460")]
#[test_case(r#####"tw`inset-x-96`"#####, r#####"({
  left: "24rem",
  right: "24rem",
})
;"##### ; "461")]
#[test_case(r#####"tw`-inset-y-96`"#####, r#####"({
  top: "-24rem",
  bottom: "-24rem",
})
;"##### ; "462")]
#[test_case(r#####"tw`-inset-x-96`"#####, r#####"({
  left: "-24rem",
  right: "-24rem",
})
;"##### ; "463")]
#[test_case(r#####"tw`top-96`"#####, r#####"({
  top: "24rem",
})
;"##### ; "464")]
#[test_case(r#####"tw`right-96`"#####, r#####"({
  right: "24rem",
})
;"##### ; "465")]
#[test_case(r#####"tw`bottom-96`"#####, r#####"({
  bottom: "24rem",
})
;"##### ; "466")]
#[test_case(r#####"tw`left-96`"#####, r#####"({
  left: "24rem",
})
;"##### ; "467")]
#[test_case(r#####"tw`-top-96`"#####, r#####"({
  top: "-24rem",
})
;"##### ; "468")]
#[test_case(r#####"tw`-right-96`"#####, r#####"({
  right: "-24rem",
})
;"##### ; "469")]
#[test_case(r#####"tw`-bottom-96`"#####, r#####"({
  bottom: "-24rem",
})
;"##### ; "470")]
#[test_case(r#####"tw`-left-96`"#####, r#####"({
  left: "-24rem",
})
;"##### ; "471")]
#[test_case(r#####"tw`inset-auto`"#####, r#####"({
  inset: "auto",
})
;"##### ; "472")]
#[test_case(r#####"tw`inset-px`"#####, r#####"({
  inset: "1px",
})
;"##### ; "473")]
#[test_case(r#####"tw`-inset-px`"#####, r#####"({
  inset: "-1px",
})
;"##### ; "474")]
#[test_case(r#####"tw`inset-1/2`"#####, r#####"({
  inset: "50%",
})
;"##### ; "475")]
#[test_case(r#####"tw`inset-1/3`"#####, r#####"({
  inset: "33.333333%",
})
;"##### ; "476")]
#[test_case(r#####"tw`inset-2/3`"#####, r#####"({
  inset: "66.666667%",
})
;"##### ; "477")]
#[test_case(r#####"tw`inset-1/4`"#####, r#####"({
  inset: "25%",
})
;"##### ; "478")]
#[test_case(r#####"tw`inset-2/4`"#####, r#####"({
  inset: "50%",
})
;"##### ; "479")]
#[test_case(r#####"tw`inset-3/4`"#####, r#####"({
  inset: "75%",
})
;"##### ; "480")]
#[test_case(r#####"tw`inset-full`"#####, r#####"({
  inset: "100%",
})
;"##### ; "481")]
#[test_case(r#####"tw`-inset-1/2`"#####, r#####"({
  inset: "-50%",
})
;"##### ; "482")]
#[test_case(r#####"tw`-inset-1/3`"#####, r#####"({
  inset: "-33.333333%",
})
;"##### ; "483")]
#[test_case(r#####"tw`-inset-2/3`"#####, r#####"({
  inset: "-66.666667%",
})
;"##### ; "484")]
#[test_case(r#####"tw`-inset-1/4`"#####, r#####"({
  inset: "-25%",
})
;"##### ; "485")]
#[test_case(r#####"tw`-inset-2/4`"#####, r#####"({
  inset: "-50%",
})
;"##### ; "486")]
#[test_case(r#####"tw`-inset-3/4`"#####, r#####"({
  inset: "-75%",
})
;"##### ; "487")]
#[test_case(r#####"tw`-inset-full`"#####, r#####"({
  inset: "-100%",
})
;"##### ; "488")]
#[test_case(r#####"tw`inset-y-auto`"#####, r#####"({
  top: "auto",
  bottom: "auto",
})
;"##### ; "489")]
#[test_case(r#####"tw`inset-x-auto`"#####, r#####"({
  left: "auto",
  right: "auto",
})
;"##### ; "490")]
#[test_case(r#####"tw`inset-y-px`"#####, r#####"({
  top: "1px",
  bottom: "1px",
})
;"##### ; "491")]
#[test_case(r#####"tw`inset-x-px`"#####, r#####"({
  left: "1px",
  right: "1px",
})
;"##### ; "492")]
#[test_case(r#####"tw`-inset-y-px`"#####, r#####"({
  top: "-1px",
  bottom: "-1px",
})
;"##### ; "493")]
#[test_case(r#####"tw`-inset-x-px`"#####, r#####"({
  left: "-1px",
  right: "-1px",
})
;"##### ; "494")]
#[test_case(r#####"tw`inset-y-1/2`"#####, r#####"({
  top: "50%",
  bottom: "50%",
})
;"##### ; "495")]
#[test_case(r#####"tw`inset-x-1/2`"#####, r#####"({
  left: "50%",
  right: "50%",
})
;"##### ; "496")]
#[test_case(r#####"tw`inset-y-1/3`"#####, r#####"({
  top: "33.333333%",
  bottom: "33.333333%",
})
;"##### ; "497")]
#[test_case(r#####"tw`inset-x-1/3`"#####, r#####"({
  left: "33.333333%",
  right: "33.333333%",
})
;"##### ; "498")]
#[test_case(r#####"tw`inset-y-2/3`"#####, r#####"({
  top: "66.666667%",
  bottom: "66.666667%",
})
;"##### ; "499")]
#[test_case(r#####"tw`inset-x-2/3`"#####, r#####"({
  left: "66.666667%",
  right: "66.666667%",
})
;"##### ; "500")]
#[test_case(r#####"tw`inset-y-1/4`"#####, r#####"({
  top: "25%",
  bottom: "25%",
})
;"##### ; "501")]
#[test_case(r#####"tw`inset-x-1/4`"#####, r#####"({
  left: "25%",
  right: "25%",
})
;"##### ; "502")]
#[test_case(r#####"tw`inset-y-2/4`"#####, r#####"({
  top: "50%",
  bottom: "50%",
})
;"##### ; "503")]
#[test_case(r#####"tw`inset-x-2/4`"#####, r#####"({
  left: "50%",
  right: "50%",
})
;"##### ; "504")]
#[test_case(r#####"tw`inset-y-3/4`"#####, r#####"({
  top: "75%",
  bottom: "75%",
})
;"##### ; "505")]
#[test_case(r#####"tw`inset-x-3/4`"#####, r#####"({
  left: "75%",
  right: "75%",
})
;"##### ; "506")]
#[test_case(r#####"tw`inset-y-full`"#####, r#####"({
  top: "100%",
  bottom: "100%",
})
;"##### ; "507")]
#[test_case(r#####"tw`inset-x-full`"#####, r#####"({
  left: "100%",
  right: "100%",
})
;"##### ; "508")]
#[test_case(r#####"tw`-inset-y-1/2`"#####, r#####"({
  top: "-50%",
  bottom: "-50%",
})
;"##### ; "509")]
#[test_case(r#####"tw`-inset-x-1/2`"#####, r#####"({
  left: "-50%",
  right: "-50%",
})
;"##### ; "510")]
#[test_case(r#####"tw`-inset-y-1/3`"#####, r#####"({
  top: "-33.333333%",
  bottom: "-33.333333%",
})
;"##### ; "511")]
#[test_case(r#####"tw`-inset-x-1/3`"#####, r#####"({
  left: "-33.333333%",
  right: "-33.333333%",
})
;"##### ; "512")]
#[test_case(r#####"tw`-inset-y-2/3`"#####, r#####"({
  top: "-66.666667%",
  bottom: "-66.666667%",
})
;"##### ; "513")]
#[test_case(r#####"tw`-inset-x-2/3`"#####, r#####"({
  left: "-66.666667%",
  right: "-66.666667%",
})
;"##### ; "514")]
#[test_case(r#####"tw`-inset-y-1/4`"#####, r#####"({
  top: "-25%",
  bottom: "-25%",
})
;"##### ; "515")]
#[test_case(r#####"tw`-inset-x-1/4`"#####, r#####"({
  left: "-25%",
  right: "-25%",
})
;"##### ; "516")]
#[test_case(r#####"tw`-inset-y-2/4`"#####, r#####"({
  top: "-50%",
  bottom: "-50%",
})
;"##### ; "517")]
#[test_case(r#####"tw`-inset-x-2/4`"#####, r#####"({
  left: "-50%",
  right: "-50%",
})
;"##### ; "518")]
#[test_case(r#####"tw`-inset-y-3/4`"#####, r#####"({
  top: "-75%",
  bottom: "-75%",
})
;"##### ; "519")]
#[test_case(r#####"tw`-inset-x-3/4`"#####, r#####"({
  left: "-75%",
  right: "-75%",
})
;"##### ; "520")]
#[test_case(r#####"tw`-inset-y-full`"#####, r#####"({
  top: "-100%",
  bottom: "-100%",
})
;"##### ; "521")]
#[test_case(r#####"tw`-inset-x-full`"#####, r#####"({
  left: "-100%",
  right: "-100%",
})
;"##### ; "522")]
#[test_case(r#####"tw`top-auto`"#####, r#####"({
  top: "auto",
})
;"##### ; "523")]
#[test_case(r#####"tw`right-auto`"#####, r#####"({
  right: "auto",
})
;"##### ; "524")]
#[test_case(r#####"tw`bottom-auto`"#####, r#####"({
  bottom: "auto",
})
;"##### ; "525")]
#[test_case(r#####"tw`left-auto`"#####, r#####"({
  left: "auto",
})
;"##### ; "526")]
#[test_case(r#####"tw`top-px`"#####, r#####"({
  top: "1px",
})
;"##### ; "527")]
#[test_case(r#####"tw`right-px`"#####, r#####"({
  right: "1px",
})
;"##### ; "528")]
#[test_case(r#####"tw`bottom-px`"#####, r#####"({
  bottom: "1px",
})
;"##### ; "529")]
#[test_case(r#####"tw`left-px`"#####, r#####"({
  left: "1px",
})
;"##### ; "530")]
#[test_case(r#####"tw`-top-px`"#####, r#####"({
  top: "-1px",
})
;"##### ; "531")]
#[test_case(r#####"tw`-right-px`"#####, r#####"({
  right: "-1px",
})
;"##### ; "532")]
#[test_case(r#####"tw`-bottom-px`"#####, r#####"({
  bottom: "-1px",
})
;"##### ; "533")]
#[test_case(r#####"tw`-left-px`"#####, r#####"({
  left: "-1px",
})
;"##### ; "534")]
#[test_case(r#####"tw`top-1/2`"#####, r#####"({
  top: "50%",
})
;"##### ; "535")]
#[test_case(r#####"tw`right-1/2`"#####, r#####"({
  right: "50%",
})
;"##### ; "536")]
#[test_case(r#####"tw`bottom-1/2`"#####, r#####"({
  bottom: "50%",
})
;"##### ; "537")]
#[test_case(r#####"tw`left-1/2`"#####, r#####"({
  left: "50%",
})
;"##### ; "538")]
#[test_case(r#####"tw`top-1/3`"#####, r#####"({
  top: "33.333333%",
})
;"##### ; "539")]
#[test_case(r#####"tw`right-1/3`"#####, r#####"({
  right: "33.333333%",
})
;"##### ; "540")]
#[test_case(r#####"tw`bottom-1/3`"#####, r#####"({
  bottom: "33.333333%",
})
;"##### ; "541")]
#[test_case(r#####"tw`left-1/3`"#####, r#####"({
  left: "33.333333%",
})
;"##### ; "542")]
#[test_case(r#####"tw`top-2/3`"#####, r#####"({
  top: "66.666667%",
})
;"##### ; "543")]
#[test_case(r#####"tw`right-2/3`"#####, r#####"({
  right: "66.666667%",
})
;"##### ; "544")]
#[test_case(r#####"tw`bottom-2/3`"#####, r#####"({
  bottom: "66.666667%",
})
;"##### ; "545")]
#[test_case(r#####"tw`left-2/3`"#####, r#####"({
  left: "66.666667%",
})
;"##### ; "546")]
#[test_case(r#####"tw`top-1/4`"#####, r#####"({
  top: "25%",
})
;"##### ; "547")]
#[test_case(r#####"tw`right-1/4`"#####, r#####"({
  right: "25%",
})
;"##### ; "548")]
#[test_case(r#####"tw`bottom-1/4`"#####, r#####"({
  bottom: "25%",
})
;"##### ; "549")]
#[test_case(r#####"tw`left-1/4`"#####, r#####"({
  left: "25%",
})
;"##### ; "550")]
#[test_case(r#####"tw`top-2/4`"#####, r#####"({
  top: "50%",
})
;"##### ; "551")]
#[test_case(r#####"tw`right-2/4`"#####, r#####"({
  right: "50%",
})
;"##### ; "552")]
#[test_case(r#####"tw`bottom-2/4`"#####, r#####"({
  bottom: "50%",
})
;"##### ; "553")]
#[test_case(r#####"tw`left-2/4`"#####, r#####"({
  left: "50%",
})
;"##### ; "554")]
#[test_case(r#####"tw`top-3/4`"#####, r#####"({
  top: "75%",
})
;"##### ; "555")]
#[test_case(r#####"tw`right-3/4`"#####, r#####"({
  right: "75%",
})
;"##### ; "556")]
#[test_case(r#####"tw`bottom-3/4`"#####, r#####"({
  bottom: "75%",
})
;"##### ; "557")]
#[test_case(r#####"tw`left-3/4`"#####, r#####"({
  left: "75%",
})
;"##### ; "558")]
#[test_case(r#####"tw`top-full`"#####, r#####"({
  top: "100%",
})
;"##### ; "559")]
#[test_case(r#####"tw`right-full`"#####, r#####"({
  right: "100%",
})
;"##### ; "560")]
#[test_case(r#####"tw`bottom-full`"#####, r#####"({
  bottom: "100%",
})
;"##### ; "561")]
#[test_case(r#####"tw`left-full`"#####, r#####"({
  left: "100%",
})
;"##### ; "562")]
#[test_case(r#####"tw`-top-1/2`"#####, r#####"({
  top: "-50%",
})
;"##### ; "563")]
#[test_case(r#####"tw`-right-1/2`"#####, r#####"({
  right: "-50%",
})
;"##### ; "564")]
#[test_case(r#####"tw`-bottom-1/2`"#####, r#####"({
  bottom: "-50%",
})
;"##### ; "565")]
#[test_case(r#####"tw`-left-1/2`"#####, r#####"({
  left: "-50%",
})
;"##### ; "566")]
#[test_case(r#####"tw`-top-1/3`"#####, r#####"({
  top: "-33.333333%",
})
;"##### ; "567")]
#[test_case(r#####"tw`-right-1/3`"#####, r#####"({
  right: "-33.333333%",
})
;"##### ; "568")]
#[test_case(r#####"tw`-bottom-1/3`"#####, r#####"({
  bottom: "-33.333333%",
})
;"##### ; "569")]
#[test_case(r#####"tw`-left-1/3`"#####, r#####"({
  left: "-33.333333%",
})
;"##### ; "570")]
#[test_case(r#####"tw`-top-2/3`"#####, r#####"({
  top: "-66.666667%",
})
;"##### ; "571")]
#[test_case(r#####"tw`-right-2/3`"#####, r#####"({
  right: "-66.666667%",
})
;"##### ; "572")]
#[test_case(r#####"tw`-bottom-2/3`"#####, r#####"({
  bottom: "-66.666667%",
})
;"##### ; "573")]
#[test_case(r#####"tw`-left-2/3`"#####, r#####"({
  left: "-66.666667%",
})
;"##### ; "574")]
#[test_case(r#####"tw`-top-1/4`"#####, r#####"({
  top: "-25%",
})
;"##### ; "575")]
#[test_case(r#####"tw`-right-1/4`"#####, r#####"({
  right: "-25%",
})
;"##### ; "576")]
#[test_case(r#####"tw`-bottom-1/4`"#####, r#####"({
  bottom: "-25%",
})
;"##### ; "577")]
#[test_case(r#####"tw`-left-1/4`"#####, r#####"({
  left: "-25%",
})
;"##### ; "578")]
#[test_case(r#####"tw`-top-2/4`"#####, r#####"({
  top: "-50%",
})
;"##### ; "579")]
#[test_case(r#####"tw`-right-2/4`"#####, r#####"({
  right: "-50%",
})
;"##### ; "580")]
#[test_case(r#####"tw`-bottom-2/4`"#####, r#####"({
  bottom: "-50%",
})
;"##### ; "581")]
#[test_case(r#####"tw`-left-2/4`"#####, r#####"({
  left: "-50%",
})
;"##### ; "582")]
#[test_case(r#####"tw`-top-3/4`"#####, r#####"({
  top: "-75%",
})
;"##### ; "583")]
#[test_case(r#####"tw`-right-3/4`"#####, r#####"({
  right: "-75%",
})
;"##### ; "584")]
#[test_case(r#####"tw`-bottom-3/4`"#####, r#####"({
  bottom: "-75%",
})
;"##### ; "585")]
#[test_case(r#####"tw`-left-3/4`"#####, r#####"({
  left: "-75%",
})
;"##### ; "586")]
#[test_case(r#####"tw`-top-full`"#####, r#####"({
  top: "-100%",
})
;"##### ; "587")]
#[test_case(r#####"tw`-right-full`"#####, r#####"({
  right: "-100%",
})
;"##### ; "588")]
#[test_case(r#####"tw`-bottom-full`"#####, r#####"({
  bottom: "-100%",
})
;"##### ; "589")]
#[test_case(r#####"tw`-left-full`"#####, r#####"({
  left: "-100%",
})
;"##### ; "590")]
#[test_case(r#####"tw`top-[3px]`"#####, r#####"({
  top: "3px",
})
;"##### ; "591")]
#[test_case(r#####"tw`inset-[50px]!`"#####, r#####"({
  inset: "50px !important",
})
;"##### ; "592")]
#[test_case(r#####"tw`inset-6 inset-x-1 start-4 end-8`"#####, r#####"({
  inset: "1.5rem",
  left: "0.25rem",
  right: "0.25rem",
  insetInlineEnd: "2rem",
  insetInlineStart: "1rem",
})"##### ; "593")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
