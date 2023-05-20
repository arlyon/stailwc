use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`bg-bottom`"#####, r#####"({
  backgroundPosition: "bottom",
})
;"##### ; "1")]
#[test_case(r#####"tw`bg-center`"#####, r#####"({
  backgroundPosition: "center",
})
;"##### ; "2")]
#[test_case(r#####"tw`bg-left`"#####, r#####"({
  backgroundPosition: "left",
})
;"##### ; "3")]
#[test_case(r#####"tw`bg-left-bottom`"#####, r#####"({
  backgroundPosition: "left bottom",
})
;"##### ; "4")]
#[test_case(r#####"tw`bg-left-top`"#####, r#####"({
  backgroundPosition: "left top",
})
;"##### ; "5")]
#[test_case(r#####"tw`bg-right`"#####, r#####"({
  backgroundPosition: "right",
})
;"##### ; "6")]
#[test_case(r#####"tw`bg-right-bottom`"#####, r#####"({
  backgroundPosition: "right bottom",
})
;"##### ; "7")]
#[test_case(r#####"tw`bg-right-top`"#####, r#####"({
  backgroundPosition: "right top",
})
;"##### ; "8")]
#[test_case(r#####"tw`bg-top`"#####, r#####"({
  backgroundPosition: "top",
})
;"##### ; "9")]
#[test_case(r#####"tw`bg-[position:200px 100px]`"#####, r#####"({
  backgroundPosition: "200px 100px",
})
;"##### ; "10")]
#[test_case(r#####"tw`bg-[position:var(--value)]`"#####, r#####"({
  backgroundPosition: "var(--value)",
})
;"##### ; "11")]
#[test_case(r#####"tw`bg-[center top 1rem]`"#####, r#####"({
  backgroundPosition: "center top 1rem",
})"##### ; "12")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
