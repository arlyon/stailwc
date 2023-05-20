use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"<div cs="maxWidth[100vw - 2rem]" />
;"#####, r#####"<div
  css={{
    maxWidth: "100vw - 2rem",
  }}
/>
;"##### ; "0")]
#[test_case(r#####"<div cs="maxWidth[100vw - 2rem]!" />
;"#####, r#####"<div
  css={{
    maxWidth: "100vw - 2rem !important",
  }}
/>
;"##### ; "1")]
#[test_case(r#####"<div cs="md:maxWidth[100vw - 2rem]" />
;"#####, r#####"<div
  css={{
    '@media (min-width: 768px)': {
      maxWidth: "100vw - 2rem",
    },
  }}
/>
;"##### ; "2")]
#[test_case(r#####"<div cs="hover:(maxWidth[100vw - 2rem] width[2rem])" />
;"#####, r#####"<div
  css={{
    ':hover': {
      maxWidth: "100vw - 2rem",
      width: "2rem",
    },
  }}
/>
;"##### ; "3")]
#[test_case(r#####"<div cs="hover:(maxWidth[100vw - 2rem] before:content['test'])" />
;"#####, r#####"<div
  css={{
    ':hover': {
      maxWidth: "100vw - 2rem",
    },
    ':hover::before': {
      content: "'test'",
    },
  }}
/>
;"##### ; "4")]
#[test_case(r#####"<div cs="hover:(maxWidth[100vw - 2rem] before:content['test'])!" />
;"#####, r#####"<div
  css={{
    ':hover': {
      maxWidth: "100vw - 2rem !important",
    },
    ':hover::before': {
      content: "'test' !important",
    },
  }}
/>
;"##### ; "5")]
#[test_case(r#####"<div cs="hover:(maxWidth[100vw - 2rem]! before:content['test'])" />

// within tw prop
;"#####, r#####"<div
  css={{
    ':hover': {
      maxWidth: "100vw - 2rem !important",
    },
    ':hover::before': {
      content: "'test'",
    },
  }}
/> // within tw prop
;"##### ; "6")]
#[test_case(r#####"<div tw="maxWidth[100vw - 2rem]" />
;"#####, r#####"<div
  css={{
    maxWidth: "100vw - 2rem",
  }}
/>
;"##### ; "7")]
#[test_case(r#####"<div tw="maxWidth[100vw - 2rem] block" />
;"#####, r#####"<div
  css={{
    display: "block",
    maxWidth: "100vw - 2rem",
  }}
/>
;"##### ; "8")]
#[test_case(r#####"<div tw="md:maxWidth[100vw - 2rem]" />
;"#####, r#####"<div
  css={{
    '@media (min-width: 768px)': {
      maxWidth: "100vw - 2rem",
    },
  }}
/>
;"##### ; "9")]
#[test_case(r#####"<div tw="hover:(maxWidth[100vw - 2rem] width[2rem])" />
;"#####, r#####"<div
  css={{
    ':hover': {
      maxWidth: "100vw - 2rem",
      width: "2rem",
    },
  }}
/>
;"##### ; "10")]
#[test_case(r#####"<div tw="hover:(maxWidth[100vw - 2rem] before:content['test'])" />
;"#####, r#####"<div
  css={{
    ':hover': {
      maxWidth: "100vw - 2rem",
    },
    ':hover::before': {
      content: "'test'",
    },
  }}
/>
;"##### ; "11")]
#[test_case(r#####"<div tw="hover:(maxWidth[100vw - 2rem] before:content['test'])!" />
;"#####, r#####"<div
  css={{
    ':hover': {
      maxWidth: "100vw - 2rem !important",
    },
    ':hover::before': {
      content: "'test' !important",
    },
  }}
/>
;"##### ; "12")]
#[test_case(r#####"<div tw="hover:(maxWidth[100vw - 2rem]! before:content['test'])" />

// within css prop
;"#####, r#####"<div
  css={{
    ':hover': {
      maxWidth: "100vw - 2rem !important",
    },
    ':hover::before': {
      content: "'test'",
    },
  }}
/> // within css prop
;"##### ; "13")]
#[test_case(r#####"<div css={tw`lg:bg-red-500 max-width[100vw]`} />"#####, r#####"<div
  css={{
    maxWidth: "100vw",
    '@media (min-width: 1024px)': {
      '--tw-bg-opacity': "1",
      backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
    },
  }}
/> // within tw import

