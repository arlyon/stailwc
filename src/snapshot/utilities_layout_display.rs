use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`block`"#####, r#####"({
  display: "block",
})
;"##### ; "0")]
#[test_case(r#####"tw`inline-block`"#####, r#####"({
  display: "inline-block",
})
;"##### ; "1")]
#[test_case(r#####"tw`inline`"#####, r#####"({
  display: "inline",
})
;"##### ; "2")]
#[test_case(r#####"tw`flex`"#####, r#####"({
  display: "flex",
})
;"##### ; "3")]
#[test_case(r#####"tw`inline-flex`"#####, r#####"({
  display: "inline-flex",
})
;"##### ; "4")]
#[test_case(r#####"tw`table`"#####, r#####"({
  display: "table",
})
;"##### ; "5")]
#[test_case(r#####"tw`table-caption`"#####, r#####"({
  display: "table-caption",
})
;"##### ; "6")]
#[test_case(r#####"tw`table-cell`"#####, r#####"({
  display: "table-cell",
})
;"##### ; "7")]
#[test_case(r#####"tw`table-column`"#####, r#####"({
  display: "table-column",
})
;"##### ; "8")]
#[test_case(r#####"tw`table-column-group`"#####, r#####"({
  display: "table-column-group",
})
;"##### ; "9")]
#[test_case(r#####"tw`table-footer-group`"#####, r#####"({
  display: "table-footer-group",
})
;"##### ; "10")]
#[test_case(r#####"tw`table-header-group`"#####, r#####"({
  display: "table-header-group",
})
;"##### ; "11")]
#[test_case(r#####"tw`table-row-group`"#####, r#####"({
  display: "table-row-group",
})
;"##### ; "12")]
#[test_case(r#####"tw`table-row`"#####, r#####"({
  display: "table-row",
})
;"##### ; "13")]
#[test_case(r#####"tw`flow-root`"#####, r#####"({
  display: "flow-root",
})
;"##### ; "14")]
#[test_case(r#####"tw`grid`"#####, r#####"({
  display: "grid",
})
;"##### ; "15")]
#[test_case(r#####"tw`inline-grid`"#####, r#####"({
  display: "inline-grid",
})
;"##### ; "16")]
#[test_case(r#####"tw`contents`"#####, r#####"({
  display: "contents",
})
;"##### ; "17")]
#[test_case(r#####"tw`list-item`"#####, r#####"({
  display: "list-item",
})
;"##### ; "18")]
#[test_case(r#####"tw`hidden`"#####, r#####"({
  display: "none",
})"##### ; "19")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
