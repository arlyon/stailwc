use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`font-thin`"#####, r#####"({
  fontWeight: "100",
})
;"##### ; "1")]
#[test_case(r#####"tw`font-extralight`"#####, r#####"({
  fontWeight: "200",
})
;"##### ; "2")]
#[test_case(r#####"tw`font-light`"#####, r#####"({
  fontWeight: "300",
})
;"##### ; "3")]
#[test_case(r#####"tw`font-normal`"#####, r#####"({
  fontWeight: "400",
})
;"##### ; "4")]
#[test_case(r#####"tw`font-medium`"#####, r#####"({
  fontWeight: "500",
})
;"##### ; "5")]
#[test_case(r#####"tw`font-semibold`"#####, r#####"({
  fontWeight: "600",
})
;"##### ; "6")]
#[test_case(r#####"tw`font-bold`"#####, r#####"({
  fontWeight: "700",
})
;"##### ; "7")]
#[test_case(r#####"tw`font-extrabold`"#####, r#####"({
  fontWeight: "800",
})
;"##### ; "8")]
#[test_case(r#####"tw`font-black`"#####, r#####"({
  fontWeight: "900",
})
;"##### ; "9")]
#[test_case(r#####"tw`font-[300]`"#####, r#####"({
  fontWeight: "300",
})
;"##### ; "10")]
#[test_case(r#####"tw`font-[number:medium]`"#####, r#####"({
  fontWeight: "medium",
})"##### ; "11")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
