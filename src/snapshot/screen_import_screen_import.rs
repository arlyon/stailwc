use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"screen`sm`"#####, r#####"("@media (min-width: 100px)")
;"##### ; "0")]
#[test_case(r#####"screen.md"#####, r#####"("@media (min-width: 200px)") // Can't work with screen values that begin with a number, eg: screen.2xl

;"##### ; "1")]
#[test_case(r#####"screen("lg")"#####, r#####"("@media (min-width: 300px)")
;"##### ; "2")]
#[test_case(r#####"screen(`xl`)

// Constructed media queries
;"#####, r#####"("@media (min-width: 400px)") // Constructed media queries
;"##### ; "3")]
#[test_case(r#####"`
    ${screen`sm`} {
        display: block;
        ${tw`inline`}
    }
`
;"#####, r#####"`
    ${"@media (min-width: 100px)"} {
        display: block;
        ${{
          display: "inline",
        }}
    }
`
;"##### ; "4")]
#[test_case(r#####"({ [screen`sm`]: `display: block; ${tw`inline`}` })
;"#####, r#####"({
  '@media (min-width: 100px)': `display: block; ${{
    display: "inline",
  }}`,
})
;"##### ; "5")]
#[test_case(r#####"({ [screen`sm`]: { display: "block", ...tw`inline` } })"#####, r#####"({
  '@media (min-width: 100px)': {
    display: "block",
    ...{
      display: "inline",
    },
  },
}) // Media queries with styles

;"##### ; "6")]
#[test_case(r#####"screen.sm({ color: `red` })"#####, r#####"({
  '@media (min-width: 100px)': {
    color: `red`,
  },
})
;"##### ; "7")]
#[test_case(r#####"screen`md`({ color: `red` })"#####, r#####"({
  '@media (min-width: 200px)': {
    color: `red`,
  },
})
;"##### ; "8")]
#[test_case(r#####"screen("lg")({ color: `red` })"#####, r#####"({
  '@media (min-width: 300px)': {
    color: `red`,
  },
})
;"##### ; "9")]
#[test_case(r#####"screen(`xl`)({ color: `red` })"#####, r#####"({
  '@media (min-width: 400px)': {
    color: `red`,
  },
})
;"##### ; "10")]
#[test_case(r#####"screen.sm`color: red;`"#####, r#####"`@media (min-width: 100px) { ${`color: red;`} }`
;"##### ; "11")]
#[test_case(r#####"screen`md``color: red;`"#####, r#####"`@media (min-width: 200px) { ${`color: red;`} }`
;"##### ; "12")]
#[test_case(r#####"screen("lg")`color: red;`"#####, r#####"`@media (min-width: 300px) { ${`color: red;`} }`
;"##### ; "13")]
#[test_case(r#####"screen(`xl`)`color: red;`"#####, r#####"`@media (min-width: 400px) { ${`color: red;`} }`
;"##### ; "14")]
#[test_case(r#####"screen.xl(tw`inline`)"#####, r#####"({
  '@media (min-width: 400px)': {
    display: "inline",
  },
})
;"##### ; "15")]
#[test_case(r#####"screen.xl({ ...tw`inline` })"#####, r#####"({
  '@media (min-width: 400px)': {
    ...{
      display: "inline",
    },
  },
})
;"##### ; "16")]
#[test_case(r#####"screen.xl({ ...tw`inline`, display: "block" })"#####, r#####"({
  '@media (min-width: 400px)': {
    ...{
      display: "inline",
    },
    display: "block",
  },
})
;"##### ; "17")]
#[test_case(r#####"screen.xl`
    ${tw`inline`}
    display: block;
