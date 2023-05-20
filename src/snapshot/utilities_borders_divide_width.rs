use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`divideWidth.`"#####, r#####"({
  0: "0px",
  2: "2px",
  4: "4px",
  8: "8px",
  DEFAULT: "1px",
})
;"##### ; "0")]
#[test_case(r#####"tw`divide-x-0`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': "0",
    borderRightWidth: "calc(0px * var(--tw-divide-x-reverse))",
    borderLeftWidth: "calc(0px * calc(1 - var(--tw-divide-x-reverse)))",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`divide-x-2`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': "0",
    borderRightWidth: "calc(2px * var(--tw-divide-x-reverse))",
    borderLeftWidth: "calc(2px * calc(1 - var(--tw-divide-x-reverse)))",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`divide-x-4`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': "0",
    borderRightWidth: "calc(4px * var(--tw-divide-x-reverse))",
    borderLeftWidth: "calc(4px * calc(1 - var(--tw-divide-x-reverse)))",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`divide-x-8`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': "0",
    borderRightWidth: "calc(8px * var(--tw-divide-x-reverse))",
    borderLeftWidth: "calc(8px * calc(1 - var(--tw-divide-x-reverse)))",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`divide-x`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': "0",
    borderRightWidth: "calc(1px * var(--tw-divide-x-reverse))",
    borderLeftWidth: "calc(1px * calc(1 - var(--tw-divide-x-reverse)))",
  },
})
;"##### ; "5")]
#[test_case(r#####"tw`divide-y-0`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': "0",
    borderTopWidth: "calc(0px * calc(1 - var(--tw-divide-y-reverse)))",
    borderBottomWidth: "calc(0px * var(--tw-divide-y-reverse))",
  },
})
;"##### ; "6")]
#[test_case(r#####"tw`divide-y-2`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': "0",
    borderTopWidth: "calc(2px * calc(1 - var(--tw-divide-y-reverse)))",
    borderBottomWidth: "calc(2px * var(--tw-divide-y-reverse))",
  },
})
;"##### ; "7")]
#[test_case(r#####"tw`divide-y-4`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': "0",
    borderTopWidth: "calc(4px * calc(1 - var(--tw-divide-y-reverse)))",
    borderBottomWidth: "calc(4px * var(--tw-divide-y-reverse))",
  },
})
;"##### ; "8")]
#[test_case(r#####"tw`divide-y-8`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': "0",
    borderTopWidth: "calc(8px * calc(1 - var(--tw-divide-y-reverse)))",
    borderBottomWidth: "calc(8px * var(--tw-divide-y-reverse))",
  },
})
;"##### ; "9")]
#[test_case(r#####"tw`divide-y`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': "0",
    borderTopWidth: "calc(1px * calc(1 - var(--tw-divide-y-reverse)))",
    borderBottomWidth: "calc(1px * var(--tw-divide-y-reverse))",
  },
})
;"##### ; "10")]
#[test_case(r#####"tw`divide-x-reverse`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': "1",
  },
})
;"##### ; "11")]
#[test_case(r#####"tw`divide-y-reverse`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': "1",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`divide-x-[3px]`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': "0",
    borderRightWidth: "calc(3px * var(--tw-divide-x-reverse))",
    borderLeftWidth: "calc(3px * calc(1 - var(--tw-divide-x-reverse)))",
  },
})
;"##### ; "13")]
#[test_case(r#####"tw`divide-y-[3px]`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': "0",
    borderTopWidth: "calc(3px * calc(1 - var(--tw-divide-y-reverse)))",
    borderBottomWidth: "calc(3px * var(--tw-divide-y-reverse))",
  },
})
;"##### ; "14")]
#[test_case(r#####"tw`divide-x-[line-width:3px]`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': "0",
    borderRightWidth: "calc(3px * var(--tw-divide-x-reverse))",
    borderLeftWidth: "calc(3px * calc(1 - var(--tw-divide-x-reverse)))",
  },
})
;"##### ; "15")]
#[test_case(r#####"tw`divide-x-[length:3px]`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-x-reverse': "0",
    borderRightWidth: "calc(3px * var(--tw-divide-x-reverse))",
    borderLeftWidth: "calc(3px * calc(1 - var(--tw-divide-x-reverse)))",
  },
})
;"##### ; "16")]
#[test_case(r#####"tw`divide-y-[line-width:3px]`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': "0",
    borderTopWidth: "calc(3px * calc(1 - var(--tw-divide-y-reverse)))",
    borderBottomWidth: "calc(3px * var(--tw-divide-y-reverse))",
  },
})
;"##### ; "17")]
#[test_case(r#####"tw`divide-y-[length:3px]`"#####, r#####"({
  '> :not([hidden]) ~ :not([hidden])': {
    '--tw-divide-y-reverse': "0",
    borderTopWidth: "calc(3px * calc(1 - var(--tw-divide-y-reverse)))",
    borderBottomWidth: "calc(3px * var(--tw-divide-y-reverse))",
  },
})"##### ; "18")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
