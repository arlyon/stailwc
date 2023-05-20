use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`rotate`"#####, r#####"({
  0: "0deg",
  1: "1deg",
  2: "2deg",
  3: "3deg",
  6: "6deg",
  12: "12deg",
  45: "45deg",
  90: "90deg",
  180: "180deg",
})
;"##### ; "1")]
#[test_case(r#####"tw`rotate-0`"#####, r#####"({
  '--tw-rotate': "0deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "2")]
#[test_case(r#####"tw`rotate-1`"#####, r#####"({
  '--tw-rotate': "1deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "3")]
#[test_case(r#####"tw`rotate-2`"#####, r#####"({
  '--tw-rotate': "2deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "4")]
#[test_case(r#####"tw`rotate-3`"#####, r#####"({
  '--tw-rotate': "3deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "5")]
#[test_case(r#####"tw`rotate-6`"#####, r#####"({
  '--tw-rotate': "6deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "6")]
#[test_case(r#####"tw`rotate-12`"#####, r#####"({
  '--tw-rotate': "12deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "7")]
#[test_case(r#####"tw`rotate-45`"#####, r#####"({
  '--tw-rotate': "45deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "8")]
#[test_case(r#####"tw`rotate-90`"#####, r#####"({
  '--tw-rotate': "90deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "9")]
#[test_case(r#####"tw`rotate-180`"#####, r#####"({
  '--tw-rotate': "180deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "10")]
#[test_case(r#####"tw`-rotate-1`"#####, r#####"({
  '--tw-rotate': "-1deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "11")]
#[test_case(r#####"tw`-rotate-2`"#####, r#####"({
  '--tw-rotate': "-2deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "12")]
#[test_case(r#####"tw`-rotate-3`"#####, r#####"({
  '--tw-rotate': "-3deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "13")]
#[test_case(r#####"tw`-rotate-6`"#####, r#####"({
  '--tw-rotate': "-6deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "14")]
#[test_case(r#####"tw`-rotate-12`"#####, r#####"({
  '--tw-rotate': "-12deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "15")]
#[test_case(r#####"tw`-rotate-45`"#####, r#####"({
  '--tw-rotate': "-45deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "16")]
#[test_case(r#####"tw`-rotate-90`"#####, r#####"({
  '--tw-rotate': "-90deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "17")]
#[test_case(r#####"tw`-rotate-180`"#####, r#####"({
  '--tw-rotate': "-180deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "18")]
#[test_case(r#####"tw`rotate-[17deg]`"#####, r#####"({
  '--tw-rotate': "17deg",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})
;"##### ; "19")]
#[test_case(r#####"tw`rotate-[23deg] rotate-[2.3rad] rotate-[401grad] rotate-[1.5turn]`"#####, r#####"({
  '--tw-rotate': "401grad",
  transform:
    "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
})"##### ; "20")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
