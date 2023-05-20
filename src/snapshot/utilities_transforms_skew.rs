use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`skew`"#####, r#####"({
  0: "0deg",
  1: "1deg",
  2: "2deg",
  3: "3deg",
  6: "6deg",
  12: "12deg",
})
;"##### ; "1")]
#[test_case(r#####"tw`skew-x-0`"#####, r#####"({
  '--tw-skew-x': "0deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "2")]
#[test_case(r#####"tw`skew-y-0`"#####, r#####"({
  '--tw-skew-y': "0deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "3")]
#[test_case(r#####"tw`skew-x-1`"#####, r#####"({
  '--tw-skew-x': "1deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "4")]
#[test_case(r#####"tw`skew-y-1`"#####, r#####"({
  '--tw-skew-y': "1deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "5")]
#[test_case(r#####"tw`skew-x-2`"#####, r#####"({
  '--tw-skew-x': "2deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "6")]
#[test_case(r#####"tw`skew-y-2`"#####, r#####"({
  '--tw-skew-y': "2deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "7")]
#[test_case(r#####"tw`skew-x-3`"#####, r#####"({
  '--tw-skew-x': "3deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "8")]
#[test_case(r#####"tw`skew-y-3`"#####, r#####"({
  '--tw-skew-y': "3deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "9")]
#[test_case(r#####"tw`skew-x-6`"#####, r#####"({
  '--tw-skew-x': "6deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "10")]
#[test_case(r#####"tw`skew-y-6`"#####, r#####"({
  '--tw-skew-y': "6deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "11")]
#[test_case(r#####"tw`skew-x-12`"#####, r#####"({
  '--tw-skew-x': "12deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "12")]
#[test_case(r#####"tw`skew-y-12`"#####, r#####"({
  '--tw-skew-y': "12deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "13")]
#[test_case(r#####"tw`-skew-x-1`"#####, r#####"({
  '--tw-skew-x': "-1deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "14")]
#[test_case(r#####"tw`-skew-y-1`"#####, r#####"({
  '--tw-skew-y': "-1deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "15")]
#[test_case(r#####"tw`-skew-x-2`"#####, r#####"({
  '--tw-skew-x': "-2deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "16")]
#[test_case(r#####"tw`-skew-y-2`"#####, r#####"({
  '--tw-skew-y': "-2deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "17")]
#[test_case(r#####"tw`-skew-x-3`"#####, r#####"({
  '--tw-skew-x': "-3deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "18")]
#[test_case(r#####"tw`-skew-y-3`"#####, r#####"({
  '--tw-skew-y': "-3deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "19")]
#[test_case(r#####"tw`-skew-x-6`"#####, r#####"({
  '--tw-skew-x': "-6deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "20")]
#[test_case(r#####"tw`-skew-y-6`"#####, r#####"({
  '--tw-skew-y': "-6deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "21")]
#[test_case(r#####"tw`-skew-x-12`"#####, r#####"({
  '--tw-skew-x': "-12deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "22")]
#[test_case(r#####"tw`-skew-y-12`"#####, r#####"({
  '--tw-skew-y': "-12deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "23")]
#[test_case(r#####"tw`skew-x-[17deg]`"#####, r#####"({
  '--tw-skew-x': "17deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "24")]
#[test_case(r#####"tw`-skew-x-[17deg]`"#####, r#####"({
  '--tw-skew-x': "-17deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "25")]
#[test_case(r#####"tw`skew-y-[17deg]`"#####, r#####"({
  '--tw-skew-y': "17deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "26")]
#[test_case(r#####"tw`-skew-y-[17deg]`"#####, r#####"({
  '--tw-skew-y': "-17deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})"##### ; "27")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
