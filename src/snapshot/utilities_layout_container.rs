use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`container`"#####, r#####"({
  padding: {
    DEFAULT: ["1rem", "2rem"],
    sm: ["2rem"],
    lg: "4rem",
    xl: "6rem",
    object: "8rem",
    'object-width': "10rem",
    'object-min-max': "12rem",
  },
  margin: {
    DEFAULT: ["2rem", "3rem"],
    sm: ["auto"],
    lg: "5rem",
    xl: "7rem",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`container`"#####, r#####"({
  width: "100%",
  paddingRight: "2rem",
  paddingLeft: "2rem",
  '@media (min-width: 640px)': {
    maxWidth: "640px",
    paddingRight: "2rem",
    paddingLeft: "2rem",
  },
  '@media (min-width: 768px)': {
    maxWidth: "768px",
  },
  '@media (min-width: 968px)': {
    maxWidth: "968px",
    paddingRight: "8rem",
    paddingLeft: "8rem",
  },
  '@media (min-width: 992px)': {
    maxWidth: "992px",
    paddingRight: "10rem",
    paddingLeft: "10rem",
  },
  '@media (min-width: 1024px)': {
    maxWidth: "1024px",
    paddingRight: "4rem",
    paddingLeft: "4rem",
  },
  '@media (min-width: 1200px)': {
    maxWidth: "1200px",
    paddingRight: "12rem",
    paddingLeft: "12rem",
  },
  '@media (min-width: 1280px)': {
    maxWidth: "1280px",
    paddingRight: "6rem",
    paddingLeft: "6rem",
  },
  '@media (min-width: 1536px)': {
    maxWidth: "1536px",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`md:container md:mx-auto`"#####, r#####"({
  '@media (min-width: 768px)': {
    width: "100%",
    paddingRight: "2rem",
    paddingLeft: "2rem",
    '@media (min-width: 640px)': {
      maxWidth: "640px",
      paddingRight: "2rem",
      paddingLeft: "2rem",
    },
    '@media (min-width: 768px)': {
      maxWidth: "768px",
    },
    '@media (min-width: 968px)': {
      maxWidth: "968px",
      paddingRight: "8rem",
      paddingLeft: "8rem",
    },
    '@media (min-width: 992px)': {
      maxWidth: "992px",
      paddingRight: "10rem",
      paddingLeft: "10rem",
    },
    '@media (min-width: 1024px)': {
      maxWidth: "1024px",
      paddingRight: "4rem",
      paddingLeft: "4rem",
    },
    '@media (min-width: 1200px)': {
      maxWidth: "1200px",
      paddingRight: "12rem",
      paddingLeft: "12rem",
    },
    '@media (min-width: 1280px)': {
      maxWidth: "1280px",
      paddingRight: "6rem",
      paddingLeft: "6rem",
    },
    '@media (min-width: 1536px)': {
      maxWidth: "1536px",
    },
    marginLeft: "auto",
    marginRight: "auto",
  },
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
