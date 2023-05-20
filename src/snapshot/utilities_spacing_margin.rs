use crate::test::snapshot_inner;
use test_case::test_case;
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
;"##### ; "0")]
#[test_case(r#####"tw`m-0`"#####, r#####"({
  margin: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`m-px`"#####, r#####"({
  margin: "1px",
})
;"##### ; "2")]
#[test_case(r#####"tw`m-0.5`"#####, r#####"({
  margin: "0.125rem",
})
;"##### ; "3")]
#[test_case(r#####"tw`m-1`"#####, r#####"({
  margin: "0.25rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`m-1.5`"#####, r#####"({
  margin: "0.375rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`m-2`"#####, r#####"({
  margin: "0.5rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`m-2.5`"#####, r#####"({
  margin: "0.625rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`m-3`"#####, r#####"({
  margin: "0.75rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`m-3.5`"#####, r#####"({
  margin: "0.875rem",
})
;"##### ; "9")]
#[test_case(r#####"tw`m-4`"#####, r#####"({
  margin: "1rem",
})
;"##### ; "10")]
#[test_case(r#####"tw`m-5`"#####, r#####"({
  margin: "1.25rem",
})
;"##### ; "11")]
#[test_case(r#####"tw`m-6`"#####, r#####"({
  margin: "1.5rem",
})
;"##### ; "12")]
#[test_case(r#####"tw`m-7`"#####, r#####"({
  margin: "1.75rem",
})
;"##### ; "13")]
#[test_case(r#####"tw`m-8`"#####, r#####"({
  margin: "2rem",
})
;"##### ; "14")]
#[test_case(r#####"tw`m-9`"#####, r#####"({
  margin: "2.25rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`m-10`"#####, r#####"({
  margin: "2.5rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`m-11`"#####, r#####"({
  margin: "2.75rem",
})
;"##### ; "17")]
#[test_case(r#####"tw`m-12`"#####, r#####"({
  margin: "3rem",
})
;"##### ; "18")]
#[test_case(r#####"tw`m-14`"#####, r#####"({
  margin: "3.5rem",
})
;"##### ; "19")]
#[test_case(r#####"tw`m-16`"#####, r#####"({
  margin: "4rem",
})
;"##### ; "20")]
#[test_case(r#####"tw`m-20`"#####, r#####"({
  margin: "5rem",
})
;"##### ; "21")]
#[test_case(r#####"tw`m-24`"#####, r#####"({
  margin: "6rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`m-28`"#####, r#####"({
  margin: "7rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`m-32`"#####, r#####"({
  margin: "8rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`m-36`"#####, r#####"({
  margin: "9rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`m-40`"#####, r#####"({
  margin: "10rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`m-44`"#####, r#####"({
  margin: "11rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`m-48`"#####, r#####"({
  margin: "12rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`m-52`"#####, r#####"({
  margin: "13rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`m-56`"#####, r#####"({
  margin: "14rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`m-60`"#####, r#####"({
  margin: "15rem",
})
;"##### ; "31")]
#[test_case(r#####"tw`m-64`"#####, r#####"({
  margin: "16rem",
})
;"##### ; "32")]
#[test_case(r#####"tw`m-72`"#####, r#####"({
  margin: "18rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`m-80`"#####, r#####"({
  margin: "20rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`m-96`"#####, r#####"({
  margin: "24rem",
})
;"##### ; "35")]
#[test_case(r#####"tw`m-auto`"#####, r#####"({
  margin: "auto",
})
;"##### ; "36")]
#[test_case(r#####"tw`-m-0`"#####, r#####"({
  margin: "-0px",
})
;"##### ; "37")]
#[test_case(r#####"tw`-m-px`"#####, r#####"({
  margin: "-1px",
})
;"##### ; "38")]
#[test_case(r#####"tw`-m-0.5`"#####, r#####"({
  margin: "-0.125rem",
})
;"##### ; "39")]
#[test_case(r#####"tw`-m-1`"#####, r#####"({
  margin: "-0.25rem",
})
;"##### ; "40")]
#[test_case(r#####"tw`-m-1.5`"#####, r#####"({
  margin: "-0.375rem",
})
;"##### ; "41")]
#[test_case(r#####"tw`-m-2`"#####, r#####"({
  margin: "-0.5rem",
})
;"##### ; "42")]
#[test_case(r#####"tw`-m-2.5`"#####, r#####"({
  margin: "-0.625rem",
})
;"##### ; "43")]
#[test_case(r#####"tw`-m-3`"#####, r#####"({
  margin: "-0.75rem",
})
;"##### ; "44")]
#[test_case(r#####"tw`-m-3.5`"#####, r#####"({
  margin: "-0.875rem",
})
;"##### ; "45")]
#[test_case(r#####"tw`-m-4`"#####, r#####"({
  margin: "-1rem",
})
;"##### ; "46")]
#[test_case(r#####"tw`-m-5`"#####, r#####"({
  margin: "-1.25rem",
})
;"##### ; "47")]
#[test_case(r#####"tw`-m-6`"#####, r#####"({
  margin: "-1.5rem",
})
;"##### ; "48")]
#[test_case(r#####"tw`-m-7`"#####, r#####"({
  margin: "-1.75rem",
})
;"##### ; "49")]
#[test_case(r#####"tw`-m-8`"#####, r#####"({
  margin: "-2rem",
})
;"##### ; "50")]
#[test_case(r#####"tw`-m-9`"#####, r#####"({
  margin: "-2.25rem",
})
;"##### ; "51")]
#[test_case(r#####"tw`-m-10`"#####, r#####"({
  margin: "-2.5rem",
})
;"##### ; "52")]
#[test_case(r#####"tw`-m-11`"#####, r#####"({
  margin: "-2.75rem",
})
;"##### ; "53")]
#[test_case(r#####"tw`-m-12`"#####, r#####"({
  margin: "-3rem",
})
;"##### ; "54")]
#[test_case(r#####"tw`-m-14`"#####, r#####"({
  margin: "-3.5rem",
})
;"##### ; "55")]
#[test_case(r#####"tw`-m-16`"#####, r#####"({
  margin: "-4rem",
})
;"##### ; "56")]
#[test_case(r#####"tw`-m-20`"#####, r#####"({
  margin: "-5rem",
})
;"##### ; "57")]
#[test_case(r#####"tw`-m-24`"#####, r#####"({
  margin: "-6rem",
})
;"##### ; "58")]
#[test_case(r#####"tw`-m-28`"#####, r#####"({
  margin: "-7rem",
})
;"##### ; "59")]
#[test_case(r#####"tw`-m-32`"#####, r#####"({
  margin: "-8rem",
})
;"##### ; "60")]
#[test_case(r#####"tw`-m-36`"#####, r#####"({
  margin: "-9rem",
})
;"##### ; "61")]
#[test_case(r#####"tw`-m-40`"#####, r#####"({
  margin: "-10rem",
})
;"##### ; "62")]
#[test_case(r#####"tw`-m-44`"#####, r#####"({
  margin: "-11rem",
})
;"##### ; "63")]
#[test_case(r#####"tw`-m-48`"#####, r#####"({
  margin: "-12rem",
})
;"##### ; "64")]
#[test_case(r#####"tw`-m-52`"#####, r#####"({
  margin: "-13rem",
})
;"##### ; "65")]
#[test_case(r#####"tw`-m-56`"#####, r#####"({
  margin: "-14rem",
})
;"##### ; "66")]
#[test_case(r#####"tw`-m-60`"#####, r#####"({
  margin: "-15rem",
})
;"##### ; "67")]
#[test_case(r#####"tw`-m-64`"#####, r#####"({
  margin: "-16rem",
})
;"##### ; "68")]
#[test_case(r#####"tw`-m-72`"#####, r#####"({
  margin: "-18rem",
})
;"##### ; "69")]
#[test_case(r#####"tw`-m-80`"#####, r#####"({
  margin: "-20rem",
})
;"##### ; "70")]
#[test_case(r#####"tw`-m-96`"#####, r#####"({
  margin: "-24rem",
})
;"##### ; "71")]
#[test_case(r#####"tw`my-0`"#####, r#####"({
  marginTop: "0px",
  marginBottom: "0px",
})
;"##### ; "72")]
#[test_case(r#####"tw`my-px`"#####, r#####"({
  marginTop: "1px",
  marginBottom: "1px",
})
;"##### ; "73")]
#[test_case(r#####"tw`my-0.5`"#####, r#####"({
  marginTop: "0.125rem",
  marginBottom: "0.125rem",
})
;"##### ; "74")]
#[test_case(r#####"tw`my-1`"#####, r#####"({
  marginTop: "0.25rem",
  marginBottom: "0.25rem",
})
;"##### ; "75")]
#[test_case(r#####"tw`my-1.5`"#####, r#####"({
  marginTop: "0.375rem",
  marginBottom: "0.375rem",
})
;"##### ; "76")]
#[test_case(r#####"tw`my-2`"#####, r#####"({
  marginTop: "0.5rem",
  marginBottom: "0.5rem",
})
;"##### ; "77")]
#[test_case(r#####"tw`my-2.5`"#####, r#####"({
  marginTop: "0.625rem",
  marginBottom: "0.625rem",
})
;"##### ; "78")]
#[test_case(r#####"tw`my-3`"#####, r#####"({
  marginTop: "0.75rem",
  marginBottom: "0.75rem",
})
;"##### ; "79")]
#[test_case(r#####"tw`my-3.5`"#####, r#####"({
  marginTop: "0.875rem",
  marginBottom: "0.875rem",
})
;"##### ; "80")]
#[test_case(r#####"tw`my-4`"#####, r#####"({
  marginTop: "1rem",
  marginBottom: "1rem",
})
;"##### ; "81")]
#[test_case(r#####"tw`my-5`"#####, r#####"({
  marginTop: "1.25rem",
  marginBottom: "1.25rem",
})
;"##### ; "82")]
#[test_case(r#####"tw`my-6`"#####, r#####"({
  marginTop: "1.5rem",
  marginBottom: "1.5rem",
})
;"##### ; "83")]
#[test_case(r#####"tw`my-7`"#####, r#####"({
  marginTop: "1.75rem",
  marginBottom: "1.75rem",
})
;"##### ; "84")]
#[test_case(r#####"tw`my-8`"#####, r#####"({
  marginTop: "2rem",
  marginBottom: "2rem",
})
;"##### ; "85")]
#[test_case(r#####"tw`my-9`"#####, r#####"({
  marginTop: "2.25rem",
  marginBottom: "2.25rem",
})
;"##### ; "86")]
#[test_case(r#####"tw`my-10`"#####, r#####"({
  marginTop: "2.5rem",
  marginBottom: "2.5rem",
})
;"##### ; "87")]
#[test_case(r#####"tw`my-11`"#####, r#####"({
  marginTop: "2.75rem",
  marginBottom: "2.75rem",
})
;"##### ; "88")]
#[test_case(r#####"tw`my-12`"#####, r#####"({
  marginTop: "3rem",
  marginBottom: "3rem",
})
;"##### ; "89")]
#[test_case(r#####"tw`my-14`"#####, r#####"({
  marginTop: "3.5rem",
  marginBottom: "3.5rem",
})
;"##### ; "90")]
#[test_case(r#####"tw`my-16`"#####, r#####"({
  marginTop: "4rem",
  marginBottom: "4rem",
})
;"##### ; "91")]
#[test_case(r#####"tw`my-20`"#####, r#####"({
  marginTop: "5rem",
  marginBottom: "5rem",
})
;"##### ; "92")]
#[test_case(r#####"tw`my-24`"#####, r#####"({
  marginTop: "6rem",
  marginBottom: "6rem",
})
;"##### ; "93")]
#[test_case(r#####"tw`my-28`"#####, r#####"({
  marginTop: "7rem",
  marginBottom: "7rem",
})
;"##### ; "94")]
#[test_case(r#####"tw`my-32`"#####, r#####"({
  marginTop: "8rem",
  marginBottom: "8rem",
})
;"##### ; "95")]
#[test_case(r#####"tw`my-36`"#####, r#####"({
  marginTop: "9rem",
  marginBottom: "9rem",
})
;"##### ; "96")]
#[test_case(r#####"tw`my-40`"#####, r#####"({
  marginTop: "10rem",
  marginBottom: "10rem",
})
;"##### ; "97")]
#[test_case(r#####"tw`my-44`"#####, r#####"({
  marginTop: "11rem",
  marginBottom: "11rem",
})
;"##### ; "98")]
#[test_case(r#####"tw`my-48`"#####, r#####"({
  marginTop: "12rem",
  marginBottom: "12rem",
})
;"##### ; "99")]
#[test_case(r#####"tw`my-52`"#####, r#####"({
  marginTop: "13rem",
  marginBottom: "13rem",
})
;"##### ; "100")]
#[test_case(r#####"tw`my-56`"#####, r#####"({
  marginTop: "14rem",
  marginBottom: "14rem",
})
;"##### ; "101")]
#[test_case(r#####"tw`my-60`"#####, r#####"({
  marginTop: "15rem",
  marginBottom: "15rem",
})
;"##### ; "102")]
#[test_case(r#####"tw`my-64`"#####, r#####"({
  marginTop: "16rem",
  marginBottom: "16rem",
})
;"##### ; "103")]
#[test_case(r#####"tw`my-72`"#####, r#####"({
  marginTop: "18rem",
  marginBottom: "18rem",
})
;"##### ; "104")]
#[test_case(r#####"tw`my-80`"#####, r#####"({
  marginTop: "20rem",
  marginBottom: "20rem",
})
;"##### ; "105")]
#[test_case(r#####"tw`my-96`"#####, r#####"({
  marginTop: "24rem",
  marginBottom: "24rem",
})
;"##### ; "106")]
#[test_case(r#####"tw`my-auto`"#####, r#####"({
  marginTop: "auto",
  marginBottom: "auto",
})
;"##### ; "107")]
#[test_case(r#####"tw`-my-0`"#####, r#####"({
  marginTop: "-0px",
  marginBottom: "-0px",
})
;"##### ; "108")]
#[test_case(r#####"tw`-my-px`"#####, r#####"({
  marginTop: "-1px",
  marginBottom: "-1px",
})
;"##### ; "109")]
#[test_case(r#####"tw`-my-0.5`"#####, r#####"({
  marginTop: "-0.125rem",
  marginBottom: "-0.125rem",
})
;"##### ; "110")]
#[test_case(r#####"tw`-my-1`"#####, r#####"({
  marginTop: "-0.25rem",
  marginBottom: "-0.25rem",
})
;"##### ; "111")]
#[test_case(r#####"tw`-my-1.5`"#####, r#####"({
  marginTop: "-0.375rem",
  marginBottom: "-0.375rem",
})
;"##### ; "112")]
#[test_case(r#####"tw`-my-2`"#####, r#####"({
  marginTop: "-0.5rem",
  marginBottom: "-0.5rem",
})
;"##### ; "113")]
#[test_case(r#####"tw`-my-2.5`"#####, r#####"({
  marginTop: "-0.625rem",
  marginBottom: "-0.625rem",
})
;"##### ; "114")]
#[test_case(r#####"tw`-my-3`"#####, r#####"({
  marginTop: "-0.75rem",
  marginBottom: "-0.75rem",
})
;"##### ; "115")]
#[test_case(r#####"tw`-my-3.5`"#####, r#####"({
  marginTop: "-0.875rem",
  marginBottom: "-0.875rem",
})
;"##### ; "116")]
#[test_case(r#####"tw`-my-4`"#####, r#####"({
  marginTop: "-1rem",
  marginBottom: "-1rem",
})
;"##### ; "117")]
#[test_case(r#####"tw`-my-5`"#####, r#####"({
  marginTop: "-1.25rem",
  marginBottom: "-1.25rem",
})
;"##### ; "118")]
#[test_case(r#####"tw`-my-6`"#####, r#####"({
  marginTop: "-1.5rem",
  marginBottom: "-1.5rem",
})
;"##### ; "119")]
#[test_case(r#####"tw`-my-7`"#####, r#####"({
  marginTop: "-1.75rem",
  marginBottom: "-1.75rem",
})
;"##### ; "120")]
#[test_case(r#####"tw`-my-8`"#####, r#####"({
  marginTop: "-2rem",
  marginBottom: "-2rem",
})
;"##### ; "121")]
#[test_case(r#####"tw`-my-9`"#####, r#####"({
  marginTop: "-2.25rem",
  marginBottom: "-2.25rem",
})
;"##### ; "122")]
#[test_case(r#####"tw`-my-10`"#####, r#####"({
  marginTop: "-2.5rem",
  marginBottom: "-2.5rem",
})
;"##### ; "123")]
#[test_case(r#####"tw`-my-11`"#####, r#####"({
  marginTop: "-2.75rem",
  marginBottom: "-2.75rem",
})
;"##### ; "124")]
#[test_case(r#####"tw`-my-12`"#####, r#####"({
  marginTop: "-3rem",
  marginBottom: "-3rem",
})
;"##### ; "125")]
#[test_case(r#####"tw`-my-14`"#####, r#####"({
  marginTop: "-3.5rem",
  marginBottom: "-3.5rem",
})
;"##### ; "126")]
#[test_case(r#####"tw`-my-16`"#####, r#####"({
  marginTop: "-4rem",
  marginBottom: "-4rem",
})
;"##### ; "127")]
#[test_case(r#####"tw`-my-20`"#####, r#####"({
  marginTop: "-5rem",
  marginBottom: "-5rem",
})
;"##### ; "128")]
#[test_case(r#####"tw`-my-24`"#####, r#####"({
  marginTop: "-6rem",
  marginBottom: "-6rem",
})
;"##### ; "129")]
#[test_case(r#####"tw`-my-28`"#####, r#####"({
  marginTop: "-7rem",
  marginBottom: "-7rem",
})
;"##### ; "130")]
#[test_case(r#####"tw`-my-32`"#####, r#####"({
  marginTop: "-8rem",
  marginBottom: "-8rem",
})
;"##### ; "131")]
#[test_case(r#####"tw`-my-36`"#####, r#####"({
  marginTop: "-9rem",
  marginBottom: "-9rem",
})
;"##### ; "132")]
#[test_case(r#####"tw`-my-40`"#####, r#####"({
  marginTop: "-10rem",
  marginBottom: "-10rem",
})
;"##### ; "133")]
#[test_case(r#####"tw`-my-44`"#####, r#####"({
  marginTop: "-11rem",
  marginBottom: "-11rem",
})
;"##### ; "134")]
#[test_case(r#####"tw`-my-48`"#####, r#####"({
  marginTop: "-12rem",
  marginBottom: "-12rem",
})
;"##### ; "135")]
#[test_case(r#####"tw`-my-52`"#####, r#####"({
  marginTop: "-13rem",
  marginBottom: "-13rem",
})
;"##### ; "136")]
#[test_case(r#####"tw`-my-56`"#####, r#####"({
  marginTop: "-14rem",
  marginBottom: "-14rem",
})
;"##### ; "137")]
#[test_case(r#####"tw`-my-60`"#####, r#####"({
  marginTop: "-15rem",
  marginBottom: "-15rem",
})
;"##### ; "138")]
#[test_case(r#####"tw`-my-64`"#####, r#####"({
  marginTop: "-16rem",
  marginBottom: "-16rem",
})
;"##### ; "139")]
#[test_case(r#####"tw`-my-72`"#####, r#####"({
  marginTop: "-18rem",
  marginBottom: "-18rem",
})
;"##### ; "140")]
#[test_case(r#####"tw`-my-80`"#####, r#####"({
  marginTop: "-20rem",
  marginBottom: "-20rem",
})
;"##### ; "141")]
#[test_case(r#####"tw`-my-96`"#####, r#####"({
  marginTop: "-24rem",
  marginBottom: "-24rem",
})
;"##### ; "142")]
#[test_case(r#####"tw`mx-0`"#####, r#####"({
  marginLeft: "0px",
  marginRight: "0px",
})
;"##### ; "143")]
#[test_case(r#####"tw`mx-px`"#####, r#####"({
  marginLeft: "1px",
  marginRight: "1px",
})
;"##### ; "144")]
#[test_case(r#####"tw`mx-0.5`"#####, r#####"({
  marginLeft: "0.125rem",
  marginRight: "0.125rem",
})
;"##### ; "145")]
#[test_case(r#####"tw`mx-1`"#####, r#####"({
  marginLeft: "0.25rem",
  marginRight: "0.25rem",
})
;"##### ; "146")]
#[test_case(r#####"tw`mx-1.5`"#####, r#####"({
  marginLeft: "0.375rem",
  marginRight: "0.375rem",
})
;"##### ; "147")]
#[test_case(r#####"tw`mx-2`"#####, r#####"({
  marginLeft: "0.5rem",
  marginRight: "0.5rem",
})
;"##### ; "148")]
#[test_case(r#####"tw`mx-2.5`"#####, r#####"({
  marginLeft: "0.625rem",
  marginRight: "0.625rem",
})
;"##### ; "149")]
#[test_case(r#####"tw`mx-3`"#####, r#####"({
  marginLeft: "0.75rem",
  marginRight: "0.75rem",
})
;"##### ; "150")]
#[test_case(r#####"tw`mx-3.5`"#####, r#####"({
  marginLeft: "0.875rem",
  marginRight: "0.875rem",
})
;"##### ; "151")]
#[test_case(r#####"tw`mx-4`"#####, r#####"({
  marginLeft: "1rem",
  marginRight: "1rem",
})
;"##### ; "152")]
#[test_case(r#####"tw`mx-5`"#####, r#####"({
  marginLeft: "1.25rem",
  marginRight: "1.25rem",
})
;"##### ; "153")]
#[test_case(r#####"tw`mx-6`"#####, r#####"({
  marginLeft: "1.5rem",
  marginRight: "1.5rem",
})
;"##### ; "154")]
#[test_case(r#####"tw`mx-7`"#####, r#####"({
  marginLeft: "1.75rem",
  marginRight: "1.75rem",
})
;"##### ; "155")]
#[test_case(r#####"tw`mx-8`"#####, r#####"({
  marginLeft: "2rem",
  marginRight: "2rem",
})
;"##### ; "156")]
#[test_case(r#####"tw`mx-9`"#####, r#####"({
  marginLeft: "2.25rem",
  marginRight: "2.25rem",
})
;"##### ; "157")]
#[test_case(r#####"tw`mx-10`"#####, r#####"({
  marginLeft: "2.5rem",
  marginRight: "2.5rem",
})
;"##### ; "158")]
#[test_case(r#####"tw`mx-12`"#####, r#####"({
  marginLeft: "3rem",
  marginRight: "3rem",
})
;"##### ; "159")]
#[test_case(r#####"tw`mx-14`"#####, r#####"({
  marginLeft: "3.5rem",
  marginRight: "3.5rem",
})
;"##### ; "160")]
#[test_case(r#####"tw`mx-16`"#####, r#####"({
  marginLeft: "4rem",
  marginRight: "4rem",
})
;"##### ; "161")]
#[test_case(r#####"tw`mx-20`"#####, r#####"({
  marginLeft: "5rem",
  marginRight: "5rem",
})
;"##### ; "162")]
#[test_case(r#####"tw`mx-24`"#####, r#####"({
  marginLeft: "6rem",
  marginRight: "6rem",
})
;"##### ; "163")]
#[test_case(r#####"tw`mx-28`"#####, r#####"({
  marginLeft: "7rem",
  marginRight: "7rem",
})
;"##### ; "164")]
#[test_case(r#####"tw`mx-32`"#####, r#####"({
  marginLeft: "8rem",
  marginRight: "8rem",
})
;"##### ; "165")]
#[test_case(r#####"tw`mx-36`"#####, r#####"({
  marginLeft: "9rem",
  marginRight: "9rem",
})
;"##### ; "166")]
#[test_case(r#####"tw`mx-40`"#####, r#####"({
  marginLeft: "10rem",
  marginRight: "10rem",
})
;"##### ; "167")]
#[test_case(r#####"tw`mx-44`"#####, r#####"({
  marginLeft: "11rem",
  marginRight: "11rem",
})
;"##### ; "168")]
#[test_case(r#####"tw`mx-48`"#####, r#####"({
  marginLeft: "12rem",
  marginRight: "12rem",
})
;"##### ; "169")]
#[test_case(r#####"tw`mx-52`"#####, r#####"({
  marginLeft: "13rem",
  marginRight: "13rem",
})
;"##### ; "170")]
#[test_case(r#####"tw`mx-56`"#####, r#####"({
  marginLeft: "14rem",
  marginRight: "14rem",
})
;"##### ; "171")]
#[test_case(r#####"tw`mx-60`"#####, r#####"({
  marginLeft: "15rem",
  marginRight: "15rem",
})
;"##### ; "172")]
#[test_case(r#####"tw`mx-64`"#####, r#####"({
  marginLeft: "16rem",
  marginRight: "16rem",
})
;"##### ; "173")]
#[test_case(r#####"tw`mx-72`"#####, r#####"({
  marginLeft: "18rem",
  marginRight: "18rem",
})
;"##### ; "174")]
#[test_case(r#####"tw`mx-80`"#####, r#####"({
  marginLeft: "20rem",
  marginRight: "20rem",
})
;"##### ; "175")]
#[test_case(r#####"tw`mx-auto`"#####, r#####"({
  marginLeft: "auto",
  marginRight: "auto",
})
;"##### ; "176")]
#[test_case(r#####"tw`-mx-0`"#####, r#####"({
  marginLeft: "-0px",
  marginRight: "-0px",
})
;"##### ; "177")]
#[test_case(r#####"tw`-mx-px`"#####, r#####"({
  marginLeft: "-1px",
  marginRight: "-1px",
})
;"##### ; "178")]
#[test_case(r#####"tw`-mx-0.5`"#####, r#####"({
  marginLeft: "-0.125rem",
  marginRight: "-0.125rem",
})
;"##### ; "179")]
#[test_case(r#####"tw`-mx-1`"#####, r#####"({
  marginLeft: "-0.25rem",
  marginRight: "-0.25rem",
})
;"##### ; "180")]
#[test_case(r#####"tw`-mx-1.5`"#####, r#####"({
  marginLeft: "-0.375rem",
  marginRight: "-0.375rem",
})
;"##### ; "181")]
#[test_case(r#####"tw`-mx-2`"#####, r#####"({
  marginLeft: "-0.5rem",
  marginRight: "-0.5rem",
})
;"##### ; "182")]
#[test_case(r#####"tw`-mx-2.5`"#####, r#####"({
  marginLeft: "-0.625rem",
  marginRight: "-0.625rem",
})
;"##### ; "183")]
#[test_case(r#####"tw`-mx-3`"#####, r#####"({
  marginLeft: "-0.75rem",
  marginRight: "-0.75rem",
})
;"##### ; "184")]
#[test_case(r#####"tw`-mx-3.5`"#####, r#####"({
  marginLeft: "-0.875rem",
  marginRight: "-0.875rem",
})
;"##### ; "185")]
#[test_case(r#####"tw`-mx-4`"#####, r#####"({
  marginLeft: "-1rem",
  marginRight: "-1rem",
})
;"##### ; "186")]
#[test_case(r#####"tw`-mx-5`"#####, r#####"({
  marginLeft: "-1.25rem",
  marginRight: "-1.25rem",
})
;"##### ; "187")]
#[test_case(r#####"tw`-mx-6`"#####, r#####"({
  marginLeft: "-1.5rem",
  marginRight: "-1.5rem",
})
;"##### ; "188")]
#[test_case(r#####"tw`-mx-7`"#####, r#####"({
  marginLeft: "-1.75rem",
  marginRight: "-1.75rem",
})
;"##### ; "189")]
#[test_case(r#####"tw`-mx-8`"#####, r#####"({
  marginLeft: "-2rem",
  marginRight: "-2rem",
})
;"##### ; "190")]
#[test_case(r#####"tw`-mx-9`"#####, r#####"({
  marginLeft: "-2.25rem",
  marginRight: "-2.25rem",
})
;"##### ; "191")]
#[test_case(r#####"tw`-mx-10`"#####, r#####"({
  marginLeft: "-2.5rem",
  marginRight: "-2.5rem",
})
;"##### ; "192")]
#[test_case(r#####"tw`-mx-11`"#####, r#####"({
  marginLeft: "-2.75rem",
  marginRight: "-2.75rem",
})
;"##### ; "193")]
#[test_case(r#####"tw`-mx-12`"#####, r#####"({
  marginLeft: "-3rem",
  marginRight: "-3rem",
})
;"##### ; "194")]
#[test_case(r#####"tw`-mx-14`"#####, r#####"({
  marginLeft: "-3.5rem",
  marginRight: "-3.5rem",
})
;"##### ; "195")]
#[test_case(r#####"tw`-mx-16`"#####, r#####"({
  marginLeft: "-4rem",
  marginRight: "-4rem",
})
;"##### ; "196")]
#[test_case(r#####"tw`-mx-20`"#####, r#####"({
  marginLeft: "-5rem",
  marginRight: "-5rem",
})
;"##### ; "197")]
#[test_case(r#####"tw`-mx-24`"#####, r#####"({
  marginLeft: "-6rem",
  marginRight: "-6rem",
})
;"##### ; "198")]
#[test_case(r#####"tw`-mx-28`"#####, r#####"({
  marginLeft: "-7rem",
  marginRight: "-7rem",
})
;"##### ; "199")]
#[test_case(r#####"tw`-mx-32`"#####, r#####"({
  marginLeft: "-8rem",
  marginRight: "-8rem",
})
;"##### ; "200")]
#[test_case(r#####"tw`-mx-36`"#####, r#####"({
  marginLeft: "-9rem",
  marginRight: "-9rem",
})
;"##### ; "201")]
#[test_case(r#####"tw`-mx-40`"#####, r#####"({
  marginLeft: "-10rem",
  marginRight: "-10rem",
})
;"##### ; "202")]
#[test_case(r#####"tw`-mx-44`"#####, r#####"({
  marginLeft: "-11rem",
  marginRight: "-11rem",
})
;"##### ; "203")]
#[test_case(r#####"tw`-mx-48`"#####, r#####"({
  marginLeft: "-12rem",
  marginRight: "-12rem",
})
;"##### ; "204")]
#[test_case(r#####"tw`-mx-52`"#####, r#####"({
  marginLeft: "-13rem",
  marginRight: "-13rem",
})
;"##### ; "205")]
#[test_case(r#####"tw`-mx-56`"#####, r#####"({
  marginLeft: "-14rem",
  marginRight: "-14rem",
})
;"##### ; "206")]
#[test_case(r#####"tw`-mx-60`"#####, r#####"({
  marginLeft: "-15rem",
  marginRight: "-15rem",
})
;"##### ; "207")]
#[test_case(r#####"tw`-mx-64`"#####, r#####"({
  marginLeft: "-16rem",
  marginRight: "-16rem",
})
;"##### ; "208")]
#[test_case(r#####"tw`-mx-72`"#####, r#####"({
  marginLeft: "-18rem",
  marginRight: "-18rem",
})
;"##### ; "209")]
#[test_case(r#####"tw`-mx-80`"#####, r#####"({
  marginLeft: "-20rem",
  marginRight: "-20rem",
})
;"##### ; "210")]
#[test_case(r#####"tw`-mx-96`"#####, r#####"({
  marginLeft: "-24rem",
  marginRight: "-24rem",
})
;"##### ; "211")]
#[test_case(r#####"tw`mt-0`"#####, r#####"({
  marginTop: "0px",
})
;"##### ; "212")]
#[test_case(r#####"tw`mt-px`"#####, r#####"({
  marginTop: "1px",
})
;"##### ; "213")]
#[test_case(r#####"tw`mt-0.5`"#####, r#####"({
  marginTop: "0.125rem",
})
;"##### ; "214")]
#[test_case(r#####"tw`mt-1`"#####, r#####"({
  marginTop: "0.25rem",
})
;"##### ; "215")]
#[test_case(r#####"tw`mt-1.5`"#####, r#####"({
  marginTop: "0.375rem",
})
;"##### ; "216")]
#[test_case(r#####"tw`mt-2`"#####, r#####"({
  marginTop: "0.5rem",
})
;"##### ; "217")]
#[test_case(r#####"tw`mt-2.5`"#####, r#####"({
  marginTop: "0.625rem",
})
;"##### ; "218")]
#[test_case(r#####"tw`mt-3`"#####, r#####"({
  marginTop: "0.75rem",
})
;"##### ; "219")]
#[test_case(r#####"tw`mt-3.5`"#####, r#####"({
  marginTop: "0.875rem",
})
;"##### ; "220")]
#[test_case(r#####"tw`mt-4`"#####, r#####"({
  marginTop: "1rem",
})
;"##### ; "221")]
#[test_case(r#####"tw`mt-5`"#####, r#####"({
  marginTop: "1.25rem",
})
;"##### ; "222")]
#[test_case(r#####"tw`mt-6`"#####, r#####"({
  marginTop: "1.5rem",
})
;"##### ; "223")]
#[test_case(r#####"tw`mt-7`"#####, r#####"({
  marginTop: "1.75rem",
})
;"##### ; "224")]
#[test_case(r#####"tw`mt-8`"#####, r#####"({
  marginTop: "2rem",
})
;"##### ; "225")]
#[test_case(r#####"tw`mt-9`"#####, r#####"({
  marginTop: "2.25rem",
})
;"##### ; "226")]
#[test_case(r#####"tw`mt-10`"#####, r#####"({
  marginTop: "2.5rem",
})
;"##### ; "227")]
#[test_case(r#####"tw`mt-11`"#####, r#####"({
  marginTop: "2.75rem",
})
;"##### ; "228")]
#[test_case(r#####"tw`mt-12`"#####, r#####"({
  marginTop: "3rem",
})
;"##### ; "229")]
#[test_case(r#####"tw`mt-14`"#####, r#####"({
  marginTop: "3.5rem",
})
;"##### ; "230")]
#[test_case(r#####"tw`mt-16`"#####, r#####"({
  marginTop: "4rem",
})
;"##### ; "231")]
#[test_case(r#####"tw`mt-20`"#####, r#####"({
  marginTop: "5rem",
})
;"##### ; "232")]
#[test_case(r#####"tw`mt-24`"#####, r#####"({
  marginTop: "6rem",
})
;"##### ; "233")]
#[test_case(r#####"tw`mt-28`"#####, r#####"({
  marginTop: "7rem",
})
;"##### ; "234")]
#[test_case(r#####"tw`mt-32`"#####, r#####"({
  marginTop: "8rem",
})
;"##### ; "235")]
#[test_case(r#####"tw`mt-36`"#####, r#####"({
  marginTop: "9rem",
})
;"##### ; "236")]
#[test_case(r#####"tw`mt-40`"#####, r#####"({
  marginTop: "10rem",
})
;"##### ; "237")]
#[test_case(r#####"tw`mt-44`"#####, r#####"({
  marginTop: "11rem",
})
;"##### ; "238")]
#[test_case(r#####"tw`mt-48`"#####, r#####"({
  marginTop: "12rem",
})
;"##### ; "239")]
#[test_case(r#####"tw`mt-52`"#####, r#####"({
  marginTop: "13rem",
})
;"##### ; "240")]
#[test_case(r#####"tw`mt-56`"#####, r#####"({
  marginTop: "14rem",
})
;"##### ; "241")]
#[test_case(r#####"tw`mt-60`"#####, r#####"({
  marginTop: "15rem",
})
;"##### ; "242")]
#[test_case(r#####"tw`mt-64`"#####, r#####"({
  marginTop: "16rem",
})
;"##### ; "243")]
#[test_case(r#####"tw`mt-72`"#####, r#####"({
  marginTop: "18rem",
})
;"##### ; "244")]
#[test_case(r#####"tw`mt-80`"#####, r#####"({
  marginTop: "20rem",
})
;"##### ; "245")]
#[test_case(r#####"tw`mt-96`"#####, r#####"({
  marginTop: "24rem",
})
;"##### ; "246")]
#[test_case(r#####"tw`mt-auto`"#####, r#####"({
  marginTop: "auto",
})
;"##### ; "247")]
#[test_case(r#####"tw`-mt-0`"#####, r#####"({
  marginTop: "-0px",
})
;"##### ; "248")]
#[test_case(r#####"tw`-mt-px`"#####, r#####"({
  marginTop: "-1px",
})
;"##### ; "249")]
#[test_case(r#####"tw`-mt-0.5`"#####, r#####"({
  marginTop: "-0.125rem",
})
;"##### ; "250")]
#[test_case(r#####"tw`-mt-1`"#####, r#####"({
  marginTop: "-0.25rem",
})
;"##### ; "251")]
#[test_case(r#####"tw`-mt-1.5`"#####, r#####"({
  marginTop: "-0.375rem",
})
;"##### ; "252")]
#[test_case(r#####"tw`-mt-2`"#####, r#####"({
  marginTop: "-0.5rem",
})
;"##### ; "253")]
#[test_case(r#####"tw`-mt-2.5`"#####, r#####"({
  marginTop: "-0.625rem",
})
;"##### ; "254")]
#[test_case(r#####"tw`-mt-3`"#####, r#####"({
  marginTop: "-0.75rem",
})
;"##### ; "255")]
#[test_case(r#####"tw`-mt-3.5`"#####, r#####"({
  marginTop: "-0.875rem",
})
;"##### ; "256")]
#[test_case(r#####"tw`-mt-4`"#####, r#####"({
  marginTop: "-1rem",
})
;"##### ; "257")]
#[test_case(r#####"tw`-mt-5`"#####, r#####"({
  marginTop: "-1.25rem",
})
;"##### ; "258")]
#[test_case(r#####"tw`-mt-6`"#####, r#####"({
  marginTop: "-1.5rem",
})
;"##### ; "259")]
#[test_case(r#####"tw`-mt-7`"#####, r#####"({
  marginTop: "-1.75rem",
})
;"##### ; "260")]
#[test_case(r#####"tw`-mt-8`"#####, r#####"({
  marginTop: "-2rem",
})
;"##### ; "261")]
#[test_case(r#####"tw`-mt-9`"#####, r#####"({
  marginTop: "-2.25rem",
})
;"##### ; "262")]
#[test_case(r#####"tw`-mt-10`"#####, r#####"({
  marginTop: "-2.5rem",
})
;"##### ; "263")]
#[test_case(r#####"tw`-mt-11`"#####, r#####"({
  marginTop: "-2.75rem",
})
;"##### ; "264")]
#[test_case(r#####"tw`-mt-12`"#####, r#####"({
  marginTop: "-3rem",
})
;"##### ; "265")]
#[test_case(r#####"tw`-mt-14`"#####, r#####"({
  marginTop: "-3.5rem",
})
;"##### ; "266")]
#[test_case(r#####"tw`-mt-16`"#####, r#####"({
  marginTop: "-4rem",
})
;"##### ; "267")]
#[test_case(r#####"tw`-mt-20`"#####, r#####"({
  marginTop: "-5rem",
})
;"##### ; "268")]
#[test_case(r#####"tw`-mt-24`"#####, r#####"({
  marginTop: "-6rem",
})
;"##### ; "269")]
#[test_case(r#####"tw`-mt-28`"#####, r#####"({
  marginTop: "-7rem",
})
;"##### ; "270")]
#[test_case(r#####"tw`-mt-32`"#####, r#####"({
  marginTop: "-8rem",
})
;"##### ; "271")]
#[test_case(r#####"tw`-mt-36`"#####, r#####"({
  marginTop: "-9rem",
})
;"##### ; "272")]
#[test_case(r#####"tw`-mt-40`"#####, r#####"({
  marginTop: "-10rem",
})
;"##### ; "273")]
#[test_case(r#####"tw`-mt-44`"#####, r#####"({
  marginTop: "-11rem",
})
;"##### ; "274")]
#[test_case(r#####"tw`-mt-48`"#####, r#####"({
  marginTop: "-12rem",
})
;"##### ; "275")]
#[test_case(r#####"tw`-mt-52`"#####, r#####"({
  marginTop: "-13rem",
})
;"##### ; "276")]
#[test_case(r#####"tw`-mt-56`"#####, r#####"({
  marginTop: "-14rem",
})
;"##### ; "277")]
#[test_case(r#####"tw`-mt-60`"#####, r#####"({
  marginTop: "-15rem",
})
;"##### ; "278")]
#[test_case(r#####"tw`-mt-64`"#####, r#####"({
  marginTop: "-16rem",
})
;"##### ; "279")]
#[test_case(r#####"tw`-mt-72`"#####, r#####"({
  marginTop: "-18rem",
})
;"##### ; "280")]
#[test_case(r#####"tw`-mt-80`"#####, r#####"({
  marginTop: "-20rem",
})
;"##### ; "281")]
#[test_case(r#####"tw`-mt-96`"#####, r#####"({
  marginTop: "-24rem",
})
;"##### ; "282")]
#[test_case(r#####"tw`mr-0`"#####, r#####"({
  marginRight: "0px",
})
;"##### ; "283")]
#[test_case(r#####"tw`mr-px`"#####, r#####"({
  marginRight: "1px",
})
;"##### ; "284")]
#[test_case(r#####"tw`mr-0.5`"#####, r#####"({
  marginRight: "0.125rem",
})
;"##### ; "285")]
#[test_case(r#####"tw`mr-1`"#####, r#####"({
  marginRight: "0.25rem",
})
;"##### ; "286")]
#[test_case(r#####"tw`mr-1.5`"#####, r#####"({
  marginRight: "0.375rem",
})
;"##### ; "287")]
#[test_case(r#####"tw`mr-2`"#####, r#####"({
  marginRight: "0.5rem",
})
;"##### ; "288")]
#[test_case(r#####"tw`mr-2.5`"#####, r#####"({
  marginRight: "0.625rem",
})
;"##### ; "289")]
#[test_case(r#####"tw`mr-3`"#####, r#####"({
  marginRight: "0.75rem",
})
;"##### ; "290")]
#[test_case(r#####"tw`mr-3.5`"#####, r#####"({
  marginRight: "0.875rem",
})
;"##### ; "291")]
#[test_case(r#####"tw`mr-4`"#####, r#####"({
  marginRight: "1rem",
})
;"##### ; "292")]
#[test_case(r#####"tw`mr-5`"#####, r#####"({
  marginRight: "1.25rem",
})
;"##### ; "293")]
#[test_case(r#####"tw`mr-6`"#####, r#####"({
  marginRight: "1.5rem",
})
;"##### ; "294")]
#[test_case(r#####"tw`mr-7`"#####, r#####"({
  marginRight: "1.75rem",
})
;"##### ; "295")]
#[test_case(r#####"tw`mr-8`"#####, r#####"({
  marginRight: "2rem",
})
;"##### ; "296")]
#[test_case(r#####"tw`mr-9`"#####, r#####"({
  marginRight: "2.25rem",
})
;"##### ; "297")]
#[test_case(r#####"tw`mr-10`"#####, r#####"({
  marginRight: "2.5rem",
})
;"##### ; "298")]
#[test_case(r#####"tw`mr-11`"#####, r#####"({
  marginRight: "2.75rem",
})
;"##### ; "299")]
#[test_case(r#####"tw`mr-12`"#####, r#####"({
  marginRight: "3rem",
})
;"##### ; "300")]
#[test_case(r#####"tw`mr-14`"#####, r#####"({
  marginRight: "3.5rem",
})
;"##### ; "301")]
#[test_case(r#####"tw`mr-16`"#####, r#####"({
  marginRight: "4rem",
})
;"##### ; "302")]
#[test_case(r#####"tw`mr-20`"#####, r#####"({
  marginRight: "5rem",
})
;"##### ; "303")]
#[test_case(r#####"tw`mr-24`"#####, r#####"({
  marginRight: "6rem",
})
;"##### ; "304")]
#[test_case(r#####"tw`mr-28`"#####, r#####"({
  marginRight: "7rem",
})
;"##### ; "305")]
#[test_case(r#####"tw`mr-32`"#####, r#####"({
  marginRight: "8rem",
})
;"##### ; "306")]
#[test_case(r#####"tw`mr-36`"#####, r#####"({
  marginRight: "9rem",
})
;"##### ; "307")]
#[test_case(r#####"tw`mr-40`"#####, r#####"({
  marginRight: "10rem",
})
;"##### ; "308")]
#[test_case(r#####"tw`mr-44`"#####, r#####"({
  marginRight: "11rem",
})
;"##### ; "309")]
#[test_case(r#####"tw`mr-48`"#####, r#####"({
  marginRight: "12rem",
})
;"##### ; "310")]
#[test_case(r#####"tw`mr-52`"#####, r#####"({
  marginRight: "13rem",
})
;"##### ; "311")]
#[test_case(r#####"tw`mr-56`"#####, r#####"({
  marginRight: "14rem",
})
;"##### ; "312")]
#[test_case(r#####"tw`mr-60`"#####, r#####"({
  marginRight: "15rem",
})
;"##### ; "313")]
#[test_case(r#####"tw`mr-64`"#####, r#####"({
  marginRight: "16rem",
})
;"##### ; "314")]
#[test_case(r#####"tw`mr-72`"#####, r#####"({
  marginRight: "18rem",
})
;"##### ; "315")]
#[test_case(r#####"tw`mr-80`"#####, r#####"({
  marginRight: "20rem",
})
;"##### ; "316")]
#[test_case(r#####"tw`mr-96`"#####, r#####"({
  marginRight: "24rem",
})
;"##### ; "317")]
#[test_case(r#####"tw`mr-auto`"#####, r#####"({
  marginRight: "auto",
})
;"##### ; "318")]
#[test_case(r#####"tw`-mr-0`"#####, r#####"({
  marginRight: "-0px",
})
;"##### ; "319")]
#[test_case(r#####"tw`-mr-px`"#####, r#####"({
  marginRight: "-1px",
})
;"##### ; "320")]
#[test_case(r#####"tw`-mr-0.5`"#####, r#####"({
  marginRight: "-0.125rem",
})
;"##### ; "321")]
#[test_case(r#####"tw`-mr-1`"#####, r#####"({
  marginRight: "-0.25rem",
})
;"##### ; "322")]
#[test_case(r#####"tw`-mr-1.5`"#####, r#####"({
  marginRight: "-0.375rem",
})
;"##### ; "323")]
#[test_case(r#####"tw`-mr-2`"#####, r#####"({
  marginRight: "-0.5rem",
})
;"##### ; "324")]
#[test_case(r#####"tw`-mr-2.5`"#####, r#####"({
  marginRight: "-0.625rem",
})
;"##### ; "325")]
#[test_case(r#####"tw`-mr-3`"#####, r#####"({
  marginRight: "-0.75rem",
})
;"##### ; "326")]
#[test_case(r#####"tw`-mr-3.5`"#####, r#####"({
  marginRight: "-0.875rem",
})
;"##### ; "327")]
#[test_case(r#####"tw`-mr-4`"#####, r#####"({
  marginRight: "-1rem",
})
;"##### ; "328")]
#[test_case(r#####"tw`-mr-5`"#####, r#####"({
  marginRight: "-1.25rem",
})
;"##### ; "329")]
#[test_case(r#####"tw`-mr-6`"#####, r#####"({
  marginRight: "-1.5rem",
})
;"##### ; "330")]
#[test_case(r#####"tw`-mr-7`"#####, r#####"({
  marginRight: "-1.75rem",
})
;"##### ; "331")]
#[test_case(r#####"tw`-mr-8`"#####, r#####"({
  marginRight: "-2rem",
})
;"##### ; "332")]
#[test_case(r#####"tw`-mr-9`"#####, r#####"({
  marginRight: "-2.25rem",
})
;"##### ; "333")]
#[test_case(r#####"tw`-mr-10`"#####, r#####"({
  marginRight: "-2.5rem",
})
;"##### ; "334")]
#[test_case(r#####"tw`-mr-11`"#####, r#####"({
  marginRight: "-2.75rem",
})
;"##### ; "335")]
#[test_case(r#####"tw`-mr-12`"#####, r#####"({
  marginRight: "-3rem",
})
;"##### ; "336")]
#[test_case(r#####"tw`-mr-14`"#####, r#####"({
  marginRight: "-3.5rem",
})
;"##### ; "337")]
#[test_case(r#####"tw`-mr-16`"#####, r#####"({
  marginRight: "-4rem",
})
;"##### ; "338")]
#[test_case(r#####"tw`-mr-20`"#####, r#####"({
  marginRight: "-5rem",
})
;"##### ; "339")]
#[test_case(r#####"tw`-mr-24`"#####, r#####"({
  marginRight: "-6rem",
})
;"##### ; "340")]
#[test_case(r#####"tw`-mr-28`"#####, r#####"({
  marginRight: "-7rem",
})
;"##### ; "341")]
#[test_case(r#####"tw`-mr-32`"#####, r#####"({
  marginRight: "-8rem",
})
;"##### ; "342")]
#[test_case(r#####"tw`-mr-36`"#####, r#####"({
  marginRight: "-9rem",
})
;"##### ; "343")]
#[test_case(r#####"tw`-mr-40`"#####, r#####"({
  marginRight: "-10rem",
})
;"##### ; "344")]
#[test_case(r#####"tw`-mr-44`"#####, r#####"({
  marginRight: "-11rem",
})
;"##### ; "345")]
#[test_case(r#####"tw`-mr-48`"#####, r#####"({
  marginRight: "-12rem",
})
;"##### ; "346")]
#[test_case(r#####"tw`-mr-52`"#####, r#####"({
  marginRight: "-13rem",
})
;"##### ; "347")]
#[test_case(r#####"tw`-mr-56`"#####, r#####"({
  marginRight: "-14rem",
})
;"##### ; "348")]
#[test_case(r#####"tw`-mr-60`"#####, r#####"({
  marginRight: "-15rem",
})
;"##### ; "349")]
#[test_case(r#####"tw`-mr-64`"#####, r#####"({
  marginRight: "-16rem",
})
;"##### ; "350")]
#[test_case(r#####"tw`-mr-72`"#####, r#####"({
  marginRight: "-18rem",
})
;"##### ; "351")]
#[test_case(r#####"tw`-mr-80`"#####, r#####"({
  marginRight: "-20rem",
})
;"##### ; "352")]
#[test_case(r#####"tw`-mr-96`"#####, r#####"({
  marginRight: "-24rem",
})
;"##### ; "353")]
#[test_case(r#####"tw`mb-0`"#####, r#####"({
  marginBottom: "0px",
})
;"##### ; "354")]
#[test_case(r#####"tw`mb-px`"#####, r#####"({
  marginBottom: "1px",
})
;"##### ; "355")]
#[test_case(r#####"tw`mb-0.5`"#####, r#####"({
  marginBottom: "0.125rem",
})
;"##### ; "356")]
#[test_case(r#####"tw`mb-1`"#####, r#####"({
  marginBottom: "0.25rem",
})
;"##### ; "357")]
#[test_case(r#####"tw`mb-1.5`"#####, r#####"({
  marginBottom: "0.375rem",
})
;"##### ; "358")]
#[test_case(r#####"tw`mb-2`"#####, r#####"({
  marginBottom: "0.5rem",
})
;"##### ; "359")]
#[test_case(r#####"tw`mb-2.5`"#####, r#####"({
  marginBottom: "0.625rem",
})
;"##### ; "360")]
#[test_case(r#####"tw`mb-3`"#####, r#####"({
  marginBottom: "0.75rem",
})
;"##### ; "361")]
#[test_case(r#####"tw`mb-3.5`"#####, r#####"({
  marginBottom: "0.875rem",
})
;"##### ; "362")]
#[test_case(r#####"tw`mb-4`"#####, r#####"({
  marginBottom: "1rem",
})
;"##### ; "363")]
#[test_case(r#####"tw`mb-5`"#####, r#####"({
  marginBottom: "1.25rem",
})
;"##### ; "364")]
#[test_case(r#####"tw`mb-6`"#####, r#####"({
  marginBottom: "1.5rem",
})
;"##### ; "365")]
#[test_case(r#####"tw`mb-7`"#####, r#####"({
  marginBottom: "1.75rem",
})
;"##### ; "366")]
#[test_case(r#####"tw`mb-8`"#####, r#####"({
  marginBottom: "2rem",
})
;"##### ; "367")]
#[test_case(r#####"tw`mb-9`"#####, r#####"({
  marginBottom: "2.25rem",
})
;"##### ; "368")]
#[test_case(r#####"tw`mb-10`"#####, r#####"({
  marginBottom: "2.5rem",
})
;"##### ; "369")]
#[test_case(r#####"tw`mb-11`"#####, r#####"({
  marginBottom: "2.75rem",
})
;"##### ; "370")]
#[test_case(r#####"tw`mb-12`"#####, r#####"({
  marginBottom: "3rem",
})
;"##### ; "371")]
#[test_case(r#####"tw`mb-14`"#####, r#####"({
  marginBottom: "3.5rem",
})
;"##### ; "372")]
#[test_case(r#####"tw`mb-16`"#####, r#####"({
  marginBottom: "4rem",
})
;"##### ; "373")]
#[test_case(r#####"tw`mb-20`"#####, r#####"({
  marginBottom: "5rem",
})
;"##### ; "374")]
#[test_case(r#####"tw`mb-24`"#####, r#####"({
  marginBottom: "6rem",
})
;"##### ; "375")]
#[test_case(r#####"tw`mb-28`"#####, r#####"({
  marginBottom: "7rem",
})
;"##### ; "376")]
#[test_case(r#####"tw`mb-32`"#####, r#####"({
  marginBottom: "8rem",
})
;"##### ; "377")]
#[test_case(r#####"tw`mb-36`"#####, r#####"({
  marginBottom: "9rem",
})
;"##### ; "378")]
#[test_case(r#####"tw`mb-40`"#####, r#####"({
  marginBottom: "10rem",
})
;"##### ; "379")]
#[test_case(r#####"tw`mb-44`"#####, r#####"({
  marginBottom: "11rem",
})
;"##### ; "380")]
#[test_case(r#####"tw`mb-48`"#####, r#####"({
  marginBottom: "12rem",
})
;"##### ; "381")]
#[test_case(r#####"tw`mb-52`"#####, r#####"({
  marginBottom: "13rem",
})
;"##### ; "382")]
#[test_case(r#####"tw`mb-56`"#####, r#####"({
  marginBottom: "14rem",
})
;"##### ; "383")]
#[test_case(r#####"tw`mb-60`"#####, r#####"({
  marginBottom: "15rem",
})
;"##### ; "384")]
#[test_case(r#####"tw`mb-64`"#####, r#####"({
  marginBottom: "16rem",
})
;"##### ; "385")]
#[test_case(r#####"tw`mb-72`"#####, r#####"({
  marginBottom: "18rem",
})
;"##### ; "386")]
#[test_case(r#####"tw`mb-80`"#####, r#####"({
  marginBottom: "20rem",
})
;"##### ; "387")]
#[test_case(r#####"tw`mb-96`"#####, r#####"({
  marginBottom: "24rem",
})
;"##### ; "388")]
#[test_case(r#####"tw`mb-auto`"#####, r#####"({
  marginBottom: "auto",
})
;"##### ; "389")]
#[test_case(r#####"tw`-mb-0`"#####, r#####"({
  marginBottom: "-0px",
})
;"##### ; "390")]
#[test_case(r#####"tw`-mb-px`"#####, r#####"({
  marginBottom: "-1px",
})
;"##### ; "391")]
#[test_case(r#####"tw`-mb-0.5`"#####, r#####"({
  marginBottom: "-0.125rem",
})
;"##### ; "392")]
#[test_case(r#####"tw`-mb-1`"#####, r#####"({
  marginBottom: "-0.25rem",
})
;"##### ; "393")]
#[test_case(r#####"tw`-mb-1.5`"#####, r#####"({
  marginBottom: "-0.375rem",
})
;"##### ; "394")]
#[test_case(r#####"tw`-mb-2`"#####, r#####"({
  marginBottom: "-0.5rem",
})
;"##### ; "395")]
#[test_case(r#####"tw`-mb-2.5`"#####, r#####"({
  marginBottom: "-0.625rem",
})
;"##### ; "396")]
#[test_case(r#####"tw`-mb-3`"#####, r#####"({
  marginBottom: "-0.75rem",
})
;"##### ; "397")]
#[test_case(r#####"tw`-mb-3.5`"#####, r#####"({
  marginBottom: "-0.875rem",
})
;"##### ; "398")]
#[test_case(r#####"tw`-mb-4`"#####, r#####"({
  marginBottom: "-1rem",
})
;"##### ; "399")]
#[test_case(r#####"tw`-mb-5`"#####, r#####"({
  marginBottom: "-1.25rem",
})
;"##### ; "400")]
#[test_case(r#####"tw`-mb-6`"#####, r#####"({
  marginBottom: "-1.5rem",
})
;"##### ; "401")]
#[test_case(r#####"tw`-mb-7`"#####, r#####"({
  marginBottom: "-1.75rem",
})
;"##### ; "402")]
#[test_case(r#####"tw`-mb-8`"#####, r#####"({
  marginBottom: "-2rem",
})
;"##### ; "403")]
#[test_case(r#####"tw`-mb-9`"#####, r#####"({
  marginBottom: "-2.25rem",
})
;"##### ; "404")]
#[test_case(r#####"tw`-mb-10`"#####, r#####"({
  marginBottom: "-2.5rem",
})
;"##### ; "405")]
#[test_case(r#####"tw`-mb-11`"#####, r#####"({
  marginBottom: "-2.75rem",
})
;"##### ; "406")]
#[test_case(r#####"tw`-mb-12`"#####, r#####"({
  marginBottom: "-3rem",
})
;"##### ; "407")]
#[test_case(r#####"tw`-mb-14`"#####, r#####"({
  marginBottom: "-3.5rem",
})
;"##### ; "408")]
#[test_case(r#####"tw`-mb-16`"#####, r#####"({
  marginBottom: "-4rem",
})
;"##### ; "409")]
#[test_case(r#####"tw`-mb-20`"#####, r#####"({
  marginBottom: "-5rem",
})
;"##### ; "410")]
#[test_case(r#####"tw`-mb-24`"#####, r#####"({
  marginBottom: "-6rem",
})
;"##### ; "411")]
#[test_case(r#####"tw`-mb-28`"#####, r#####"({
  marginBottom: "-7rem",
})
;"##### ; "412")]
#[test_case(r#####"tw`-mb-32`"#####, r#####"({
  marginBottom: "-8rem",
})
;"##### ; "413")]
#[test_case(r#####"tw`-mb-36`"#####, r#####"({
  marginBottom: "-9rem",
})
;"##### ; "414")]
#[test_case(r#####"tw`-mb-40`"#####, r#####"({
  marginBottom: "-10rem",
})
;"##### ; "415")]
#[test_case(r#####"tw`-mb-44`"#####, r#####"({
  marginBottom: "-11rem",
})
;"##### ; "416")]
#[test_case(r#####"tw`-mb-48`"#####, r#####"({
  marginBottom: "-12rem",
})
;"##### ; "417")]
#[test_case(r#####"tw`-mb-52`"#####, r#####"({
  marginBottom: "-13rem",
})
;"##### ; "418")]
#[test_case(r#####"tw`-mb-56`"#####, r#####"({
  marginBottom: "-14rem",
})
;"##### ; "419")]
#[test_case(r#####"tw`-mb-60`"#####, r#####"({
  marginBottom: "-15rem",
})
;"##### ; "420")]
#[test_case(r#####"tw`-mb-64`"#####, r#####"({
  marginBottom: "-16rem",
})
;"##### ; "421")]
#[test_case(r#####"tw`-mb-72`"#####, r#####"({
  marginBottom: "-18rem",
})
;"##### ; "422")]
#[test_case(r#####"tw`-mb-80`"#####, r#####"({
  marginBottom: "-20rem",
})
;"##### ; "423")]
#[test_case(r#####"tw`-mb-96`"#####, r#####"({
  marginBottom: "-24rem",
})
;"##### ; "424")]
#[test_case(r#####"tw`ml-0`"#####, r#####"({
  marginLeft: "0px",
})
;"##### ; "425")]
#[test_case(r#####"tw`ml-px`"#####, r#####"({
  marginLeft: "1px",
})
;"##### ; "426")]
#[test_case(r#####"tw`ml-0.5`"#####, r#####"({
  marginLeft: "0.125rem",
})
;"##### ; "427")]
#[test_case(r#####"tw`ml-1`"#####, r#####"({
  marginLeft: "0.25rem",
})
;"##### ; "428")]
#[test_case(r#####"tw`ml-1.5`"#####, r#####"({
  marginLeft: "0.375rem",
})
;"##### ; "429")]
#[test_case(r#####"tw`ml-2`"#####, r#####"({
  marginLeft: "0.5rem",
})
;"##### ; "430")]
#[test_case(r#####"tw`ml-2.5`"#####, r#####"({
  marginLeft: "0.625rem",
})
;"##### ; "431")]
#[test_case(r#####"tw`ml-3`"#####, r#####"({
  marginLeft: "0.75rem",
})
;"##### ; "432")]
#[test_case(r#####"tw`ml-3.5`"#####, r#####"({
  marginLeft: "0.875rem",
})
;"##### ; "433")]
#[test_case(r#####"tw`ml-4`"#####, r#####"({
  marginLeft: "1rem",
})
;"##### ; "434")]
#[test_case(r#####"tw`ml-5`"#####, r#####"({
  marginLeft: "1.25rem",
})
;"##### ; "435")]
#[test_case(r#####"tw`ml-6`"#####, r#####"({
  marginLeft: "1.5rem",
})
;"##### ; "436")]
#[test_case(r#####"tw`ml-7`"#####, r#####"({
  marginLeft: "1.75rem",
})
;"##### ; "437")]
#[test_case(r#####"tw`ml-8`"#####, r#####"({
  marginLeft: "2rem",
})
;"##### ; "438")]
#[test_case(r#####"tw`ml-9`"#####, r#####"({
  marginLeft: "2.25rem",
})
;"##### ; "439")]
#[test_case(r#####"tw`ml-10`"#####, r#####"({
  marginLeft: "2.5rem",
})
;"##### ; "440")]
#[test_case(r#####"tw`ml-11`"#####, r#####"({
  marginLeft: "2.75rem",
})
;"##### ; "441")]
#[test_case(r#####"tw`ml-12`"#####, r#####"({
  marginLeft: "3rem",
})
;"##### ; "442")]
#[test_case(r#####"tw`ml-14`"#####, r#####"({
  marginLeft: "3.5rem",
})
;"##### ; "443")]
#[test_case(r#####"tw`ml-16`"#####, r#####"({
  marginLeft: "4rem",
})
;"##### ; "444")]
#[test_case(r#####"tw`ml-20`"#####, r#####"({
  marginLeft: "5rem",
})
;"##### ; "445")]
#[test_case(r#####"tw`ml-24`"#####, r#####"({
  marginLeft: "6rem",
})
;"##### ; "446")]
#[test_case(r#####"tw`ml-28`"#####, r#####"({
  marginLeft: "7rem",
})
;"##### ; "447")]
#[test_case(r#####"tw`ml-32`"#####, r#####"({
  marginLeft: "8rem",
})
;"##### ; "448")]
#[test_case(r#####"tw`ml-36`"#####, r#####"({
  marginLeft: "9rem",
})
;"##### ; "449")]
#[test_case(r#####"tw`ml-40`"#####, r#####"({
  marginLeft: "10rem",
})
;"##### ; "450")]
#[test_case(r#####"tw`ml-44`"#####, r#####"({
  marginLeft: "11rem",
})
;"##### ; "451")]
#[test_case(r#####"tw`ml-48`"#####, r#####"({
  marginLeft: "12rem",
})
;"##### ; "452")]
#[test_case(r#####"tw`ml-52`"#####, r#####"({
  marginLeft: "13rem",
})
;"##### ; "453")]
#[test_case(r#####"tw`ml-56`"#####, r#####"({
  marginLeft: "14rem",
})
;"##### ; "454")]
#[test_case(r#####"tw`ml-60`"#####, r#####"({
  marginLeft: "15rem",
})
;"##### ; "455")]
#[test_case(r#####"tw`ml-64`"#####, r#####"({
  marginLeft: "16rem",
})
;"##### ; "456")]
#[test_case(r#####"tw`ml-72`"#####, r#####"({
  marginLeft: "18rem",
})
;"##### ; "457")]
#[test_case(r#####"tw`ml-80`"#####, r#####"({
  marginLeft: "20rem",
})
;"##### ; "458")]
#[test_case(r#####"tw`ml-96`"#####, r#####"({
  marginLeft: "24rem",
})
;"##### ; "459")]
#[test_case(r#####"tw`ml-auto`"#####, r#####"({
  marginLeft: "auto",
})
;"##### ; "460")]
#[test_case(r#####"tw`-ml-0`"#####, r#####"({
  marginLeft: "-0px",
})
;"##### ; "461")]
#[test_case(r#####"tw`-ml-px`"#####, r#####"({
  marginLeft: "-1px",
})
;"##### ; "462")]
#[test_case(r#####"tw`-ml-0.5`"#####, r#####"({
  marginLeft: "-0.125rem",
})
;"##### ; "463")]
#[test_case(r#####"tw`-ml-1`"#####, r#####"({
  marginLeft: "-0.25rem",
})
;"##### ; "464")]
#[test_case(r#####"tw`-ml-1.5`"#####, r#####"({
  marginLeft: "-0.375rem",
})
;"##### ; "465")]
#[test_case(r#####"tw`-ml-2`"#####, r#####"({
  marginLeft: "-0.5rem",
})
;"##### ; "466")]
#[test_case(r#####"tw`-ml-2.5`"#####, r#####"({
  marginLeft: "-0.625rem",
})
;"##### ; "467")]
#[test_case(r#####"tw`-ml-3`"#####, r#####"({
  marginLeft: "-0.75rem",
})
;"##### ; "468")]
#[test_case(r#####"tw`-ml-3.5`"#####, r#####"({
  marginLeft: "-0.875rem",
})
;"##### ; "469")]
#[test_case(r#####"tw`-ml-4`"#####, r#####"({
  marginLeft: "-1rem",
})
;"##### ; "470")]
#[test_case(r#####"tw`-ml-5`"#####, r#####"({
  marginLeft: "-1.25rem",
})
;"##### ; "471")]
#[test_case(r#####"tw`-ml-6`"#####, r#####"({
  marginLeft: "-1.5rem",
})
;"##### ; "472")]
#[test_case(r#####"tw`-ml-7`"#####, r#####"({
  marginLeft: "-1.75rem",
})
;"##### ; "473")]
#[test_case(r#####"tw`-ml-8`"#####, r#####"({
  marginLeft: "-2rem",
})
;"##### ; "474")]
#[test_case(r#####"tw`-ml-9`"#####, r#####"({
  marginLeft: "-2.25rem",
})
;"##### ; "475")]
#[test_case(r#####"tw`-ml-10`"#####, r#####"({
  marginLeft: "-2.5rem",
})
;"##### ; "476")]
#[test_case(r#####"tw`-ml-11`"#####, r#####"({
  marginLeft: "-2.75rem",
})
;"##### ; "477")]
#[test_case(r#####"tw`-ml-12`"#####, r#####"({
  marginLeft: "-3rem",
})
;"##### ; "478")]
#[test_case(r#####"tw`-ml-14`"#####, r#####"({
  marginLeft: "-3.5rem",
})
;"##### ; "479")]
#[test_case(r#####"tw`-ml-16`"#####, r#####"({
  marginLeft: "-4rem",
})
;"##### ; "480")]
#[test_case(r#####"tw`-ml-20`"#####, r#####"({
  marginLeft: "-5rem",
})
;"##### ; "481")]
#[test_case(r#####"tw`-ml-24`"#####, r#####"({
  marginLeft: "-6rem",
})
;"##### ; "482")]
#[test_case(r#####"tw`-ml-28`"#####, r#####"({
  marginLeft: "-7rem",
})
;"##### ; "483")]
#[test_case(r#####"tw`-ml-32`"#####, r#####"({
  marginLeft: "-8rem",
})
;"##### ; "484")]
#[test_case(r#####"tw`-ml-36`"#####, r#####"({
  marginLeft: "-9rem",
})
;"##### ; "485")]
#[test_case(r#####"tw`-ml-40`"#####, r#####"({
  marginLeft: "-10rem",
})
;"##### ; "486")]
#[test_case(r#####"tw`-ml-44`"#####, r#####"({
  marginLeft: "-11rem",
})
;"##### ; "487")]
#[test_case(r#####"tw`-ml-48`"#####, r#####"({
  marginLeft: "-12rem",
})
;"##### ; "488")]
#[test_case(r#####"tw`-ml-52`"#####, r#####"({
  marginLeft: "-13rem",
})
;"##### ; "489")]
#[test_case(r#####"tw`-ml-56`"#####, r#####"({
  marginLeft: "-14rem",
})
;"##### ; "490")]
#[test_case(r#####"tw`-ml-60`"#####, r#####"({
  marginLeft: "-15rem",
})
;"##### ; "491")]
#[test_case(r#####"tw`-ml-64`"#####, r#####"({
  marginLeft: "-16rem",
})
;"##### ; "492")]
#[test_case(r#####"tw`-ml-72`"#####, r#####"({
  marginLeft: "-18rem",
})
;"##### ; "493")]
#[test_case(r#####"tw`-ml-80`"#####, r#####"({
  marginLeft: "-20rem",
})
;"##### ; "494")]
#[test_case(r#####"tw`-ml-96`"#####, r#####"({
  marginLeft: "-24rem",
})
;"##### ; "495")]
#[test_case(r#####"tw`m-[5px]`"#####, r#####"({
  margin: "5px",
})
;"##### ; "496")]
#[test_case(r#####"tw`mt-[5px]`"#####, r#####"({
  marginTop: "5px",
})
;"##### ; "497")]
#[test_case(r#####"tw`ml-[5px]`"#####, r#####"({
  marginLeft: "5px",
})
;"##### ; "498")]
#[test_case(r#####"tw`mr-[5px]`"#####, r#####"({
  marginRight: "5px",
})
;"##### ; "499")]
#[test_case(r#####"tw`mb-[5px]`"#####, r#####"({
  marginBottom: "5px",
})
;"##### ; "500")]
#[test_case(r#####"tw`-m-[5px]`"#####, r#####"({
  margin: "-5px",
})
;"##### ; "501")]
#[test_case(r#####"tw`-mt-[5px]`"#####, r#####"({
  marginTop: "-5px",
})
;"##### ; "502")]
#[test_case(r#####"tw`-ml-[5px]`"#####, r#####"({
  marginLeft: "-5px",
})
;"##### ; "503")]
#[test_case(r#####"tw`-mr-[5px]`"#####, r#####"({
  marginRight: "-5px",
})
;"##### ; "504")]
#[test_case(r#####"tw`-mb-[5px]`"#####, r#####"({
  marginBottom: "-5px",
})
;"##### ; "505")]
#[test_case(r#####"tw`mt-[clamp(30px,100px)]`"#####, r#####"({
  marginTop: "clamp(30px,100px)",
})
;"##### ; "506")]
#[test_case(r#####"tw`!-mt-4`"#####, r#####"({
  marginTop: "-1rem !important",
})
;"##### ; "507")]
#[test_case(r#####"tw`mt-6 mx-1 ms-4 me-8`"#####, r#####"({
  marginLeft: "0.25rem",
  marginRight: "0.25rem",
  marginInlineEnd: "2rem",
  marginInlineStart: "1rem",
  marginTop: "1.5rem",
})"##### ; "508")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
