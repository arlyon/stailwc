use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`backgroundSize`"#####, r#####"({
  auto: "auto",
  cover: "cover",
  contain: "contain",
})
;"##### ; "0")]
#[test_case(r#####"tw`bg-auto`"#####, r#####"({
  backgroundSize: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`bg-cover`"#####, r#####"({
  backgroundSize: "cover",
})
;"##### ; "2")]
#[test_case(r#####"tw`bg-contain`"#####, r#####"({
  backgroundSize: "contain",
})
;"##### ; "3")]
#[test_case(r#####"tw`bg-[length:var(--value)]`"#####, r#####"({
  backgroundSize: "var(--value)",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
