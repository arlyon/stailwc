use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"const Box = tw.div`text-red-500`"#####, r#####"const Box = _styled.div({
  '--tw-text-opacity': "1",
  color: "rgb(239 68 68 / var(--tw-text-opacity))",
})"##### ; "0")]
#[test_case(r#####"const Important = tw`lg:uppercase!`"#####, r#####"const Important = {
  '@media (min-width: 1024px)': {
    textTransform: "uppercase !important",
  },
}"##### ; "1")]
#[test_case(r#####"const MediaImportant = tw.div`lg:text-red-500!`"#####, r#####"const MediaImportant = _styled.div({
  '@media (min-width: 1024px)': {
    '--tw-text-opacity': "1 !important",
    color: "rgb(239 68 68 / var(--tw-text-opacity)) !important",
  },
})"##### ; "2")]
#[test_case(r#####"const ElementMediaImportant = tw(Box)`lg:bg-blue-500!`"#####, r#####"const ElementMediaImportant = _styled(Box)({
  '@media (min-width: 1024px)': {
    '--tw-bg-opacity': "1 !important",
    backgroundColor: "rgb(59 130 246 / var(--tw-bg-opacity)) !important",
  },
})"##### ; "3")]
#[test_case(r#####"const PlaceholderImportant = tw.input`placeholder-red-500!`"#####, r#####"const PlaceholderImportant = _styled.input({
  '::placeholder': {
    '--tw-placeholder-opacity': "1 !important",
    color: "rgb(239 68 68 / var(--tw-placeholder-opacity)) !important",
  },
})"##### ; "4")]
#[test_case(r#####"const StateImportant = tw.input`hover:text-red-500!`"#####, r#####"const StateImportant = _styled.input({
  ':hover': {
    '--tw-text-opacity': "1 !important",
    color: "rgb(239 68 68 / var(--tw-text-opacity)) !important",
  },
})"##### ; "5")]
#[test_case(r#####"const StatePlaceholderImportant = tw.input`hover:placeholder-red-500!`"#####, r#####"const StatePlaceholderImportant = _styled.input({
  ':hover::placeholder': {
    '--tw-placeholder-opacity': "1 !important",
    color: "rgb(239 68 68 / var(--tw-placeholder-opacity)) !important",
  },
})"##### ; "6")]
#[test_case(r#####"const StateStatePlaceholderImportant = tw.input`active:hover:placeholder-red-500!`"#####, r#####"const StateStatePlaceholderImportant = _styled.input({
  ':active:hover::placeholder': {
    '--tw-placeholder-opacity': "1 !important",
    color: "rgb(239 68 68 / var(--tw-placeholder-opacity)) !important",
  },
})"##### ; "7")]
#[test_case(r#####"const StateMultiplePropertiesImportant = tw.input`hover:truncate!`"#####, r#####"const StateMultiplePropertiesImportant = _styled.input({
  ':hover': {
    overflow: "hidden !important",
    textOverflow: "ellipsis !important",
    whiteSpace: "nowrap !important",
  },
})"##### ; "8")]
#[test_case(r#####"const MediaStateMultiplePropertiesImportant = tw.input`lg:hover:truncate!`"#####, r#####"const MediaStateMultiplePropertiesImportant = _styled.input({
  '@media (min-width: 1024px)': {
    ':hover': {
      overflow: "hidden !important",
      textOverflow: "ellipsis !important",
      whiteSpace: "nowrap !important",
    },
  },
})"##### ; "9")]
#[test_case(r#####"const ElementMediaStateMultiplePropertiesImportant = tw(Box)`lg:hover:truncate!`"#####, r#####"const ElementMediaStateMultiplePropertiesImportant = _styled(Box)({
  '@media (min-width: 1024px)': {
    ':hover': {
      overflow: "hidden !important",
      textOverflow: "ellipsis !important",
      whiteSpace: "nowrap !important",
    },
  },
})"##### ; "10")]
#[test_case(r#####"const JsxPlaceholderImportant = () => <input tw="placeholder-green-500!" />"#####, r#####"const JsxPlaceholderImportant = () => (
  <input
    css={{
      '::placeholder': {
        '--tw-placeholder-opacity': "1 !important",
        color: "rgb(34 197 94 / var(--tw-placeholder-opacity)) !important",
      },
    }}
  />
)"##### ; "11")]
#[test_case(r#####"const ImportantPrefixPrefix = tw`lg:!uppercase`"#####, r#####"const ImportantPrefixPrefix = {
  '@media (min-width: 1024px)': {
    textTransform: "uppercase !important",
  },
}"##### ; "12")]
#[test_case(r#####"const MediaImportantPrefix = tw.div`lg:!text-red-500`"#####, r#####"const MediaImportantPrefix = _styled.div({
  '@media (min-width: 1024px)': {
    '--tw-text-opacity': "1 !important",
    color: "rgb(239 68 68 / var(--tw-text-opacity)) !important",
  },
})"##### ; "13")]
#[test_case(r#####"const ElementMediaImportantPrefix = tw(Box)`lg:!bg-blue-500`"#####, r#####"const ElementMediaImportantPrefix = _styled(Box)({
  '@media (min-width: 1024px)': {
    '--tw-bg-opacity': "1 !important",
    backgroundColor: "rgb(59 130 246 / var(--tw-bg-opacity)) !important",
  },
})"##### ; "14")]
#[test_case(r#####"const PlaceholderImportantPrefix = tw.input`!placeholder-red-500`"#####, r#####"const PlaceholderImportantPrefix = _styled.input({
  '::placeholder': {
    '--tw-placeholder-opacity': "1 !important",
    color: "rgb(239 68 68 / var(--tw-placeholder-opacity)) !important",
  },
})"##### ; "15")]
#[test_case(r#####"const StateImportantPrefix = tw.input`hover:!text-red-500`"#####, r#####"const StateImportantPrefix = _styled.input({
  ':hover': {
    '--tw-text-opacity': "1 !important",
    color: "rgb(239 68 68 / var(--tw-text-opacity)) !important",
  },
})"##### ; "16")]
#[test_case(r#####"const StatePlaceholderImportatntPrefix = tw.input`hover:!placeholder-red-500`"#####, r#####"const StatePlaceholderImportantPrefix = _styled.input({
  ':hover::placeholder': {
    '--tw-placeholder-opacity': "1 !important",
    color: "rgb(239 68 68 / var(--tw-placeholder-opacity)) !important",
  },
})"##### ; "17")]
#[test_case(r#####"const StateStatePlaceholderImportantPrefix = tw.input`active:hover:!placeholder-red-500`"#####, r#####"const StateStatePlaceholderImportantPrefix = _styled.input({
  ':active:hover::placeholder': {
    '--tw-placeholder-opacity': "1 !important",
    color: "rgb(239 68 68 / var(--tw-placeholder-opacity)) !important",
  },
})"##### ; "18")]
#[test_case(r#####"const StateMultiplePropertiesImportantPrefix = tw.input`hover:!truncate`"#####, r#####"const StateMultiplePropertiesImportantPrefix = _styled.input({
  ':hover': {
    overflow: "hidden !important",
    textOverflow: "ellipsis !important",
    whiteSpace: "nowrap !important",
  },
})"##### ; "19")]
#[test_case(r#####"const MediaStateMultiplePropertiesImportantPrefix = tw.input`lg:hover:!truncate`"#####, r#####"const MediaStateMultiplePropertiesImportantPrefix = _styled.input({
  '@media (min-width: 1024px)': {
    ':hover': {
      overflow: "hidden !important",
      textOverflow: "ellipsis !important",
      whiteSpace: "nowrap !important",
    },
  },
})"##### ; "20")]
#[test_case(r#####"const ElementMediaStateMultiplePropertiesImportantPrefix = tw(
  Box
)`lg:hover:!truncate`"#####, r#####"const ElementMediaStateMultiplePropertiesImportantPrefix = _styled(Box)({
  '@media (min-width: 1024px)': {
    ':hover': {
      overflow: "hidden !important",
      textOverflow: "ellipsis !important",
      whiteSpace: "nowrap !important",
    },
  },
})"##### ; "21")]
#[test_case(r#####"const VariantImportantPrefixMergeCheck = tw.div`md:!from-black to-[#dc4fc2] bg-gradient-to-r`"#####, r#####"const VariantImportantPrefixMergeCheck = _styled.div({
  backgroundImage: "linear-gradient(to right, var(--tw-gradient-stops))",
  '--tw-gradient-to': "#dc4fc2 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '@media (min-width: 768px)': {
    '--tw-gradient-from': "#000 var(--tw-gradient-from-position) !important",
    '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
    '--tw-gradient-to':
      "rgb(0 0 0 / 0)  var(--tw-gradient-from-position) !important",
    '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
    '--tw-gradient-stops':
      "var(--tw-gradient-from), var(--tw-gradient-to) !important",
  },
})"##### ; "22")]
#[test_case(r#####"const MultiVariantImportantPrefixMergeCheck = tw.div`first:md:!from-black to-[#dc4fc2] bg-gradient-to-r`"#####, r#####"const MultiVariantImportantPrefixMergeCheck = _styled.div({
  backgroundImage: "linear-gradient(to right, var(--tw-gradient-stops))",
  '--tw-gradient-to': "#dc4fc2 var(--tw-gradient-to-position)",
  '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
  '@media (min-width: 768px)': {
    ':first-child': {
      '--tw-gradient-from': "#000 var(--tw-gradient-from-position) !important",
      '--tw-gradient-from-position': "var(--tw-empty,/*!*/ /*!*/)",
      '--tw-gradient-to':
        "rgb(0 0 0 / 0)  var(--tw-gradient-from-position) !important",
      '--tw-gradient-to-position': "var(--tw-empty,/*!*/ /*!*/)",
      '--tw-gradient-stops':
        "var(--tw-gradient-from), var(--tw-gradient-to) !important",
    },
  },
})"##### ; "23")]
#[test_case(r#####"const JsxPlaceholderImportantPrefix = () => (
  <input tw="!placeholder-green-500" />
)"#####, r#####"const JsxPlaceholderImportantPrefix = () => (
  <input
    css={{
      '::placeholder': {
        '--tw-placeholder-opacity': "1 !important",
        color: "rgb(34 197 94 / var(--tw-placeholder-opacity)) !important",
      },
    }}
  />
)"##### ; "24")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
