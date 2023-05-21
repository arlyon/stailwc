use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`my-class1`"#####, r#####"({
  backgroundColor: "black",
  "& h2": {
    backgroundColor: "red",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`my-class2`"#####, r#####"({
  backgroundColor: "green",
  "h2 &": {
    backgroundColor: "yellow",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`my-class3`"#####, r#####"({
  backgroundColor: "green",
  ".dark &:hover": {
    backgroundColor: "yellow",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`my-class4`"#####, r#####"({
  ".test & :hover": {
    backgroundColor: "orange",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`my-class5`"#####, r#####"({
  backgroundColor: "brown",
  ":hover": {
    backgroundColor: "pink",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`my-class6`"#####, r#####"({
  backgroundColor: "blue",
  "& :hover": {
    backgroundColor: "orange",
  },
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