;"##### ; "14")]
#[test_case(r#####"tw`maxWidth[100vw - 2rem]`"#####, r#####"({
  maxWidth: "100vw - 2rem",
})
;"##### ; "15")]
#[test_case(r#####"tw`maxWidth[100vw - 2rem] block`"#####, r#####"({
  display: "block",
  maxWidth: "100vw - 2rem",
})
;"##### ; "16")]
#[test_case(r#####"tw`md:maxWidth[100vw - 2rem]`"#####, r#####"({
  '@media (min-width: 768px)': {
    maxWidth: "100vw - 2rem",
  },
})
;"##### ; "17")]
#[test_case(r#####"tw`hover:(maxWidth[100vw - 2rem] width[2rem])`"#####, r#####"({
  ':hover': {
    maxWidth: "100vw - 2rem",
    width: "2rem",
  },
})
;"##### ; "18")]
#[test_case(r#####"tw`hover:(maxWidth[100vw - 2rem] before:content['test'])`"#####, r#####"({
  ':hover': {
    maxWidth: "100vw - 2rem",
  },
  ':hover::before': {
    content: "'test'",
  },
})
;"##### ; "19")]
#[test_case(r#####"tw`hover:(maxWidth[100vw - 2rem] before:content['test'])!`"#####, r#####"({
  ':hover': {
    maxWidth: "100vw - 2rem !important",
  },
  ':hover::before': {
    content: "'test' !important",
  },
})
;"##### ; "20")]
#[test_case(r#####"tw`hover:(maxWidth[100vw - 2rem]! before:content['test'])`

// prop ordering
;"#####, r#####"({
  ':hover': {
    maxWidth: "100vw - 2rem !important",
  },
  ':hover::before': {
    content: "'test'",
  },
}) // prop ordering
;"##### ; "21")]
#[test_case(r#####"<div css={{ color: "red" }} cs="margin[50px]" tw="mt-4 content['content']" />"#####, r#####"<div
  css={[
    {
      color: "red",
    },
    {
      margin: "50px",
    },
    {
      marginTop: "1rem",
      content: "'content'",
    },
  ]}
/> // Setting css variables

;"##### ; "22")]
#[test_case(r#####"tw`--css-prop[true] md:--css-prop[false]`"#####, r#####"({
  '--css-prop': "true",
  '@media (min-width: 768px)': {
    '--css-prop': "false",
  },
}) // Using css variables

;"##### ; "23")]
#[test_case(r#####"tw`max-width[var(--css-react)] md:max-width[var(--css-react-md)]`"#####, r#####"({
  maxWidth: "var(--css-react)",
  '@media (min-width: 768px)': {
    maxWidth: "var(--css-react-md)",
  },
}) // Browser vendor prefixes

;"##### ; "24")]
#[test_case(r#####"tw`-webkit-gradient[gradient here] md:-webkit-gradient[gradient here md]`"#####, r#####"({
  WebkitGradient: "gradient here",
  '@media (min-width: 768px)': {
    WebkitGradient: "gradient here md",
  },
}) // Grid template

;"##### ; "25")]
#[test_case(r#####"tw`grid-template-columns[[main-start] 1fr [content-start] 1fr [content-end] 1fr [main-end]] md:grid-template-columns[[main-start-md] 1fr [content-start-md] 1fr [content-end-md] 1fr [main-end-md]]`"#####, r#####"({
  gridTemplateColumns:
    "[main-start] 1fr [content-start] 1fr [content-end] 1fr [main-end]",
  '@media (min-width: 768px)': {
    gridTemplateColumns:
      "[main-start-md] 1fr [content-start-md] 1fr [content-end-md] 1fr [main-end-md]",
  },
}) // Short css trumps core plugins

;"##### ; "26")]
#[test_case(r#####"tw`transition-property[margin]`"#####, r#####"({
  transitionProperty: "margin",
}) // Crazy calcs

;"##### ; "27")]
#[test_case(r#####"tw`padding[calc((2em * -1) + var(--myVar))]`"#####, r#####"({
  padding: "calc((2em * -1) + var(--myVar))",
}) // Multiline

;"##### ; "28")]
#[test_case(r#####"tw`padding[
    calc((2em * -1) + var(--myVar))
]`"#####, r#####"({
  padding: "calc((2em * -1) + var(--myVar))",
})
;"##### ; "29")]
#[test_case(r#####"tw`padding[
    calc((2em * -1) +
    var(--myVar))
]`"#####, r#####"({
  padding: "calc((2em * -1) + var(--myVar))",
}) // Theme value

;"##### ; "30")]
#[test_case(r#####"tw`--color[theme(colors.red.500)]`"#####, r#####"({
  '--color': "#ef4444",
})
;"##### ; "31")]
#[test_case(r#####"tw`--color[this theme(colors.red.500) that]`"#####, r#####"({
  '--color': "this #ef4444 that",
}) // Automatic '' value

;"##### ; "32")]
#[test_case(r#####"tw`touch-action[]`"#####, r#####"({
  touchAction: "''",
})"##### ; "33")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
