use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`text-badass`"#####, r#####"({
  "--tw-text-opacity": "1",
  color: "rgb(186 218 85 / var(--tw-text-opacity))",
})
;"##### ; "0")]
#[test_case(r#####"tw`text-banana`"#####, r#####"({
  "--tw-text-opacity": "1",
  color: "rgb(255 255 0 / var(--tw-text-opacity))",
})
;"##### ; "1")]
#[test_case(r#####"tw`text-hamburger`"#####, r#####"({
  "--tw-text-opacity": "1",
  color: "rgb(165 42 42 / var(--tw-text-opacity))",
})
;"##### ; "2")]
#[test_case(r#####"tw`active:text-white`"#####, r#####"({
  ":active": {
    "--tw-text-opacity": "1",
    color: "rgb(255 255 255 / var(--tw-text-opacity))",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`hocus:text-black`"#####, r#####"({
  ":hover": {
    "--tw-text-opacity": "1",
    color: "rgb(0 0 0 / var(--tw-text-opacity))",
  },
  ":focus": {
    "--tw-text-opacity": "1",
    color: "rgb(0 0 0 / var(--tw-text-opacity))",
  },
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
