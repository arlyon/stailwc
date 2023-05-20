use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`[mask-image:linear-gradient(180deg,white, rgba(255,255,255,0))]`"#####, r#####"({
  maskImage: "linear-gradient(180deg,white, rgba(255,255,255,0))",
})
;"##### ; "0")]
#[test_case(r#####"tw`[-webkit-property:bg-black]`"#####, r#####"({
  WebkitProperty: "bg-black",
})
;"##### ; "1")]
#[test_case(r#####"tw`[--my-var:blue]`"#####, r#####"({
  '--my-var': "blue",
})
;"##### ; "2")]
#[test_case(r#####"tw`[color:var(--my-var)]`"#####, r#####"({
  color: "var(--my-var)",
})
;"##### ; "3")]
#[test_case(r#####"tw`bg-black md:[color:var(--my-var)]`"#####, r#####"({
  '--tw-bg-opacity': "1",
  backgroundColor: "rgb(0 0 0 / var(--tw-bg-opacity))",
  '@media (min-width: 768px)': {
    color: "var(--my-var)",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`[margin:2px_4px_5px_1px]`"#####, r#####"({
  margin: "2px 4px 5px 1px",
})
;"##### ; "5")]
#[test_case(r#####"tw`[content:'—']`"#####, r#####"({
  content: "'\\u2014'",
})"##### ; "6")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
