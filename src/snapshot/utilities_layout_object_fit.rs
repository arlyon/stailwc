use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`object-contain`"#####, r#####"({
  objectFit: "contain",
})
;"##### ; "0")]
#[test_case(r#####"tw`object-cover`"#####, r#####"({
  objectFit: "cover",
})
;"##### ; "1")]
#[test_case(r#####"tw`object-fill`"#####, r#####"({
  objectFit: "fill",
})
;"##### ; "2")]
#[test_case(r#####"tw`object-none`"#####, r#####"({
  objectFit: "none",
})
;"##### ; "3")]
#[test_case(r#####"tw`object-scale-down`"#####, r#####"({
  objectFit: "scale-down",
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
