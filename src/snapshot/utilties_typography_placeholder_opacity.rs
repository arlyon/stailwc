use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`placeholder-opacity-0`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`placeholder-opacity-5`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.05",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`placeholder-opacity-10`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.1",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`placeholder-opacity-20`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.2",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`placeholder-opacity-25`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.25",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`placeholder-opacity-30`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.3",
  },
})
;"##### ; "5")]
#[test_case(r#####"tw`placeholder-opacity-40`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.4",
  },
})
;"##### ; "6")]
#[test_case(r#####"tw`placeholder-opacity-50`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.5",
  },
})
;"##### ; "7")]
#[test_case(r#####"tw`placeholder-opacity-60`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.6",
  },
})
;"##### ; "8")]
#[test_case(r#####"tw`placeholder-opacity-70`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.7",
  },
})
;"##### ; "9")]
#[test_case(r#####"tw`placeholder-opacity-75`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.75",
  },
})
;"##### ; "10")]
#[test_case(r#####"tw`placeholder-opacity-80`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.8",
  },
})
;"##### ; "11")]
#[test_case(r#####"tw`placeholder-opacity-90`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.9",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`placeholder-opacity-95`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "0.95",
  },
})
;"##### ; "13")]
#[test_case(r#####"tw`placeholder-opacity-100`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "1",
  },
})
;"##### ; "14")]
#[test_case(r#####"tw`placeholder-opacity-[var(--placeholder-opacity)]`"#####, r#####"({
  '::placeholder': {
    '--tw-placeholder-opacity': "var(--placeholder-opacity)",
  },
})"##### ; "15")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
