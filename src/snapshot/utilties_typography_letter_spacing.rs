use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`letterSpacing`"#####, r#####"({
  tighter: "-0.05em",
  tight: "-0.025em",
  normal: "0em",
  wide: "0.025em",
  wider: "0.05em",
  widest: "0.1em",
})
;"##### ; "1")]
#[test_case(r#####"tw`tracking-tighter`"#####, r#####"({
  letterSpacing: "-0.05em",
})
;"##### ; "2")]
#[test_case(r#####"tw`tracking-tight`"#####, r#####"({
  letterSpacing: "-0.025em",
})
;"##### ; "3")]
#[test_case(r#####"tw`tracking-normal`"#####, r#####"({
  letterSpacing: "0em",
})
;"##### ; "4")]
#[test_case(r#####"tw`tracking-wide`"#####, r#####"({
  letterSpacing: "0.025em",
})
;"##### ; "5")]
#[test_case(r#####"tw`tracking-wider`"#####, r#####"({
  letterSpacing: "0.05em",
})
;"##### ; "6")]
#[test_case(r#####"tw`tracking-widest`"#####, r#####"({
  letterSpacing: "0.1em",
})
;"##### ; "7")]
#[test_case(r#####"tw`-tracking-tighter`"#####, r#####"({
  letterSpacing: "0.05em",
})
;"##### ; "8")]
#[test_case(r#####"tw`-tracking-tight`"#####, r#####"({
  letterSpacing: "0.025em",
})
;"##### ; "9")]
#[test_case(r#####"tw`-tracking-normal`"#####, r#####"({
  letterSpacing: "-0em",
})
;"##### ; "10")]
#[test_case(r#####"tw`-tracking-wide`"#####, r#####"({
  letterSpacing: "-0.025em",
})
;"##### ; "11")]
#[test_case(r#####"tw`-tracking-wider`"#####, r#####"({
  letterSpacing: "-0.05em",
})
;"##### ; "12")]
#[test_case(r#####"tw`-tracking-widest`"#####, r#####"({
  letterSpacing: "-0.1em",
})
;"##### ; "13")]
#[test_case(r#####"tw`-tracking-[var(--tracking)]`"#####, r#####"({
  letterSpacing: "calc(var(--tracking) * -1)",
})
;"##### ; "14")]
#[test_case(r#####"tw`tracking-[var(--tracking)]`"#####, r#####"({
  letterSpacing: "var(--tracking)",
})
;"##### ; "15")]
#[test_case(r#####"tw`-tracking-[2em]`"#####, r#####"({
  letterSpacing: "-2em",
})
;"##### ; "16")]
#[test_case(r#####"tw`tracking-[.25em]`"#####, r#####"({
  letterSpacing: ".25em",
})"##### ; "17")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
