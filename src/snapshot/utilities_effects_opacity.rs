use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`opacity`"#####, r#####"({
  0: "0",
  5: "0.05",
  10: "0.1",
  20: "0.2",
  25: "0.25",
  30: "0.3",
  40: "0.4",
  50: "0.5",
  60: "0.6",
  70: "0.7",
  75: "0.75",
  80: "0.8",
  90: "0.9",
  95: "0.95",
  100: "1",
})
;"##### ; "0")]
#[test_case(r#####"tw`opacity-0`"#####, r#####"({
  opacity: "0",
})
;"##### ; "1")]
#[test_case(r#####"tw`opacity-5`"#####, r#####"({
  opacity: "0.05",
})
;"##### ; "2")]
#[test_case(r#####"tw`opacity-10`"#####, r#####"({
  opacity: "0.1",
})
;"##### ; "3")]
#[test_case(r#####"tw`opacity-20`"#####, r#####"({
  opacity: "0.2",
})
;"##### ; "4")]
#[test_case(r#####"tw`opacity-25`"#####, r#####"({
  opacity: "0.25",
})
;"##### ; "5")]
#[test_case(r#####"tw`opacity-30`"#####, r#####"({
  opacity: "0.3",
})
;"##### ; "6")]
#[test_case(r#####"tw`opacity-40`"#####, r#####"({
  opacity: "0.4",
})
;"##### ; "7")]
#[test_case(r#####"tw`opacity-50`"#####, r#####"({
  opacity: "0.5",
})
;"##### ; "8")]
#[test_case(r#####"tw`opacity-60`"#####, r#####"({
  opacity: "0.6",
})
;"##### ; "9")]
#[test_case(r#####"tw`opacity-70`"#####, r#####"({
  opacity: "0.7",
})
;"##### ; "10")]
#[test_case(r#####"tw`opacity-75`"#####, r#####"({
  opacity: "0.75",
})
;"##### ; "11")]
#[test_case(r#####"tw`opacity-80`"#####, r#####"({
  opacity: "0.8",
})
;"##### ; "12")]
#[test_case(r#####"tw`opacity-90`"#####, r#####"({
  opacity: "0.9",
})
;"##### ; "13")]
#[test_case(r#####"tw`opacity-95`"#####, r#####"({
  opacity: "0.95",
})
;"##### ; "14")]
#[test_case(r#####"tw`opacity-100`"#####, r#####"({
  opacity: "1",
})
;"##### ; "15")]
#[test_case(r#####"tw`opacity-[.1]`"#####, r#####"({
  opacity: ".1",
})
;"##### ; "16")]
#[test_case(r#####"tw`opacity-[var(--opacity)]`"#####, r#####"({
  opacity: "var(--opacity)",
})"##### ; "17")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
