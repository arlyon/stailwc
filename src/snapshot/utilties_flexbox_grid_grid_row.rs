use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`gridRow`"#####, r#####"({
  auto: "auto",
  'span-1': "span 1 / span 1",
  'span-2': "span 2 / span 2",
  'span-3': "span 3 / span 3",
  'span-4': "span 4 / span 4",
  'span-5': "span 5 / span 5",
  'span-6': "span 6 / span 6",
  'span-full': "1 / -1",
})
;"##### ; "0")]
#[test_case(r#####"tw`row-auto`"#####, r#####"({
  gridRow: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`row-span-1`"#####, r#####"({
  gridRow: "span 1 / span 1",
})
;"##### ; "2")]
#[test_case(r#####"tw`row-span-2`"#####, r#####"({
  gridRow: "span 2 / span 2",
})
;"##### ; "3")]
#[test_case(r#####"tw`row-span-3`"#####, r#####"({
  gridRow: "span 3 / span 3",
})
;"##### ; "4")]
#[test_case(r#####"tw`row-span-4`"#####, r#####"({
  gridRow: "span 4 / span 4",
})
;"##### ; "5")]
#[test_case(r#####"tw`row-span-5`"#####, r#####"({
  gridRow: "span 5 / span 5",
})
;"##### ; "6")]
#[test_case(r#####"tw`row-span-6`"#####, r#####"({
  gridRow: "span 6 / span 6",
})
;"##### ; "7")]
#[test_case(r#####"tw`row-span-full`"#####, r#####"({
  gridRow: "1 / -1",
})
;"##### ; "8")]
#[test_case(r#####"tw`row-start-1`"#####, r#####"({
  gridRowStart: "1",
})
;"##### ; "9")]
#[test_case(r#####"tw`row-start-2`"#####, r#####"({
  gridRowStart: "2",
})
;"##### ; "10")]
#[test_case(r#####"tw`row-start-3`"#####, r#####"({
  gridRowStart: "3",
})
;"##### ; "11")]
#[test_case(r#####"tw`row-start-4`"#####, r#####"({
  gridRowStart: "4",
})
;"##### ; "12")]
#[test_case(r#####"tw`row-start-5`"#####, r#####"({
  gridRowStart: "5",
})
;"##### ; "13")]
#[test_case(r#####"tw`row-start-6`"#####, r#####"({
  gridRowStart: "6",
})
;"##### ; "14")]
#[test_case(r#####"tw`row-start-7`"#####, r#####"({
  gridRowStart: "7",
})
;"##### ; "15")]
#[test_case(r#####"tw`row-start-auto`"#####, r#####"({
  gridRowStart: "auto",
})
;"##### ; "16")]
#[test_case(r#####"tw`row-end-1`"#####, r#####"({
  gridRowEnd: "1",
})
;"##### ; "17")]
#[test_case(r#####"tw`row-end-2`"#####, r#####"({
  gridRowEnd: "2",
})
;"##### ; "18")]
#[test_case(r#####"tw`row-end-3`"#####, r#####"({
  gridRowEnd: "3",
})
;"##### ; "19")]
#[test_case(r#####"tw`row-end-4`"#####, r#####"({
  gridRowEnd: "4",
})
;"##### ; "20")]
#[test_case(r#####"tw`row-end-5`"#####, r#####"({
  gridRowEnd: "5",
})
;"##### ; "21")]
#[test_case(r#####"tw`row-end-6`"#####, r#####"({
  gridRowEnd: "6",
})
;"##### ; "22")]
#[test_case(r#####"tw`row-end-7`"#####, r#####"({
  gridRowEnd: "7",
})
;"##### ; "23")]
#[test_case(r#####"tw`row-end-auto`"#####, r#####"({
  gridRowEnd: "auto",
})
;"##### ; "24")]
#[test_case(r#####"tw`row-[span 16 / span 16]`"#####, r#####"({
  gridRow: "span 16 / span 16",
})
;"##### ; "25")]
#[test_case(r#####"tw`row-[7]`"#####, r#####"({
  gridRow: "7",
})
;"##### ; "26")]
#[test_case(r#####"tw`row-end-[7]`"#####, r#####"({
  gridRowEnd: "7",
})
;"##### ; "27")]
#[test_case(r#####"tw`row-start-[7]`"#####, r#####"({
  gridRowStart: "7",
})"##### ; "28")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
