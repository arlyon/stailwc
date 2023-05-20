use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`ringWidth.`"#####, r#####"({
  0: "0px",
  1: "1px",
  2: "2px",
  4: "4px",
  8: "8px",
  DEFAULT: "3px",
})
;"##### ; "0")]
#[test_case(r#####"tw`ring-0`"#####, r#####"({
  '--tw-ring-offset-shadow':
    "var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)",
  '--tw-ring-shadow':
    "var(--tw-ring-inset) 0 0 0 calc(0px + var(--tw-ring-offset-width)) var(--tw-ring-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)",
})
;"##### ; "1")]
#[test_case(r#####"tw`ring-1`"#####, r#####"({
  '--tw-ring-offset-shadow':
    "var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)",
  '--tw-ring-shadow':
    "var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)",
})
;"##### ; "2")]
#[test_case(r#####"tw`ring-2`"#####, r#####"({
  '--tw-ring-offset-shadow':
    "var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)",
  '--tw-ring-shadow':
    "var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)",
})
;"##### ; "3")]
#[test_case(r#####"tw`ring`"#####, r#####"({
  '--tw-ring-offset-shadow':
    "var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)",
  '--tw-ring-shadow':
    "var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)",
})
;"##### ; "4")]
#[test_case(r#####"tw`ring-4`"#####, r#####"({
  '--tw-ring-offset-shadow':
    "var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)",
  '--tw-ring-shadow':
    "var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)",
})
;"##### ; "5")]
#[test_case(r#####"tw`ring-8`"#####, r#####"({
  '--tw-ring-offset-shadow':
    "var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)",
  '--tw-ring-shadow':
    "var(--tw-ring-inset) 0 0 0 calc(8px + var(--tw-ring-offset-width)) var(--tw-ring-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)",
})
;"##### ; "6")]
#[test_case(r#####"tw`ring-inset`"#####, r#####"({
  '--tw-ring-inset': "inset",
})
;"##### ; "7")]
#[test_case(r#####"tw`ring-[10px]`"#####, r#####"({
  '--tw-ring-offset-shadow':
    "var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)",
  '--tw-ring-shadow':
    "var(--tw-ring-inset) 0 0 0 calc(10px + var(--tw-ring-offset-width)) var(--tw-ring-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)",
})"##### ; "8")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
