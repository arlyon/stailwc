use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`bg-sky-500/100`"#####, r#####"({
  backgroundColor: "rgb(14 165 233 / 1)",
})
;"##### ; "0")]
#[test_case(r#####"tw`bg-sky-500/75`"#####, r#####"({
  backgroundColor: "rgb(14 165 233 / 0.75)",
})
;"##### ; "1")]
#[test_case(r#####"tw`bg-sky-500/50`"#####, r#####"({
  backgroundColor: "rgb(14 165 233 / 0.5)",
})
;"##### ; "2")]
#[test_case(r#####"tw`bg-sky-500/[.06]`"#####, r#####"({
  backgroundColor: "rgb(14 165 233 / .06)",
})
;"##### ; "3")]
#[test_case(r#####"tw`bg-opacity-0`"#####, r#####"({
  '--tw-bg-opacity': "0",
})
;"##### ; "4")]
#[test_case(r#####"tw`bg-opacity-5`"#####, r#####"({
  '--tw-bg-opacity': "0.05",
})
;"##### ; "5")]
#[test_case(r#####"tw`bg-opacity-10`"#####, r#####"({
  '--tw-bg-opacity': "0.1",
})
;"##### ; "6")]
#[test_case(r#####"tw`bg-opacity-20`"#####, r#####"({
  '--tw-bg-opacity': "0.2",
})
;"##### ; "7")]
#[test_case(r#####"tw`bg-opacity-25`"#####, r#####"({
  '--tw-bg-opacity': "0.25",
})
;"##### ; "8")]
#[test_case(r#####"tw`bg-opacity-30`"#####, r#####"({
  '--tw-bg-opacity': "0.3",
})
;"##### ; "9")]
#[test_case(r#####"tw`bg-opacity-40`"#####, r#####"({
  '--tw-bg-opacity': "0.4",
})
;"##### ; "10")]
#[test_case(r#####"tw`bg-opacity-50`"#####, r#####"({
  '--tw-bg-opacity': "0.5",
})
;"##### ; "11")]
#[test_case(r#####"tw`bg-opacity-60`"#####, r#####"({
  '--tw-bg-opacity': "0.6",
})
;"##### ; "12")]
#[test_case(r#####"tw`bg-opacity-70`"#####, r#####"({
  '--tw-bg-opacity': "0.7",
})
;"##### ; "13")]
#[test_case(r#####"tw`bg-opacity-75`"#####, r#####"({
  '--tw-bg-opacity': "0.75",
})
;"##### ; "14")]
#[test_case(r#####"tw`bg-opacity-80`"#####, r#####"({
  '--tw-bg-opacity': "0.8",
})
;"##### ; "15")]
#[test_case(r#####"tw`bg-opacity-90`"#####, r#####"({
  '--tw-bg-opacity': "0.9",
})
;"##### ; "16")]
#[test_case(r#####"tw`bg-opacity-95`"#####, r#####"({
  '--tw-bg-opacity': "0.95",
})
;"##### ; "17")]
#[test_case(r#####"tw`bg-opacity-100`"#####, r#####"({
  '--tw-bg-opacity': "1",
})
;"##### ; "18")]
#[test_case(r#####"tw`bg-opacity-[0.11]`"#####, r#####"({
  '--tw-bg-opacity': "0.11",
})
;"##### ; "19")]
#[test_case(r#####"tw`bg-opacity-[var(--value)]`"#####, r#####"({
  '--tw-bg-opacity': "var(--value)",
})"##### ; "20")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
