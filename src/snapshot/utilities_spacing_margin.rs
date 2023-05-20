use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`margin`"#####, r#####"({
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
})
;"##### ; "1")]
#[test_case(r#####"tw`m-0`"#####, r#####"({
  margin: "0px",
})
;"##### ; "2")]
#[test_case(r#####"tw`m-px`"#####, r#####"({
  margin: "1px",
})
;"##### ; "3")]
#[test_case(r#####"tw`m-0.5`"#####, r#####"({
  margin: "0.125rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`m-1`"#####, r#####"({
  margin: "0.25rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`m-1.5`"#####, r#####"({
  margin: "0.375rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`m-2`"#####, r#####"({
  margin: "0.5rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`m-2.5`"#####, r#####"({
  margin: "0.625rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`m-3`"#####, r#####"({
  margin: "0.75rem",
})
;"##### ; "9")]
#[test_case(r#####"tw`m-3.5`"#####, r#####"({
  margin: "0.875rem",
})
;"##### ; "10")]
#[test_case(r#####"tw`m-4`"#####, r#####"({
  margin: "1rem",
})
;"##### ; "11")]
#[test_case(r#####"tw`m-5`"#####, r#####"({
  margin: "1.25rem",
})
;"##### ; "12")]
#[test_case(r#####"tw`m-6`"#####, r#####"({
  margin: "1.5rem",
})
;"##### ; "13")]
#[test_case(r#####"tw`m-7`"#####, r#####"({
  margin: "1.75rem",
})
;"##### ; "14")]
#[test_case(r#####"tw`m-8`"#####, r#####"({
  margin: "2rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`m-9`"#####, r#####"({
  margin: "2.25rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`m-10`"#####, r#####"({
  margin: "2.5rem",
})
;"##### ; "17")]
#[test_case(r#####"tw`m-11`"#####, r#####"({
  margin: "2.75rem",
})
;"##### ; "18")]
#[test_case(r#####"tw`m-12`"#####, r#####"({
  margin: "3rem",
})
;"##### ; "19")]
#[test_case(r#####"tw`m-14`"#####, r#####"({
  margin: "3.5rem",
})
;"##### ; "20")]
#[test_case(r#####"tw`m-16`"#####, r#####"({
  margin: "4rem",
})
;"##### ; "21")]
#[test_case(r#####"tw`m-20`"#####, r#####"({
  margin: "5rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`m-24`"#####, r#####"({
  margin: "6rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`m-28`"#####, r#####"({
  margin: "7rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`m-32`"#####, r#####"({
  margin: "8rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`m-36`"#####, r#####"({
  margin: "9rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`m-40`"#####, r#####"({
  margin: "10rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`m-44`"#####, r#####"({
  margin: "11rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`m-48`"#####, r#####"({
  margin: "12rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`m-52`"#####, r#####"({
  margin: "13rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`m-56`"#####, r#####"({
  margin: "14rem",
})
;"##### ; "31")]
#[test_case(r#####"tw`m-60`"#####, r#####"({
  margin: "15rem",
})
;"##### ; "32")]
#[test_case(r#####"tw`m-64`"#####, r#####"({
  margin: "16rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`m-72`"#####, r#####"({
  margin: "18rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`m-80`"#####, r#####"({
  margin: "20rem",
})
;"##### ; "35")]
#[test_case(r#####"tw`m-96`"#####, r#####"({
  margin: "24rem",
})
;"##### ; "36")]
#[test_case(r#####"tw`m-auto`"#####, r#####"({
  margin: "auto",
})
;"##### ; "37")]
#[test_case(r#####"tw`-m-0`"#####, r#####"({
  margin: "-0px",
})
;"##### ; "38")]
#[test_case(r#####"tw`-m-px`"#####, r#####"({
  margin: "-1px",
})
;"##### ; "39")]
#[test_case(r#####"tw`-m-0.5`"#####, r#####"({
  margin: "-0.125rem",
})
;"##### ; "40")]
#[test_case(r#####"tw`-m-1`"#####, r#####"({
  margin: "-0.25rem",
})
;"##### ; "41")]
#[test_case(r#####"tw`-m-1.5`"#####, r#####"({
  margin: "-0.375rem",
})
;"##### ; "42")]
#[test_case(r#####"tw`-m-2`"#####, r#####"({
  margin: "-0.5rem",
})
;"##### ; "43")]
#[test_case(r#####"tw`-m-2.5`"#####, r#####"({
  margin: "-0.625rem",
})
;"##### ; "44")]
#[test_case(r#####"tw`-m-3`"#####, r#####"({
  margin: "-0.75rem",
})
;"##### ; "45")]
#[test_case(r#####"tw`-m-3.5`"#####, r#####"({
  margin: "-0.875rem",
})
;"##### ; "46")]
#[test_case(r#####"tw`-m-4`"#####, r#####"({
  margin: "-1rem",
})
;"##### ; "47")]
#[test_case(r#####"tw`-m-5`"#####, r#####"({
  margin: "-1.25rem",
})
;"##### ; "48")]
#[test_case(r#####"tw`-m-6`"#####, r#####"({
  margin: "-1.5rem",
})
;"##### ; "49")]
#[test_case(r#####"tw`-m-7`"#####, r#####"({
  margin: "-1.75rem",
})
;"##### ; "50")]
#[test_case(r#####"tw`-m-8`"#####, r#####"({
  margin: "-2rem",
})
;"##### ; "51")]
#[test_case(r#####"tw`-m-9`"#####, r#####"({
  margin: "-2.25rem",
})
;"##### ; "52")]
#[test_case(r#####"tw`-m-10`"#####, r#####"({
  margin: "-2.5rem",
})
;"##### ; "53")]
#[test_case(r#####"tw`-m-11`"#####, r#####"({
  margin: "-2.75rem",
})
;"##### ; "54")]
#[test_case(r#####"tw`-m-12`"#####, r#####"({
  margin: "-3rem",
})
;"##### ; "55")]
#[test_case(r#####"tw`-m-14`"#####, r#####"({
  margin: "-3.5rem",
})
;"##### ; "56")]
#[test_case(r#####"tw`-m-16`"#####, r#####"({
  margin: "-4rem",
})
;"##### ; "57")]
#[test_case(r#####"tw`-m-20`"#####, r#####"({
  margin: "-5rem",
})
;"##### ; "58")]
#[test_case(r#####"tw`-m-24`"#####, r#####"({
  margin: "-6rem",
})
;"##### ; "59")]
#[test_case(r#####"tw`-m-28`"#####, r#####"({
  margin: "-7rem",
})
;"##### ; "60")]
#[test_case(r#####"tw`-m-32`"#####, r#####"({
  margin: "-8rem",
})
;"##### ; "61")]
#[test_case(r#####"tw`-m-36`"#####, r#####"({
  margin: "-9rem",
})
;"##### ; "62")]
#[test_case(r#####"tw`-m-40`"#####, r#####"({
  margin: "-10rem",
})
;"##### ; "63")]
#[test_case(r#####"tw`-m-44`"#####, r#####"({
  margin: "-11rem",
})
;"##### ; "64")]
#[test_case(r#####"tw`-m-48`"#####, r#####"({
  margin: "-12rem",
})
;"##### ; "65")]
#[test_case(r#####"tw`-m-52`"#####, r#####"({
  margin: "-13rem",
})
;"##### ; "66")]
#[test_case(r#####"tw`-m-56`"#####, r#####"({
  margin: "-14rem",
})
;"##### ; "67")]
#[test_case(r#####"tw`-m-60`"#####, r#####"({
  margin: "-15rem",
})
;"##### ; "68")]
#[test_case(r#####"tw`-m-64`"#####, r#####"({
  margin: "-16rem",
})
;"##### ; "69")]
#[test_case(r#####"tw`-m-72`"#####, r#####"({
  margin: "-18rem",
})
;"##### ; "70")]
#[test_case(r#####"tw`-m-80`"#####, r#####"({
  margin: "-20rem",
})
;"##### ; "71")]
#[test_case(r#####"tw`-m-96`"#####, r#####"({
  margin: "-24rem",
})
;"##### ; "72")]
#[test_case(r#####"tw`my-0`"#####, r#####"({
  marginTop: "0px",
  marginBottom: "0px",
})
;"##### ; "73")]
#[test_case(r#####"tw`my-px`"#####, r#####"({
  marginTop: "1px",
  marginBottom: "1px",
})
;"##### ; "74")]
#[test_case(r#####"tw`my-0.5`"#####, r#####"({
  marginTop: "0.125rem",
  marginBottom: "0.125rem",
})
;"##### ; "75")]
#[test_case(r#####"tw`my-1`"#####, r#####"({
  marginTop: "0.25rem",
  marginBottom: "0.25rem",
})
;"##### ; "76")]
#[test_case(r#####"tw`my-1.5`"#####, r#####"({
  marginTop: "0.375rem",
  marginBottom: "0.375rem",
})
;"##### ; "77")]
#[test_case(r#####"tw`my-2`"#####, r#####"({
  marginTop: "0.5rem",
  marginBottom: "0.5rem",
})
;"##### ; "78")]
#[test_case(r#####"tw`my-2.5`"#####, r#####"({
  marginTop: "0.625rem",
  marginBottom: "0.625rem",
})
;"##### ; "79")]
#[test_case(r#####"tw`my-3`"#####, r#####"({
  marginTop: "0.75rem",
  marginBottom: "0.75rem",
})
;"##### ; "80")]
#[test_case(r#####"tw`my-3.5`"#####, r#####"({
  marginTop: "0.875rem",
  marginBottom: "0.875rem",
})
;"##### ; "81")]
#[test_case(r#####"tw`my-4`"#####, r#####"({
  marginTop: "1rem",
  marginBottom: "1rem",
})
;"##### ; "82")]
#[test_case(r#####"tw`my-5`"#####, r#####"({
  marginTop: "1.25rem",
  marginBottom: "1.25rem",
})
;"##### ; "83")]
#[test_case(r#####"tw`my-6`"#####, r#####"({
  marginTop: "1.5rem",
  marginBottom: "1.5rem",
})
;"##### ; "84")]
#[test_case(r#####"tw`my-7`"#####, r#####"({
  marginTop: "1.75rem",
  marginBottom: "1.75rem",
})
;"##### ; "85")]
#[test_case(r#####"tw`my-8`"#####, r#####"({
  marginTop: "2rem",
  marginBottom: "2rem",
})
;"##### ; "86")]
#[test_case(r#####"tw`my-9`"#####, r#####"({
  marginTop: "2.25rem",
  marginBottom: "2.25rem",
})
;"##### ; "87")]
#[test_case(r#####"tw`my-10`"#####, r#####"({
  marginTop: "2.5rem",
  marginBottom: "2.5rem",
})
;"##### ; "88")]
#[test_case(r#####"tw`my-11`"#####, r#####"({
  marginTop: "2.75rem",
  marginBottom: "2.75rem",
})
;"##### ; "89")]
#[test_case(r#####"tw`my-12`"#####, r#####"({
  marginTop: "3rem",
  marginBottom: "3rem",
})
;"##### ; "90")]
#[test_case(r#####"tw`my-14`"#####, r#####"({
  marginTop: "3.5rem",
  marginBottom: "3.5rem",
})
;"##### ; "91")]
#[test_case(r#####"tw`my-16`"#####, r#####"({
  marginTop: "4rem",
  marginBottom: "4rem",
})
;"##### ; "92")]
#[test_case(r#####"tw`my-20`"#####, r#####"({
  marginTop: "5rem",
  marginBottom: "5rem",
})
;"##### ; "93")]
#[test_case(r#####"tw`my-24`"#####, r#####"({
  marginTop: "6rem",
  marginBottom: "6rem",
})
;"##### ; "94")]
#[test_case(r#####"tw`my-28`"#####, r#####"({
  marginTop: "7rem",
  marginBottom: "7rem",
})
;"##### ; "95")]
#[test_case(r#####"tw`my-32`"#####, r#####"({
  marginTop: "8rem",
  marginBottom: "8rem",
})
;"##### ; "96")]
#[test_case(r#####"tw`my-36`"#####, r#####"({
  marginTop: "9rem",
  marginBottom: "9rem",
})
;"##### ; "97")]
#[test_case(r#####"tw`my-40`"#####, r#####"({
  marginTop: "10rem",
  marginBottom: "10rem",
})
;"##### ; "98")]
#[test_case(r#####"tw`my-44`"#####, r#####"({
  marginTop: "11rem",
  marginBottom: "11rem",
})
;"##### ; "99")]
#[test_case(r#####"tw`my-48`"#####, r#####"({
  marginTop: "12rem",
  marginBottom: "12rem",
})
;"##### ; "100")]
#[test_case(r#####"tw`my-52`"#####, r#####"({
  marginTop: "13rem",
  marginBottom: "13rem",
})
;"##### ; "101")]
#[test_case(r#####"tw`my-56`"#####, r#####"({
  marginTop: "14rem",
  marginBottom: "14rem",
})
;"##### ; "102")]
#[test_case(r#####"tw`my-60`"#####, r#####"({
  marginTop: "15rem",
  marginBottom: "15rem",
})
;"##### ; "103")]
#[test_case(r#####"tw`my-64`"#####, r#####"({
  marginTop: "16rem",
  marginBottom: "16rem",
})
;"##### ; "104")]
#[test_case(r#####"tw`my-72`"#####, r#####"({
  marginTop: "18rem",
  marginBottom: "18rem",
})
;"##### ; "105")]
#[test_case(r#####"tw`my-80`"#####, r#####"({
  marginTop: "20rem",
  marginBottom: "20rem",
})
;"##### ; "106")]
#[test_case(r#####"tw`my-96`"#####, r#####"({
  marginTop: "24rem",
  marginBottom: "24rem",
})
;"##### ; "107")]
#[test_case(r#####"tw`my-auto`"#####, r#####"({
  marginTop: "auto",
  marginBottom: "auto",
})
;"##### ; "108")]
#[test_case(r#####"tw`-my-0`"#####, r#####"({
  marginTop: "-0px",
  marginBottom: "-0px",
})
;"##### ; "109")]
#[test_case(r#####"tw`-my-px`"#####, r#####"({
  marginTop: "-1px",
  marginBottom: "-1px",
})
;"##### ; "110")]
#[test_case(r#####"tw`-my-0.5`"#####, r#####"({
  marginTop: "-0.125rem",
  marginBottom: "-0.125rem",
})
;"##### ; "111")]
#[test_case(r#####"tw`-my-1`"#####, r#####"({
  marginTop: "-0.25rem",
  marginBottom: "-0.25rem",
})
;"##### ; "112")]
#[test_case(r#####"tw`-my-1.5`"#####, r#####"({
  marginTop: "-0.375rem",
  marginBottom: "-0.375rem",
})
;"##### ; "113")]
#[test_case(r#####"tw`-my-2`"#####, r#####"({
  marginTop: "-0.5rem",
  marginBottom: "-0.5rem",
})
;"##### ; "114")]
#[test_case(r#####"tw`-my-2.5`"#####, r#####"({
  marginTop: "-0.625rem",
  marginBottom: "-0.625rem",
})
;"##### ; "115")]
#[test_case(r#####"tw`-my-3`"#####, r#####"({
  marginTop: "-0.75rem",
  marginBottom: "-0.75rem",
})
;"##### ; "116")]
#[test_case(r#####"tw`-my-3.5`"#####, r#####"({
  marginTop: "-0.875rem",
  marginBottom: "-0.875rem",
})
;"##### ; "117")]
#[test_case(r#####"tw`-my-4`"#####, r#####"({
  marginTop: "-1rem",
  marginBottom: "-1rem",
})
;"##### ; "118")]
#[test_case(r#####"tw`-my-5`"#####, r#####"({
  marginTop: "-1.25rem",
  marginBottom: "-1.25rem",
})
;"##### ; "119")]
#[test_case(r#####"tw`-my-6`"#####, r#####"({
  marginTop: "-1.5rem",
  marginBottom: "-1.5rem",
})
;"##### ; "120")]
#[test_case(r#####"tw`-my-7`"#####, r#####"({
  marginTop: "-1.75rem",
  marginBottom: "-1.75rem",
})
;"##### ; "121")]
#[test_case(r#####"tw`-my-8`"#####, r#####"({
  marginTop: "-2rem",
  marginBottom: "-2rem",
})
;"##### ; "122")]
#[test_case(r#####"tw`-my-9`"#####, r#####"({
  marginTop: "-2.25rem",
  marginBottom: "-2.25rem",
})
;"##### ; "123")]
#[test_case(r#####"tw`-my-10`"#####, r#####"({
  marginTop: "-2.5rem",
  marginBottom: "-2.5rem",
})
;"##### ; "124")]
#[test_case(r#####"tw`-my-11`"#####, r#####"({
  marginTop: "-2.75rem",
  marginBottom: "-2.75rem",
})
;"##### ; "125")]
#[test_case(r#####"tw`-my-12`"#####, r#####"({
  marginTop: "-3rem",
  marginBottom: "-3rem",
})
;"##### ; "126")]
#[test_case(r#####"tw`-my-14`"#####, r#####"({
  marginTop: "-3.5rem",
  marginBottom: "-3.5rem",
})
;"##### ; "127")]
#[test_case(r#####"tw`-my-16`"#####, r#####"({
  marginTop: "-4rem",
  marginBottom: "-4rem",
})
;"##### ; "128")]
#[test_case(r#####"tw`-my-20`"#####, r#####"({
  marginTop: "-5rem",
  marginBottom: "-5rem",
})
;"##### ; "129")]
#[test_case(r#####"tw`-my-24`"#####, r#####"({
  marginTop: "-6rem",
  marginBottom: "-6rem",
})
;"##### ; "130")]
#[test_case(r#####"tw`-my-28`"#####, r#####"({
  marginTop: "-7rem",
  marginBottom: "-7rem",
})
;"##### ; "131")]
#[test_case(r#####"tw`-my-32`"#####, r#####"({
  marginTop: "-8rem",
  marginBottom: "-8rem",
})
;"##### ; "132")]
#[test_case(r#####"tw`-my-36`"#####, r#####"({
  marginTop: "-9rem",
  marginBottom: "-9rem",
})
;"##### ; "133")]
#[test_case(r#####"tw`-my-40`"#####, r#####"({
  marginTop: "-10rem",
  marginBottom: "-10rem",
})
;"##### ; "134")]
#[test_case(r#####"tw`-my-44`"#####, r#####"({
  marginTop: "-11rem",
  marginBottom: "-11rem",
})
;"##### ; "135")]
#[test_case(r#####"tw`-my-48`"#####, r#####"({
  marginTop: "-12rem",
  marginBottom: "-12rem",
})
;"##### ; "136")]
#[test_case(r#####"tw`-my-52`"#####, r#####"({
  marginTop: "-13rem",
  marginBottom: "-13rem",
})
;"##### ; "137")]
#[test_case(r#####"tw`-my-56`"#####, r#####"({
  marginTop: "-14rem",
  marginBottom: "-14rem",
})
;"##### ; "138")]
#[test_case(r#####"tw`-my-60`"#####, r#####"({
  marginTop: "-15rem",
  marginBottom: "-15rem",
})
;"##### ; "139")]
#[test_case(r#####"tw`-my-64`"#####, r#####"({
  marginTop: "-16rem",
  marginBottom: "-16rem",
})
;"##### ; "140")]
#[test_case(r#####"tw`-my-72`"#####, r#####"({
  marginTop: "-18rem",
  marginBottom: "-18rem",
})
;"##### ; "141")]
#[test_case(r#####"tw`-my-80`"#####, r#####"({
  marginTop: "-20rem",
  marginBottom: "-20rem",
})
;"##### ; "142")]
#[test_case(r#####"tw`-my-96`"#####, r#####"({
  marginTop: "-24rem",
  marginBottom: "-24rem",
})
;"##### ; "143")]
#[test_case(r#####"tw`mx-0`"#####, r#####"({
  marginLeft: "0px",
  marginRight: "0px",
})
;"##### ; "144")]
#[test_case(r#####"tw`mx-px`"#####, r#####"({
  marginLeft: "1px",
  marginRight: "1px",
})
;"##### ; "145")]
#[test_case(r#####"tw`mx-0.5`"#####, r#####"({
  marginLeft: "0.125rem",
  marginRight: "0.125rem",
})
;"##### ; "146")]
#[test_case(r#####"tw`mx-1`"#####, r#####"({
  marginLeft: "0.25rem",
  marginRight: "0.25rem",
})
;"##### ; "147")]
#[test_case(r#####"tw`mx-1.5`"#####, r#####"({
  marginLeft: "0.375rem",
  marginRight: "0.375rem",
})
;"##### ; "148")]
#[test_case(r#####"tw`mx-2`"#####, r#####"({
  marginLeft: "0.5rem",
  marginRight: "0.5rem",
})
;"##### ; "149")]
#[test_case(r#####"tw`mx-2.5`"#####, r#####"({
  marginLeft: "0.625rem",
  marginRight: "0.625rem",
})
;"##### ; "150")]
#[test_case(r#####"tw`mx-3`"#####, r#####"({
  marginLeft: "0.75rem",
  marginRight: "0.75rem",
})
;"##### ; "151")]
#[test_case(r#####"tw`mx-3.5`"#####, r#####"({
  marginLeft: "0.875rem",
  marginRight: "0.875rem",
})
;"##### ; "152")]
#[test_case(r#####"tw`mx-4`"#####, r#####"({
  marginLeft: "1rem",
  marginRight: "1rem",
})
;"##### ; "153")]
#[test_case(r#####"tw`mx-5`"#####, r#####"({
  marginLeft: "1.25rem",
  marginRight: "1.25rem",
})
;"##### ; "154")]
#[test_case(r#####"tw`mx-6`"#####, r#####"({
  marginLeft: "1.5rem",
  marginRight: "1.5rem",
})
;"##### ; "155")]
#[test_case(r#####"tw`mx-7`"#####, r#####"({
  marginLeft: "1.75rem",
  marginRight: "1.75rem",
})
;"##### ; "156")]
#[test_case(r#####"tw`mx-8`"#####, r#####"({
  marginLeft: "2rem",
  marginRight: "2rem",
})
;"##### ; "157")]
#[test_case(r#####"tw`mx-9`"#####, r#####"({
  marginLeft: "2.25rem",
  marginRight: "2.25rem",
})
;"##### ; "158")]
#[test_case(r#####"tw`mx-10`"#####, r#####"({
  marginLeft: "2.5rem",
  marginRight: "2.5rem",
})
;"##### ; "159")]
#[test_case(r#####"tw`mx-12`"#####, r#####"({
  marginLeft: "3rem",
  marginRight: "3rem",
})
;"##### ; "160")]
#[test_case(r#####"tw`mx-14`"#####, r#####"({
  marginLeft: "3.5rem",
  marginRight: "3.5rem",
})
;"##### ; "161")]
#[test_case(r#####"tw`mx-16`"#####, r#####"({
  marginLeft: "4rem",
  marginRight: "4rem",
})
;"##### ; "162")]
#[test_case(r#####"tw`mx-20`"#####, r#####"({
  marginLeft: "5rem",
  marginRight: "5rem",
})
;"##### ; "163")]
#[test_case(r#####"tw`mx-24`"#####, r#####"({
  marginLeft: "6rem",
  marginRight: "6rem",
})
;"##### ; "164")]
#[test_case(r#####"tw`mx-28`"#####, r#####"({
  marginLeft: "7rem",
  marginRight: "7rem",
})
;"##### ; "165")]
#[test_case(r#####"tw`mx-32`"#####, r#####"({
  marginLeft: "8rem",
  marginRight: "8rem",
})
;"##### ; "166")]
#[test_case(r#####"tw`mx-36`"#####, r#####"({
  marginLeft: "9rem",
  marginRight: "9rem",
})
;"##### ; "167")]
#[test_case(r#####"tw`mx-40`"#####, r#####"({
  marginLeft: "10rem",
  marginRight: "10rem",
})
;"##### ; "168")]
#[test_case(r#####"tw`mx-44`"#####, r#####"({
  marginLeft: "11rem",
  marginRight: "11rem",
})
;"##### ; "169")]
#[test_case(r#####"tw`mx-48`"#####, r#####"({
  marginLeft: "12rem",
  marginRight: "12rem",
})
;"##### ; "170")]
#[test_case(r#####"tw`mx-52`"#####, r#####"({
  marginLeft: "13rem",
  marginRight: "13rem",
})
;"##### ; "171")]
#[test_case(r#####"tw`mx-56`"#####, r#####"({
  marginLeft: "14rem",
  marginRight: "14rem",
})
;"##### ; "172")]
#[test_case(r#####"tw`mx-60`"#####, r#####"({
  marginLeft: "15rem",
  marginRight: "15rem",
})
;"##### ; "173")]
#[test_case(r#####"tw`mx-64`"#####, r#####"({
  marginLeft: "16rem",
  marginRight: "16rem",
})
;"##### ; "174")]
#[test_case(r#####"tw`mx-72`"#####, r#####"({
  marginLeft: "18rem",
  marginRight: "18rem",
})
;"##### ; "175")]
#[test_case(r#####"tw`mx-80`"#####, r#####"({
  marginLeft: "20rem",
  marginRight: "20rem",
})
;"##### ; "176")]
#[test_case(r#####"tw`mx-auto`"#####, r#####"({
  marginLeft: "auto",
  marginRight: "auto",
})
;"##### ; "177")]
#[test_case(r#####"tw`-mx-0`"#####, r#####"({
  marginLeft: "-0px",
  marginRight: "-0px",
})
;"##### ; "178")]
#[test_case(r#####"tw`-mx-px`"#####, r#####"({
  marginLeft: "-1px",
  marginRight: "-1px",
})
;"##### ; "179")]
#[test_case(r#####"tw`-mx-0.5`"#####, r#####"({
  marginLeft: "-0.125rem",
  marginRight: "-0.125rem",
})
;"##### ; "180")]
#[test_case(r#####"tw`-mx-1`"#####, r#####"({
  marginLeft: "-0.25rem",
  marginRight: "-0.25rem",
})
;"##### ; "181")]
#[test_case(r#####"tw`-mx-1.5`"#####, r#####"({
  marginLeft: "-0.375rem",
  marginRight: "-0.375rem",
})
;"##### ; "182")]
#[test_case(r#####"tw`-mx-2`"#####, r#####"({
  marginLeft: "-0.5rem",
  marginRight: "-0.5rem",
})
;"##### ; "183")]
#[test_case(r#####"tw`-mx-2.5`"#####, r#####"({
  marginLeft: "-0.625rem",
  marginRight: "-0.625rem",
})
;"##### ; "184")]
#[test_case(r#####"tw`-mx-3`"#####, r#####"({
  marginLeft: "-0.75rem",
  marginRight: "-0.75rem",
})
;"##### ; "185")]
#[test_case(r#####"tw`-mx-3.5`"#####, r#####"({
  marginLeft: "-0.875rem",
  marginRight: "-0.875rem",
})
;"##### ; "186")]
#[test_case(r#####"tw`-mx-4`"#####, r#####"({
  marginLeft: "-1rem",
  marginRight: "-1rem",
})
;"##### ; "187")]
#[test_case(r#####"tw`-mx-5`"#####, r#####"({
  marginLeft: "-1.25rem",
  marginRight: "-1.25rem",
})
;"##### ; "188")]
#[test_case(r#####"tw`-mx-6`"#####, r#####"({
  marginLeft: "-1.5rem",
  marginRight: "-1.5rem",
})
;"##### ; "189")]
#[test_case(r#####"tw`-mx-7`"#####, r#####"({
  marginLeft: "-1.75rem",
  marginRight: "-1.75rem",
})
;"##### ; "190")]
#[test_case(r#####"tw`-mx-8`"#####, r#####"({
  marginLeft: "-2rem",
  marginRight: "-2rem",
})
;"##### ; "191")]
#[test_case(r#####"tw`-mx-9`"#####, r#####"({
  marginLeft: "-2.25rem",
  marginRight: "-2.25rem",
})
;"##### ; "192")]
#[test_case(r#####"tw`-mx-10`"#####, r#####"({
  marginLeft: "-2.5rem",
  marginRight: "-2.5rem",
})
;"##### ; "193")]
#[test_case(r#####"tw`-mx-11`"#####, r#####"({
  marginLeft: "-2.75rem",
  marginRight: "-2.75rem",
})
;"##### ; "194")]
#[test_case(r#####"tw`-mx-12`"#####, r#####"({
  marginLeft: "-3rem",
  marginRight: "-3rem",
})
;"##### ; "195")]
#[test_case(r#####"tw`-mx-14`"#####, r#####"({
  marginLeft: "-3.5rem",
  marginRight: "-3.5rem",
})
;"##### ; "196")]
#[test_case(r#####"tw`-mx-16`"#####, r#####"({
  marginLeft: "-4rem",
  marginRight: "-4rem",
})
;"##### ; "197")]
#[test_case(r#####"tw`-mx-20`"#####, r#####"({
  marginLeft: "-5rem",
  marginRight: "-5rem",
})
;"##### ; "198")]
#[test_case(r#####"tw`-mx-24`"#####, r#####"({
  marginLeft: "-6rem",
  marginRight: "-6rem",
})
;"##### ; "199")]
#[test_case(r#####"tw`-mx-28`"#####, r#####"({
  marginLeft: "-7rem",
  marginRight: "-7rem",
})
;"##### ; "200")]
#[test_case(r#####"tw`-mx-32`"#####, r#####"({
  marginLeft: "-8rem",
  marginRight: "-8rem",
})
;"##### ; "201")]
#[test_case(r#####"tw`-mx-36`"#####, r#####"({
  marginLeft: "-9rem",
  marginRight: "-9rem",
})
;"##### ; "202")]
#[test_case(r#####"tw`-mx-40`"#####, r#####"({
  marginLeft: "-10rem",
  marginRight: "-10rem",
})
;"##### ; "203")]
#[test_case(r#####"tw`-mx-44`"#####, r#####"({
  marginLeft: "-11rem",
  marginRight: "-11rem",
})
;"##### ; "204")]
#[test_case(r#####"tw`-mx-48`"#####, r#####"({
  marginLeft: "-12rem",
  marginRight: "-12rem",
})
;"##### ; "205")]
#[test_case(r#####"tw`-mx-52`"#####, r#####"({
  marginLeft: "-13rem",
  marginRight: "-13rem",
})
;"##### ; "206")]
#[test_case(r#####"tw`-mx-56`"#####, r#####"({
  marginLeft: "-14rem",
  marginRight: "-14rem",
})
;"##### ; "207")]
#[test_case(r#####"tw`-mx-60`"#####, r#####"({
  marginLeft: "-15rem",
  marginRight: "-15rem",
})
;"##### ; "208")]
#[test_case(r#####"tw`-mx-64`"#####, r#####"({
  marginLeft: "-16rem",
  marginRight: "-16rem",
})
;"##### ; "209")]
#[test_case(r#####"tw`-mx-72`"#####, r#####"({
  marginLeft: "-18rem",
  marginRight: "-18rem",
})
;"##### ; "210")]
#[test_case(r#####"tw`-mx-80`"#####, r#####"({
  marginLeft: "-20rem",
  marginRight: "-20rem",
})
;"##### ; "211")]
#[test_case(r#####"tw`-mx-96`"#####, r#####"({
  marginLeft: "-24rem",
  marginRight: "-24rem",
})
;"##### ; "212")]
#[test_case(r#####"tw`mt-0`"#####, r#####"({
  marginTop: "0px",
})
;"##### ; "213")]
#[test_case(r#####"tw`mt-px`"#####, r#####"({
  marginTop: "1px",
})
;"##### ; "214")]
#[test_case(r#####"tw`mt-0.5`"#####, r#####"({
  marginTop: "0.125rem",
})
;"##### ; "215")]
#[test_case(r#####"tw`mt-1`"#####, r#####"({
  marginTop: "0.25rem",
})
;"##### ; "216")]
#[test_case(r#####"tw`mt-1.5`"#####, r#####"({
  marginTop: "0.375rem",
})
;"##### ; "217")]
#[test_case(r#####"tw`mt-2`"#####, r#####"({
  marginTop: "0.5rem",
})
;"##### ; "218")]
#[test_case(r#####"tw`mt-2.5`"#####, r#####"({
  marginTop: "0.625rem",
})
;"##### ; "219")]
#[test_case(r#####"tw`mt-3`"#####, r#####"({
  marginTop: "0.75rem",
})
;"##### ; "220")]
#[test_case(r#####"tw`mt-3.5`"#####, r#####"({
  marginTop: "0.875rem",
})
;"##### ; "221")]
#[test_case(r#####"tw`mt-4`"#####, r#####"({
  marginTop: "1rem",
})
;"##### ; "222")]
#[test_case(r#####"tw`mt-5`"#####, r#####"({
  marginTop: "1.25rem",
})
;"##### ; "223")]
#[test_case(r#####"tw`mt-6`"#####, r#####"({
  marginTop: "1.5rem",
})
;"##### ; "224")]
#[test_case(r#####"tw`mt-7`"#####, r#####"({
  marginTop: "1.75rem",
})
;"##### ; "225")]
#[test_case(r#####"tw`mt-8`"#####, r#####"({
  marginTop: "2rem",
})
;"##### ; "226")]
#[test_case(r#####"tw`mt-9`"#####, r#####"({
  marginTop: "2.25rem",
})
;"##### ; "227")]
#[test_case(r#####"tw`mt-10`"#####, r#####"({
  marginTop: "2.5rem",
})
;"##### ; "228")]
#[test_case(r#####"tw`mt-11`"#####, r#####"({
  marginTop: "2.75rem",
})
;"##### ; "229")]
#[test_case(r#####"tw`mt-12`"#####, r#####"({
  marginTop: "3rem",
})
;"##### ; "230")]
#[test_case(r#####"tw`mt-14`"#####, r#####"({
  marginTop: "3.5rem",
})
;"##### ; "231")]
#[test_case(r#####"tw`mt-16`"#####, r#####"({
  marginTop: "4rem",
})
;"##### ; "232")]
#[test_case(r#####"tw`mt-20`"#####, r#####"({
  marginTop: "5rem",
})
;"##### ; "233")]
#[test_case(r#####"tw`mt-24`"#####, r#####"({
  marginTop: "6rem",
})
;"##### ; "234")]
#[test_case(r#####"tw`mt-28`"#####, r#####"({
  marginTop: "7rem",
})
;"##### ; "235")]
#[test_case(r#####"tw`mt-32`"#####, r#####"({
  marginTop: "8rem",
})
;"##### ; "236")]
#[test_case(r#####"tw`mt-36`"#####, r#####"({
  marginTop: "9rem",
})
;"##### ; "237")]
#[test_case(r#####"tw`mt-40`"#####, r#####"({
  marginTop: "10rem",
})
;"##### ; "238")]
#[test_case(r#####"tw`mt-44`"#####, r#####"({
  marginTop: "11rem",
})
;"##### ; "239")]
#[test_case(r#####"tw`mt-48`"#####, r#####"({
  marginTop: "12rem",
})
;"##### ; "240")]
#[test_case(r#####"tw`mt-52`"#####, r#####"({
  marginTop: "13rem",
})
;"##### ; "241")]
#[test_case(r#####"tw`mt-56`"#####, r#####"({
  marginTop: "14rem",
})
;"##### ; "242")]
#[test_case(r#####"tw`mt-60`"#####, r#####"({
  marginTop: "15rem",
})
;"##### ; "243")]
#[test_case(r#####"tw`mt-64`"#####, r#####"({
  marginTop: "16rem",
})
;"##### ; "244")]
#[test_case(r#####"tw`mt-72`"#####, r#####"({
  marginTop: "18rem",
})
;"##### ; "245")]
#[test_case(r#####"tw`mt-80`"#####, r#####"({
  marginTop: "20rem",
})
;"##### ; "246")]
#[test_case(r#####"tw`mt-96`"#####, r#####"({
  marginTop: "24rem",
})
;"##### ; "247")]
#[test_case(r#####"tw`mt-auto`"#####, r#####"({
  marginTop: "auto",
})
;"##### ; "248")]
#[test_case(r#####"tw`-mt-0`"#####, r#####"({
  marginTop: "-0px",
})
;"##### ; "249")]
#[test_case(r#####"tw`-mt-px`"#####, r#####"({
  marginTop: "-1px",
})
;"##### ; "250")]
#[test_case(r#####"tw`-mt-0.5`"#####, r#####"({
  marginTop: "-0.125rem",
})
;"##### ; "251")]
#[test_case(r#####"tw`-mt-1`"#####, r#####"({
  marginTop: "-0.25rem",
})
;"##### ; "252")]
#[test_case(r#####"tw`-mt-1.5`"#####, r#####"({
  marginTop: "-0.375rem",
})
;"##### ; "253")]
#[test_case(r#####"tw`-mt-2`"#####, r#####"({
  marginTop: "-0.5rem",
})
;"##### ; "254")]
#[test_case(r#####"tw`-mt-2.5`"#####, r#####"({
  marginTop: "-0.625rem",
})
;"##### ; "255")]
#[test_case(r#####"tw`-mt-3`"#####, r#####"({
  marginTop: "-0.75rem",
})
;"##### ; "256")]
#[test_case(r#####"tw`-mt-3.5`"#####, r#####"({
  marginTop: "-0.875rem",
})
;"##### ; "257")]
#[test_case(r#####"tw`-mt-4`"#####, r#####"({
  marginTop: "-1rem",
})
;"##### ; "258")]
#[test_case(r#####"tw`-mt-5`"#####, r#####"({
  marginTop: "-1.25rem",
})
;"##### ; "259")]
#[test_case(r#####"tw`-mt-6`"#####, r#####"({
  marginTop: "-1.5rem",
})
;"##### ; "260")]
#[test_case(r#####"tw`-mt-7`"#####, r#####"({
  marginTop: "-1.75rem",
})
;"##### ; "261")]
#[test_case(r#####"tw`-mt-8`"#####, r#####"({
  marginTop: "-2rem",
})
;"##### ; "262")]
#[test_case(r#####"tw`-mt-9`"#####, r#####"({
  marginTop: "-2.25rem",
})
;"##### ; "263")]
#[test_case(r#####"tw`-mt-10`"#####, r#####"({
  marginTop: "-2.5rem",
})
;"##### ; "264")]
#[test_case(r#####"tw`-mt-11`"#####, r#####"({
  marginTop: "-2.75rem",
})
;"##### ; "265")]
#[test_case(r#####"tw`-mt-12`"#####, r#####"({
  marginTop: "-3rem",
})
;"##### ; "266")]
#[test_case(r#####"tw`-mt-14`"#####, r#####"({
  marginTop: "-3.5rem",
})
;"##### ; "267")]
#[test_case(r#####"tw`-mt-16`"#####, r#####"({
  marginTop: "-4rem",
})
;"##### ; "268")]
#[test_case(r#####"tw`-mt-20`"#####, r#####"({
  marginTop: "-5rem",
})
;"##### ; "269")]
#[test_case(r#####"tw`-mt-24`"#####, r#####"({
  marginTop: "-6rem",
})
;"##### ; "270")]
#[test_case(r#####"tw`-mt-28`"#####, r#####"({
  marginTop: "-7rem",
})
;"##### ; "271")]
#[test_case(r#####"tw`-mt-32`"#####, r#####"({
  marginTop: "-8rem",
})
;"##### ; "272")]
#[test_case(r#####"tw`-mt-36`"#####, r#####"({
  marginTop: "-9rem",
})
;"##### ; "273")]
#[test_case(r#####"tw`-mt-40`"#####, r#####"({
  marginTop: "-10rem",
})
;"##### ; "274")]
#[test_case(r#####"tw`-mt-44`"#####, r#####"({
  marginTop: "-11rem",
})
;"##### ; "275")]
#[test_case(r#####"tw`-mt-48`"#####, r#####"({
  marginTop: "-12rem",
})
;"##### ; "276")]
#[test_case(r#####"tw`-mt-52`"#####, r#####"({
  marginTop: "-13rem",
})
;"##### ; "277")]
#[test_case(r#####"tw`-mt-56`"#####, r#####"({
  marginTop: "-14rem",
})
;"##### ; "278")]
#[test_case(r#####"tw`-mt-60`"#####, r#####"({
  marginTop: "-15rem",
})
;"##### ; "279")]
#[test_case(r#####"tw`-mt-64`"#####, r#####"({
  marginTop: "-16rem",
})
;"##### ; "280")]
#[test_case(r#####"tw`-mt-72`"#####, r#####"({
  marginTop: "-18rem",
})
;"##### ; "281")]
#[test_case(r#####"tw`-mt-80`"#####, r#####"({
  marginTop: "-20rem",
})
;"##### ; "282")]
#[test_case(r#####"tw`-mt-96`"#####, r#####"({
  marginTop: "-24rem",
})
;"##### ; "283")]
#[test_case(r#####"tw`mr-0`"#####, r#####"({
  marginRight: "0px",
})
;"##### ; "284")]
#[test_case(r#####"tw`mr-px`"#####, r#####"({
  marginRight: "1px",
})
;"##### ; "285")]
#[test_case(r#####"tw`mr-0.5`"#####, r#####"({
  marginRight: "0.125rem",
})
;"##### ; "286")]
#[test_case(r#####"tw`mr-1`"#####, r#####"({
  marginRight: "0.25rem",
})
;"##### ; "287")]
#[test_case(r#####"tw`mr-1.5`"#####, r#####"({
  marginRight: "0.375rem",
})
;"##### ; "288")]
#[test_case(r#####"tw`mr-2`"#####, r#####"({
  marginRight: "0.5rem",
})
;"##### ; "289")]
#[test_case(r#####"tw`mr-2.5`"#####, r#####"({
  marginRight: "0.625rem",
})
;"##### ; "290")]
#[test_case(r#####"tw`mr-3`"#####, r#####"({
  marginRight: "0.75rem",
})
;"##### ; "291")]
#[test_case(r#####"tw`mr-3.5`"#####, r#####"({
  marginRight: "0.875rem",
})
;"##### ; "292")]
#[test_case(r#####"tw`mr-4`"#####, r#####"({
  marginRight: "1rem",
})
;"##### ; "293")]
#[test_case(r#####"tw`mr-5`"#####, r#####"({
  marginRight: "1.25rem",
})
;"##### ; "294")]
#[test_case(r#####"tw`mr-6`"#####, r#####"({
  marginRight: "1.5rem",
})
;"##### ; "295")]
#[test_case(r#####"tw`mr-7`"#####, r#####"({
  marginRight: "1.75rem",
})
;"##### ; "296")]
#[test_case(r#####"tw`mr-8`"#####, r#####"({
  marginRight: "2rem",
})
;"##### ; "297")]
#[test_case(r#####"tw`mr-9`"#####, r#####"({
  marginRight: "2.25rem",
})
;"##### ; "298")]
#[test_case(r#####"tw`mr-10`"#####, r#####"({
  marginRight: "2.5rem",
})
;"##### ; "299")]
#[test_case(r#####"tw`mr-11`"#####, r#####"({
  marginRight: "2.75rem",
})
;"##### ; "300")]
#[test_case(r#####"tw`mr-12`"#####, r#####"({
  marginRight: "3rem",
})
;"##### ; "301")]
#[test_case(r#####"tw`mr-14`"#####, r#####"({
  marginRight: "3.5rem",
})
;"##### ; "302")]
#[test_case(r#####"tw`mr-16`"#####, r#####"({
  marginRight: "4rem",
})
;"##### ; "303")]
#[test_case(r#####"tw`mr-20`"#####, r#####"({
  marginRight: "5rem",
})
;"##### ; "304")]
#[test_case(r#####"tw`mr-24`"#####, r#####"({
  marginRight: "6rem",
})
;"##### ; "305")]
#[test_case(r#####"tw`mr-28`"#####, r#####"({
  marginRight: "7rem",
})
;"##### ; "306")]
#[test_case(r#####"tw`mr-32`"#####, r#####"({
  marginRight: "8rem",
})
;"##### ; "307")]
#[test_case(r#####"tw`mr-36`"#####, r#####"({
  marginRight: "9rem",
})
;"##### ; "308")]
#[test_case(r#####"tw`mr-40`"#####, r#####"({
  marginRight: "10rem",
})
;"##### ; "309")]
#[test_case(r#####"tw`mr-44`"#####, r#####"({
  marginRight: "11rem",
})
;"##### ; "310")]
#[test_case(r#####"tw`mr-48`"#####, r#####"({
  marginRight: "12rem",
})
;"##### ; "311")]
#[test_case(r#####"tw`mr-52`"#####, r#####"({
  marginRight: "13rem",
})
;"##### ; "312")]
#[test_case(r#####"tw`mr-56`"#####, r#####"({
  marginRight: "14rem",
})
;"##### ; "313")]
#[test_case(r#####"tw`mr-60`"#####, r#####"({
  marginRight: "15rem",
})
;"##### ; "314")]
#[test_case(r#####"tw`mr-64`"#####, r#####"({
  marginRight: "16rem",
})
;"##### ; "315")]
#[test_case(r#####"tw`mr-72`"#####, r#####"({
  marginRight: "18rem",
})
;"##### ; "316")]
#[test_case(r#####"tw`mr-80`"#####, r#####"({
  marginRight: "20rem",
})
;"##### ; "317")]
#[test_case(r#####"tw`mr-96`"#####, r#####"({
  marginRight: "24rem",
})
;"##### ; "318")]
#[test_case(r#####"tw`mr-auto`"#####, r#####"({
  marginRight: "auto",
})
;"##### ; "319")]
#[test_case(r#####"tw`-mr-0`"#####, r#####"({
  marginRight: "-0px",
})
;"##### ; "320")]
#[test_case(r#####"tw`-mr-px`"#####, r#####"({
  marginRight: "-1px",
})
;"##### ; "321")]
#[test_case(r#####"tw`-mr-0.5`"#####, r#####"({
  marginRight: "-0.125rem",
})
;"##### ; "322")]
#[test_case(r#####"tw`-mr-1`"#####, r#####"({
  marginRight: "-0.25rem",
})
;"##### ; "323")]
#[test_case(r#####"tw`-mr-1.5`"#####, r#####"({
  marginRight: "-0.375rem",
})
;"##### ; "324")]
#[test_case(r#####"tw`-mr-2`"#####, r#####"({
  marginRight: "-0.5rem",
})
;"##### ; "325")]
#[test_case(r#####"tw`-mr-2.5`"#####, r#####"({
  marginRight: "-0.625rem",
})
;"##### ; "326")]
#[test_case(r#####"tw`-mr-3`"#####, r#####"({
  marginRight: "-0.75rem",
})
;"##### ; "327")]
#[test_case(r#####"tw`-mr-3.5`"#####, r#####"({
  marginRight: "-0.875rem",
})
;"##### ; "328")]
#[test_case(r#####"tw`-mr-4`"#####, r#####"({
  marginRight: "-1rem",
})
;"##### ; "329")]
#[test_case(r#####"tw`-mr-5`"#####, r#####"({
  marginRight: "-1.25rem",
})
;"##### ; "330")]
#[test_case(r#####"tw`-mr-6`"#####, r#####"({
  marginRight: "-1.5rem",
})
;"##### ; "331")]
#[test_case(r#####"tw`-mr-7`"#####, r#####"({
  marginRight: "-1.75rem",
})
;"##### ; "332")]
#[test_case(r#####"tw`-mr-8`"#####, r#####"({
  marginRight: "-2rem",
})
;"##### ; "333")]
#[test_case(r#####"tw`-mr-9`"#####, r#####"({
  marginRight: "-2.25rem",
})
;"##### ; "334")]
#[test_case(r#####"tw`-mr-10`"#####, r#####"({
  marginRight: "-2.5rem",
})
;"##### ; "335")]
#[test_case(r#####"tw`-mr-11`"#####, r#####"({
  marginRight: "-2.75rem",
})
;"##### ; "336")]
#[test_case(r#####"tw`-mr-12`"#####, r#####"({
  marginRight: "-3rem",
})
;"##### ; "337")]
#[test_case(r#####"tw`-mr-14`"#####, r#####"({
  marginRight: "-3.5rem",
})
;"##### ; "338")]
#[test_case(r#####"tw`-mr-16`"#####, r#####"({
  marginRight: "-4rem",
})
;"##### ; "339")]
#[test_case(r#####"tw`-mr-20`"#####, r#####"({
  marginRight: "-5rem",
})
;"##### ; "340")]
#[test_case(r#####"tw`-mr-24`"#####, r#####"({
  marginRight: "-6rem",
})
;"##### ; "341")]
#[test_case(r#####"tw`-mr-28`"#####, r#####"({
  marginRight: "-7rem",
})
;"##### ; "342")]
#[test_case(r#####"tw`-mr-32`"#####, r#####"({
  marginRight: "-8rem",
})
;"##### ; "343")]
#[test_case(r#####"tw`-mr-36`"#####, r#####"({
  marginRight: "-9rem",
})
;"##### ; "344")]
#[test_case(r#####"tw`-mr-40`"#####, r#####"({
  marginRight: "-10rem",
})
;"##### ; "345")]
#[test_case(r#####"tw`-mr-44`"#####, r#####"({
  marginRight: "-11rem",
})
;"##### ; "346")]
#[test_case(r#####"tw`-mr-48`"#####, r#####"({
  marginRight: "-12rem",
})
;"##### ; "347")]
#[test_case(r#####"tw`-mr-52`"#####, r#####"({
  marginRight: "-13rem",
})
;"##### ; "348")]
#[test_case(r#####"tw`-mr-56`"#####, r#####"({
  marginRight: "-14rem",
})
;"##### ; "349")]
#[test_case(r#####"tw`-mr-60`"#####, r#####"({
  marginRight: "-15rem",
})
;"##### ; "350")]
#[test_case(r#####"tw`-mr-64`"#####, r#####"({
  marginRight: "-16rem",
})
;"##### ; "351")]
#[test_case(r#####"tw`-mr-72`"#####, r#####"({
  marginRight: "-18rem",
})
;"##### ; "352")]
#[test_case(r#####"tw`-mr-80`"#####, r#####"({
  marginRight: "-20rem",
})
;"##### ; "353")]
#[test_case(r#####"tw`-mr-96`"#####, r#####"({
  marginRight: "-24rem",
})
;"##### ; "354")]
#[test_case(r#####"tw`mb-0`"#####, r#####"({
  marginBottom: "0px",
})
;"##### ; "355")]
#[test_case(r#####"tw`mb-px`"#####, r#####"({
  marginBottom: "1px",
})
;"##### ; "356")]
#[test_case(r#####"tw`mb-0.5`"#####, r#####"({
  marginBottom: "0.125rem",
})
;"##### ; "357")]
#[test_case(r#####"tw`mb-1`"#####, r#####"({
  marginBottom: "0.25rem",
})
;"##### ; "358")]
#[test_case(r#####"tw`mb-1.5`"#####, r#####"({
  marginBottom: "0.375rem",
})
;"##### ; "359")]
#[test_case(r#####"tw`mb-2`"#####, r#####"({
  marginBottom: "0.5rem",
})
;"##### ; "360")]
#[test_case(r#####"tw`mb-2.5`"#####, r#####"({
  marginBottom: "0.625rem",
})
;"##### ; "361")]
#[test_case(r#####"tw`mb-3`"#####, r#####"({
  marginBottom: "0.75rem",
})
;"##### ; "362")]
#[test_case(r#####"tw`mb-3.5`"#####, r#####"({
  marginBottom: "0.875rem",
})
;"##### ; "363")]
#[test_case(r#####"tw`mb-4`"#####, r#####"({
  marginBottom: "1rem",
})
;"##### ; "364")]
#[test_case(r#####"tw`mb-5`"#####, r#####"({
  marginBottom: "1.25rem",
})
;"##### ; "365")]
#[test_case(r#####"tw`mb-6`"#####, r#####"({
  marginBottom: "1.5rem",
})
;"##### ; "366")]
#[test_case(r#####"tw`mb-7`"#####, r#####"({
  marginBottom: "1.75rem",
})
;"##### ; "367")]
#[test_case(r#####"tw`mb-8`"#####, r#####"({
  marginBottom: "2rem",
})
;"##### ; "368")]
#[test_case(r#####"tw`mb-9`"#####, r#####"({
  marginBottom: "2.25rem",
})
;"##### ; "369")]
#[test_case(r#####"tw`mb-10`"#####, r#####"({
  marginBottom: "2.5rem",
})
;"##### ; "370")]
#[test_case(r#####"tw`mb-11`"#####, r#####"({
  marginBottom: "2.75rem",
})
;"##### ; "371")]
#[test_case(r#####"tw`mb-12`"#####, r#####"({
  marginBottom: "3rem",
})
;"##### ; "372")]
#[test_case(r#####"tw`mb-14`"#####, r#####"({
  marginBottom: "3.5rem",
})
;"##### ; "373")]
#[test_case(r#####"tw`mb-16`"#####, r#####"({
  marginBottom: "4rem",
})
;"##### ; "374")]
#[test_case(r#####"tw`mb-20`"#####, r#####"({
  marginBottom: "5rem",
})
;"##### ; "375")]
#[test_case(r#####"tw`mb-24`"#####, r#####"({
  marginBottom: "6rem",
})
;"##### ; "376")]
#[test_case(r#####"tw`mb-28`"#####, r#####"({
  marginBottom: "7rem",
})
;"##### ; "377")]
#[test_case(r#####"tw`mb-32`"#####, r#####"({
  marginBottom: "8rem",
})
;"##### ; "378")]
#[test_case(r#####"tw`mb-36`"#####, r#####"({
  marginBottom: "9rem",
})
;"##### ; "379")]
#[test_case(r#####"tw`mb-40`"#####, r#####"({
  marginBottom: "10rem",
})
;"##### ; "380")]
#[test_case(r#####"tw`mb-44`"#####, r#####"({
  marginBottom: "11rem",
})
;"##### ; "381")]
#[test_case(r#####"tw`mb-48`"#####, r#####"({
  marginBottom: "12rem",
})
;"##### ; "382")]
#[test_case(r#####"tw`mb-52`"#####, r#####"({
  marginBottom: "13rem",
})
;"##### ; "383")]
#[test_case(r#####"tw`mb-56`"#####, r#####"({
  marginBottom: "14rem",
})
;"##### ; "384")]
#[test_case(r#####"tw`mb-60`"#####, r#####"({
  marginBottom: "15rem",
})
;"##### ; "385")]
#[test_case(r#####"tw`mb-64`"#####, r#####"({
  marginBottom: "16rem",
})
;"##### ; "386")]
#[test_case(r#####"tw`mb-72`"#####, r#####"({
  marginBottom: "18rem",
})
;"##### ; "387")]
#[test_case(r#####"tw`mb-80`"#####, r#####"({
  marginBottom: "20rem",
})
;"##### ; "388")]
#[test_case(r#####"tw`mb-96`"#####, r#####"({
  marginBottom: "24rem",
})
;"##### ; "389")]
#[test_case(r#####"tw`mb-auto`"#####, r#####"({
  marginBottom: "auto",
})
;"##### ; "390")]
#[test_case(r#####"tw`-mb-0`"#####, r#####"({
  marginBottom: "-0px",
})
;"##### ; "391")]
#[test_case(r#####"tw`-mb-px`"#####, r#####"({
  marginBottom: "-1px",
})
;"##### ; "392")]
#[test_case(r#####"tw`-mb-0.5`"#####, r#####"({
  marginBottom: "-0.125rem",
})
;"##### ; "393")]
#[test_case(r#####"tw`-mb-1`"#####, r#####"({
  marginBottom: "-0.25rem",
})
;"##### ; "394")]
#[test_case(r#####"tw`-mb-1.5`"#####, r#####"({
  marginBottom: "-0.375rem",
})
;"##### ; "395")]
#[test_case(r#####"tw`-mb-2`"#####, r#####"({
  marginBottom: "-0.5rem",
})
;"##### ; "396")]
#[test_case(r#####"tw`-mb-2.5`"#####, r#####"({
  marginBottom: "-0.625rem",
})
;"##### ; "397")]
#[test_case(r#####"tw`-mb-3`"#####, r#####"({
  marginBottom: "-0.75rem",
})
;"##### ; "398")]
#[test_case(r#####"tw`-mb-3.5`"#####, r#####"({
  marginBottom: "-0.875rem",
})
;"##### ; "399")]
#[test_case(r#####"tw`-mb-4`"#####, r#####"({
  marginBottom: "-1rem",
})
;"##### ; "400")]
#[test_case(r#####"tw`-mb-5`"#####, r#####"({
  marginBottom: "-1.25rem",
})
;"##### ; "401")]
#[test_case(r#####"tw`-mb-6`"#####, r#####"({
  marginBottom: "-1.5rem",
})
;"##### ; "402")]
#[test_case(r#####"tw`-mb-7`"#####, r#####"({
  marginBottom: "-1.75rem",
})
;"##### ; "403")]
#[test_case(r#####"tw`-mb-8`"#####, r#####"({
  marginBottom: "-2rem",
})
;"##### ; "404")]
#[test_case(r#####"tw`-mb-9`"#####, r#####"({
  marginBottom: "-2.25rem",
})
;"##### ; "405")]
#[test_case(r#####"tw`-mb-10`"#####, r#####"({
  marginBottom: "-2.5rem",
})
;"##### ; "406")]
#[test_case(r#####"tw`-mb-11`"#####, r#####"({
  marginBottom: "-2.75rem",
})
;"##### ; "407")]
#[test_case(r#####"tw`-mb-12`"#####, r#####"({
  marginBottom: "-3rem",
})
;"##### ; "408")]
#[test_case(r#####"tw`-mb-14`"#####, r#####"({
  marginBottom: "-3.5rem",
})
;"##### ; "409")]
#[test_case(r#####"tw`-mb-16`"#####, r#####"({
  marginBottom: "-4rem",
})
;"##### ; "410")]
#[test_case(r#####"tw`-mb-20`"#####, r#####"({
  marginBottom: "-5rem",
})
;"##### ; "411")]
#[test_case(r#####"tw`-mb-24`"#####, r#####"({
  marginBottom: "-6rem",
})
;"##### ; "412")]
#[test_case(r#####"tw`-mb-28`"#####, r#####"({
  marginBottom: "-7rem",
})
;"##### ; "413")]
#[test_case(r#####"tw`-mb-32`"#####, r#####"({
  marginBottom: "-8rem",
})
;"##### ; "414")]
#[test_case(r#####"tw`-mb-36`"#####, r#####"({
  marginBottom: "-9rem",
})
;"##### ; "415")]
#[test_case(r#####"tw`-mb-40`"#####, r#####"({
  marginBottom: "-10rem",
})
;"##### ; "416")]
#[test_case(r#####"tw`-mb-44`"#####, r#####"({
  marginBottom: "-11rem",
})
;"##### ; "417")]
#[test_case(r#####"tw`-mb-48`"#####, r#####"({
  marginBottom: "-12rem",
})
;"##### ; "418")]
#[test_case(r#####"tw`-mb-52`"#####, r#####"({
  marginBottom: "-13rem",
})
;"##### ; "419")]
#[test_case(r#####"tw`-mb-56`"#####, r#####"({
  marginBottom: "-14rem",
})
;"##### ; "420")]
#[test_case(r#####"tw`-mb-60`"#####, r#####"({
  marginBottom: "-15rem",
})
;"##### ; "421")]
#[test_case(r#####"tw`-mb-64`"#####, r#####"({
  marginBottom: "-16rem",
})
;"##### ; "422")]
#[test_case(r#####"tw`-mb-72`"#####, r#####"({
  marginBottom: "-18rem",
})
;"##### ; "423")]
#[test_case(r#####"tw`-mb-80`"#####, r#####"({
  marginBottom: "-20rem",
})
;"##### ; "424")]
#[test_case(r#####"tw`-mb-96`"#####, r#####"({
  marginBottom: "-24rem",
})
;"##### ; "425")]
#[test_case(r#####"tw`ml-0`"#####, r#####"({
  marginLeft: "0px",
})
;"##### ; "426")]
#[test_case(r#####"tw`ml-px`"#####, r#####"({
  marginLeft: "1px",
})
;"##### ; "427")]
#[test_case(r#####"tw`ml-0.5`"#####, r#####"({
  marginLeft: "0.125rem",
})
;"##### ; "428")]
#[test_case(r#####"tw`ml-1`"#####, r#####"({
  marginLeft: "0.25rem",
})
;"##### ; "429")]
#[test_case(r#####"tw`ml-1.5`"#####, r#####"({
  marginLeft: "0.375rem",
})
;"##### ; "430")]
#[test_case(r#####"tw`ml-2`"#####, r#####"({
  marginLeft: "0.5rem",
})
;"##### ; "431")]
#[test_case(r#####"tw`ml-2.5`"#####, r#####"({
  marginLeft: "0.625rem",
})
;"##### ; "432")]
#[test_case(r#####"tw`ml-3`"#####, r#####"({
  marginLeft: "0.75rem",
})
;"##### ; "433")]
#[test_case(r#####"tw`ml-3.5`"#####, r#####"({
  marginLeft: "0.875rem",
})
;"##### ; "434")]
#[test_case(r#####"tw`ml-4`"#####, r#####"({
  marginLeft: "1rem",
})
;"##### ; "435")]
#[test_case(r#####"tw`ml-5`"#####, r#####"({
  marginLeft: "1.25rem",
})
;"##### ; "436")]
#[test_case(r#####"tw`ml-6`"#####, r#####"({
  marginLeft: "1.5rem",
})
;"##### ; "437")]
#[test_case(r#####"tw`ml-7`"#####, r#####"({
  marginLeft: "1.75rem",
})
;"##### ; "438")]
#[test_case(r#####"tw`ml-8`"#####, r#####"({
  marginLeft: "2rem",
})
;"##### ; "439")]
#[test_case(r#####"tw`ml-9`"#####, r#####"({
  marginLeft: "2.25rem",
})
;"##### ; "440")]
#[test_case(r#####"tw`ml-10`"#####, r#####"({
  marginLeft: "2.5rem",
})
;"##### ; "441")]
#[test_case(r#####"tw`ml-11`"#####, r#####"({
  marginLeft: "2.75rem",
})
;"##### ; "442")]
#[test_case(r#####"tw`ml-12`"#####, r#####"({
  marginLeft: "3rem",
})
;"##### ; "443")]
#[test_case(r#####"tw`ml-14`"#####, r#####"({
  marginLeft: "3.5rem",
})
;"##### ; "444")]
#[test_case(r#####"tw`ml-16`"#####, r#####"({
  marginLeft: "4rem",
})
;"##### ; "445")]
#[test_case(r#####"tw`ml-20`"#####, r#####"({
  marginLeft: "5rem",
})
;"##### ; "446")]
#[test_case(r#####"tw`ml-24`"#####, r#####"({
  marginLeft: "6rem",
})
;"##### ; "447")]
#[test_case(r#####"tw`ml-28`"#####, r#####"({
  marginLeft: "7rem",
})
;"##### ; "448")]
#[test_case(r#####"tw`ml-32`"#####, r#####"({
  marginLeft: "8rem",
})
;"##### ; "449")]
#[test_case(r#####"tw`ml-36`"#####, r#####"({
  marginLeft: "9rem",
})
;"##### ; "450")]
#[test_case(r#####"tw`ml-40`"#####, r#####"({
  marginLeft: "10rem",
})
;"##### ; "451")]
#[test_case(r#####"tw`ml-44`"#####, r#####"({
  marginLeft: "11rem",
})
;"##### ; "452")]
#[test_case(r#####"tw`ml-48`"#####, r#####"({
  marginLeft: "12rem",
})
;"##### ; "453")]
#[test_case(r#####"tw`ml-52`"#####, r#####"({
  marginLeft: "13rem",
})
;"##### ; "454")]
#[test_case(r#####"tw`ml-56`"#####, r#####"({
  marginLeft: "14rem",
})
;"##### ; "455")]
#[test_case(r#####"tw`ml-60`"#####, r#####"({
  marginLeft: "15rem",
})
;"##### ; "456")]
#[test_case(r#####"tw`ml-64`"#####, r#####"({
  marginLeft: "16rem",
})
;"##### ; "457")]
#[test_case(r#####"tw`ml-72`"#####, r#####"({
  marginLeft: "18rem",
})
;"##### ; "458")]
#[test_case(r#####"tw`ml-80`"#####, r#####"({
  marginLeft: "20rem",
})
;"##### ; "459")]
#[test_case(r#####"tw`ml-96`"#####, r#####"({
  marginLeft: "24rem",
})
;"##### ; "460")]
#[test_case(r#####"tw`ml-auto`"#####, r#####"({
  marginLeft: "auto",
})
;"##### ; "461")]
#[test_case(r#####"tw`-ml-0`"#####, r#####"({
  marginLeft: "-0px",
})
;"##### ; "462")]
#[test_case(r#####"tw`-ml-px`"#####, r#####"({
  marginLeft: "-1px",
})
;"##### ; "463")]
#[test_case(r#####"tw`-ml-0.5`"#####, r#####"({
  marginLeft: "-0.125rem",
})
;"##### ; "464")]
#[test_case(r#####"tw`-ml-1`"#####, r#####"({
  marginLeft: "-0.25rem",
})
;"##### ; "465")]
#[test_case(r#####"tw`-ml-1.5`"#####, r#####"({
  marginLeft: "-0.375rem",
})
;"##### ; "466")]
#[test_case(r#####"tw`-ml-2`"#####, r#####"({
  marginLeft: "-0.5rem",
})
;"##### ; "467")]
#[test_case(r#####"tw`-ml-2.5`"#####, r#####"({
  marginLeft: "-0.625rem",
})
;"##### ; "468")]
#[test_case(r#####"tw`-ml-3`"#####, r#####"({
  marginLeft: "-0.75rem",
})
;"##### ; "469")]
#[test_case(r#####"tw`-ml-3.5`"#####, r#####"({
  marginLeft: "-0.875rem",
})
;"##### ; "470")]
#[test_case(r#####"tw`-ml-4`"#####, r#####"({
  marginLeft: "-1rem",
})
;"##### ; "471")]
#[test_case(r#####"tw`-ml-5`"#####, r#####"({
  marginLeft: "-1.25rem",
})
;"##### ; "472")]
#[test_case(r#####"tw`-ml-6`"#####, r#####"({
  marginLeft: "-1.5rem",
})
;"##### ; "473")]
#[test_case(r#####"tw`-ml-7`"#####, r#####"({
  marginLeft: "-1.75rem",
})
;"##### ; "474")]
#[test_case(r#####"tw`-ml-8`"#####, r#####"({
  marginLeft: "-2rem",
})
;"##### ; "475")]
#[test_case(r#####"tw`-ml-9`"#####, r#####"({
  marginLeft: "-2.25rem",
})
;"##### ; "476")]
#[test_case(r#####"tw`-ml-10`"#####, r#####"({
  marginLeft: "-2.5rem",
})
;"##### ; "477")]
#[test_case(r#####"tw`-ml-11`"#####, r#####"({
  marginLeft: "-2.75rem",
})
;"##### ; "478")]
#[test_case(r#####"tw`-ml-12`"#####, r#####"({
  marginLeft: "-3rem",
})
;"##### ; "479")]
#[test_case(r#####"tw`-ml-14`"#####, r#####"({
  marginLeft: "-3.5rem",
})
;"##### ; "480")]
#[test_case(r#####"tw`-ml-16`"#####, r#####"({
  marginLeft: "-4rem",
})
;"##### ; "481")]
#[test_case(r#####"tw`-ml-20`"#####, r#####"({
  marginLeft: "-5rem",
})
;"##### ; "482")]
#[test_case(r#####"tw`-ml-24`"#####, r#####"({
  marginLeft: "-6rem",
})
;"##### ; "483")]
#[test_case(r#####"tw`-ml-28`"#####, r#####"({
  marginLeft: "-7rem",
})
;"##### ; "484")]
#[test_case(r#####"tw`-ml-32`"#####, r#####"({
  marginLeft: "-8rem",
})
;"##### ; "485")]
#[test_case(r#####"tw`-ml-36`"#####, r#####"({
  marginLeft: "-9rem",
})
;"##### ; "486")]
#[test_case(r#####"tw`-ml-40`"#####, r#####"({
  marginLeft: "-10rem",
})
;"##### ; "487")]
#[test_case(r#####"tw`-ml-44`"#####, r#####"({
  marginLeft: "-11rem",
})
;"##### ; "488")]
#[test_case(r#####"tw`-ml-48`"#####, r#####"({
  marginLeft: "-12rem",
})
;"##### ; "489")]
#[test_case(r#####"tw`-ml-52`"#####, r#####"({
  marginLeft: "-13rem",
})
;"##### ; "490")]
#[test_case(r#####"tw`-ml-56`"#####, r#####"({
  marginLeft: "-14rem",
})
;"##### ; "491")]
#[test_case(r#####"tw`-ml-60`"#####, r#####"({
  marginLeft: "-15rem",
})
;"##### ; "492")]
#[test_case(r#####"tw`-ml-64`"#####, r#####"({
  marginLeft: "-16rem",
})
;"##### ; "493")]
#[test_case(r#####"tw`-ml-72`"#####, r#####"({
  marginLeft: "-18rem",
})
;"##### ; "494")]
#[test_case(r#####"tw`-ml-80`"#####, r#####"({
  marginLeft: "-20rem",
})
;"##### ; "495")]
#[test_case(r#####"tw`-ml-96`"#####, r#####"({
  marginLeft: "-24rem",
})
;"##### ; "496")]
#[test_case(r#####"tw`m-[5px]`"#####, r#####"({
  margin: "5px",
})
;"##### ; "497")]
#[test_case(r#####"tw`mt-[5px]`"#####, r#####"({
  marginTop: "5px",
})
;"##### ; "498")]
#[test_case(r#####"tw`ml-[5px]`"#####, r#####"({
  marginLeft: "5px",
})
;"##### ; "499")]
#[test_case(r#####"tw`mr-[5px]`"#####, r#####"({
  marginRight: "5px",
})
;"##### ; "500")]
#[test_case(r#####"tw`mb-[5px]`"#####, r#####"({
  marginBottom: "5px",
})
;"##### ; "501")]
#[test_case(r#####"tw`-m-[5px]`"#####, r#####"({
  margin: "-5px",
})
;"##### ; "502")]
#[test_case(r#####"tw`-mt-[5px]`"#####, r#####"({
  marginTop: "-5px",
})
;"##### ; "503")]
#[test_case(r#####"tw`-ml-[5px]`"#####, r#####"({
  marginLeft: "-5px",
})
;"##### ; "504")]
#[test_case(r#####"tw`-mr-[5px]`"#####, r#####"({
  marginRight: "-5px",
})
;"##### ; "505")]
#[test_case(r#####"tw`-mb-[5px]`"#####, r#####"({
  marginBottom: "-5px",
})
;"##### ; "506")]
#[test_case(r#####"tw`mt-[clamp(30px,100px)]`"#####, r#####"({
  marginTop: "clamp(30px,100px)",
})
;"##### ; "507")]
#[test_case(r#####"tw`!-mt-4`"#####, r#####"({
  marginTop: "-1rem !important",
})
;"##### ; "508")]
#[test_case(r#####"tw`mt-6 mx-1 ms-4 me-8`"#####, r#####"({
  marginLeft: "0.25rem",
  marginRight: "0.25rem",
  marginInlineEnd: "2rem",
  marginInlineStart: "1rem",
  marginTop: "1.5rem",
})"##### ; "509")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
