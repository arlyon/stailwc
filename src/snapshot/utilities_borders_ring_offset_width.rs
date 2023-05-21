use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`ringOffsetWidth`"#####, r#####"({
  0: "0px",
  1: "1px",
  2: "2px",
  4: "4px",
  8: "8px",
})
;"##### ; "0")]
#[test_case(r#####"tw`ring-offset-0`"#####, r#####"({
  "--tw-ring-offset-width": "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`ring-offset-1`"#####, r#####"({
  "--tw-ring-offset-width": "1px",
})
;"##### ; "2")]
#[test_case(r#####"tw`ring-offset-2`"#####, r#####"({
  "--tw-ring-offset-width": "2px",
})
;"##### ; "3")]
#[test_case(r#####"tw`ring-offset-4`"#####, r#####"({
  "--tw-ring-offset-width": "4px",
})
;"##### ; "4")]
#[test_case(r#####"tw`ring-offset-8`"#####, r#####"({
  "--tw-ring-offset-width": "8px",
})
;"##### ; "5")]
#[test_case(r#####"tw`ring-offset-[3px]`"#####, r#####"({
  "--tw-ring-offset-width": "3px",
})
;"##### ; "6")]
#[test_case(r#####"tw`ring-offset-[19rem]`"#####, r#####"({
  "--tw-ring-offset-width": "19rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`ring-offset-[#76ad65]`"#####, r#####"({
  "--tw-ring-offset-color": "#76ad65",
})"##### ; "8")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
