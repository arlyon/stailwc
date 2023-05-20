use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`borderWidth.`"#####, r#####"({
  0: "0px",
  2: "2px",
  4: "4px",
  8: "8px",
  DEFAULT: "1px",
})
;"##### ; "0")]
#[test_case(r#####"tw`border-0`"#####, r#####"({
  borderWidth: "0px",
})
;"##### ; "1")]
#[test_case(r#####"tw`border-2`"#####, r#####"({
  borderWidth: "2px",
})
;"##### ; "2")]
#[test_case(r#####"tw`border-4`"#####, r#####"({
  borderWidth: "4px",
})
;"##### ; "3")]
#[test_case(r#####"tw`border-8`"#####, r#####"({
  borderWidth: "8px",
})
;"##### ; "4")]
#[test_case(r#####"tw`border`"#####, r#####"({
  borderWidth: "1px",
})
;"##### ; "5")]
#[test_case(r#####"tw`border-x-0`"#####, r#####"({
  borderLeftWidth: "0px",
  borderRightWidth: "0px",
})
;"##### ; "6")]
#[test_case(r#####"tw`border-x-2`"#####, r#####"({
  borderLeftWidth: "2px",
  borderRightWidth: "2px",
})
;"##### ; "7")]
#[test_case(r#####"tw`border-x-4`"#####, r#####"({
  borderLeftWidth: "4px",
  borderRightWidth: "4px",
})
;"##### ; "8")]
#[test_case(r#####"tw`border-x-8`"#####, r#####"({
  borderLeftWidth: "8px",
  borderRightWidth: "8px",
})
;"##### ; "9")]
#[test_case(r#####"tw`border-x`"#####, r#####"({
  borderLeftWidth: "1px",
  borderRightWidth: "1px",
})
;"##### ; "10")]
#[test_case(r#####"tw`border-y-0`"#####, r#####"({
  borderTopWidth: "0px",
  borderBottomWidth: "0px",
})
;"##### ; "11")]
#[test_case(r#####"tw`border-y-2`"#####, r#####"({
  borderTopWidth: "2px",
  borderBottomWidth: "2px",
})
;"##### ; "12")]
#[test_case(r#####"tw`border-y-4`"#####, r#####"({
  borderTopWidth: "4px",
  borderBottomWidth: "4px",
})
;"##### ; "13")]
#[test_case(r#####"tw`border-y-8`"#####, r#####"({
  borderTopWidth: "8px",
  borderBottomWidth: "8px",
})
;"##### ; "14")]
#[test_case(r#####"tw`border-y`"#####, r#####"({
  borderTopWidth: "1px",
  borderBottomWidth: "1px",
})
;"##### ; "15")]
#[test_case(r#####"tw`border-t-0`"#####, r#####"({
  borderTopWidth: "0px",
})
;"##### ; "16")]
#[test_case(r#####"tw`border-t-2`"#####, r#####"({
  borderTopWidth: "2px",
})
;"##### ; "17")]
#[test_case(r#####"tw`border-t-4`"#####, r#####"({
  borderTopWidth: "4px",
})
;"##### ; "18")]
#[test_case(r#####"tw`border-t-8`"#####, r#####"({
  borderTopWidth: "8px",
})
;"##### ; "19")]
#[test_case(r#####"tw`border-t`"#####, r#####"({
  borderTopWidth: "1px",
})
;"##### ; "20")]
#[test_case(r#####"tw`border-r-0`"#####, r#####"({
  borderRightWidth: "0px",
})
;"##### ; "21")]
#[test_case(r#####"tw`border-r-2`"#####, r#####"({
  borderRightWidth: "2px",
})
;"##### ; "22")]
#[test_case(r#####"tw`border-r-4`"#####, r#####"({
  borderRightWidth: "4px",
})
;"##### ; "23")]
#[test_case(r#####"tw`border-r-8`"#####, r#####"({
  borderRightWidth: "8px",
})
;"##### ; "24")]
#[test_case(r#####"tw`border-r`"#####, r#####"({
  borderRightWidth: "1px",
})
;"##### ; "25")]
#[test_case(r#####"tw`border-b-0`"#####, r#####"({
  borderBottomWidth: "0px",
})
;"##### ; "26")]
#[test_case(r#####"tw`border-b-2`"#####, r#####"({
  borderBottomWidth: "2px",
})
;"##### ; "27")]
#[test_case(r#####"tw`border-b-4`"#####, r#####"({
  borderBottomWidth: "4px",
})
;"##### ; "28")]
#[test_case(r#####"tw`border-b-8`"#####, r#####"({
  borderBottomWidth: "8px",
})
;"##### ; "29")]
#[test_case(r#####"tw`border-b`"#####, r#####"({
  borderBottomWidth: "1px",
})
;"##### ; "30")]
#[test_case(r#####"tw`border-l-0`"#####, r#####"({
  borderLeftWidth: "0px",
})
;"##### ; "31")]
#[test_case(r#####"tw`border-l-2`"#####, r#####"({
  borderLeftWidth: "2px",
})
;"##### ; "32")]
#[test_case(r#####"tw`border-l-4`"#####, r#####"({
  borderLeftWidth: "4px",
})
;"##### ; "33")]
#[test_case(r#####"tw`border-l-8`"#####, r#####"({
  borderLeftWidth: "8px",
})
;"##### ; "34")]
#[test_case(r#####"tw`border-l`"#####, r#####"({
  borderLeftWidth: "1px",
})
;"##### ; "35")]
#[test_case(r#####"tw`border-[2.5px]`"#####, r#####"({
  borderWidth: "2.5px",
})
;"##### ; "36")]
#[test_case(r#####"tw`border-t-[2.5px]`"#####, r#####"({
  borderTopWidth: "2.5px",
})
;"##### ; "37")]
#[test_case(r#####"tw`border-t-[length:10px]`"#####, r#####"({
  borderTopWidth: "10px",
})
;"##### ; "38")]
#[test_case(r#####"tw`border-r-[length:10px]`"#####, r#####"({
  borderRightWidth: "10px",
})
;"##### ; "39")]
#[test_case(r#####"tw`border-b-[length:10px]`"#####, r#####"({
  borderBottomWidth: "10px",
})
;"##### ; "40")]
#[test_case(r#####"tw`border-l-[length:10px]`"#####, r#####"({
  borderLeftWidth: "10px",
})
;"##### ; "41")]
#[test_case(r#####"tw`border-l-[length:10px]`"#####, r#####"({
  borderLeftWidth: "10px",
})
;"##### ; "42")]
#[test_case(r#####"tw`border-x-[length:10px]`"#####, r#####"({
  borderLeftWidth: "10px",
  borderRightWidth: "10px",
})
;"##### ; "43")]
#[test_case(r#####"tw`border-y-[length:10px]`"#####, r#####"({
  borderTopWidth: "10px",
  borderBottomWidth: "10px",
})
;"##### ; "44")]
#[test_case(r#####"tw`border-[length:10px]`"#####, r#####"({
  borderWidth: "10px",
})
;"##### ; "45")]
#[test_case(r#####"tw`border-2 border-s-0 border-e-4`"#####, r#####"({
  borderWidth: "2px",
  borderInlineEndWidth: "4px",
  borderInlineStartWidth: "0px",
})"##### ; "46")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
