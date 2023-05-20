use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`translate-x-0`"#####, r#####"({
  '--tw-translate-x': "0px",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "0")]
#[test_case(r#####"tw`translate-y-0`"#####, r#####"({
  '--tw-translate-y': "0px",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "1")]
#[test_case(r#####"tw`translate-x-px`"#####, r#####"({
  '--tw-translate-x': "1px",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "2")]
#[test_case(r#####"tw`translate-y-px`"#####, r#####"({
  '--tw-translate-y': "1px",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "3")]
#[test_case(r#####"tw`translate-x-0.5`"#####, r#####"({
  '--tw-translate-x': "0.125rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "4")]
#[test_case(r#####"tw`translate-y-0.5`"#####, r#####"({
  '--tw-translate-y': "0.125rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "5")]
#[test_case(r#####"tw`translate-x-1`"#####, r#####"({
  '--tw-translate-x': "0.25rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "6")]
#[test_case(r#####"tw`translate-y-1`"#####, r#####"({
  '--tw-translate-y': "0.25rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "7")]
#[test_case(r#####"tw`translate-x-1.5`"#####, r#####"({
  '--tw-translate-x': "0.375rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "8")]
#[test_case(r#####"tw`translate-y-1.5`"#####, r#####"({
  '--tw-translate-y': "0.375rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "9")]
#[test_case(r#####"tw`translate-x-2`"#####, r#####"({
  '--tw-translate-x': "0.5rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "10")]
#[test_case(r#####"tw`translate-y-2`"#####, r#####"({
  '--tw-translate-y': "0.5rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "11")]
#[test_case(r#####"tw`translate-x-2.5`"#####, r#####"({
  '--tw-translate-x': "0.625rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "12")]
#[test_case(r#####"tw`translate-y-2.5`"#####, r#####"({
  '--tw-translate-y': "0.625rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "13")]
#[test_case(r#####"tw`translate-x-3`"#####, r#####"({
  '--tw-translate-x': "0.75rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "14")]
#[test_case(r#####"tw`translate-y-3`"#####, r#####"({
  '--tw-translate-y': "0.75rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "15")]
#[test_case(r#####"tw`translate-x-3.5`"#####, r#####"({
  '--tw-translate-x': "0.875rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "16")]
#[test_case(r#####"tw`translate-y-3.5`"#####, r#####"({
  '--tw-translate-y': "0.875rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "17")]
#[test_case(r#####"tw`translate-x-4`"#####, r#####"({
  '--tw-translate-x': "1rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "18")]
#[test_case(r#####"tw`translate-y-4`"#####, r#####"({
  '--tw-translate-y': "1rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "19")]
#[test_case(r#####"tw`translate-x-5`"#####, r#####"({
  '--tw-translate-x': "1.25rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "20")]
#[test_case(r#####"tw`translate-y-5`"#####, r#####"({
  '--tw-translate-y': "1.25rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "21")]
#[test_case(r#####"tw`translate-x-6`"#####, r#####"({
  '--tw-translate-x': "1.5rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "22")]
#[test_case(r#####"tw`translate-y-6`"#####, r#####"({
  '--tw-translate-y': "1.5rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "23")]
#[test_case(r#####"tw`translate-x-7`"#####, r#####"({
  '--tw-translate-x': "1.75rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "24")]
#[test_case(r#####"tw`translate-y-7`"#####, r#####"({
  '--tw-translate-y': "1.75rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "25")]
#[test_case(r#####"tw`translate-x-8`"#####, r#####"({
  '--tw-translate-x': "2rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "26")]
#[test_case(r#####"tw`translate-y-8`"#####, r#####"({
  '--tw-translate-y': "2rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "27")]
#[test_case(r#####"tw`translate-x-9`"#####, r#####"({
  '--tw-translate-x': "2.25rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "28")]
#[test_case(r#####"tw`translate-y-9`"#####, r#####"({
  '--tw-translate-y': "2.25rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "29")]
