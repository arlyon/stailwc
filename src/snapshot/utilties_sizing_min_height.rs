use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`minHeight`"#####, r#####"({
  0: "0px",
  full: "100%",
  screen: "100vh",
  min: "min-content",
  max: "max-content",
  fit: "fit-content",
})
;"##### ; "0")]
#[test_case(r#####"tw`min-h-0`"#####, r#####"({
  minHeight: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`min-h-full`"#####, r#####"({
  minHeight: "100%",
})
;"##### ; "2")]
#[test_case(r#####"tw`min-h-screen`"#####, r#####"({
  minHeight: "100vh",
})
;"##### ; "3")]
#[test_case(r#####"tw`min-h-min`"#####, r#####"({
  minHeight: "min-content",
})
;"##### ; "4")]
#[test_case(r#####"tw`min-h-max`"#####, r#####"({
  minHeight: "max-content",
})
;"##### ; "5")]
#[test_case(r#####"tw`min-h-fit`"#####, r#####"({
  minHeight: "fit-content",
})
;"##### ; "6")]
#[test_case(r#####"tw`min-h-[3.23rem]`"#####, r#####"({
  minHeight: "3.23rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`min-h-[calc(100%+1rem)]`"#####, r#####"({
  minHeight: "calc(100% + 1rem)",
})
;"##### ; "8")]
#[test_case(r#####"tw`min-h-[var(--height)]`"#####, r#####"({
  minHeight: "var(--height)",
})
;"##### ; "9")]
#[test_case(r#####"tw`max-h-[3.23rem]`"#####, r#####"({
  maxHeight: "3.23rem",
})
;"##### ; "10")]
#[test_case(r#####"tw`max-h-[calc(100%+1rem)]`"#####, r#####"({
  maxHeight: "calc(100% + 1rem)",
})
;"##### ; "11")]
#[test_case(r#####"tw`max-h-[var(--height)]`"#####, r#####"({
  maxHeight: "var(--height)",
})"##### ; "12")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
