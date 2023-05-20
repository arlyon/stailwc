use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`gridAutoColumns`"#####, r#####"({
  auto: "auto",
  min: "min-content",
  max: "max-content",
  fr: "minmax(0, 1fr)",
})
;"##### ; "1")]
#[test_case(r#####"tw`auto-cols-auto`"#####, r#####"({
  gridAutoColumns: "auto",
})
;"##### ; "2")]
#[test_case(r#####"tw`auto-cols-min`"#####, r#####"({
  gridAutoColumns: "min-content",
})
;"##### ; "3")]
#[test_case(r#####"tw`auto-cols-max`"#####, r#####"({
  gridAutoColumns: "max-content",
})
;"##### ; "4")]
#[test_case(r#####"tw`auto-cols-fr`"#####, r#####"({
  gridAutoColumns: "minmax(0, 1fr)",
})
;"##### ; "5")]
#[test_case(r#####"tw`auto-cols-[minmax(0, 2fr)]`"#####, r#####"({
  gridAutoColumns: "minmax(0, 2fr)",
})
;"##### ; "6")]
#[test_case(r#####"tw`grid-cols-[200px,repeat(auto-fill,minmax(15%,100px)),300px]`"#####, r#####"({
  gridTemplateColumns: "200px repeat(auto-fill,minmax(15%,100px)) 300px",
})
;"##### ; "7")]
#[test_case(r#####"tw`lg:grid-cols-[200px,repeat(auto-fill,minmax(15%,100px)),300px]`"#####, r#####"({
  '@media (min-width: 1024px)': {
    gridTemplateColumns: "200px repeat(auto-fill,minmax(15%,100px)) 300px",
  },
})"##### ; "8")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
