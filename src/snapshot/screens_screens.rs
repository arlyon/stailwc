use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`sm:block`"#####, r#####"({
  '@media (min-width: 640px)': {
    display: "block",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`md:block`"#####, r#####"({
  '@media (min-width: 768px)': {
    display: "block",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`lg:block`"#####, r#####"({
  '@media (min-width: 1024px)': {
    display: "block",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`xl:block`"#####, r#####"({
  '@media (min-width: 1280px)': {
    display: "block",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`2xl:block`"#####, r#####"({
  '@media (min-width: 1536px)': {
    display: "block",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`<sm:underline md>:font-bold`"#####, r#####"({
  '@media (max-width: 399px)': {
    textDecorationLine: "underline",
  },
  '@media (min-width: 500px)': {
    fontWeight: "700",
  },
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
