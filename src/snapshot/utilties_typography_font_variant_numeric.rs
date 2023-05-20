use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`normal-nums`"#####, r#####"({
  fontVariantNumeric: "normal",
})
;"##### ; "0")]
#[test_case(r#####"tw`ordinal`"#####, r#####"({
  '--tw-ordinal': "ordinal",
  fontVariantNumeric:
    "var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)",
})
;"##### ; "1")]
#[test_case(r#####"tw`slashed-zero`"#####, r#####"({
  '--tw-slashed-zero': "slashed-zero",
  fontVariantNumeric:
    "var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)",
})
;"##### ; "2")]
#[test_case(r#####"tw`lining-nums`"#####, r#####"({
  '--tw-numeric-figure': "lining-nums",
  fontVariantNumeric:
    "var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)",
})
;"##### ; "3")]
#[test_case(r#####"tw`oldstyle-nums`"#####, r#####"({
  '--tw-numeric-figure': "oldstyle-nums",
  fontVariantNumeric:
    "var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)",
})
;"##### ; "4")]
#[test_case(r#####"tw`proportional-nums`"#####, r#####"({
  '--tw-numeric-spacing': "proportional-nums",
  fontVariantNumeric:
    "var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)",
})
;"##### ; "5")]
#[test_case(r#####"tw`tabular-nums`"#####, r#####"({
  '--tw-numeric-spacing': "tabular-nums",
  fontVariantNumeric:
    "var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)",
})
;"##### ; "6")]
#[test_case(r#####"tw`diagonal-fractions`"#####, r#####"({
  '--tw-numeric-fraction': "diagonal-fractions",
  fontVariantNumeric:
    "var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)",
})
;"##### ; "7")]
#[test_case(r#####"tw`stacked-fractions`"#####, r#####"({
  '--tw-numeric-fraction': "stacked-fractions",
  fontVariantNumeric:
    "var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)",
})"##### ; "8")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