#[test_case(r#####"tw`translate-x-10`"#####, r#####"({
  '--tw-translate-x': "2.5rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "30")]
#[test_case(r#####"tw`translate-y-10`"#####, r#####"({
  '--tw-translate-y': "2.5rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "31")]
#[test_case(r#####"tw`translate-x-11`"#####, r#####"({
  '--tw-translate-x': "2.75rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "32")]
#[test_case(r#####"tw`translate-y-11`"#####, r#####"({
  '--tw-translate-y': "2.75rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "33")]
#[test_case(r#####"tw`translate-x-12`"#####, r#####"({
  '--tw-translate-x': "3rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "34")]
#[test_case(r#####"tw`translate-y-12`"#####, r#####"({
  '--tw-translate-y': "3rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "35")]
#[test_case(r#####"tw`translate-x-14`"#####, r#####"({
  '--tw-translate-x': "3.5rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "36")]
#[test_case(r#####"tw`translate-y-14`"#####, r#####"({
  '--tw-translate-y': "3.5rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "37")]
#[test_case(r#####"tw`translate-x-16`"#####, r#####"({
  '--tw-translate-x': "4rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "38")]
#[test_case(r#####"tw`translate-y-16`"#####, r#####"({
  '--tw-translate-y': "4rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "39")]
#[test_case(r#####"tw`translate-x-20`"#####, r#####"({
  '--tw-translate-x': "5rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "40")]
#[test_case(r#####"tw`translate-y-20`"#####, r#####"({
  '--tw-translate-y': "5rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "41")]
#[test_case(r#####"tw`translate-x-24`"#####, r#####"({
  '--tw-translate-x': "6rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "42")]
#[test_case(r#####"tw`translate-y-24`"#####, r#####"({
  '--tw-translate-y': "6rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "43")]
#[test_case(r#####"tw`translate-x-28`"#####, r#####"({
  '--tw-translate-x': "7rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "44")]
#[test_case(r#####"tw`translate-y-28`"#####, r#####"({
  '--tw-translate-y': "7rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "45")]
#[test_case(r#####"tw`translate-x-32`"#####, r#####"({
  '--tw-translate-x': "8rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "46")]
#[test_case(r#####"tw`translate-y-32`"#####, r#####"({
  '--tw-translate-y': "8rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "47")]
#[test_case(r#####"tw`translate-x-36`"#####, r#####"({
  '--tw-translate-x': "9rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "48")]
#[test_case(r#####"tw`translate-y-36`"#####, r#####"({
  '--tw-translate-y': "9rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "49")]
#[test_case(r#####"tw`translate-x-40`"#####, r#####"({
  '--tw-translate-x': "10rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "50")]
#[test_case(r#####"tw`translate-y-40`"#####, r#####"({
  '--tw-translate-y': "10rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "51")]
#[test_case(r#####"tw`translate-x-44`"#####, r#####"({
  '--tw-translate-x': "11rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "52")]
#[test_case(r#####"tw`translate-y-44`"#####, r#####"({
  '--tw-translate-y': "11rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "53")]
#[test_case(r#####"tw`translate-x-48`"#####, r#####"({
  '--tw-translate-x': "12rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "54")]
#[test_case(r#####"tw`translate-y-48`"#####, r#####"({
  '--tw-translate-y': "12rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "55")]
#[test_case(r#####"tw`translate-x-52`"#####, r#####"({
  '--tw-translate-x': "13rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "56")]
#[test_case(r#####"tw`translate-y-52`"#####, r#####"({
  '--tw-translate-y': "13rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "57")]
#[test_case(r#####"tw`translate-x-56`"#####, r#####"({
  '--tw-translate-x': "14rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "58")]
#[test_case(r#####"tw`translate-y-56`"#####, r#####"({
  '--tw-translate-y': "14rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "59")]
