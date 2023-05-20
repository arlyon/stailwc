use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`gridColumn`"#####, r#####"({
  auto: "auto",
  'span-1': "span 1 / span 1",
  'span-2': "span 2 / span 2",
  'span-3': "span 3 / span 3",
  'span-4': "span 4 / span 4",
  'span-5': "span 5 / span 5",
  'span-6': "span 6 / span 6",
  'span-7': "span 7 / span 7",
  'span-8': "span 8 / span 8",
  'span-9': "span 9 / span 9",
  'span-10': "span 10 / span 10",
  'span-11': "span 11 / span 11",
  'span-12': "span 12 / span 12",
  'span-full': "1 / -1",
})
;"##### ; "0")]
#[test_case(r#####"tw`col-auto`"#####, r#####"({
  gridColumn: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`col-span-1`"#####, r#####"({
  gridColumn: "span 1 / span 1",
})
;"##### ; "2")]
#[test_case(r#####"tw`col-span-2`"#####, r#####"({
  gridColumn: "span 2 / span 2",
})
;"##### ; "3")]
#[test_case(r#####"tw`col-span-3`"#####, r#####"({
  gridColumn: "span 3 / span 3",
})
;"##### ; "4")]
#[test_case(r#####"tw`col-span-4`"#####, r#####"({
  gridColumn: "span 4 / span 4",
})
;"##### ; "5")]
#[test_case(r#####"tw`col-span-5`"#####, r#####"({
  gridColumn: "span 5 / span 5",
})
;"##### ; "6")]
#[test_case(r#####"tw`col-span-6`"#####, r#####"({
  gridColumn: "span 6 / span 6",
})
;"##### ; "7")]
#[test_case(r#####"tw`col-span-7`"#####, r#####"({
  gridColumn: "span 7 / span 7",
})
;"##### ; "8")]
#[test_case(r#####"tw`col-span-8`"#####, r#####"({
  gridColumn: "span 8 / span 8",
})
;"##### ; "9")]
#[test_case(r#####"tw`col-span-9`"#####, r#####"({
  gridColumn: "span 9 / span 9",
})
;"##### ; "10")]
#[test_case(r#####"tw`col-span-10`"#####, r#####"({
  gridColumn: "span 10 / span 10",
})
;"##### ; "11")]
#[test_case(r#####"tw`col-span-11`"#####, r#####"({
  gridColumn: "span 11 / span 11",
})
;"##### ; "12")]
#[test_case(r#####"tw`col-span-12`"#####, r#####"({
  gridColumn: "span 12 / span 12",
})
;"##### ; "13")]
#[test_case(r#####"tw`col-span-full`"#####, r#####"({
  gridColumn: "1 / -1",
})
;"##### ; "14")]
#[test_case(r#####"tw`col-start-1`"#####, r#####"({
  gridColumnStart: "1",
})
;"##### ; "15")]
#[test_case(r#####"tw`col-start-2`"#####, r#####"({
  gridColumnStart: "2",
})
;"##### ; "16")]
#[test_case(r#####"tw`col-start-3`"#####, r#####"({
  gridColumnStart: "3",
})
;"##### ; "17")]
#[test_case(r#####"tw`col-start-4`"#####, r#####"({
  gridColumnStart: "4",
})
;"##### ; "18")]
#[test_case(r#####"tw`col-start-5`"#####, r#####"({
  gridColumnStart: "5",
})
;"##### ; "19")]
#[test_case(r#####"tw`col-start-6`"#####, r#####"({
  gridColumnStart: "6",
})
;"##### ; "20")]
#[test_case(r#####"tw`col-start-7`"#####, r#####"({
  gridColumnStart: "7",
})
;"##### ; "21")]
#[test_case(r#####"tw`col-start-8`"#####, r#####"({
  gridColumnStart: "8",
})
;"##### ; "22")]
#[test_case(r#####"tw`col-start-9`"#####, r#####"({
  gridColumnStart: "9",
})
;"##### ; "23")]
#[test_case(r#####"tw`col-start-10`"#####, r#####"({
  gridColumnStart: "10",
})
;"##### ; "24")]
#[test_case(r#####"tw`col-start-11`"#####, r#####"({
  gridColumnStart: "11",
})
;"##### ; "25")]
#[test_case(r#####"tw`col-start-12`"#####, r#####"({
  gridColumnStart: "12",
})
;"##### ; "26")]
#[test_case(r#####"tw`col-start-13`"#####, r#####"({
  gridColumnStart: "13",
})
;"##### ; "27")]
#[test_case(r#####"tw`col-start-auto`"#####, r#####"({
  gridColumnStart: "auto",
})
;"##### ; "28")]
#[test_case(r#####"tw`col-end-1`"#####, r#####"({
  gridColumnEnd: "1",
})
;"##### ; "29")]
#[test_case(r#####"tw`col-end-2`"#####, r#####"({
  gridColumnEnd: "2",
})
;"##### ; "30")]
#[test_case(r#####"tw`col-end-3`"#####, r#####"({
  gridColumnEnd: "3",
})
;"##### ; "31")]
#[test_case(r#####"tw`col-end-4`"#####, r#####"({
  gridColumnEnd: "4",
})
;"##### ; "32")]
#[test_case(r#####"tw`col-end-5`"#####, r#####"({
  gridColumnEnd: "5",
})
;"##### ; "33")]
#[test_case(r#####"tw`col-end-6`"#####, r#####"({
  gridColumnEnd: "6",
})
;"##### ; "34")]
#[test_case(r#####"tw`col-end-7`"#####, r#####"({
  gridColumnEnd: "7",
})
;"##### ; "35")]
#[test_case(r#####"tw`col-end-8`"#####, r#####"({
  gridColumnEnd: "8",
})
;"##### ; "36")]
#[test_case(r#####"tw`col-end-9`"#####, r#####"({
  gridColumnEnd: "9",
})
;"##### ; "37")]
#[test_case(r#####"tw`col-end-10`"#####, r#####"({
  gridColumnEnd: "10",
})
;"##### ; "38")]
#[test_case(r#####"tw`col-end-11`"#####, r#####"({
  gridColumnEnd: "11",
})
;"##### ; "39")]
#[test_case(r#####"tw`col-end-12`"#####, r#####"({
  gridColumnEnd: "12",
})
;"##### ; "40")]
#[test_case(r#####"tw`col-end-13`"#####, r#####"({
  gridColumnEnd: "13",
})
;"##### ; "41")]
#[test_case(r#####"tw`col-end-auto`"#####, r#####"({
  gridColumnEnd: "auto",
})
;"##### ; "42")]
#[test_case(r#####"tw`col-[7]`"#####, r#####"({
  gridColumn: "7",
})
;"##### ; "43")]
#[test_case(r#####"tw`col-end-[7]`"#####, r#####"({
  gridColumnEnd: "7",
})
;"##### ; "44")]
#[test_case(r#####"tw`col-start-[7]`"#####, r#####"({
  gridColumnStart: "7",
})"##### ; "45")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
