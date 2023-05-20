use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`transitionTimingFunction.`"#####, r#####"({
  DEFAULT: "cubic-bezier(0.4, 0, 0.2, 1)",
  linear: "linear",
  in: "cubic-bezier(0.4, 0, 1, 1)",
  out: "cubic-bezier(0, 0, 0.2, 1)",
  'in-out': "cubic-bezier(0.4, 0, 0.2, 1)",
})
;"##### ; "0")]
#[test_case(r#####"tw`ease-linear`"#####, r#####"({
  transitionTimingFunction: "linear",
})
;"##### ; "1")]
#[test_case(r#####"tw`ease-in`"#####, r#####"({
  transitionTimingFunction: "cubic-bezier(0.4, 0, 1, 1)",
})
;"##### ; "2")]
#[test_case(r#####"tw`ease-out`"#####, r#####"({
  transitionTimingFunction: "cubic-bezier(0, 0, 0.2, 1)",
})
;"##### ; "3")]
#[test_case(r#####"tw`ease-in-out`"#####, r#####"({
  transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
})
;"##### ; "4")]
#[test_case(r#####"tw`ease-[cubic-bezier(0.95, 0.05, 0.795, 0.035)]`"#####, r#####"({
  transitionTimingFunction: "cubic-bezier(0.95, 0.05, 0.795, 0.035)",
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
