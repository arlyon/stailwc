use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`columns`"#####, r#####"({
  1: "1",
  2: "2",
  3: "3",
  4: "4",
  5: "5",
  6: "6",
  7: "7",
  8: "8",
  9: "9",
  10: "10",
  11: "11",
  12: "12",
  auto: "auto",
  '3xs': "16rem",
  '2xs': "18rem",
  xs: "20rem",
  sm: "24rem",
  md: "28rem",
  lg: "32rem",
  xl: "36rem",
  '2xl': "42rem",
  '3xl': "48rem",
  '4xl': "56rem",
  '5xl': "64rem",
  '6xl': "72rem",
  '7xl': "80rem",
})
;"##### ; "1")]
#[test_case(r#####"tw`columns-1`"#####, r#####"({
  columns: "1",
})
;"##### ; "2")]
#[test_case(r#####"tw`columns-2`"#####, r#####"({
  columns: "2",
})
;"##### ; "3")]
#[test_case(r#####"tw`columns-3`"#####, r#####"({
  columns: "3",
})
;"##### ; "4")]
#[test_case(r#####"tw`columns-4`"#####, r#####"({
  columns: "4",
})
;"##### ; "5")]
#[test_case(r#####"tw`columns-5`"#####, r#####"({
  columns: "5",
})
;"##### ; "6")]
#[test_case(r#####"tw`columns-6`"#####, r#####"({
  columns: "6",
})
;"##### ; "7")]
#[test_case(r#####"tw`columns-7`"#####, r#####"({
  columns: "7",
})
;"##### ; "8")]
#[test_case(r#####"tw`columns-8`"#####, r#####"({
  columns: "8",
})
;"##### ; "9")]
#[test_case(r#####"tw`columns-9`"#####, r#####"({
  columns: "9",
})
;"##### ; "10")]
#[test_case(r#####"tw`columns-10`"#####, r#####"({
  columns: "10",
})
;"##### ; "11")]
#[test_case(r#####"tw`columns-11`"#####, r#####"({
  columns: "11",
})
;"##### ; "12")]
#[test_case(r#####"tw`columns-12`"#####, r#####"({
  columns: "12",
})
;"##### ; "13")]
#[test_case(r#####"tw`columns-auto`"#####, r#####"({
  columns: "auto",
})
;"##### ; "14")]
#[test_case(r#####"tw`columns-3xs`"#####, r#####"({
  columns: "16rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`columns-2xs`"#####, r#####"({
  columns: "18rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`columns-xs`"#####, r#####"({
  columns: "20rem",
})
;"##### ; "17")]
#[test_case(r#####"tw`columns-sm`"#####, r#####"({
  columns: "24rem",
})
;"##### ; "18")]
#[test_case(r#####"tw`columns-md`"#####, r#####"({
  columns: "28rem",
})
;"##### ; "19")]
#[test_case(r#####"tw`columns-lg`"#####, r#####"({
  columns: "32rem",
})
;"##### ; "20")]
#[test_case(r#####"tw`columns-xl`"#####, r#####"({
  columns: "36rem",
})
;"##### ; "21")]
#[test_case(r#####"tw`columns-2xl`"#####, r#####"({
  columns: "42rem",
})
;"##### ; "22")]
#[test_case(r#####"tw`columns-3xl`"#####, r#####"({
  columns: "48rem",
})
;"##### ; "23")]
#[test_case(r#####"tw`columns-4xl`"#####, r#####"({
  columns: "56rem",
})
;"##### ; "24")]
#[test_case(r#####"tw`columns-5xl`"#####, r#####"({
  columns: "64rem",
})
;"##### ; "25")]
#[test_case(r#####"tw`columns-6xl`"#####, r#####"({
  columns: "72rem",
})
;"##### ; "26")]
#[test_case(r#####"tw`columns-7xl`"#####, r#####"({
  columns: "80rem",
})
;"##### ; "27")]
#[test_case(r#####"tw`columns-[10rem]`"#####, r#####"({
  columns: "10rem",
})"##### ; "28")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
