use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`boxShadow.`"#####, r#####"({
  sm: "0 1px 2px 0 rgb(0 0 0 / 0.05)",
  DEFAULT: "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
  md: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
  lg: "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
  xl: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
  '2xl': "0 25px 50px -12px rgb(0 0 0 / 0.25)",
  inner: "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)",
  none: "none",
})
;"##### ; "0")]
#[test_case(r#####"tw`shadow-sm`"#####, r#####"({
  '--tw-shadow': "0 1px 2px 0 rgb(0 0 0 / 0.05)",
  '--tw-shadow-colored': "0 1px 2px 0 var(--tw-shadow-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)",
})
;"##### ; "1")]
#[test_case(r#####"tw`shadow`"#####, r#####"({
  '--tw-shadow':
    "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
  '--tw-shadow-colored':
    "0 1px 3px 0 var(--tw-shadow-color), 0 1px 2px -1px var(--tw-shadow-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)",
})
;"##### ; "2")]
#[test_case(r#####"tw`shadow-md`"#####, r#####"({
  '--tw-shadow':
    "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
  '--tw-shadow-colored':
    "0 4px 6px -1px var(--tw-shadow-color), 0 2px 4px -2px var(--tw-shadow-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)",
})
;"##### ; "3")]
#[test_case(r#####"tw`shadow-lg`"#####, r#####"({
  '--tw-shadow':
    "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
  '--tw-shadow-colored':
    "0 10px 15px -3px var(--tw-shadow-color), 0 4px 6px -4px var(--tw-shadow-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)",
})
;"##### ; "4")]
#[test_case(r#####"tw`shadow-xl`"#####, r#####"({
  '--tw-shadow':
    "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
  '--tw-shadow-colored':
    "0 20px 25px -5px var(--tw-shadow-color), 0 8px 10px -6px var(--tw-shadow-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)",
})
;"##### ; "5")]
#[test_case(r#####"tw`shadow-2xl`"#####, r#####"({
  '--tw-shadow': "0 25px 50px -12px rgb(0 0 0 / 0.25)",
  '--tw-shadow-colored': "0 25px 50px -12px var(--tw-shadow-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)",
})
;"##### ; "6")]
#[test_case(r#####"tw`shadow-inner`"#####, r#####"({
  '--tw-shadow': "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)",
  '--tw-shadow-colored': "inset 0 2px 4px 0 var(--tw-shadow-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)",
})
;"##### ; "7")]
#[test_case(r#####"tw`shadow-none`"#####, r#####"({
  '--tw-shadow': "0 0 #0000",
  '--tw-shadow-colored': "0 0 #0000",
  boxShadow:
    "var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)",
})
;"##### ; "8")]
#[test_case(r#####"tw`shadow-[0 35px 60px -15px rgba(0,0,0,0.3)]`"#####, r#####"({
  '--tw-shadow': "0 35px 60px -15px rgba(0,0,0,0.3)",
  '--tw-shadow-colored': "0 35px 60px -15px var(--tw-shadow-color)",
  boxShadow:
    "var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)",
})"##### ; "9")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
