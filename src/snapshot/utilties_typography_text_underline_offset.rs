use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`textUnderlineOffset`"#####, r#####"({
  0: "0px",
  1: "1px",
  2: "2px",
  4: "4px",
  8: "8px",
  auto: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`underline-offset-auto`"#####, r#####"({
  textUnderlineOffset: "auto",
})
;"##### ; "2")]
#[test_case(r#####"tw`underline-offset-0`"#####, r#####"({
  textUnderlineOffset: "0px",
})
;"##### ; "3")]
#[test_case(r#####"tw`underline-offset-1`"#####, r#####"({
  textUnderlineOffset: "1px",
})
;"##### ; "4")]
#[test_case(r#####"tw`underline-offset-2`"#####, r#####"({
  textUnderlineOffset: "2px",
})
;"##### ; "5")]
#[test_case(r#####"tw`underline-offset-4`"#####, r#####"({
  textUnderlineOffset: "4px",
})
;"##### ; "6")]
#[test_case(r#####"tw`underline-offset-8`"#####, r#####"({
  textUnderlineOffset: "8px",
})
;"##### ; "7")]
#[test_case(r#####"tw`underline-offset-[3px]`"#####, r#####"({
  textUnderlineOffset: "3px",
})
;"##### ; "8")]
#[test_case(r#####"tw`underline-offset-[length:3px]`"#####, r#####"({
  textUnderlineOffset: "3px",
})
;"##### ; "9")]
#[test_case(r#####"tw`underline-offset-[30%]`"#####, r#####"({
  textUnderlineOffset: "30%",
})
;"##### ; "10")]
#[test_case(r#####"tw`underline-offset-[percentage:30%]`"#####, r#####"({
  textUnderlineOffset: "30%",
})"##### ; "11")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
