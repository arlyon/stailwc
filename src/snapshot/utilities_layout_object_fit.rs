use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`object-contain`"#####, r#####"({
  objectFit: "contain",
})
;"##### ; "1")]
#[test_case(r#####"tw`object-cover`"#####, r#####"({
  objectFit: "cover",
})
;"##### ; "2")]
#[test_case(r#####"tw`object-fill`"#####, r#####"({
  objectFit: "fill",
})
;"##### ; "3")]
#[test_case(r#####"tw`object-none`"#####, r#####"({
  objectFit: "none",
})
;"##### ; "4")]
#[test_case(r#####"tw`object-scale-down`"#####, r#####"({
  objectFit: "scale-down",
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
