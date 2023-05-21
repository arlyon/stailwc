use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`divide-opacity-0`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`divide-opacity-5`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.05",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`divide-opacity-10`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.1",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`divide-opacity-20`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.2",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`divide-opacity-25`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.25",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`divide-opacity-30`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.3",
  },
})
;"##### ; "5")]
#[test_case(r#####"tw`divide-opacity-40`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.4",
  },
})
;"##### ; "6")]
#[test_case(r#####"tw`divide-opacity-50`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.5",
  },
})
;"##### ; "7")]
#[test_case(r#####"tw`divide-opacity-60`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.6",
  },
})
;"##### ; "8")]
#[test_case(r#####"tw`divide-opacity-70`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.7",
  },
})
;"##### ; "9")]
#[test_case(r#####"tw`divide-opacity-75`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.75",
  },
})
;"##### ; "10")]
#[test_case(r#####"tw`divide-opacity-80`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.8",
  },
})
;"##### ; "11")]
#[test_case(r#####"tw`divide-opacity-90`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.9",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`divide-opacity-95`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "0.95",
  },
})
;"##### ; "13")]
#[test_case(r#####"tw`divide-opacity-100`"#####, r#####"({
  "> :not([hidden]) ~ :not([hidden])": {
    "--tw-divide-opacity": "1",
  },
})"##### ; "14")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
