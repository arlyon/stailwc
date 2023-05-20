use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`aspectRatio`"#####, r#####"({
  auto: "auto",
  square: "1 / 1",
  video: "16 / 9",
})
;"##### ; "1")]
#[test_case(r#####"tw`aspect-auto`"#####, r#####"({
  aspectRatio: "auto",
})
;"##### ; "2")]
#[test_case(r#####"tw`aspect-square`"#####, r#####"({
  aspectRatio: "1 / 1",
})
;"##### ; "3")]
#[test_case(r#####"tw`aspect-video`"#####, r#####"({
  aspectRatio: "16 / 9",
})
;"##### ; "4")]
#[test_case(r#####"tw`aspect-[4/3]`"#####, r#####"({
  aspectRatio: "4/3",
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
