use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`overflow-auto`"#####, r#####"({
  overflow: "auto",
})
;"##### ; "0")]
#[test_case(r#####"tw`overflow-hidden`"#####, r#####"({
  overflow: "hidden",
})
;"##### ; "1")]
#[test_case(r#####"tw`overflow-clip`"#####, r#####"({
  overflow: "clip",
})
;"##### ; "2")]
#[test_case(r#####"tw`overflow-x-clip`"#####, r#####"({
  overflowX: "clip",
})
;"##### ; "3")]
#[test_case(r#####"tw`overflow-y-clip`"#####, r#####"({
  overflowY: "clip",
})
;"##### ; "4")]
#[test_case(r#####"tw`overflow-visible`"#####, r#####"({
  overflow: "visible",
})
;"##### ; "5")]
#[test_case(r#####"tw`overflow-scroll`"#####, r#####"({
  overflow: "scroll",
})
;"##### ; "6")]
#[test_case(r#####"tw`overflow-x-auto`"#####, r#####"({
  overflowX: "auto",
})
;"##### ; "7")]
#[test_case(r#####"tw`overflow-y-auto`"#####, r#####"({
  overflowY: "auto",
})
;"##### ; "8")]
#[test_case(r#####"tw`overflow-x-hidden`"#####, r#####"({
  overflowX: "hidden",
})
;"##### ; "9")]
#[test_case(r#####"tw`overflow-y-hidden`"#####, r#####"({
  overflowY: "hidden",
})
;"##### ; "10")]
#[test_case(r#####"tw`overflow-x-visible`"#####, r#####"({
  overflowX: "visible",
})
;"##### ; "11")]
#[test_case(r#####"tw`overflow-y-visible`"#####, r#####"({
  overflowY: "visible",
})
;"##### ; "12")]
#[test_case(r#####"tw`overflow-x-scroll`"#####, r#####"({
  overflowX: "scroll",
})
;"##### ; "13")]
#[test_case(r#####"tw`overflow-y-scroll`"#####, r#####"({
  overflowY: "scroll",
})"##### ; "14")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
