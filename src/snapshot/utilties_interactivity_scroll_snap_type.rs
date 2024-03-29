use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`snap-none`"#####, r#####"({
  scrollSnapType: "none",
})
;"##### ; "0")]
#[test_case(r#####"tw`snap-x`"#####, r#####"({
  scrollSnapType: "x var(--tw-scroll-snap-strictness)",
})
;"##### ; "1")]
#[test_case(r#####"tw`snap-y`"#####, r#####"({
  scrollSnapType: "y var(--tw-scroll-snap-strictness)",
})
;"##### ; "2")]
#[test_case(r#####"tw`snap-both`"#####, r#####"({
  scrollSnapType: "both var(--tw-scroll-snap-strictness)",
})
;"##### ; "3")]
#[test_case(r#####"tw`snap-mandatory`"#####, r#####"({
  "--tw-scroll-snap-strictness": "mandatory",
})
;"##### ; "4")]
#[test_case(r#####"tw`snap-proximity`"#####, r#####"({
  "--tw-scroll-snap-strictness": "proximity",
})"##### ; "5")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
