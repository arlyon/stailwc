use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`flex-gap-0`"#####, r#####"({
  margin: "-0px",
  "> *": {
    margin: "0px",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`flex-gap-3`"#####, r#####"({
  margin: "-0.375rem",
  "> *": {
    margin: "0.375rem",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`flex-gap-x-3`"#####, r#####"({
  marginRight: "-0.375rem",
  marginLeft: "-0.375rem",
  "> *": {
    marginRight: "0.375rem",
    marginLeft: "0.375rem",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`flex-gap-y-3`"#####, r#####"({
  marginTop: "-0.375rem",
  marginBottom: "-0.375rem",
  "> *": {
    marginTop: "0.375rem",
    marginBottom: "0.375rem",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`flex-gap-px`"#####, r#####"({
  margin: "-0.5px",
  "> *": {
    margin: "0.5px",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`flex-gap-x-1.5`"#####, r#####"({
  marginRight: "-0.1875rem",
  marginLeft: "-0.1875rem",
  "> *": {
    marginRight: "0.1875rem",
    marginLeft: "0.1875rem",
  },
})
;"##### ; "5")]
#[test_case(r#####"tw`gap-0`"#####, r#####"({
  ".no-flex-gap &": {
    margin: "-0px",
  },
  ".no-flex-gap & > *": {
    margin: "0px",
  },
  gap: "0px",
})
;"##### ; "6")]
#[test_case(r#####"tw`gap-3`"#####, r#####"({
  ".no-flex-gap &": {
    margin: "-0.375rem",
  },
  ".no-flex-gap & > *": {
    margin: "0.375rem",
  },
  gap: "0.75rem",
})
;"##### ; "7")]
#[test_case(r#####"tw`gap-x-3`"#####, r#####"({
  ".no-flex-gap &": {
    marginRight: "-0.375rem",
    marginLeft: "-0.375rem",
  },
  ".no-flex-gap & > *": {
    marginRight: "0.375rem",
    marginLeft: "0.375rem",
  },
  columnGap: "0.75rem",
})
;"##### ; "8")]
#[test_case(r#####"tw`gap-y-3`"#####, r#####"({
  ".no-flex-gap &": {
    marginTop: "-0.375rem",
    marginBottom: "-0.375rem",
  },
  ".no-flex-gap & > *": {
    marginTop: "0.375rem",
    marginBottom: "0.375rem",
  },
  rowGap: "0.75rem",
})
;"##### ; "9")]
#[test_case(r#####"tw`gap-px`"#####, r#####"({
  ".no-flex-gap &": {
    margin: "-0.5px",
  },
  ".no-flex-gap & > *": {
    margin: "0.5px",
  },
  gap: "1px",
})
;"##### ; "10")]
#[test_case(r#####"tw`gap-x-1.5`"#####, r#####"({
  ".no-flex-gap &": {
    marginRight: "-0.1875rem",
    marginLeft: "-0.1875rem",
  },
  ".no-flex-gap & > *": {
    marginRight: "0.1875rem",
    marginLeft: "0.1875rem",
  },
  columnGap: "0.375rem",
})
;"##### ; "11")]
#[test_case(r#####"tw`test-1`"#####, r#####"({
  background: "5px",
  ".a-class & .some-class": {
    margin: "10px",
  },
  ".a-class & > *": {
    margin: "20px",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`test-2`"#####, r#####"({
  ".a-class & .some-class": {
    margin: "10px",
  },
  ".a-class & > *": {
    margin: "20px",
  },
})"##### ; "13")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
