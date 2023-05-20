use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`overscroll-auto`"#####, r#####"({
  overscrollBehavior: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`overscroll-contain`"#####, r#####"({
  overscrollBehavior: "contain",
})
;"##### ; "2")]
#[test_case(r#####"tw`overscroll-none`"#####, r#####"({
  overscrollBehavior: "none",
})
;"##### ; "3")]
#[test_case(r#####"tw`overscroll-y-auto`"#####, r#####"({
  overscrollBehaviorY: "auto",
})
;"##### ; "4")]
#[test_case(r#####"tw`overscroll-y-contain`"#####, r#####"({
  overscrollBehaviorY: "contain",
})
;"##### ; "5")]
#[test_case(r#####"tw`overscroll-y-none`"#####, r#####"({
  overscrollBehaviorY: "none",
})
;"##### ; "6")]
#[test_case(r#####"tw`overscroll-x-auto`"#####, r#####"({
  overscrollBehaviorX: "auto",
})
;"##### ; "7")]
#[test_case(r#####"tw`overscroll-x-contain`"#####, r#####"({
  overscrollBehaviorX: "contain",
})
;"##### ; "8")]
#[test_case(r#####"tw`overscroll-x-none`"#####, r#####"({
  overscrollBehaviorX: "none",
})"##### ; "9")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
