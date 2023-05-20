use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`scale`"#####, r#####"({
  0: "0",
  50: ".5",
  75: ".75",
  90: ".9",
  95: ".95",
  100: "1",
  105: "1.05",
  110: "1.1",
  125: "1.25",
  150: "1.5",
})
;"##### ; "0")]
#[test_case(r#####"tw`scale-0`"#####, r#####"({
  '--tw-scale-x': "0",
  '--tw-scale-y': "0",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "1")]
#[test_case(r#####"tw`scale-x-0`"#####, r#####"({
  '--tw-scale-x': "0",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "2")]
#[test_case(r#####"tw`scale-y-0`"#####, r#####"({
  '--tw-scale-y': "0",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "3")]
#[test_case(r#####"tw`scale-50`"#####, r#####"({
  '--tw-scale-x': ".5",
  '--tw-scale-y': ".5",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "4")]
#[test_case(r#####"tw`scale-x-50`"#####, r#####"({
  '--tw-scale-x': ".5",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "5")]
#[test_case(r#####"tw`scale-y-50`"#####, r#####"({
  '--tw-scale-y': ".5",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "6")]
#[test_case(r#####"tw`scale-75`"#####, r#####"({
  '--tw-scale-x': ".75",
  '--tw-scale-y': ".75",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "7")]
#[test_case(r#####"tw`scale-x-75`"#####, r#####"({
  '--tw-scale-x': ".75",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "8")]
#[test_case(r#####"tw`scale-y-75`"#####, r#####"({
  '--tw-scale-y': ".75",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "9")]
#[test_case(r#####"tw`scale-90`"#####, r#####"({
  '--tw-scale-x': ".9",
  '--tw-scale-y': ".9",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "10")]
#[test_case(r#####"tw`scale-x-90`"#####, r#####"({
  '--tw-scale-x': ".9",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "11")]
#[test_case(r#####"tw`scale-y-90`"#####, r#####"({
  '--tw-scale-y': ".9",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "12")]
#[test_case(r#####"tw`scale-95`"#####, r#####"({
  '--tw-scale-x': ".95",
  '--tw-scale-y': ".95",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "13")]
#[test_case(r#####"tw`scale-x-95`"#####, r#####"({
  '--tw-scale-x': ".95",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "14")]
#[test_case(r#####"tw`scale-y-95`"#####, r#####"({
  '--tw-scale-y': ".95",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "15")]
#[test_case(r#####"tw`scale-100`"#####, r#####"({
  '--tw-scale-x': "1",
  '--tw-scale-y': "1",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "16")]
#[test_case(r#####"tw`scale-x-100`"#####, r#####"({
  '--tw-scale-x': "1",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "17")]
#[test_case(r#####"tw`scale-y-100`"#####, r#####"({
  '--tw-scale-y': "1",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "18")]
#[test_case(r#####"tw`scale-105`"#####, r#####"({
  '--tw-scale-x': "1.05",
  '--tw-scale-y': "1.05",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "19")]
#[test_case(r#####"tw`scale-x-105`"#####, r#####"({
  '--tw-scale-x': "1.05",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "20")]
#[test_case(r#####"tw`scale-y-105`"#####, r#####"({
  '--tw-scale-y': "1.05",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "21")]
#[test_case(r#####"tw`scale-110`"#####, r#####"({
  '--tw-scale-x': "1.1",
  '--tw-scale-y': "1.1",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "22")]
#[test_case(r#####"tw`scale-x-110`"#####, r#####"({
  '--tw-scale-x': "1.1",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "23")]
#[test_case(r#####"tw`scale-y-110`"#####, r#####"({
  '--tw-scale-y': "1.1",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "24")]
#[test_case(r#####"tw`scale-125`"#####, r#####"({
  '--tw-scale-x': "1.25",
  '--tw-scale-y': "1.25",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "25")]
#[test_case(r#####"tw`scale-x-125`"#####, r#####"({
  '--tw-scale-x': "1.25",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "26")]
#[test_case(r#####"tw`scale-y-125`"#####, r#####"({
  '--tw-scale-y': "1.25",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "27")]
#[test_case(r#####"tw`scale-150`"#####, r#####"({
  '--tw-scale-x': "1.5",
  '--tw-scale-y': "1.5",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "28")]
#[test_case(r#####"tw`scale-x-150`"#####, r#####"({
  '--tw-scale-x': "1.5",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "29")]
#[test_case(r#####"tw`scale-y-150`"#####, r#####"({
  '--tw-scale-y': "1.5",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "30")]
#[test_case(r#####"tw`scale-[1.7]`"#####, r#####"({
  '--tw-scale-x': "1.7",
  '--tw-scale-y': "1.7",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})"##### ; "31")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
