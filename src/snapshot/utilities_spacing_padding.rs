use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`padding`"#####, r#####"({
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
#[test_case(r#####"tw`p-0`"#####, r#####"({
  padding: "0px",
})
;"##### ; "2")]
#[test_case(r#####"tw`p-px`"#####, r#####"({
  padding: "1px",
})
;"##### ; "3")]
#[test_case(r#####"tw`p-0.5`"#####, r#####"({
  padding: "0.125rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`p-1`"#####, r#####"({
  padding: "0.25rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`p-1.5`"#####, r#####"({
  padding: "0.375rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`p-2`"#####, r#####"({
  padding: "0.5rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`p-2.5`"#####, r#####"({
  padding: "0.625rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`p-3`"#####, r#####"({
  padding: "0.75rem",
})
;"##### ; "9")]
#[test_case(r#####"tw`p-3.5`"#####, r#####"({
  padding: "0.875rem",
})
;"##### ; "10")]
#[test_case(r#####"tw`p-4`"#####, r#####"({
  padding: "1rem",
})
;"##### ; "11")]
#[test_case(r#####"tw`p-5`"#####, r#####"({
  padding: "1.25rem",
})
;"##### ; "12")]
#[test_case(r#####"tw`p-6`"#####, r#####"({
  padding: "1.5rem",
})
;"##### ; "13")]
#[test_case(r#####"tw`p-7`"#####, r#####"({
  padding: "1.75rem",
})
;"##### ; "14")]
#[test_case(r#####"tw`p-8`"#####, r#####"({
  padding: "2rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`p-9`"#####, r#####"({
  padding: "2.25rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`p-10`"#####, r#####"({
  padding: "2.5rem",
})
;"##### ; "17")]
#[test_case(r#####"tw`p-11`"#####, r#####"({
  padding: "2.75rem",
})
;"##### ; "18")]
#[test_case(r#####"tw`p-12`"#####, r#####"({
  padding: "3rem",
})
;"##### ; "19")]
#[test_case(r#####"tw`p-14`"#####, r#####"({
  padding: "3.5rem",
})
;"##### ; "20")]
#[test_case(r#####"tw`p-16`"#####, r#####"({
  padding: "4rem",
})
;"##### ; "21")]
#[test_case(r#####"tw`p-20`"#####, r#####"({
  padding: "5rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`p-24`"#####, r#####"({
  padding: "6rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`p-28`"#####, r#####"({
  padding: "7rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`p-32`"#####, r#####"({
  padding: "8rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`p-36`"#####, r#####"({
  padding: "9rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`p-40`"#####, r#####"({
  padding: "10rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`p-44`"#####, r#####"({
  padding: "11rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`p-48`"#####, r#####"({
  padding: "12rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`p-52`"#####, r#####"({
  padding: "13rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`p-56`"#####, r#####"({
  padding: "14rem",
})
;"##### ; "31")]
#[test_case(r#####"tw`p-60`"#####, r#####"({
  padding: "15rem",
})
;"##### ; "32")]
#[test_case(r#####"tw`p-64`"#####, r#####"({
  padding: "16rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`p-72`"#####, r#####"({
  padding: "18rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`p-80`"#####, r#####"({
  padding: "20rem",
})
;"##### ; "35")]
#[test_case(r#####"tw`p-96`"#####, r#####"({
  padding: "24rem",
})
;"##### ; "36")]
#[test_case(r#####"tw`py-0`"#####, r#####"({
  paddingTop: "0px",
  paddingBottom: "0px",
})
;"##### ; "37")]
#[test_case(r#####"tw`py-px`"#####, r#####"({
  paddingTop: "1px",
  paddingBottom: "1px",
})
;"##### ; "38")]
#[test_case(r#####"tw`py-0.5`"#####, r#####"({
  paddingTop: "0.125rem",
  paddingBottom: "0.125rem",
})
;"##### ; "39")]
#[test_case(r#####"tw`py-1`"#####, r#####"({
  paddingTop: "0.25rem",
  paddingBottom: "0.25rem",
})
;"##### ; "40")]
#[test_case(r#####"tw`py-1.5`"#####, r#####"({
  paddingTop: "0.375rem",
  paddingBottom: "0.375rem",
})
;"##### ; "41")]
#[test_case(r#####"tw`py-2`"#####, r#####"({
  paddingTop: "0.5rem",
  paddingBottom: "0.5rem",
})
;"##### ; "42")]
#[test_case(r#####"tw`py-2.5`"#####, r#####"({
  paddingTop: "0.625rem",
  paddingBottom: "0.625rem",
})
;"##### ; "43")]
#[test_case(r#####"tw`py-3`"#####, r#####"({
  paddingTop: "0.75rem",
  paddingBottom: "0.75rem",
})
;"##### ; "44")]
#[test_case(r#####"tw`py-3.5`"#####, r#####"({
  paddingTop: "0.875rem",
  paddingBottom: "0.875rem",
})
;"##### ; "45")]
#[test_case(r#####"tw`py-4`"#####, r#####"({
  paddingTop: "1rem",
  paddingBottom: "1rem",
})
;"##### ; "46")]
#[test_case(r#####"tw`py-5`"#####, r#####"({
  paddingTop: "1.25rem",
  paddingBottom: "1.25rem",
})
;"##### ; "47")]
#[test_case(r#####"tw`py-6`"#####, r#####"({
  paddingTop: "1.5rem",
  paddingBottom: "1.5rem",
})
;"##### ; "48")]
#[test_case(r#####"tw`py-7`"#####, r#####"({
  paddingTop: "1.75rem",
  paddingBottom: "1.75rem",
})
;"##### ; "49")]
#[test_case(r#####"tw`py-8`"#####, r#####"({
  paddingTop: "2rem",
  paddingBottom: "2rem",
})
;"##### ; "50")]
#[test_case(r#####"tw`py-9`"#####, r#####"({
  paddingTop: "2.25rem",
  paddingBottom: "2.25rem",
})
;"##### ; "51")]
#[test_case(r#####"tw`py-10`"#####, r#####"({
  paddingTop: "2.5rem",
  paddingBottom: "2.5rem",
})
;"##### ; "52")]
#[test_case(r#####"tw`py-11`"#####, r#####"({
  paddingTop: "2.75rem",
  paddingBottom: "2.75rem",
})
;"##### ; "53")]
#[test_case(r#####"tw`py-12`"#####, r#####"({
  paddingTop: "3rem",
  paddingBottom: "3rem",
})
;"##### ; "54")]
#[test_case(r#####"tw`py-14`"#####, r#####"({
  paddingTop: "3.5rem",
  paddingBottom: "3.5rem",
})
;"##### ; "55")]
#[test_case(r#####"tw`py-16`"#####, r#####"({
  paddingTop: "4rem",
  paddingBottom: "4rem",
})
;"##### ; "56")]
#[test_case(r#####"tw`py-20`"#####, r#####"({
  paddingTop: "5rem",
  paddingBottom: "5rem",
})
;"##### ; "57")]
#[test_case(r#####"tw`py-24`"#####, r#####"({
  paddingTop: "6rem",
  paddingBottom: "6rem",
})
;"##### ; "58")]
#[test_case(r#####"tw`py-28`"#####, r#####"({
  paddingTop: "7rem",
  paddingBottom: "7rem",
})
;"##### ; "59")]
#[test_case(r#####"tw`py-32`"#####, r#####"({
  paddingTop: "8rem",
  paddingBottom: "8rem",
})
;"##### ; "60")]
#[test_case(r#####"tw`py-36`"#####, r#####"({
  paddingTop: "9rem",
  paddingBottom: "9rem",
})
;"##### ; "61")]
#[test_case(r#####"tw`py-40`"#####, r#####"({
  paddingTop: "10rem",
  paddingBottom: "10rem",
})
;"##### ; "62")]
#[test_case(r#####"tw`py-44`"#####, r#####"({
  paddingTop: "11rem",
  paddingBottom: "11rem",
})
;"##### ; "63")]
#[test_case(r#####"tw`py-48`"#####, r#####"({
  paddingTop: "12rem",
  paddingBottom: "12rem",
})
;"##### ; "64")]
#[test_case(r#####"tw`py-52`"#####, r#####"({
  paddingTop: "13rem",
  paddingBottom: "13rem",
})
;"##### ; "65")]
#[test_case(r#####"tw`py-56`"#####, r#####"({
  paddingTop: "14rem",
  paddingBottom: "14rem",
})
;"##### ; "66")]
#[test_case(r#####"tw`py-60`"#####, r#####"({
  paddingTop: "15rem",
  paddingBottom: "15rem",
})
;"##### ; "67")]
#[test_case(r#####"tw`py-64`"#####, r#####"({
  paddingTop: "16rem",
  paddingBottom: "16rem",
})
;"##### ; "68")]
#[test_case(r#####"tw`py-72`"#####, r#####"({
  paddingTop: "18rem",
  paddingBottom: "18rem",
})
;"##### ; "69")]
#[test_case(r#####"tw`py-80`"#####, r#####"({
  paddingTop: "20rem",
  paddingBottom: "20rem",
})
;"##### ; "70")]
#[test_case(r#####"tw`py-96`"#####, r#####"({
  paddingTop: "24rem",
  paddingBottom: "24rem",
})
;"##### ; "71")]
#[test_case(r#####"tw`px-0`"#####, r#####"({
  paddingLeft: "0px",
  paddingRight: "0px",
})
;"##### ; "72")]
#[test_case(r#####"tw`px-px`"#####, r#####"({
  paddingLeft: "1px",
  paddingRight: "1px",
})
;"##### ; "73")]
#[test_case(r#####"tw`px-0.5`"#####, r#####"({
  paddingLeft: "0.125rem",
  paddingRight: "0.125rem",
})
;"##### ; "74")]
#[test_case(r#####"tw`px-1`"#####, r#####"({
  paddingLeft: "0.25rem",
  paddingRight: "0.25rem",
})
;"##### ; "75")]
#[test_case(r#####"tw`px-1.5`"#####, r#####"({
  paddingLeft: "0.375rem",
  paddingRight: "0.375rem",
})
;"##### ; "76")]
#[test_case(r#####"tw`px-2`"#####, r#####"({
  paddingLeft: "0.5rem",
  paddingRight: "0.5rem",
})
;"##### ; "77")]
#[test_case(r#####"tw`px-2.5`"#####, r#####"({
  paddingLeft: "0.625rem",
  paddingRight: "0.625rem",
})
;"##### ; "78")]
#[test_case(r#####"tw`px-3`"#####, r#####"({
  paddingLeft: "0.75rem",
  paddingRight: "0.75rem",
})
;"##### ; "79")]
#[test_case(r#####"tw`px-3.5`"#####, r#####"({
  paddingLeft: "0.875rem",
  paddingRight: "0.875rem",
})
;"##### ; "80")]
#[test_case(r#####"tw`px-4`"#####, r#####"({
  paddingLeft: "1rem",
  paddingRight: "1rem",
})
;"##### ; "81")]
#[test_case(r#####"tw`px-5`"#####, r#####"({
  paddingLeft: "1.25rem",
  paddingRight: "1.25rem",
})
;"##### ; "82")]
#[test_case(r#####"tw`px-6`"#####, r#####"({
  paddingLeft: "1.5rem",
  paddingRight: "1.5rem",
})
;"##### ; "83")]
#[test_case(r#####"tw`px-7`"#####, r#####"({
  paddingLeft: "1.75rem",
  paddingRight: "1.75rem",
})
;"##### ; "84")]
#[test_case(r#####"tw`px-8`"#####, r#####"({
  paddingLeft: "2rem",
  paddingRight: "2rem",
})
;"##### ; "85")]
#[test_case(r#####"tw`px-9`"#####, r#####"({
  paddingLeft: "2.25rem",
  paddingRight: "2.25rem",
})
;"##### ; "86")]
#[test_case(r#####"tw`px-10`"#####, r#####"({
  paddingLeft: "2.5rem",
  paddingRight: "2.5rem",
})
;"##### ; "87")]
#[test_case(r#####"tw`px-11`"#####, r#####"({
  paddingLeft: "2.75rem",
  paddingRight: "2.75rem",
})
;"##### ; "88")]
#[test_case(r#####"tw`px-12`"#####, r#####"({
  paddingLeft: "3rem",
  paddingRight: "3rem",
})
;"##### ; "89")]
#[test_case(r#####"tw`px-14`"#####, r#####"({
  paddingLeft: "3.5rem",
  paddingRight: "3.5rem",
})
;"##### ; "90")]
#[test_case(r#####"tw`px-16`"#####, r#####"({
  paddingLeft: "4rem",
  paddingRight: "4rem",
})
;"##### ; "91")]
#[test_case(r#####"tw`px-20`"#####, r#####"({
  paddingLeft: "5rem",
  paddingRight: "5rem",
})
;"##### ; "92")]
#[test_case(r#####"tw`px-24`"#####, r#####"({
  paddingLeft: "6rem",
  paddingRight: "6rem",
})
;"##### ; "93")]
#[test_case(r#####"tw`px-28`"#####, r#####"({
  paddingLeft: "7rem",
  paddingRight: "7rem",
})
;"##### ; "94")]
#[test_case(r#####"tw`px-32`"#####, r#####"({
  paddingLeft: "8rem",
  paddingRight: "8rem",
})
;"##### ; "95")]
#[test_case(r#####"tw`px-36`"#####, r#####"({
  paddingLeft: "9rem",
  paddingRight: "9rem",
})
;"##### ; "96")]
#[test_case(r#####"tw`px-40`"#####, r#####"({
  paddingLeft: "10rem",
  paddingRight: "10rem",
})
;"##### ; "97")]
#[test_case(r#####"tw`px-44`"#####, r#####"({
  paddingLeft: "11rem",
  paddingRight: "11rem",
})
;"##### ; "98")]
#[test_case(r#####"tw`px-48`"#####, r#####"({
  paddingLeft: "12rem",
  paddingRight: "12rem",
})
;"##### ; "99")]
#[test_case(r#####"tw`px-52`"#####, r#####"({
  paddingLeft: "13rem",
  paddingRight: "13rem",
})
;"##### ; "100")]
#[test_case(r#####"tw`px-56`"#####, r#####"({
  paddingLeft: "14rem",
  paddingRight: "14rem",
})
;"##### ; "101")]
#[test_case(r#####"tw`px-60`"#####, r#####"({
  paddingLeft: "15rem",
  paddingRight: "15rem",
})
;"##### ; "102")]
#[test_case(r#####"tw`px-64`"#####, r#####"({
  paddingLeft: "16rem",
  paddingRight: "16rem",
})
;"##### ; "103")]
#[test_case(r#####"tw`px-72`"#####, r#####"({
  paddingLeft: "18rem",
  paddingRight: "18rem",
})
;"##### ; "104")]
#[test_case(r#####"tw`px-80`"#####, r#####"({
  paddingLeft: "20rem",
  paddingRight: "20rem",
})
;"##### ; "105")]
#[test_case(r#####"tw`px-96`"#####, r#####"({
  paddingLeft: "24rem",
  paddingRight: "24rem",
})
;"##### ; "106")]
#[test_case(r#####"tw`pt-0`"#####, r#####"({
  paddingTop: "0px",
})
;"##### ; "107")]
#[test_case(r#####"tw`pt-px`"#####, r#####"({
  paddingTop: "1px",
})
;"##### ; "108")]
#[test_case(r#####"tw`pt-0.5`"#####, r#####"({
  paddingTop: "0.125rem",
})
;"##### ; "109")]
#[test_case(r#####"tw`pt-1`"#####, r#####"({
  paddingTop: "0.25rem",
})
;"##### ; "110")]
#[test_case(r#####"tw`pt-1.5`"#####, r#####"({
  paddingTop: "0.375rem",
})
;"##### ; "111")]
#[test_case(r#####"tw`pt-2`"#####, r#####"({
  paddingTop: "0.5rem",
})
;"##### ; "112")]
#[test_case(r#####"tw`pt-2.5`"#####, r#####"({
  paddingTop: "0.625rem",
})
;"##### ; "113")]
#[test_case(r#####"tw`pt-3`"#####, r#####"({
  paddingTop: "0.75rem",
})
;"##### ; "114")]
#[test_case(r#####"tw`pt-3.5`"#####, r#####"({
  paddingTop: "0.875rem",
})
;"##### ; "115")]
#[test_case(r#####"tw`pt-4`"#####, r#####"({
  paddingTop: "1rem",
})
;"##### ; "116")]
#[test_case(r#####"tw`pt-5`"#####, r#####"({
  paddingTop: "1.25rem",
})
;"##### ; "117")]
#[test_case(r#####"tw`pt-6`"#####, r#####"({
  paddingTop: "1.5rem",
})
;"##### ; "118")]
#[test_case(r#####"tw`pt-7`"#####, r#####"({
  paddingTop: "1.75rem",
})
;"##### ; "119")]
#[test_case(r#####"tw`pt-8`"#####, r#####"({
  paddingTop: "2rem",
})
;"##### ; "120")]
#[test_case(r#####"tw`pt-9`"#####, r#####"({
  paddingTop: "2.25rem",
})
;"##### ; "121")]
#[test_case(r#####"tw`pt-10`"#####, r#####"({
  paddingTop: "2.5rem",
})
;"##### ; "122")]
#[test_case(r#####"tw`pt-11`"#####, r#####"({
  paddingTop: "2.75rem",
})
;"##### ; "123")]
#[test_case(r#####"tw`pt-12`"#####, r#####"({
  paddingTop: "3rem",
})
;"##### ; "124")]
#[test_case(r#####"tw`pt-14`"#####, r#####"({
  paddingTop: "3.5rem",
})
;"##### ; "125")]
#[test_case(r#####"tw`pt-16`"#####, r#####"({
  paddingTop: "4rem",
})
;"##### ; "126")]
#[test_case(r#####"tw`pt-20`"#####, r#####"({
  paddingTop: "5rem",
})
;"##### ; "127")]
#[test_case(r#####"tw`pt-24`"#####, r#####"({
  paddingTop: "6rem",
})
;"##### ; "128")]
#[test_case(r#####"tw`pt-28`"#####, r#####"({
  paddingTop: "7rem",
})
;"##### ; "129")]
#[test_case(r#####"tw`pt-32`"#####, r#####"({
  paddingTop: "8rem",
})
;"##### ; "130")]
#[test_case(r#####"tw`pt-36`"#####, r#####"({
  paddingTop: "9rem",
})
;"##### ; "131")]
#[test_case(r#####"tw`pt-40`"#####, r#####"({
  paddingTop: "10rem",
})
;"##### ; "132")]
#[test_case(r#####"tw`pt-44`"#####, r#####"({
  paddingTop: "11rem",
})
;"##### ; "133")]
#[test_case(r#####"tw`pt-48`"#####, r#####"({
  paddingTop: "12rem",
})
;"##### ; "134")]
#[test_case(r#####"tw`pt-52`"#####, r#####"({
  paddingTop: "13rem",
})
;"##### ; "135")]
#[test_case(r#####"tw`pt-56`"#####, r#####"({
  paddingTop: "14rem",
})
;"##### ; "136")]
#[test_case(r#####"tw`pt-60`"#####, r#####"({
  paddingTop: "15rem",
})
;"##### ; "137")]
#[test_case(r#####"tw`pt-64`"#####, r#####"({
  paddingTop: "16rem",
})
;"##### ; "138")]
#[test_case(r#####"tw`pt-72`"#####, r#####"({
  paddingTop: "18rem",
})
;"##### ; "139")]
#[test_case(r#####"tw`pt-80`"#####, r#####"({
  paddingTop: "20rem",
})
;"##### ; "140")]
#[test_case(r#####"tw`pt-96`"#####, r#####"({
  paddingTop: "24rem",
})
;"##### ; "141")]
#[test_case(r#####"tw`pr-0`"#####, r#####"({
  paddingRight: "0px",
})
;"##### ; "142")]
#[test_case(r#####"tw`pr-0.5`"#####, r#####"({
  paddingRight: "0.125rem",
})
;"##### ; "143")]
#[test_case(r#####"tw`pr-1`"#####, r#####"({
  paddingRight: "0.25rem",
})
;"##### ; "144")]
#[test_case(r#####"tw`pr-1.5`"#####, r#####"({
  paddingRight: "0.375rem",
})
;"##### ; "145")]
#[test_case(r#####"tw`pr-2`"#####, r#####"({
  paddingRight: "0.5rem",
})
;"##### ; "146")]
#[test_case(r#####"tw`pr-2.5`"#####, r#####"({
  paddingRight: "0.625rem",
})
;"##### ; "147")]
#[test_case(r#####"tw`pr-3`"#####, r#####"({
  paddingRight: "0.75rem",
})
;"##### ; "148")]
#[test_case(r#####"tw`pr-3.5`"#####, r#####"({
  paddingRight: "0.875rem",
})
;"##### ; "149")]
#[test_case(r#####"tw`pr-4`"#####, r#####"({
  paddingRight: "1rem",
})
;"##### ; "150")]
#[test_case(r#####"tw`pr-5`"#####, r#####"({
  paddingRight: "1.25rem",
})
;"##### ; "151")]
#[test_case(r#####"tw`pr-6`"#####, r#####"({
  paddingRight: "1.5rem",
})
;"##### ; "152")]
#[test_case(r#####"tw`pr-7`"#####, r#####"({
  paddingRight: "1.75rem",
})
;"##### ; "153")]
#[test_case(r#####"tw`pr-8`"#####, r#####"({
  paddingRight: "2rem",
})
;"##### ; "154")]
#[test_case(r#####"tw`pr-9`"#####, r#####"({
  paddingRight: "2.25rem",
})
;"##### ; "155")]
#[test_case(r#####"tw`pr-10`"#####, r#####"({
  paddingRight: "2.5rem",
})
;"##### ; "156")]
#[test_case(r#####"tw`pr-11`"#####, r#####"({
  paddingRight: "2.75rem",
})
;"##### ; "157")]
#[test_case(r#####"tw`pr-12`"#####, r#####"({
  paddingRight: "3rem",
})
;"##### ; "158")]
#[test_case(r#####"tw`pr-14`"#####, r#####"({
  paddingRight: "3.5rem",
})
;"##### ; "159")]
#[test_case(r#####"tw`pr-16`"#####, r#####"({
  paddingRight: "4rem",
})
;"##### ; "160")]
#[test_case(r#####"tw`pr-20`"#####, r#####"({
  paddingRight: "5rem",
})
;"##### ; "161")]
#[test_case(r#####"tw`pr-24`"#####, r#####"({
  paddingRight: "6rem",
})
;"##### ; "162")]
#[test_case(r#####"tw`pr-28`"#####, r#####"({
  paddingRight: "7rem",
})
;"##### ; "163")]
#[test_case(r#####"tw`pr-32`"#####, r#####"({
  paddingRight: "8rem",
})
;"##### ; "164")]
#[test_case(r#####"tw`pr-36`"#####, r#####"({
  paddingRight: "9rem",
})
;"##### ; "165")]
#[test_case(r#####"tw`pr-40`"#####, r#####"({
  paddingRight: "10rem",
})
;"##### ; "166")]
#[test_case(r#####"tw`pr-44`"#####, r#####"({
  paddingRight: "11rem",
})
;"##### ; "167")]
#[test_case(r#####"tw`pr-48`"#####, r#####"({
  paddingRight: "12rem",
})
;"##### ; "168")]
#[test_case(r#####"tw`pr-52`"#####, r#####"({
  paddingRight: "13rem",
})
;"##### ; "169")]
#[test_case(r#####"tw`pr-56`"#####, r#####"({
  paddingRight: "14rem",
})
;"##### ; "170")]
#[test_case(r#####"tw`pr-60`"#####, r#####"({
  paddingRight: "15rem",
})
;"##### ; "171")]
#[test_case(r#####"tw`pr-64`"#####, r#####"({
  paddingRight: "16rem",
})
;"##### ; "172")]
#[test_case(r#####"tw`pr-72`"#####, r#####"({
  paddingRight: "18rem",
})
;"##### ; "173")]
#[test_case(r#####"tw`pr-80`"#####, r#####"({
  paddingRight: "20rem",
})
;"##### ; "174")]
#[test_case(r#####"tw`pr-96`"#####, r#####"({
  paddingRight: "24rem",
})
;"##### ; "175")]
#[test_case(r#####"tw`pr-px`"#####, r#####"({
  paddingRight: "1px",
})
;"##### ; "176")]
#[test_case(r#####"tw`pb-0`"#####, r#####"({
  paddingBottom: "0px",
})
;"##### ; "177")]
#[test_case(r#####"tw`pb-px`"#####, r#####"({
  paddingBottom: "1px",
})
;"##### ; "178")]
#[test_case(r#####"tw`pb-0.5`"#####, r#####"({
  paddingBottom: "0.125rem",
})
;"##### ; "179")]
#[test_case(r#####"tw`pb-1`"#####, r#####"({
  paddingBottom: "0.25rem",
})
;"##### ; "180")]
#[test_case(r#####"tw`pb-1.5`"#####, r#####"({
  paddingBottom: "0.375rem",
})
;"##### ; "181")]
#[test_case(r#####"tw`pb-2`"#####, r#####"({
  paddingBottom: "0.5rem",
})
;"##### ; "182")]
#[test_case(r#####"tw`pb-2.5`"#####, r#####"({
  paddingBottom: "0.625rem",
})
;"##### ; "183")]
#[test_case(r#####"tw`pb-3`"#####, r#####"({
  paddingBottom: "0.75rem",
})
;"##### ; "184")]
#[test_case(r#####"tw`pb-3.5`"#####, r#####"({
  paddingBottom: "0.875rem",
})
;"##### ; "185")]
#[test_case(r#####"tw`pb-4`"#####, r#####"({
  paddingBottom: "1rem",
})
;"##### ; "186")]
#[test_case(r#####"tw`pb-5`"#####, r#####"({
  paddingBottom: "1.25rem",
})
;"##### ; "187")]
#[test_case(r#####"tw`pb-6`"#####, r#####"({
  paddingBottom: "1.5rem",
})
;"##### ; "188")]
#[test_case(r#####"tw`pb-7`"#####, r#####"({
  paddingBottom: "1.75rem",
})
;"##### ; "189")]
#[test_case(r#####"tw`pb-8`"#####, r#####"({
  paddingBottom: "2rem",
})
;"##### ; "190")]
#[test_case(r#####"tw`pb-9`"#####, r#####"({
  paddingBottom: "2.25rem",
})
;"##### ; "191")]
#[test_case(r#####"tw`pb-10`"#####, r#####"({
  paddingBottom: "2.5rem",
})
;"##### ; "192")]
#[test_case(r#####"tw`pb-11`"#####, r#####"({
  paddingBottom: "2.75rem",
})
;"##### ; "193")]
#[test_case(r#####"tw`pb-12`"#####, r#####"({
  paddingBottom: "3rem",
})
;"##### ; "194")]
#[test_case(r#####"tw`pb-14`"#####, r#####"({
  paddingBottom: "3.5rem",
})
;"##### ; "195")]
#[test_case(r#####"tw`pb-16`"#####, r#####"({
  paddingBottom: "4rem",
})
;"##### ; "196")]
#[test_case(r#####"tw`pb-20`"#####, r#####"({
  paddingBottom: "5rem",
})
;"##### ; "197")]
#[test_case(r#####"tw`pb-24`"#####, r#####"({
  paddingBottom: "6rem",
})
;"##### ; "198")]
#[test_case(r#####"tw`pb-28`"#####, r#####"({
  paddingBottom: "7rem",
})
;"##### ; "199")]
#[test_case(r#####"tw`pb-32`"#####, r#####"({
  paddingBottom: "8rem",
})
;"##### ; "200")]
#[test_case(r#####"tw`pb-36`"#####, r#####"({
  paddingBottom: "9rem",
})
;"##### ; "201")]
#[test_case(r#####"tw`pb-40`"#####, r#####"({
  paddingBottom: "10rem",
})
;"##### ; "202")]
#[test_case(r#####"tw`pb-44`"#####, r#####"({
  paddingBottom: "11rem",
})
;"##### ; "203")]
#[test_case(r#####"tw`pb-48`"#####, r#####"({
  paddingBottom: "12rem",
})
;"##### ; "204")]
#[test_case(r#####"tw`pb-52`"#####, r#####"({
  paddingBottom: "13rem",
})
;"##### ; "205")]
#[test_case(r#####"tw`pb-56`"#####, r#####"({
  paddingBottom: "14rem",
})
;"##### ; "206")]
#[test_case(r#####"tw`pb-60`"#####, r#####"({
  paddingBottom: "15rem",
})
;"##### ; "207")]
#[test_case(r#####"tw`pb-64`"#####, r#####"({
  paddingBottom: "16rem",
})
;"##### ; "208")]
#[test_case(r#####"tw`pb-72`"#####, r#####"({
  paddingBottom: "18rem",
})
;"##### ; "209")]
#[test_case(r#####"tw`pl-0`"#####, r#####"({
  paddingLeft: "0px",
})
;"##### ; "210")]
#[test_case(r#####"tw`pl-px`"#####, r#####"({
  paddingLeft: "1px",
})
;"##### ; "211")]
#[test_case(r#####"tw`pl-0.5`"#####, r#####"({
  paddingLeft: "0.125rem",
})
;"##### ; "212")]
#[test_case(r#####"tw`pl-1`"#####, r#####"({
  paddingLeft: "0.25rem",
})
;"##### ; "213")]
#[test_case(r#####"tw`pl-1.5`"#####, r#####"({
  paddingLeft: "0.375rem",
})
;"##### ; "214")]
#[test_case(r#####"tw`pl-2`"#####, r#####"({
  paddingLeft: "0.5rem",
})
;"##### ; "215")]
#[test_case(r#####"tw`pl-2.5`"#####, r#####"({
  paddingLeft: "0.625rem",
})
;"##### ; "216")]
#[test_case(r#####"tw`pl-3`"#####, r#####"({
  paddingLeft: "0.75rem",
})
;"##### ; "217")]
#[test_case(r#####"tw`pl-3.5`"#####, r#####"({
  paddingLeft: "0.875rem",
})
;"##### ; "218")]
#[test_case(r#####"tw`pl-4`"#####, r#####"({
  paddingLeft: "1rem",
})
;"##### ; "219")]
#[test_case(r#####"tw`pl-5`"#####, r#####"({
  paddingLeft: "1.25rem",
})
;"##### ; "220")]
#[test_case(r#####"tw`pl-6`"#####, r#####"({
  paddingLeft: "1.5rem",
})
;"##### ; "221")]
#[test_case(r#####"tw`pl-7`"#####, r#####"({
  paddingLeft: "1.75rem",
})
;"##### ; "222")]
#[test_case(r#####"tw`pl-8`"#####, r#####"({
  paddingLeft: "2rem",
})
;"##### ; "223")]
#[test_case(r#####"tw`pl-9`"#####, r#####"({
  paddingLeft: "2.25rem",
})
;"##### ; "224")]
#[test_case(r#####"tw`pl-10`"#####, r#####"({
  paddingLeft: "2.5rem",
})
;"##### ; "225")]
#[test_case(r#####"tw`pl-11`"#####, r#####"({
  paddingLeft: "2.75rem",
})
;"##### ; "226")]
#[test_case(r#####"tw`pl-12`"#####, r#####"({
  paddingLeft: "3rem",
})
;"##### ; "227")]
#[test_case(r#####"tw`pl-14`"#####, r#####"({
  paddingLeft: "3.5rem",
})
;"##### ; "228")]
#[test_case(r#####"tw`pl-16`"#####, r#####"({
  paddingLeft: "4rem",
})
;"##### ; "229")]
#[test_case(r#####"tw`pl-20`"#####, r#####"({
  paddingLeft: "5rem",
})
;"##### ; "230")]
#[test_case(r#####"tw`pl-24`"#####, r#####"({
  paddingLeft: "6rem",
})
;"##### ; "231")]
#[test_case(r#####"tw`pl-28`"#####, r#####"({
  paddingLeft: "7rem",
})
;"##### ; "232")]
#[test_case(r#####"tw`pl-32`"#####, r#####"({
  paddingLeft: "8rem",
})
;"##### ; "233")]
#[test_case(r#####"tw`pl-36`"#####, r#####"({
  paddingLeft: "9rem",
})
;"##### ; "234")]
#[test_case(r#####"tw`pl-40`"#####, r#####"({
  paddingLeft: "10rem",
})
;"##### ; "235")]
#[test_case(r#####"tw`pl-44`"#####, r#####"({
  paddingLeft: "11rem",
})
;"##### ; "236")]
#[test_case(r#####"tw`pl-48`"#####, r#####"({
  paddingLeft: "12rem",
})
;"##### ; "237")]
#[test_case(r#####"tw`pl-52`"#####, r#####"({
  paddingLeft: "13rem",
})
;"##### ; "238")]
#[test_case(r#####"tw`pl-56`"#####, r#####"({
  paddingLeft: "14rem",
})
;"##### ; "239")]
#[test_case(r#####"tw`pl-60`"#####, r#####"({
  paddingLeft: "15rem",
})
;"##### ; "240")]
#[test_case(r#####"tw`pl-64`"#####, r#####"({
  paddingLeft: "16rem",
})
;"##### ; "241")]
#[test_case(r#####"tw`pl-72`"#####, r#####"({
  paddingLeft: "18rem",
})
;"##### ; "242")]
#[test_case(r#####"tw`pl-80`"#####, r#####"({
  paddingLeft: "20rem",
})
;"##### ; "243")]
#[test_case(r#####"tw`pl-96`"#####, r#####"({
  paddingLeft: "24rem",
})
;"##### ; "244")]
#[test_case(r#####"tw`p-[5px]`"#####, r#####"({
  padding: "5px",
})
;"##### ; "245")]
#[test_case(r#####"tw`pt-[5px]`"#####, r#####"({
  paddingTop: "5px",
})
;"##### ; "246")]
#[test_case(r#####"tw`pl-[5px]`"#####, r#####"({
  paddingLeft: "5px",
})
;"##### ; "247")]
#[test_case(r#####"tw`pr-[5px]`"#####, r#####"({
  paddingRight: "5px",
})
;"##### ; "248")]
#[test_case(r#####"tw`pb-[5px]`"#####, r#####"({
  paddingBottom: "5px",
})
;"##### ; "249")]
#[test_case(r#####"tw`p-[var(--app-padding)]`"#####, r#####"({
  padding: "var(--app-padding)",
})
;"##### ; "250")]
#[test_case(r#####"tw`pt-6 px-1 ps-4 pe-8`"#####, r#####"({
  paddingLeft: "0.25rem",
  paddingRight: "0.25rem",
  paddingInlineEnd: "2rem",
  paddingInlineStart: "1rem",
  paddingTop: "1.5rem",
})"##### ; "251")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
