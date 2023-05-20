use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"theme`cursor`"#####, r#####"({
  auto: "auto",
  default: "default",
  pointer: "pointer",
  wait: "wait",
  text: "text",
  move: "move",
  help: "help",
  'not-allowed': "not-allowed",
  none: "none",
  'context-menu': "context-menu",
  progress: "progress",
  cell: "cell",
  crosshair: "crosshair",
  'vertical-text': "vertical-text",
  alias: "alias",
  copy: "copy",
  'no-drop': "no-drop",
  grab: "grab",
  grabbing: "grabbing",
  'all-scroll': "all-scroll",
  'col-resize': "col-resize",
  'row-resize': "row-resize",
  'n-resize': "n-resize",
  'e-resize': "e-resize",
  's-resize': "s-resize",
  'w-resize': "w-resize",
  'ne-resize': "ne-resize",
  'nw-resize': "nw-resize",
  'se-resize': "se-resize",
  'sw-resize': "sw-resize",
  'ew-resize': "ew-resize",
  'ns-resize': "ns-resize",
  'nesw-resize': "nesw-resize",
  'nwse-resize': "nwse-resize",
  'zoom-in': "zoom-in",
  'zoom-out': "zoom-out",
})
;"##### ; "0")]
#[test_case(r#####"tw`cursor-auto`"#####, r#####"({
  cursor: "auto",
})
;"##### ; "1")]
#[test_case(r#####"tw`cursor-default`"#####, r#####"({
  cursor: "default",
})
;"##### ; "2")]
#[test_case(r#####"tw`cursor-pointer`"#####, r#####"({
  cursor: "pointer",
})
;"##### ; "3")]
#[test_case(r#####"tw`cursor-wait`"#####, r#####"({
  cursor: "wait",
})
;"##### ; "4")]
#[test_case(r#####"tw`cursor-text`"#####, r#####"({
  cursor: "text",
})
;"##### ; "5")]
#[test_case(r#####"tw`cursor-move`"#####, r#####"({
  cursor: "move",
})
;"##### ; "6")]
#[test_case(r#####"tw`cursor-help`"#####, r#####"({
  cursor: "help",
})
;"##### ; "7")]
#[test_case(r#####"tw`cursor-not-allowed`"#####, r#####"({
  cursor: "not-allowed",
})
;"##### ; "8")]
#[test_case(r#####"tw`cursor-none`"#####, r#####"({
  cursor: "none",
})
;"##### ; "9")]
#[test_case(r#####"tw`cursor-context-menu`"#####, r#####"({
  cursor: "context-menu",
})
;"##### ; "10")]
#[test_case(r#####"tw`cursor-progress`"#####, r#####"({
  cursor: "progress",
})
;"##### ; "11")]
#[test_case(r#####"tw`cursor-cell`"#####, r#####"({
  cursor: "cell",
})
;"##### ; "12")]
#[test_case(r#####"tw`cursor-crosshair`"#####, r#####"({
  cursor: "crosshair",
})
;"##### ; "13")]
#[test_case(r#####"tw`cursor-vertical-text`"#####, r#####"({
  cursor: "vertical-text",
})
;"##### ; "14")]
#[test_case(r#####"tw`cursor-alias`"#####, r#####"({
  cursor: "alias",
})
;"##### ; "15")]
#[test_case(r#####"tw`cursor-copy`"#####, r#####"({
  cursor: "copy",
})
;"##### ; "16")]
#[test_case(r#####"tw`cursor-no-drop`"#####, r#####"({
  cursor: "no-drop",
})
;"##### ; "17")]
#[test_case(r#####"tw`cursor-grab`"#####, r#####"({
  cursor: "grab",
})
;"##### ; "18")]
#[test_case(r#####"tw`cursor-grabbing`"#####, r#####"({
  cursor: "grabbing",
})
;"##### ; "19")]
#[test_case(r#####"tw`cursor-all-scroll`"#####, r#####"({
  cursor: "all-scroll",
})
;"##### ; "20")]
#[test_case(r#####"tw`cursor-col-resize`"#####, r#####"({
  cursor: "col-resize",
})
;"##### ; "21")]
#[test_case(r#####"tw`cursor-row-resize`"#####, r#####"({
  cursor: "row-resize",
})
;"##### ; "22")]
#[test_case(r#####"tw`cursor-n-resize`"#####, r#####"({
  cursor: "n-resize",
})
;"##### ; "23")]
#[test_case(r#####"tw`cursor-e-resize`"#####, r#####"({
  cursor: "e-resize",
})
;"##### ; "24")]
#[test_case(r#####"tw`cursor-s-resize`"#####, r#####"({
  cursor: "s-resize",
})
;"##### ; "25")]
#[test_case(r#####"tw`cursor-w-resize`"#####, r#####"({
  cursor: "w-resize",
})
;"##### ; "26")]
#[test_case(r#####"tw`cursor-ne-resize`"#####, r#####"({
  cursor: "ne-resize",
})
;"##### ; "27")]
#[test_case(r#####"tw`cursor-nw-resize`"#####, r#####"({
  cursor: "nw-resize",
})
;"##### ; "28")]
#[test_case(r#####"tw`cursor-se-resize`"#####, r#####"({
  cursor: "se-resize",
})
;"##### ; "29")]
#[test_case(r#####"tw`cursor-sw-resize`"#####, r#####"({
  cursor: "sw-resize",
})
;"##### ; "30")]
#[test_case(r#####"tw`cursor-ew-resize`"#####, r#####"({
  cursor: "ew-resize",
})
;"##### ; "31")]
#[test_case(r#####"tw`cursor-ns-resize`"#####, r#####"({
  cursor: "ns-resize",
})
;"##### ; "32")]
#[test_case(r#####"tw`cursor-nesw-resize`"#####, r#####"({
  cursor: "nesw-resize",
})
;"##### ; "33")]
#[test_case(r#####"tw`cursor-nwse-resize`"#####, r#####"({
  cursor: "nwse-resize",
})
;"##### ; "34")]
#[test_case(r#####"tw`cursor-zoom-in`"#####, r#####"({
  cursor: "zoom-in",
})
;"##### ; "35")]
#[test_case(r#####"tw`cursor-zoom-out`"#####, r#####"({
  cursor: "zoom-out",
})
;"##### ; "36")]
#[test_case(r#####"tw`cursor-[url(hand.cur), pointer]`"#####, r#####"({
  cursor: "url(hand.cur), pointer",
})"##### ; "37")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