#[test_case(r#####"tw`translate-x-60`"#####, r#####"({
  '--tw-translate-x': "15rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "60")]
#[test_case(r#####"tw`translate-y-60`"#####, r#####"({
  '--tw-translate-y': "15rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "61")]
#[test_case(r#####"tw`translate-x-64`"#####, r#####"({
  '--tw-translate-x': "16rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "62")]
#[test_case(r#####"tw`translate-y-64`"#####, r#####"({
  '--tw-translate-y': "16rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "63")]
#[test_case(r#####"tw`translate-x-72`"#####, r#####"({
  '--tw-translate-x': "18rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "64")]
#[test_case(r#####"tw`translate-y-72`"#####, r#####"({
  '--tw-translate-y': "18rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "65")]
#[test_case(r#####"tw`translate-x-80`"#####, r#####"({
  '--tw-translate-x': "20rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "66")]
#[test_case(r#####"tw`translate-y-80`"#####, r#####"({
  '--tw-translate-y': "20rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "67")]
#[test_case(r#####"tw`translate-x-96`"#####, r#####"({
  '--tw-translate-x': "24rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "68")]
#[test_case(r#####"tw`translate-y-96`"#####, r#####"({
  '--tw-translate-y': "24rem",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "69")]
#[test_case(r#####"tw`translate-x-1/2`"#####, r#####"({
  '--tw-translate-x': "50%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "70")]
#[test_case(r#####"tw`translate-x-1/3`"#####, r#####"({
  '--tw-translate-x': "33.333333%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "71")]
#[test_case(r#####"tw`translate-x-2/3`"#####, r#####"({
  '--tw-translate-x': "66.666667%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "72")]
#[test_case(r#####"tw`translate-x-1/4`"#####, r#####"({
  '--tw-translate-x': "25%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "73")]
#[test_case(r#####"tw`translate-x-2/4`"#####, r#####"({
  '--tw-translate-x': "50%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "74")]
#[test_case(r#####"tw`translate-x-3/4`"#####, r#####"({
  '--tw-translate-x': "75%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "75")]
#[test_case(r#####"tw`translate-x-full`"#####, r#####"({
  '--tw-translate-x': "100%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "76")]
#[test_case(r#####"tw`translate-y-1/2`"#####, r#####"({
  '--tw-translate-y': "50%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "77")]
#[test_case(r#####"tw`translate-y-1/3`"#####, r#####"({
  '--tw-translate-y': "33.333333%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "78")]
#[test_case(r#####"tw`translate-y-2/3`"#####, r#####"({
  '--tw-translate-y': "66.666667%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "79")]
#[test_case(r#####"tw`translate-y-1/4`"#####, r#####"({
  '--tw-translate-y': "25%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "80")]
#[test_case(r#####"tw`translate-y-2/4`"#####, r#####"({
  '--tw-translate-y': "50%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "81")]
#[test_case(r#####"tw`translate-y-3/4`"#####, r#####"({
  '--tw-translate-y': "75%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "82")]
#[test_case(r#####"tw`translate-y-full`"#####, r#####"({
  '--tw-translate-y': "100%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "83")]
#[test_case(r#####"tw`transform`"#####, r#####"({
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "84")]
#[test_case(r#####"tw`transform-gpu`"#####, r#####"({
  transform:
    "translate3d(var(--tw-translate-x), var(--tw-translate-y), 0) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "85")]
#[test_case(r#####"tw`transform-cpu`"#####, r#####"({
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "86")]
#[test_case(r#####"tw`transform-none`"#####, r#####"({
  transform: "none",
})
;"##### ; "87")]
#[test_case(r#####"tw`translate-x-10000`"#####, r#####"({
  '--tw-translate-x': "5000%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "88")]
#[test_case(r#####"tw`-translate-x-10000`"#####, r#####"({
  '--tw-translate-x': "-5000%",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})"##### ; "89")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
