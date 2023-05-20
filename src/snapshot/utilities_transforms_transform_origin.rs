use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`transformOrigin`"#####, r#####"({
  center: "center",
  top: "top",
  'top-right': "top right",
  right: "right",
  'bottom-right': "bottom right",
  bottom: "bottom",
  'bottom-left': "bottom left",
  left: "left",
  'top-left': "top left",
})
;"##### ; "1")]
#[test_case(r#####"tw`origin-center`"#####, r#####"({
  transformOrigin: "center",
})
;"##### ; "2")]
#[test_case(r#####"tw`origin-top`"#####, r#####"({
  transformOrigin: "top",
})
;"##### ; "3")]
#[test_case(r#####"tw`origin-top-right`"#####, r#####"({
  transformOrigin: "top right",
})
;"##### ; "4")]
#[test_case(r#####"tw`origin-right`"#####, r#####"({
  transformOrigin: "right",
})
;"##### ; "5")]
#[test_case(r#####"tw`origin-bottom-right`"#####, r#####"({
  transformOrigin: "bottom right",
})
;"##### ; "6")]
#[test_case(r#####"tw`origin-bottom`"#####, r#####"({
  transformOrigin: "bottom",
})
;"##### ; "7")]
#[test_case(r#####"tw`origin-bottom-left`"#####, r#####"({
  transformOrigin: "bottom left",
})
;"##### ; "8")]
#[test_case(r#####"tw`origin-left`"#####, r#####"({
  transformOrigin: "left",
})
;"##### ; "9")]
#[test_case(r#####"tw`origin-top-left`"#####, r#####"({
  transformOrigin: "top left",
})
;"##### ; "10")]
#[test_case(r#####"tw`origin-[33% 75%]`"#####, r#####"({
  transformOrigin: "33% 75%",
})"##### ; "11")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
