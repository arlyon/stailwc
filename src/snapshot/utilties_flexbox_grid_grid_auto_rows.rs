use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`gridAutoRows`"#####, r#####"({
  auto: "auto",
  min: "min-content",
  max: "max-content",
  fr: "minmax(0, 1fr)",
})
;"##### ; "1")]
#[test_case(r#####"tw`auto-rows-auto`"#####, r#####"({
  gridAutoRows: "auto",
})
;"##### ; "2")]
#[test_case(r#####"tw`auto-rows-min`"#####, r#####"({
  gridAutoRows: "min-content",
})
;"##### ; "3")]
#[test_case(r#####"tw`auto-rows-max`"#####, r#####"({
  gridAutoRows: "max-content",
})
;"##### ; "4")]
#[test_case(r#####"tw`auto-rows-fr`"#####, r#####"({
  gridAutoRows: "minmax(0, 1fr)",
})
;"##### ; "5")]
#[test_case(r#####"tw`grid-rows-[200px, repeat(auto-fill, minmax(15%, 100px)), 300px]`"#####, r#####"({
  gridTemplateRows: "200px repeat(auto-fill, minmax(15%, 100px)) 300px",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
