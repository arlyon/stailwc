use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`backgroundImage`"#####, r#####"({
  none: "none",
  'gradient-to-t': "linear-gradient(to top, var(--tw-gradient-stops))",
  'gradient-to-tr': "linear-gradient(to top right, var(--tw-gradient-stops))",
  'gradient-to-r': "linear-gradient(to right, var(--tw-gradient-stops))",
  'gradient-to-br':
    "linear-gradient(to bottom right, var(--tw-gradient-stops))",
  'gradient-to-b': "linear-gradient(to bottom, var(--tw-gradient-stops))",
  'gradient-to-bl': "linear-gradient(to bottom left, var(--tw-gradient-stops))",
  'gradient-to-l': "linear-gradient(to left, var(--tw-gradient-stops))",
  'gradient-to-tl': "linear-gradient(to top left, var(--tw-gradient-stops))",
})
;"##### ; "1")]
#[test_case(r#####"tw`bg-none`"#####, r#####"({
  backgroundImage: "none",
})
;"##### ; "2")]
#[test_case(r#####"tw`bg-gradient-to-t`"#####, r#####"({
  backgroundImage: "linear-gradient(to top, var(--tw-gradient-stops))",
})
;"##### ; "3")]
#[test_case(r#####"tw`bg-gradient-to-tr`"#####, r#####"({
  backgroundImage: "linear-gradient(to top right, var(--tw-gradient-stops))",
})
;"##### ; "4")]
#[test_case(r#####"tw`bg-gradient-to-r`"#####, r#####"({
  backgroundImage: "linear-gradient(to right, var(--tw-gradient-stops))",
})
;"##### ; "5")]
#[test_case(r#####"tw`bg-gradient-to-br`"#####, r#####"({
  backgroundImage: "linear-gradient(to bottom right, var(--tw-gradient-stops))",
})
;"##### ; "6")]
#[test_case(r#####"tw`bg-gradient-to-b`"#####, r#####"({
  backgroundImage: "linear-gradient(to bottom, var(--tw-gradient-stops))",
})
;"##### ; "7")]
#[test_case(r#####"tw`bg-gradient-to-bl`"#####, r#####"({
  backgroundImage: "linear-gradient(to bottom left, var(--tw-gradient-stops))",
})
;"##### ; "8")]
#[test_case(r#####"tw`bg-gradient-to-l`"#####, r#####"({
  backgroundImage: "linear-gradient(to left, var(--tw-gradient-stops))",
})
;"##### ; "9")]
#[test_case(r#####"tw`bg-gradient-to-tl`"#####, r#####"({
  backgroundImage: "linear-gradient(to top left, var(--tw-gradient-stops))",
})
;"##### ; "10")]
#[test_case(r#####"tw`bg-[image:custom]`"#####, r#####"({
  backgroundImage: "custom",
})"##### ; "11")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
