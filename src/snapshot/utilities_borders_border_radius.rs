use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`borderRadius.`"#####, r#####"({
  none: "0px",
  sm: "0.125rem",
  DEFAULT: "0.25rem",
  md: "0.375rem",
  lg: "0.5rem",
  xl: "0.75rem",
  "2xl": "1rem",
  "3xl": "1.5rem",
  full: "9999px",
})
;"##### ; "0")]
#[test_case(r#####"tw`rounded-none`"#####, r#####"({
  borderRadius: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`rounded-sm`"#####, r#####"({
  borderRadius: "0.125rem",
})
;"##### ; "2")]
#[test_case(r#####"tw`rounded`"#####, r#####"({
  borderRadius: "0.25rem",
})
;"##### ; "3")]
#[test_case(r#####"tw`rounded-md`"#####, r#####"({
  borderRadius: "0.375rem",
})
;"##### ; "4")]
#[test_case(r#####"tw`rounded-lg`"#####, r#####"({
  borderRadius: "0.5rem",
})
;"##### ; "5")]
#[test_case(r#####"tw`rounded-xl`"#####, r#####"({
  borderRadius: "0.75rem",
})
;"##### ; "6")]
#[test_case(r#####"tw`rounded-2xl`"#####, r#####"({
  borderRadius: "1rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`rounded-3xl`"#####, r#####"({
  borderRadius: "1.5rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`rounded-full`"#####, r#####"({
  borderRadius: "9999px",
})
;"##### ; "9")]
#[test_case(r#####"tw`rounded-t-none`"#####, r#####"({
  borderTopLeftRadius: "0px",
  borderTopRightRadius: "0px",
})
;"##### ; "10")]
#[test_case(r#####"tw`rounded-r-none`"#####, r#####"({
  borderTopRightRadius: "0px",
  borderBottomRightRadius: "0px",
})
;"##### ; "11")]
#[test_case(r#####"tw`rounded-b-none`"#####, r#####"({
  borderBottomRightRadius: "0px",
  borderBottomLeftRadius: "0px",
})
;"##### ; "12")]
#[test_case(r#####"tw`rounded-l-none`"#####, r#####"({
  borderTopLeftRadius: "0px",
  borderBottomLeftRadius: "0px",
})
;"##### ; "13")]
#[test_case(r#####"tw`rounded-t-sm`"#####, r#####"({
  borderTopLeftRadius: "0.125rem",
  borderTopRightRadius: "0.125rem",
})
;"##### ; "14")]
#[test_case(r#####"tw`rounded-r-sm`"#####, r#####"({
  borderTopRightRadius: "0.125rem",
  borderBottomRightRadius: "0.125rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`rounded-b-sm`"#####, r#####"({
  borderBottomRightRadius: "0.125rem",
  borderBottomLeftRadius: "0.125rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`rounded-l-sm`"#####, r#####"({
  borderTopLeftRadius: "0.125rem",
  borderBottomLeftRadius: "0.125rem",
})
;"##### ; "17")]
#[test_case(r#####"tw`rounded-t`"#####, r#####"({
  borderTopLeftRadius: "0.25rem",
  borderTopRightRadius: "0.25rem",
})
;"##### ; "18")]
#[test_case(r#####"tw`rounded-r`"#####, r#####"({
  borderTopRightRadius: "0.25rem",
  borderBottomRightRadius: "0.25rem",
})
;"##### ; "19")]
#[test_case(r#####"tw`rounded-b`"#####, r#####"({
  borderBottomRightRadius: "0.25rem",
  borderBottomLeftRadius: "0.25rem",
})
;"##### ; "20")]
#[test_case(r#####"tw`rounded-l`"#####, r#####"({
  borderTopLeftRadius: "0.25rem",
  borderBottomLeftRadius: "0.25rem",
})
;"##### ; "21")]
#[test_case(r#####"tw`rounded-t-lg`"#####, r#####"({
  borderTopLeftRadius: "0.5rem",
  borderTopRightRadius: "0.5rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`rounded-r-lg`"#####, r#####"({
  borderTopRightRadius: "0.5rem",
  borderBottomRightRadius: "0.5rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`rounded-b-lg`"#####, r#####"({
  borderBottomRightRadius: "0.5rem",
  borderBottomLeftRadius: "0.5rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`rounded-l-lg`"#####, r#####"({
  borderTopLeftRadius: "0.5rem",
  borderBottomLeftRadius: "0.5rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`rounded-t-xl`"#####, r#####"({
  borderTopLeftRadius: "0.75rem",
  borderTopRightRadius: "0.75rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`rounded-r-xl`"#####, r#####"({
  borderTopRightRadius: "0.75rem",
  borderBottomRightRadius: "0.75rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`rounded-b-xl`"#####, r#####"({
  borderBottomRightRadius: "0.75rem",
  borderBottomLeftRadius: "0.75rem",
})
;"##### ; "28")]
#[test_case(r#####"tw`rounded-l-xl`"#####, r#####"({
  borderTopLeftRadius: "0.75rem",
  borderBottomLeftRadius: "0.75rem",
})
;"##### ; "29")]
#[test_case(r#####"tw`rounded-t-2xl`"#####, r#####"({
  borderTopLeftRadius: "1rem",
  borderTopRightRadius: "1rem",
})
;"##### ; "30")]
#[test_case(r#####"tw`rounded-r-2xl`"#####, r#####"({
  borderTopRightRadius: "1rem",
  borderBottomRightRadius: "1rem",
})
;"##### ; "31")]
#[test_case(r#####"tw`rounded-b-2xl`"#####, r#####"({
  borderBottomRightRadius: "1rem",
  borderBottomLeftRadius: "1rem",
})
;"##### ; "32")]
#[test_case(r#####"tw`rounded-l-2xl`"#####, r#####"({
  borderTopLeftRadius: "1rem",
  borderBottomLeftRadius: "1rem",
})
;"##### ; "33")]
#[test_case(r#####"tw`rounded-t-3xl`"#####, r#####"({
  borderTopLeftRadius: "1.5rem",
  borderTopRightRadius: "1.5rem",
})
;"##### ; "34")]
#[test_case(r#####"tw`rounded-r-3xl`"#####, r#####"({
  borderTopRightRadius: "1.5rem",
  borderBottomRightRadius: "1.5rem",
})
;"##### ; "35")]
#[test_case(r#####"tw`rounded-b-3xl`"#####, r#####"({
  borderBottomRightRadius: "1.5rem",
  borderBottomLeftRadius: "1.5rem",
})
;"##### ; "36")]
#[test_case(r#####"tw`rounded-l-3xl`"#####, r#####"({
  borderTopLeftRadius: "1.5rem",
  borderBottomLeftRadius: "1.5rem",
})
;"##### ; "37")]
#[test_case(r#####"tw`rounded-t-full`"#####, r#####"({
  borderTopLeftRadius: "9999px",
  borderTopRightRadius: "9999px",
})
;"##### ; "38")]
#[test_case(r#####"tw`rounded-r-full`"#####, r#####"({
  borderTopRightRadius: "9999px",
  borderBottomRightRadius: "9999px",
})
;"##### ; "39")]
#[test_case(r#####"tw`rounded-b-full`"#####, r#####"({
  borderBottomRightRadius: "9999px",
  borderBottomLeftRadius: "9999px",
})
;"##### ; "40")]
#[test_case(r#####"tw`rounded-l-full`"#####, r#####"({
  borderTopLeftRadius: "9999px",
  borderBottomLeftRadius: "9999px",
})
;"##### ; "41")]
#[test_case(r#####"tw`rounded-tl-none`"#####, r#####"({
  borderTopLeftRadius: "0px",
})
;"##### ; "42")]
#[test_case(r#####"tw`rounded-tr-none`"#####, r#####"({
  borderTopRightRadius: "0px",
})
;"##### ; "43")]
#[test_case(r#####"tw`rounded-br-none`"#####, r#####"({
  borderBottomRightRadius: "0px",
})
;"##### ; "44")]
#[test_case(r#####"tw`rounded-bl-none`"#####, r#####"({
  borderBottomLeftRadius: "0px",
})
;"##### ; "45")]
#[test_case(r#####"tw`rounded-tl-sm`"#####, r#####"({
  borderTopLeftRadius: "0.125rem",
})
;"##### ; "46")]
#[test_case(r#####"tw`rounded-tr-sm`"#####, r#####"({
  borderTopRightRadius: "0.125rem",
})
;"##### ; "47")]
#[test_case(r#####"tw`rounded-br-sm`"#####, r#####"({
  borderBottomRightRadius: "0.125rem",
})
;"##### ; "48")]
#[test_case(r#####"tw`rounded-bl-sm`"#####, r#####"({
  borderBottomLeftRadius: "0.125rem",
})
;"##### ; "49")]
#[test_case(r#####"tw`rounded-tl`"#####, r#####"({
  borderTopLeftRadius: "0.25rem",
})
;"##### ; "50")]
#[test_case(r#####"tw`rounded-tr`"#####, r#####"({
  borderTopRightRadius: "0.25rem",
})
;"##### ; "51")]
#[test_case(r#####"tw`rounded-br`"#####, r#####"({
  borderBottomRightRadius: "0.25rem",
})
;"##### ; "52")]
#[test_case(r#####"tw`rounded-bl`"#####, r#####"({
  borderBottomLeftRadius: "0.25rem",
})
;"##### ; "53")]
#[test_case(r#####"tw`rounded-tl-lg`"#####, r#####"({
  borderTopLeftRadius: "0.5rem",
})
;"##### ; "54")]
#[test_case(r#####"tw`rounded-tr-lg`"#####, r#####"({
  borderTopRightRadius: "0.5rem",
})
;"##### ; "55")]
#[test_case(r#####"tw`rounded-br-lg`"#####, r#####"({
  borderBottomRightRadius: "0.5rem",
})
;"##### ; "56")]
#[test_case(r#####"tw`rounded-bl-lg`"#####, r#####"({
  borderBottomLeftRadius: "0.5rem",
})
;"##### ; "57")]
#[test_case(r#####"tw`rounded-tl-xl`"#####, r#####"({
  borderTopLeftRadius: "0.75rem",
})
;"##### ; "58")]
#[test_case(r#####"tw`rounded-tr-xl`"#####, r#####"({
  borderTopRightRadius: "0.75rem",
})
;"##### ; "59")]
#[test_case(r#####"tw`rounded-br-xl`"#####, r#####"({
  borderBottomRightRadius: "0.75rem",
})
;"##### ; "60")]
#[test_case(r#####"tw`rounded-bl-xl`"#####, r#####"({
  borderBottomLeftRadius: "0.75rem",
})
;"##### ; "61")]
#[test_case(r#####"tw`rounded-tl-2xl`"#####, r#####"({
  borderTopLeftRadius: "1rem",
})
;"##### ; "62")]
#[test_case(r#####"tw`rounded-tr-2xl`"#####, r#####"({
  borderTopRightRadius: "1rem",
})
;"##### ; "63")]
#[test_case(r#####"tw`rounded-br-2xl`"#####, r#####"({
  borderBottomRightRadius: "1rem",
})
;"##### ; "64")]
#[test_case(r#####"tw`rounded-bl-2xl`"#####, r#####"({
  borderBottomLeftRadius: "1rem",
})
;"##### ; "65")]
#[test_case(r#####"tw`rounded-tl-3xl`"#####, r#####"({
  borderTopLeftRadius: "1.5rem",
})
;"##### ; "66")]
#[test_case(r#####"tw`rounded-tr-3xl`"#####, r#####"({
  borderTopRightRadius: "1.5rem",
})
;"##### ; "67")]
#[test_case(r#####"tw`rounded-br-3xl`"#####, r#####"({
  borderBottomRightRadius: "1.5rem",
})
;"##### ; "68")]
#[test_case(r#####"tw`rounded-bl-3xl`"#####, r#####"({
  borderBottomLeftRadius: "1.5rem",
})
;"##### ; "69")]
#[test_case(r#####"tw`rounded-tl-full`"#####, r#####"({
  borderTopLeftRadius: "9999px",
})
;"##### ; "70")]
#[test_case(r#####"tw`rounded-tr-full`"#####, r#####"({
  borderTopRightRadius: "9999px",
})
;"##### ; "71")]
#[test_case(r#####"tw`rounded-br-full`"#####, r#####"({
  borderBottomRightRadius: "9999px",
})
;"##### ; "72")]
#[test_case(r#####"tw`rounded-bl-full`"#####, r#####"({
  borderBottomLeftRadius: "9999px",
})
;"##### ; "73")]
#[test_case(r#####"tw`rounded-[12px]`"#####, r#####"({
  borderRadius: "12px",
})
;"##### ; "74")]
#[test_case(r#####"tw`rounded-t-[var(--radius)] rounded-r-[var(--radius)] rounded-b-[var(--radius)] rounded-l-[var(--radius)]`"#####, r#####"({
  borderBottomRightRadius: "var(--radius)",
  borderBottomLeftRadius: "var(--radius)",
  borderTopLeftRadius: "var(--radius)",
  borderTopRightRadius: "var(--radius)",
})
;"##### ; "75")]
#[test_case(r#####"tw`rounded-tr-[var(--radius)] rounded-br-[var(--radius)] rounded-bl-[var(--radius)] rounded-tl-[var(--radius)]`"#####, r#####"({
  borderBottomLeftRadius: "var(--radius)",
  borderBottomRightRadius: "var(--radius)",
  borderTopLeftRadius: "var(--radius)",
  borderTopRightRadius: "var(--radius)",
})
;"##### ; "76")]
#[test_case(r#####"tw`rounded-s rounded-e rounded-ss rounded-es`"#####, r#####"({
  borderStartEndRadius: "0.25rem",
  borderEndEndRadius: "0.25rem",
  borderStartStartRadius: "0.25rem",
  borderEndStartRadius: "0.25rem",
})"##### ; "77")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
