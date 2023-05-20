use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
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
;"##### ; "1")]
#[test_case(r#####"tw`col-auto`"#####, r#####"({
  gridColumn: "auto",
})
;"##### ; "2")]
#[test_case(r#####"tw`col-span-1`"#####, r#####"({
  gridColumn: "span 1 / span 1",
})
;"##### ; "3")]
#[test_case(r#####"tw`col-span-2`"#####, r#####"({
  gridColumn: "span 2 / span 2",
})
;"##### ; "4")]
#[test_case(r#####"tw`col-span-3`"#####, r#####"({
  gridColumn: "span 3 / span 3",
})
;"##### ; "5")]
#[test_case(r#####"tw`col-span-4`"#####, r#####"({
  gridColumn: "span 4 / span 4",
})
;"##### ; "6")]
#[test_case(r#####"tw`col-span-5`"#####, r#####"({
  gridColumn: "span 5 / span 5",
})
;"##### ; "7")]
#[test_case(r#####"tw`col-span-6`"#####, r#####"({
  gridColumn: "span 6 / span 6",
})
;"##### ; "8")]
#[test_case(r#####"tw`col-span-7`"#####, r#####"({
  gridColumn: "span 7 / span 7",
})
;"##### ; "9")]
#[test_case(r#####"tw`col-span-8`"#####, r#####"({
  gridColumn: "span 8 / span 8",
})
;"##### ; "10")]
#[test_case(r#####"tw`col-span-9`"#####, r#####"({
  gridColumn: "span 9 / span 9",
})
;"##### ; "11")]
#[test_case(r#####"tw`col-span-10`"#####, r#####"({
  gridColumn: "span 10 / span 10",
})
;"##### ; "12")]
#[test_case(r#####"tw`col-span-11`"#####, r#####"({
  gridColumn: "span 11 / span 11",
})
;"##### ; "13")]
#[test_case(r#####"tw`col-span-12`"#####, r#####"({
  gridColumn: "span 12 / span 12",
})
;"##### ; "14")]
#[test_case(r#####"tw`col-span-full`"#####, r#####"({
  gridColumn: "1 / -1",
})
;"##### ; "15")]
#[test_case(r#####"tw`col-start-1`"#####, r#####"({
  gridColumnStart: "1",
})
;"##### ; "16")]
#[test_case(r#####"tw`col-start-2`"#####, r#####"({
  gridColumnStart: "2",
})
;"##### ; "17")]
#[test_case(r#####"tw`col-start-3`"#####, r#####"({
  gridColumnStart: "3",
})
;"##### ; "18")]
#[test_case(r#####"tw`col-start-4`"#####, r#####"({
  gridColumnStart: "4",
})
;"##### ; "19")]
#[test_case(r#####"tw`col-start-5`"#####, r#####"({
  gridColumnStart: "5",
})
;"##### ; "20")]
#[test_case(r#####"tw`col-start-6`"#####, r#####"({
  gridColumnStart: "6",
})
;"##### ; "21")]
#[test_case(r#####"tw`col-start-7`"#####, r#####"({
  gridColumnStart: "7",
})
;"##### ; "22")]
#[test_case(r#####"tw`col-start-8`"#####, r#####"({
  gridColumnStart: "8",
})
;"##### ; "23")]
#[test_case(r#####"tw`col-start-9`"#####, r#####"({
  gridColumnStart: "9",
})
;"##### ; "24")]
#[test_case(r#####"tw`col-start-10`"#####, r#####"({
  gridColumnStart: "10",
})
;"##### ; "25")]
#[test_case(r#####"tw`col-start-11`"#####, r#####"({
  gridColumnStart: "11",
})
;"##### ; "26")]
#[test_case(r#####"tw`col-start-12`"#####, r#####"({
  gridColumnStart: "12",
})
;"##### ; "27")]
#[test_case(r#####"tw`col-start-13`"#####, r#####"({
  gridColumnStart: "13",
})
;"##### ; "28")]
#[test_case(r#####"tw`col-start-auto`"#####, r#####"({
  gridColumnStart: "auto",
})
;"##### ; "29")]
#[test_case(r#####"tw`col-end-1`"#####, r#####"({
  gridColumnEnd: "1",
})
;"##### ; "30")]
#[test_case(r#####"tw`col-end-2`"#####, r#####"({
  gridColumnEnd: "2",
})
;"##### ; "31")]
#[test_case(r#####"tw`col-end-3`"#####, r#####"({
  gridColumnEnd: "3",
})
;"##### ; "32")]
#[test_case(r#####"tw`col-end-4`"#####, r#####"({
  gridColumnEnd: "4",
})
;"##### ; "33")]
#[test_case(r#####"tw`col-end-5`"#####, r#####"({
  gridColumnEnd: "5",
})
;"##### ; "34")]
#[test_case(r#####"tw`col-end-6`"#####, r#####"({
  gridColumnEnd: "6",
})
;"##### ; "35")]
#[test_case(r#####"tw`col-end-7`"#####, r#####"({
  gridColumnEnd: "7",
})
;"##### ; "36")]
#[test_case(r#####"tw`col-end-8`"#####, r#####"({
  gridColumnEnd: "8",
})
;"##### ; "37")]
#[test_case(r#####"tw`col-end-9`"#####, r#####"({
  gridColumnEnd: "9",
})
;"##### ; "38")]
#[test_case(r#####"tw`col-end-10`"#####, r#####"({
  gridColumnEnd: "10",
})
;"##### ; "39")]
#[test_case(r#####"tw`col-end-11`"#####, r#####"({
  gridColumnEnd: "11",
})
;"##### ; "40")]
#[test_case(r#####"tw`col-end-12`"#####, r#####"({
  gridColumnEnd: "12",
})
;"##### ; "41")]
#[test_case(r#####"tw`col-end-13`"#####, r#####"({
  gridColumnEnd: "13",
})
;"##### ; "42")]
#[test_case(r#####"tw`col-end-auto`"#####, r#####"({
  gridColumnEnd: "auto",
})
;"##### ; "43")]
#[test_case(r#####"tw`col-[7]`"#####, r#####"({
  gridColumn: "7",
})
;"##### ; "44")]
#[test_case(r#####"tw`col-end-[7]`"#####, r#####"({
  gridColumnEnd: "7",
})
;"##### ; "45")]
#[test_case(r#####"tw`col-start-[7]`"#####, r#####"({
  gridColumnStart: "7",
})"##### ; "46")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
