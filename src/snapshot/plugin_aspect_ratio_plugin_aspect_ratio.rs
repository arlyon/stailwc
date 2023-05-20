use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`aspect-w-1`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "1",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`aspect-h-1`"#####, r#####"({
  '--tw-aspect-h': "1",
})
;"##### ; "2")]
#[test_case(r#####"tw`aspect-w-2`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "2",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`aspect-h-2`"#####, r#####"({
  '--tw-aspect-h': "2",
})
;"##### ; "4")]
#[test_case(r#####"tw`aspect-w-3`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "3",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "5")]
#[test_case(r#####"tw`aspect-h-3`"#####, r#####"({
  '--tw-aspect-h': "3",
})
;"##### ; "6")]
#[test_case(r#####"tw`aspect-w-4`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "4",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "7")]
#[test_case(r#####"tw`aspect-h-4`"#####, r#####"({
  '--tw-aspect-h': "4",
})
;"##### ; "8")]
#[test_case(r#####"tw`aspect-w-5`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "5",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "9")]
#[test_case(r#####"tw`aspect-h-5`"#####, r#####"({
  '--tw-aspect-h': "5",
})
;"##### ; "10")]
#[test_case(r#####"tw`aspect-w-6`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "6",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "11")]
#[test_case(r#####"tw`aspect-h-6`"#####, r#####"({
  '--tw-aspect-h': "6",
})
;"##### ; "12")]
#[test_case(r#####"tw`aspect-w-7`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "7",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "13")]
#[test_case(r#####"tw`aspect-h-7`"#####, r#####"({
  '--tw-aspect-h': "7",
})
;"##### ; "14")]
#[test_case(r#####"tw`aspect-w-8`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "8",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "15")]
#[test_case(r#####"tw`aspect-h-8`"#####, r#####"({
  '--tw-aspect-h': "8",
})
;"##### ; "16")]
#[test_case(r#####"tw`aspect-w-9`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "9",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "17")]
#[test_case(r#####"tw`aspect-h-9`"#####, r#####"({
  '--tw-aspect-h': "9",
})
;"##### ; "18")]
#[test_case(r#####"tw`aspect-w-10`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "10",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "19")]
#[test_case(r#####"tw`aspect-h-10`"#####, r#####"({
  '--tw-aspect-h': "10",
})
;"##### ; "20")]
#[test_case(r#####"tw`aspect-w-11`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "11",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "21")]
#[test_case(r#####"tw`aspect-h-11`"#####, r#####"({
  '--tw-aspect-h': "11",
})
;"##### ; "22")]
#[test_case(r#####"tw`aspect-w-12`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "12",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "23")]
#[test_case(r#####"tw`aspect-h-12`"#####, r#####"({
  '--tw-aspect-h': "12",
})
;"##### ; "24")]
#[test_case(r#####"tw`aspect-w-13`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "13",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "25")]
#[test_case(r#####"tw`aspect-h-13`"#####, r#####"({
  '--tw-aspect-h': "13",
})
;"##### ; "26")]
#[test_case(r#####"tw`aspect-w-14`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "14",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "27")]
#[test_case(r#####"tw`aspect-h-14`"#####, r#####"({
  '--tw-aspect-h': "14",
})
;"##### ; "28")]
#[test_case(r#####"tw`aspect-w-15`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "15",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "29")]
#[test_case(r#####"tw`aspect-h-15`"#####, r#####"({
  '--tw-aspect-h': "15",
})
;"##### ; "30")]
#[test_case(r#####"tw`aspect-w-16`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  '--tw-aspect-w': "16",
  '> *': {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "31")]
#[test_case(r#####"tw`aspect-h-16`"#####, r#####"({
  '--tw-aspect-h': "16",
})"##### ; "32")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
