use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`visited:border-red-500 visited:bg-red-500 visited:text-red-500`"#####, r#####"({
  ':visited': {
    borderColor: "rgb(239 68 68 )",
    backgroundColor: "rgb(239 68 68 )",
    color: "rgb(239 68 68 )",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`visited:border-red-500/20 visited:bg-red-500/20 visited:text-red-500/20`"#####, r#####"({
  ':visited': {
    borderColor: "rgb(239 68 68 / 0.2)",
    backgroundColor: "rgb(239 68 68 / 0.2)",
    color: "rgb(239 68 68 / 0.2)",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`visited:border-red-500/[20] visited:bg-red-500/[20] visited:text-red-500/[20]`"#####, r#####"({
  ':visited': {
    borderColor: "rgb(239 68 68 / 20)",
    backgroundColor: "rgb(239 68 68 / 20)",
    color: "rgb(239 68 68 / 20)",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`visited:(border-red-500) visited:(bg-red-500) visited:(text-red-500)`"#####, r#####"({
  ':visited': {
    borderColor: "rgb(239 68 68 )",
    backgroundColor: "rgb(239 68 68 )",
    color: "rgb(239 68 68 )",
  },
})"##### ; "4")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
