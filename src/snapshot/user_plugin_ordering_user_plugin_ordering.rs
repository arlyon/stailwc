use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`selector`"#####, r#####"({
  '@media (min-width: 1px)': {
    content: "@media .selector",
    '& .selector3': {
      content: "@media .selector .selector3",
    },
    ':hover .selector2': {
      content: "@media .selector:hover .selector2",
    },
    ':hover.selector2': {
      content: "@media .selector:hover.selector2",
    },
  },
  content: ".selector",
  '& .selector2': {
    content: ".selector .selector2",
  },
  ':hover': {
    content: ".selector:hover",
  },
  ':hover .selector3': {
    content: ".selector:hover .selector3",
  },
  margin: "1px",
  padding: "padding",
  display: "block",
  '@media (min-width: 2px)': {
    content: "@media .selector",
  },
})"##### ; "1")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
