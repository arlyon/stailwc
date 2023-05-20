use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`block`"#####, r#####"({
  display: "block",
})
;"##### ; "1")]
#[test_case(r#####"tw`inline-block`"#####, r#####"({
  display: "inline-block",
})
;"##### ; "2")]
#[test_case(r#####"tw`inline`"#####, r#####"({
  display: "inline",
})
;"##### ; "3")]
#[test_case(r#####"tw`flex`"#####, r#####"({
  display: "flex",
})
;"##### ; "4")]
#[test_case(r#####"tw`inline-flex`"#####, r#####"({
  display: "inline-flex",
})
;"##### ; "5")]
#[test_case(r#####"tw`table`"#####, r#####"({
  display: "table",
})
;"##### ; "6")]
#[test_case(r#####"tw`table-caption`"#####, r#####"({
  display: "table-caption",
})
;"##### ; "7")]
#[test_case(r#####"tw`table-cell`"#####, r#####"({
  display: "table-cell",
})
;"##### ; "8")]
#[test_case(r#####"tw`table-column`"#####, r#####"({
  display: "table-column",
})
;"##### ; "9")]
#[test_case(r#####"tw`table-column-group`"#####, r#####"({
  display: "table-column-group",
})
;"##### ; "10")]
#[test_case(r#####"tw`table-footer-group`"#####, r#####"({
  display: "table-footer-group",
})
;"##### ; "11")]
#[test_case(r#####"tw`table-header-group`"#####, r#####"({
  display: "table-header-group",
})
;"##### ; "12")]
#[test_case(r#####"tw`table-row-group`"#####, r#####"({
  display: "table-row-group",
})
;"##### ; "13")]
#[test_case(r#####"tw`table-row`"#####, r#####"({
  display: "table-row",
})
;"##### ; "14")]
#[test_case(r#####"tw`flow-root`"#####, r#####"({
  display: "flow-root",
})
;"##### ; "15")]
#[test_case(r#####"tw`grid`"#####, r#####"({
  display: "grid",
})
;"##### ; "16")]
#[test_case(r#####"tw`inline-grid`"#####, r#####"({
  display: "inline-grid",
})
;"##### ; "17")]
#[test_case(r#####"tw`contents`"#####, r#####"({
  display: "contents",
})
;"##### ; "18")]
#[test_case(r#####"tw`list-item`"#####, r#####"({
  display: "list-item",
})
;"##### ; "19")]
#[test_case(r#####"tw`hidden`"#####, r#####"({
  display: "none",
})"##### ; "20")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
