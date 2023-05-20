use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`minWidth`"#####, r#####"({
  0: "0px",
  full: "100%",
  min: "min-content",
  max: "max-content",
  fit: "fit-content",
})
;"##### ; "0")]
#[test_case(r#####"tw`min-w-0`"#####, r#####"({
  minWidth: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`min-w-full`"#####, r#####"({
  minWidth: "100%",
})
;"##### ; "2")]
#[test_case(r#####"tw`min-w-min`"#####, r#####"({
  minWidth: "min-content",
})
;"##### ; "3")]
#[test_case(r#####"tw`min-w-max`"#####, r#####"({
  minWidth: "max-content",
})
;"##### ; "4")]
#[test_case(r#####"tw`min-w-fit`"#####, r#####"({
  minWidth: "fit-content",
})
;"##### ; "5")]
#[test_case(r#####"tw`min-w-[3.23rem]`"#####, r#####"({
  minWidth: "3.23rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`min-w-[calc(100%+1rem)]`"#####, r#####"({
  minWidth: "calc(100% + 1rem)",
})
;"##### ; "7")]
#[test_case(r#####"tw`min-w-[var(--width)]`"#####, r#####"({
  minWidth: "var(--width)",
})
;"##### ; "8")]
#[test_case(r#####"tw`max-w-[3.23rem]`"#####, r#####"({
  maxWidth: "3.23rem",
})
;"##### ; "9")]
#[test_case(r#####"tw`max-w-[calc(100%+1rem)]`"#####, r#####"({
  maxWidth: "calc(100% + 1rem)",
})
;"##### ; "10")]
#[test_case(r#####"tw`max-w-[var(--width)]`"#####, r#####"({
  maxWidth: "var(--width)",
})"##### ; "11")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
