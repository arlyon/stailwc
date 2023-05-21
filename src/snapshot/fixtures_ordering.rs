use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw.div`xl:bg-red-500 lg:bg-blue-500 bg-green-500 fill-current md:bg-pink-500 sm:bg-green-500 sm:text-yellow-500 hidden`"#####, r#####"_styled.div({
  display: "none",
  "--tw-bg-opacity": "1",
  backgroundColor: "rgb(34 197 94 / var(--tw-bg-opacity))",
  fill: "currentColor",
  "@media (min-width: 640px)": {
    "--tw-bg-opacity": "1",
    backgroundColor: "rgb(34 197 94 / var(--tw-bg-opacity))",
    "--tw-text-opacity": "1",
    color: "rgb(234 179 8 / var(--tw-text-opacity))",
  },
  "@media (min-width: 768px)": {
    "--tw-bg-opacity": "1",
    backgroundColor: "rgb(236 72 153 / var(--tw-bg-opacity))",
  },
  "@media (min-width: 1024px)": {
    "--tw-bg-opacity": "1",
    backgroundColor: "rgb(59 130 246 / var(--tw-bg-opacity))",
  },
  "@media (min-width: 1280px)": {
    "--tw-bg-opacity": "1",
    backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
  },
}) // Bg opacity should trump the default bg opacity

;"##### ; "0")]
#[test_case(r#####"tw`bg-opacity-50 bg-red-500`"#####, r#####"({
  "--tw-bg-opacity": "0.5",
  backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
