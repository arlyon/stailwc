use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`aspect-w-1`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "1",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`aspect-h-1`"#####, r#####"({
  "--tw-aspect-h": "1",
})
;"##### ; "1")]
#[test_case(r#####"tw`aspect-w-2`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "2",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`aspect-h-2`"#####, r#####"({
  "--tw-aspect-h": "2",
})
;"##### ; "3")]
#[test_case(r#####"tw`aspect-w-3`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "3",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`aspect-h-3`"#####, r#####"({
  "--tw-aspect-h": "3",
})
;"##### ; "5")]
#[test_case(r#####"tw`aspect-w-4`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "4",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "6")]
#[test_case(r#####"tw`aspect-h-4`"#####, r#####"({
  "--tw-aspect-h": "4",
})
;"##### ; "7")]
#[test_case(r#####"tw`aspect-w-5`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "5",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "8")]
#[test_case(r#####"tw`aspect-h-5`"#####, r#####"({
  "--tw-aspect-h": "5",
})
;"##### ; "9")]
#[test_case(r#####"tw`aspect-w-6`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "6",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "10")]
#[test_case(r#####"tw`aspect-h-6`"#####, r#####"({
  "--tw-aspect-h": "6",
})
;"##### ; "11")]
#[test_case(r#####"tw`aspect-w-7`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "7",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`aspect-h-7`"#####, r#####"({
  "--tw-aspect-h": "7",
})
;"##### ; "13")]
#[test_case(r#####"tw`aspect-w-8`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "8",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "14")]
#[test_case(r#####"tw`aspect-h-8`"#####, r#####"({
  "--tw-aspect-h": "8",
})
;"##### ; "15")]
#[test_case(r#####"tw`aspect-w-9`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "9",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "16")]
#[test_case(r#####"tw`aspect-h-9`"#####, r#####"({
  "--tw-aspect-h": "9",
})
;"##### ; "17")]
#[test_case(r#####"tw`aspect-w-10`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "10",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "18")]
#[test_case(r#####"tw`aspect-h-10`"#####, r#####"({
  "--tw-aspect-h": "10",
})
;"##### ; "19")]
#[test_case(r#####"tw`aspect-w-11`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "11",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "20")]
#[test_case(r#####"tw`aspect-h-11`"#####, r#####"({
  "--tw-aspect-h": "11",
})
;"##### ; "21")]
#[test_case(r#####"tw`aspect-w-12`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "12",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "22")]
#[test_case(r#####"tw`aspect-h-12`"#####, r#####"({
  "--tw-aspect-h": "12",
})
;"##### ; "23")]
#[test_case(r#####"tw`aspect-w-13`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "13",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "24")]
#[test_case(r#####"tw`aspect-h-13`"#####, r#####"({
  "--tw-aspect-h": "13",
})
;"##### ; "25")]
#[test_case(r#####"tw`aspect-w-14`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "14",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "26")]
#[test_case(r#####"tw`aspect-h-14`"#####, r#####"({
  "--tw-aspect-h": "14",
})
;"##### ; "27")]
#[test_case(r#####"tw`aspect-w-15`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "15",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "28")]
#[test_case(r#####"tw`aspect-h-15`"#####, r#####"({
  "--tw-aspect-h": "15",
})
;"##### ; "29")]
#[test_case(r#####"tw`aspect-w-16`"#####, r#####"({
  position: "relative",
  paddingBottom: "calc(var(--tw-aspect-h) / var(--tw-aspect-w) * 100%)",
  "--tw-aspect-w": "16",
  "> *": {
    position: "absolute",
    height: "100%",
    width: "100%",
    top: "0",
    right: "0",
    bottom: "0",
    left: "0",
  },
})
;"##### ; "30")]
#[test_case(r#####"tw`aspect-h-16`"#####, r#####"({
  "--tw-aspect-h": "16",
})"##### ; "31")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
