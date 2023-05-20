use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`fluid-container ml-10`"#####, r#####"({
  marginLeft: "2.5rem",
  marginRight: "auto",
  width: "100%",
  ':focus': {
    marginLeft: "10rem",
    marginRight: "11rem",
    width: "100%",
  },
  '@media (min-width: 1440px)': {
    display: "block",
    width: "75%",
    backgroundColor: "black",
  },
  '@media (min-width: 768px)': {
    ':hover': {
      width: "25%",
    },
    ':focus': {
      marginLeft: "auto",
      marginRight: "auto",
      width: "100%",
    },
  },
  '@media only screen and (max-width: 540px)': {
    width: "33%",
    backgroundColor: "red",
  },
})"##### ; "0")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
