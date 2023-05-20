use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`text-opacity-0`"#####, r#####"({
  '--tw-text-opacity': "0",
})
;"##### ; "0")]
#[test_case(r#####"tw`text-opacity-5`"#####, r#####"({
  '--tw-text-opacity': "0.05",
})
;"##### ; "1")]
#[test_case(r#####"tw`text-opacity-10`"#####, r#####"({
  '--tw-text-opacity': "0.1",
})
;"##### ; "2")]
#[test_case(r#####"tw`text-opacity-20`"#####, r#####"({
  '--tw-text-opacity': "0.2",
})
;"##### ; "3")]
#[test_case(r#####"tw`text-opacity-25`"#####, r#####"({
  '--tw-text-opacity': "0.25",
})
;"##### ; "4")]
#[test_case(r#####"tw`text-opacity-30`"#####, r#####"({
  '--tw-text-opacity': "0.3",
})
;"##### ; "5")]
#[test_case(r#####"tw`text-opacity-40`"#####, r#####"({
  '--tw-text-opacity': "0.4",
})
;"##### ; "6")]
#[test_case(r#####"tw`text-opacity-50`"#####, r#####"({
  '--tw-text-opacity': "0.5",
})
;"##### ; "7")]
#[test_case(r#####"tw`text-opacity-60`"#####, r#####"({
  '--tw-text-opacity': "0.6",
})
;"##### ; "8")]
#[test_case(r#####"tw`text-opacity-70`"#####, r#####"({
  '--tw-text-opacity': "0.7",
})
;"##### ; "9")]
#[test_case(r#####"tw`text-opacity-75`"#####, r#####"({
  '--tw-text-opacity': "0.75",
})
;"##### ; "10")]
#[test_case(r#####"tw`text-opacity-80`"#####, r#####"({
  '--tw-text-opacity': "0.8",
})
;"##### ; "11")]
#[test_case(r#####"tw`text-opacity-90`"#####, r#####"({
  '--tw-text-opacity': "0.9",
})
;"##### ; "12")]
#[test_case(r#####"tw`text-opacity-95`"#####, r#####"({
  '--tw-text-opacity': "0.95",
})
;"##### ; "13")]
#[test_case(r#####"tw`text-opacity-100`"#####, r#####"({
  '--tw-text-opacity': "1",
})"##### ; "14")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
