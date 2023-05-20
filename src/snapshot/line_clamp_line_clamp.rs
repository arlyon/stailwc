use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`line-clamp-1`"#####, r#####"({
  overflow: "hidden",
  display: "-webkit-box",
  WebkitBoxOrient: "vertical",
  WebkitLineClamp: "1",
})
;"##### ; "1")]
#[test_case(r#####"tw`line-clamp-2`"#####, r#####"({
  overflow: "hidden",
  display: "-webkit-box",
  WebkitBoxOrient: "vertical",
  WebkitLineClamp: "2",
})
;"##### ; "2")]
#[test_case(r#####"tw`line-clamp-3`"#####, r#####"({
  overflow: "hidden",
  display: "-webkit-box",
  WebkitBoxOrient: "vertical",
  WebkitLineClamp: "3",
})
;"##### ; "3")]
#[test_case(r#####"tw`line-clamp-4`"#####, r#####"({
  overflow: "hidden",
  display: "-webkit-box",
  WebkitBoxOrient: "vertical",
  WebkitLineClamp: "4",
})
;"##### ; "4")]
#[test_case(r#####"tw`line-clamp-5`"#####, r#####"({
  overflow: "hidden",
  display: "-webkit-box",
  WebkitBoxOrient: "vertical",
  WebkitLineClamp: "5",
})
;"##### ; "5")]
#[test_case(r#####"tw`line-clamp-6`"#####, r#####"({
  overflow: "hidden",
  display: "-webkit-box",
  WebkitBoxOrient: "vertical",
  WebkitLineClamp: "6",
})
;"##### ; "6")]
#[test_case(r#####"tw`line-clamp-none`"#####, r#####"({
  overflow: "visible",
  display: "block",
  WebkitBoxOrient: "horizontal",
  WebkitLineClamp: "none",
})"##### ; "7")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
