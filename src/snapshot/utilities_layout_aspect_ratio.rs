use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`aspectRatio`"#####, r#####"({
  auto: "auto",
  square: "1 / 1",
  video: "16 / 9",
})
;"##### ; "0")]
#[test_case(r#####"tw`aspect-auto`"#####, r#####"({
  aspectRatio: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`aspect-square`"#####, r#####"({
  aspectRatio: "1 / 1",
})
;"##### ; "2")]
#[test_case(r#####"tw`aspect-video`"#####, r#####"({
  aspectRatio: "16 / 9",
})
;"##### ; "3")]
#[test_case(r#####"tw`aspect-[4/3]`"#####, r#####"({
  aspectRatio: "4/3",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
