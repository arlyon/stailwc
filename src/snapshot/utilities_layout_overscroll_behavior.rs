use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`overscroll-auto`"#####, r#####"({
  overscrollBehavior: "auto",
})
;"##### ; "0")]
#[test_case(r#####"tw`overscroll-contain`"#####, r#####"({
  overscrollBehavior: "contain",
})
;"##### ; "1")]
#[test_case(r#####"tw`overscroll-none`"#####, r#####"({
  overscrollBehavior: "none",
})
;"##### ; "2")]
#[test_case(r#####"tw`overscroll-y-auto`"#####, r#####"({
  overscrollBehaviorY: "auto",
})
;"##### ; "3")]
#[test_case(r#####"tw`overscroll-y-contain`"#####, r#####"({
  overscrollBehaviorY: "contain",
})
;"##### ; "4")]
#[test_case(r#####"tw`overscroll-y-none`"#####, r#####"({
  overscrollBehaviorY: "none",
})
;"##### ; "5")]
#[test_case(r#####"tw`overscroll-x-auto`"#####, r#####"({
  overscrollBehaviorX: "auto",
})
;"##### ; "6")]
#[test_case(r#####"tw`overscroll-x-contain`"#####, r#####"({
  overscrollBehaviorX: "contain",
})
;"##### ; "7")]
#[test_case(r#####"tw`overscroll-x-none`"#####, r#####"({
  overscrollBehaviorX: "none",
})"##### ; "8")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