`"#####, r#####"`@media (min-width: 400px) { ${`
    ${{
      display: "inline",
    }}
    display: block;
`} }`
;"##### ; "18")]
#[test_case(r#####"screen.xl`color: ${true && "blue"};`

// Within template literals
;"#####, r#####"`@media (min-width: 400px) { ${`color: ${true && "blue"};`} }` // Within template literals
;"##### ; "19")]
#[test_case(r#####"`${screen.lg}`
;"#####, r#####"`${"@media (min-width: 300px)"}`
;"##### ; "20")]
#[test_case(r#####"`${screen`xl`}`
;"#####, r#####"`${"@media (min-width: 400px)"}`
;"##### ; "21")]
#[test_case(r#####"`${screen(`xl`)}`
;"#####, r#####"`${"@media (min-width: 400px)"}`
;"##### ; "22")]
#[test_case(r#####"`${screen("xl")}`

// Screen keys
;"#####, r#####"`${"@media (min-width: 400px)"}` // Screen keys
;"##### ; "23")]
#[test_case(r#####"<div
  css={{
    [screen.xl]: { color: "red" },
  }}
/>
;"#####, r#####"<div
  css={{
    '@media (min-width: 400px)': {
      color: "red",
    },
  }}
/>
;"##### ; "24")]
#[test_case(r#####"<div
  css={`
    ${{ [screen.xl]: { color: "red" } }}
  `}
/>
;"#####, r#####"<div
  css={`
    ${{
      '@media (min-width: 400px)': {
        color: "red",
      },
    }}
  `}
/>
;"##### ; "25")]
#[test_case(r#####"<div css={[{ [screen.xl]: { color: "red" } }]} />
;"#####, r#####"<div
  css={[
    {
      '@media (min-width: 400px)': {
        color: "red",
      },
    },
  ]}
/>
;"##### ; "26")]
#[test_case(r#####"<div
  css={`
    ${screen.xl} {
      color: red;
    }
  `}
/>"#####, r#####"<div
  css={`
    ${"@media (min-width: 400px)"} {
      color: red;
    }
  `}
/>"##### ; "27")]
#[test_case(r#####"styled.div`
  ${{ [screen.xl]: { color: "red" } }}
`"#####, r#####"_styled.div`
  ${{
    '@media (min-width: 400px)': {
      color: "red",
    },
  }}
`"##### ; "28")]
#[test_case(r#####"styled.div([{ [screen.xl]: { color: "red" } }])

// Logical expressions
;"#####, r#####"_styled.div([
  {
    '@media (min-width: 400px)': {
      color: "red",
    },
  },
]) // Logical expressions

;"##### ; "29")]
#[test_case(r#####"<div
  css={{
    [true && screen.xl]: { color: "red" },
  }}
/>"#####, r#####"<div
  css={{
    [true && "@media (min-width: 400px)"]: {
      color: "red",
    },
  }}
/>"##### ; "30")]
#[test_case(r#####"styled.div([{ [true && screen.xl]: { color: "red" } }])

// Conditional expressions
;"#####, r#####"_styled.div([
  {
    [true && "@media (min-width: 400px)"]: {
      color: "red",
    },
  },
]) // Conditional expressions

;"##### ; "31")]
#[test_case(r#####"<div
  css={{
    // eslint-disable-next-line no-constant-condition
    [true ? screen.xl : screen.sm]: { color: "red" },
  }}
/>"#####, r#####"<div
  css={{
    // eslint-disable-next-line no-constant-condition
    [true ? "@media (min-width: 400px)" : "@media (min-width: 100px)"]: {
      color: "red",
    },
  }}
/>"##### ; "32")]
#[test_case(r#####"styled.div`
  ${{
    // eslint-disable-next-line no-constant-condition
    [true ? screen.xl : screen.sm]: { color: "red" },
  }}
`

// Screen with values
;"#####, r#####"_styled.div`
  ${{
    // eslint-disable-next-line no-constant-condition
    [true ? "@media (min-width: 400px)" : "@media (min-width: 100px)"]: {
      color: "red",
    },
  }}
` // Screen with values
;"##### ; "33")]
#[test_case(r#####"<div css={screen.xl({ color: "red" })} />
;"#####, r#####"<div
  css={{
    '@media (min-width: 400px)': {
      color: "red",
    },
  }}
/>
;"##### ; "34")]
#[test_case(r#####"<div css={[screen.xl({ color: "red" })]} />
;"#####, r#####"<div
  css={[
    {
      '@media (min-width: 400px)': {
        color: "red",
      },
    },
  ]}
/>
;"##### ; "35")]
#[test_case(r#####"<div
  css={`
    ${screen.xl({ color: "red" })}
  `}
/>
;"#####, r#####"<div
  css={`
    ${{
      '@media (min-width: 400px)': {
        color: "red",
      },
    }}
  `}
/>
;"##### ; "36")]
#[test_case(r#####"<div css={screen.xl`color: red;`} />
;"#####, r#####"<div
  css={`
    @media (min-width: 400px) {
      ${`color: red;`}
    }
  `}
/>
;"##### ; "37")]
#[test_case(r#####"<div css={[screen.xl`color: red;`]} />
;"#####, r#####"<div css={[`@media (min-width: 400px) { ${`color: red;`} }`]} />
;"##### ; "38")]
#[test_case(r#####"<div
  css={`
    ${screen.xl`color: red;`}
  `}
/>"#####, r#####"<div
  css={`
    ${`@media (min-width: 400px) { ${`color: red;`} }`}
  `}
/>"##### ; "39")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
