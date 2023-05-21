use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`border-opacity-0`"#####, r#####"({
  "--tw-border-opacity": "0",
})
;"##### ; "0")]
#[test_case(r#####"tw`border-opacity-5`"#####, r#####"({
  "--tw-border-opacity": "0.05",
})
;"##### ; "1")]
#[test_case(r#####"tw`border-opacity-10`"#####, r#####"({
  "--tw-border-opacity": "0.1",
})
;"##### ; "2")]
#[test_case(r#####"tw`border-opacity-20`"#####, r#####"({
  "--tw-border-opacity": "0.2",
})
;"##### ; "3")]
#[test_case(r#####"tw`border-opacity-25`"#####, r#####"({
  "--tw-border-opacity": "0.25",
})
;"##### ; "4")]
#[test_case(r#####"tw`border-opacity-30`"#####, r#####"({
  "--tw-border-opacity": "0.3",
})
;"##### ; "5")]
#[test_case(r#####"tw`border-opacity-40`"#####, r#####"({
  "--tw-border-opacity": "0.4",
})
;"##### ; "6")]
#[test_case(r#####"tw`border-opacity-50`"#####, r#####"({
  "--tw-border-opacity": "0.5",
})
;"##### ; "7")]
#[test_case(r#####"tw`border-opacity-60`"#####, r#####"({
  "--tw-border-opacity": "0.6",
})
;"##### ; "8")]
#[test_case(r#####"tw`border-opacity-70`"#####, r#####"({
  "--tw-border-opacity": "0.7",
})
;"##### ; "9")]
#[test_case(r#####"tw`border-opacity-75`"#####, r#####"({
  "--tw-border-opacity": "0.75",
})
;"##### ; "10")]
#[test_case(r#####"tw`border-opacity-80`"#####, r#####"({
  "--tw-border-opacity": "0.8",
})
;"##### ; "11")]
#[test_case(r#####"tw`border-opacity-90`"#####, r#####"({
  "--tw-border-opacity": "0.9",
})
;"##### ; "12")]
#[test_case(r#####"tw`border-opacity-95`"#####, r#####"({
  "--tw-border-opacity": "0.95",
})
;"##### ; "13")]
#[test_case(r#####"tw`border-opacity-100`"#####, r#####"({
  "--tw-border-opacity": "1",
})"##### ; "14")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
