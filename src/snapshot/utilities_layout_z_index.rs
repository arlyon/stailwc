use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`zIndex`"#####, r#####"({
  0: "0",
  1: "1",
  10: "10",
  20: "20",
  30: "30",
  40: "40",
  50: "50",
  auto: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`z-0`"#####, r#####"({
  zIndex: "0",
})
;"##### ; "2")]
#[test_case(r#####"tw`-z-0`"#####, r#####"({
  zIndex: "0",
})
;"##### ; "3")]
#[test_case(r#####"tw`z-10`"#####, r#####"({
  zIndex: "10",
})
;"##### ; "4")]
#[test_case(r#####"tw`-z-10`"#####, r#####"({
  zIndex: "-10",
})
;"##### ; "5")]
#[test_case(r#####"tw`z-20`"#####, r#####"({
  zIndex: "20",
})
;"##### ; "6")]
#[test_case(r#####"tw`-z-20`"#####, r#####"({
  zIndex: "-20",
})
;"##### ; "7")]
#[test_case(r#####"tw`z-30`"#####, r#####"({
  zIndex: "30",
})
;"##### ; "8")]
#[test_case(r#####"tw`-z-30`"#####, r#####"({
  zIndex: "-30",
})
;"##### ; "9")]
#[test_case(r#####"tw`z-40`"#####, r#####"({
  zIndex: "40",
})
;"##### ; "10")]
#[test_case(r#####"tw`-z-40`"#####, r#####"({
  zIndex: "-40",
})
;"##### ; "11")]
#[test_case(r#####"tw`z-50`"#####, r#####"({
  zIndex: "50",
})
;"##### ; "12")]
#[test_case(r#####"tw`-z-50`"#####, r#####"({
  zIndex: "-50",
})
;"##### ; "13")]
#[test_case(r#####"tw`z-auto`"#####, r#####"({
  zIndex: "auto",
})
;"##### ; "14")]
#[test_case(r#####"tw`z-[100]`"#####, r#####"({
  zIndex: "100",
})
;"##### ; "15")]
#[test_case(r#####"tw`z-[-100]`"#####, r#####"({
  zIndex: "-100",
})
;"##### ; "16")]
#[test_case(r#####"tw`-z-[100]`"#####, r#####"({
  zIndex: "-100",
})
;"##### ; "17")]
#[test_case(r#####"tw`-z-[-100]`"#####, r#####"({
  zIndex: "100",
})"##### ; "18")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
