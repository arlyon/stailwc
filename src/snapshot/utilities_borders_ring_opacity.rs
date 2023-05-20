use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`ring-opacity-0`"#####, r#####"({
  '--tw-ring-opacity': "0",
})
;"##### ; "1")]
#[test_case(r#####"tw`ring-opacity-5`"#####, r#####"({
  '--tw-ring-opacity': "0.05",
})
;"##### ; "2")]
#[test_case(r#####"tw`ring-opacity-10`"#####, r#####"({
  '--tw-ring-opacity': "0.1",
})
;"##### ; "3")]
#[test_case(r#####"tw`ring-opacity-20`"#####, r#####"({
  '--tw-ring-opacity': "0.2",
})
;"##### ; "4")]
#[test_case(r#####"tw`ring-opacity-25`"#####, r#####"({
  '--tw-ring-opacity': "0.25",
})
;"##### ; "5")]
#[test_case(r#####"tw`ring-opacity-30`"#####, r#####"({
  '--tw-ring-opacity': "0.3",
})
;"##### ; "6")]
#[test_case(r#####"tw`ring-opacity-40`"#####, r#####"({
  '--tw-ring-opacity': "0.4",
})
;"##### ; "7")]
#[test_case(r#####"tw`ring-opacity-50`"#####, r#####"({
  '--tw-ring-opacity': "0.5",
})
;"##### ; "8")]
#[test_case(r#####"tw`ring-opacity-60`"#####, r#####"({
  '--tw-ring-opacity': "0.6",
})
;"##### ; "9")]
#[test_case(r#####"tw`ring-opacity-70`"#####, r#####"({
  '--tw-ring-opacity': "0.7",
})
;"##### ; "10")]
#[test_case(r#####"tw`ring-opacity-75`"#####, r#####"({
  '--tw-ring-opacity': "0.75",
})
;"##### ; "11")]
#[test_case(r#####"tw`ring-opacity-80`"#####, r#####"({
  '--tw-ring-opacity': "0.8",
})
;"##### ; "12")]
#[test_case(r#####"tw`ring-opacity-90`"#####, r#####"({
  '--tw-ring-opacity': "0.9",
})
;"##### ; "13")]
#[test_case(r#####"tw`ring-opacity-95`"#####, r#####"({
  '--tw-ring-opacity': "0.95",
})
;"##### ; "14")]
#[test_case(r#####"tw`ring-opacity-100`"#####, r#####"({
  '--tw-ring-opacity': "1",
})
;"##### ; "15")]
#[test_case(r#####"tw`ring-opacity-[var(--ring-opacity)]`"#####, r#####"({
  '--tw-ring-opacity': "var(--ring-opacity)",
})"##### ; "16")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
