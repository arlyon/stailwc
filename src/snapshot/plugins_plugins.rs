use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`type-sm`"#####, r#####"({
  fontSize: "0.875rem",
  fontWeight: "500",
  lineHeight: "1.25",
})"##### ; "0")]
#[test_case(r#####"const addUtilitiesTest = tw`type-sm text-red-500 lg:type-sm md:type-sm!`"#####, r#####"const addUtilitiesTest = {
  '--tw-text-opacity': "1",
  color: "rgb(239 68 68 / var(--tw-text-opacity))",
  fontSize: "0.875rem",
  fontWeight: "500",
  lineHeight: "1.25",
  '@media (min-width: 768px)': {
    fontSize: "0.875rem !important",
    fontWeight: "500 !important",
    lineHeight: "1.25 !important",
  },
  '@media (min-width: 1024px)': {
    fontSize: "0.875rem",
    fontWeight: "500",
    lineHeight: "1.25",
  },
}"##### ; "1")]
#[test_case(r#####"const addUtilitiesTest2 = tw`skew-15deg`"#####, r#####"const addUtilitiesTest2 = {
  transform: "skewY(-15deg)",
}"##### ; "2")]
#[test_case(r#####"const addUtilitiesTest2Important = tw`skew-15deg! type-sm!`"#####, r#####"const addUtilitiesTest2Important = {
  fontSize: "0.875rem !important",
  fontWeight: "500 !important",
  lineHeight: "1.25 !important",
  transform: "skewY(-15deg) !important",
}"##### ; "3")]
#[test_case(r#####"const addUtilitiesTest2Media = tw`sm:skew-15deg lg:type-sm`"#####, r#####"const addUtilitiesTest2Media = {
  '@media (min-width: 640px)': {
    transform: "skewY(-15deg)",
  },
  '@media (min-width: 1024px)': {
    fontSize: "0.875rem",
    fontWeight: "500",
    lineHeight: "1.25",
  },
}"##### ; "4")]
#[test_case(r#####"const addUtilitiesTest2Variants = tw`hover:active:skew-15deg even:visited:skew-15deg`"#####, r#####"const addUtilitiesTest2Variants = {
  ':nth-child(even):visited': {
    transform: "skewY(-15deg)",
  },
  ':hover:active': {
    transform: "skewY(-15deg)",
  },
}"##### ; "5")]
#[test_case(r#####"const addComponentsTest = tw`btn btn-blue btn-red`"#####, r#####"const addComponentsTest = {
  padding: ".5rem 1rem",
  borderRadius: ".25rem",
  fontWeight: "600",
  backgroundColor: "#e3342f",
  color: "#fff",
  ':hover': {
    backgroundColor: "#cc1f1a",
  },
}"##### ; "6")]
#[test_case(r#####"const addComponentsTestMedia = tw`xl:btn sm:btn-blue lg:btn-red`"#####, r#####"const addComponentsTestMedia = {
  '@media (min-width: 640px)': {
    backgroundColor: "#3490dc",
    color: "#fff",
    ':hover': {
      backgroundColor: "#2779bd",
    },
  },
  '@media (min-width: 1024px)': {
    backgroundColor: "#e3342f",
    color: "#fff",
    ':hover': {
      backgroundColor: "#cc1f1a",
    },
  },
  '@media (min-width: 1280px)': {
    padding: ".5rem 1rem",
    borderRadius: ".25rem",
    fontWeight: "600",
  },
}"##### ; "7")]
#[test_case(r#####"const addComponentsTestVariants = tw`hover:active:btn hocus:before:btn-blue even:visited:btn-red`"#####, r#####"const addComponentsTestVariants = {
  ':nth-child(even):visited': {
    backgroundColor: "#e3342f",
    color: "#fff",
  },
  ':nth-child(even):visited:hover': {
    backgroundColor: "#cc1f1a",
  },
  ':hover:active': {
    padding: ".5rem 1rem",
    borderRadius: ".25rem",
    fontWeight: "600",
  },
  ':hover::before': {
    content: "var(--tw-content)",
    backgroundColor: "#3490dc",
    color: "#fff",
  },
  ':focus::before': {
    content: "var(--tw-content)",
    backgroundColor: "#3490dc",
    color: "#fff",
  },
  ':hover:hover::before': {
    content: "var(--tw-content)",
    backgroundColor: "#2779bd",
  },
  ':focus:hover::before': {
    content: "var(--tw-content)",
    backgroundColor: "#2779bd",
  },
}"##### ; "8")]
#[test_case(r#####"const addComponentsTestElementPrefixes = tw`prefixes`"#####, r#####"const addComponentsTestElementPrefixes = {
  '& h1': {
    margin: "auto",
    marginRight: "10px",
  },
  '& h2:hover': {
    color: "red",
  },
  '& h3:hover,& h3:active': {
    color: "green",
  },
  '& :focus': {
    display: "none",
  },
}"##### ; "9")]
#[test_case(r#####"const addComponentsTestElementScreenReplacements = tw`screenies`"#####, r#####"const addComponentsTestElementScreenReplacements = {
  '@media (min-width: 640px)': {
    display: "block",
  },
  '@media (min-width: 1024px)': {
    display: "inline-block",
  },
  '@media (min-width: 768px)': {
    display: "flex",
  },
  '@media (min-width: 1280px)': {
    '& h1': {
      marginTop: "50px",
    },
    '& h1:hover,& h1:focus': {
      color: "blue",
    },
  },
}"##### ; "10")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
