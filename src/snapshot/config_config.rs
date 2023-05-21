use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`animate-zoom-.5`"#####, r#####"({
  animation: "zoom-.5 2s",
})
;"##### ; "0")]
#[test_case(r#####"tw`text-number`"#####, r#####"({
  color: "0",
})
;"##### ; "1")]
#[test_case(r#####"tw`text-purple`"#####, r#####"({
  "--tw-text-opacity": "1",
  color: "rgb(128 0 128 / var(--tw-text-opacity))",
})
;"##### ; "2")]
#[test_case(r#####"tw`text-purple-hyphen`"#####, r#####"({
  color: "purple-hyphen",
})
;"##### ; "3")]
#[test_case(r#####"tw`text-mycolors`"#####, r#####"({
  "--tw-text-opacity": "1",
  color: "rgb(0 0 255 / var(--tw-text-opacity))",
})
;"##### ; "4")]
#[test_case(r#####"tw`text-mycolors-a-purple`"#####, r#####"({
  "--tw-text-opacity": "1",
  color: "rgb(128 0 128 / var(--tw-text-opacity))",
})
;"##### ; "5")]
#[test_case(r#####"tw`text-mycolors-a-number`"#####, r#####"({
  color: "0",
}) // tw`text-mycolors-array` // Arrays aren't supported

;"##### ; "6")]
#[test_case(r#####"tw`text-my-blue-100`"#####, r#####"({
  "--tw-text-opacity": "1",
  color: "rgb(0 0 255 / var(--tw-text-opacity))",
})
;"##### ; "7")]
#[test_case(r#####"tw`text-color-opacity`"#####, r#####"({
  "--tw-text-opacity": "1",
  color: "rgba(var(--color-primary), var(--tw-text-opacity, 1))",
})
;"##### ; "8")]
#[test_case(r#####"tw`text-color-deep-config-500`"#####, r#####"({
  "--tw-text-opacity": "1",
  color: "rgb(7 71 166 / var(--tw-text-opacity))",
})
;"##### ; "9")]
#[test_case(r#####"tw`bg-number`"#####, r#####"({
  backgroundColor: "0",
})
;"##### ; "10")]
#[test_case(r#####"tw`bg-purple`"#####, r#####"({
  "--tw-bg-opacity": "1",
  backgroundColor: "rgb(128 0 128 / var(--tw-bg-opacity))",
})
;"##### ; "11")]
#[test_case(r#####"tw`bg-purple-hyphen`"#####, r#####"({
  backgroundColor: "purple-hyphen",
})
;"##### ; "12")]
#[test_case(r#####"tw`bg-mycolors`"#####, r#####"({
  "--tw-bg-opacity": "1",
  backgroundColor: "rgb(0 0 255 / var(--tw-bg-opacity))",
})
;"##### ; "13")]
#[test_case(r#####"tw`bg-mycolors-a-purple`"#####, r#####"({
  "--tw-bg-opacity": "1",
  backgroundColor: "rgb(128 0 128 / var(--tw-bg-opacity))",
})
;"##### ; "14")]
#[test_case(r#####"tw`bg-mycolors-a-number`"#####, r#####"({
  backgroundColor: "0",
}) // tw`bg-mycolors-array` // Arrays aren't supported

;"##### ; "15")]
#[test_case(r#####"tw`bg-my-blue-100`"#####, r#####"({
  "--tw-bg-opacity": "1",
  backgroundColor: "rgb(0 0 255 / var(--tw-bg-opacity))",
})
;"##### ; "16")]
#[test_case(r#####"tw`bg-color-opacity`"#####, r#####"({
  "--tw-bg-opacity": "1",
  backgroundColor: "rgba(var(--color-primary), var(--tw-bg-opacity, 1))",
})
;"##### ; "17")]
#[test_case(r#####"tw`bg-color-deep-config-500`"#####, r#####"({
  "--tw-bg-opacity": "1",
  backgroundColor: "rgb(7 71 166 / var(--tw-bg-opacity))",
})
;"##### ; "18")]
#[test_case(r#####"tw`bg-blue`"#####, r#####"({
  backgroundColor: "blue-default",
})
;"##### ; "19")]
#[test_case(r#####"tw`bg-blue-gray`"#####, r#####"({
  backgroundColor: "blue-gray-default",
})
;"##### ; "20")]
#[test_case(r#####"tw`bg-blue-gray-200`"#####, r#####"({
  backgroundColor: "blue-gray-200",
})
;"##### ; "21")]
#[test_case(r#####"tw`bg-blue-gray-green`"#####, r#####"({
  backgroundColor: "blue-gray-green-default",
})
;"##### ; "22")]
#[test_case(r#####"tw`bg-blue-gray-green-200`"#####, r#####"({
  backgroundColor: "blue-gray-green-200",
})
;"##### ; "23")]
#[test_case(r#####"tw`bg-blue-gray-green-deep-dish`"#####, r#####"({
  backgroundColor: "blue-gray-green-deep-dish-default",
})
;"##### ; "24")]
#[test_case(r#####"tw`bg-blue-gray-green-deep-dish-200`"#####, r#####"({
  backgroundColor: "blue-gray-green-deep-dish-200",
})
;"##### ; "25")]
#[test_case(r#####"tw`bg-blue-gray-green-pink`"#####, r#####"({
  backgroundColor: "blue-gray-green-pink",
})
;"##### ; "26")]
#[test_case(r#####"tw`font-customFontWeightAsString`"#####, r#####"({
  fontWeight: "700",
})
;"##### ; "27")]
#[test_case(r#####"tw`font-customFontWeightAsNumber`"#####, r#####"({
  fontWeight: "800",
})"##### ; "28")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
