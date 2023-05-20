use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`hover:block first:mt-2 last-of-type:max-width[20px]`"#####, r#####"({
  '&:first-child': {
    marginTop: "0.5rem",
  },
  '&:last-of-type': {
    maxWidth: "20px",
  },
  '&:hover': {
    display: "block",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`hover:block first:mt-2 last-of-type:[max-width:20px]`"#####, r#####"({
  '&:first-child': {
    marginTop: "0.5rem",
  },
  '&:last-of-type': {
    maxWidth: "20px",
  },
  '&:hover': {
    display: "block",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`my-class1`"#####, r#####"({
  '&:hover': {
    backgroundColor: "pink",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`my-class2`"#####, r#####"({
  '& :hover': {
    backgroundColor: "orange",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`my-class3`"#####, r#####"({
  '.test & :hover': {
    backgroundColor: "orange",
  },
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
