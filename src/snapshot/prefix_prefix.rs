use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'

// tw prop prefix
;"#####, r#####";"##### ; "0")]
#[test_case(r#####"<div tw="tw-text-black" />

// tw import prefix
;"#####, r#####"<div
  css={{
    '--tw-text-opacity': "1",
    color: "rgb(0 0 0 / var(--tw-text-opacity))",
  }}
  data-tw="tw-text-black"
/> // tw import prefix
;"##### ; "1")]
#[test_case(r#####"<div css={tw`tw-bg-red-500`} />

// tw prop + import prefix
;"#####, r#####"<div
  css={{
    '--tw-bg-opacity': "1",
    backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
  }}
  data-tw={"tw-bg-red-500"}
/> // tw prop + import prefix
;"##### ; "2")]
#[test_case(r#####"<div tw="tw-text-black" css={tw`lg:tw-bg-red-500`} />

// tw import + short css
;"#####, r#####"<div
  css={[
    {
      '--tw-text-opacity': "1",
      color: "rgb(0 0 0 / var(--tw-text-opacity))",
    },
    {
      '@media (min-width: 1024px)': {
        '--tw-bg-opacity': "1",
        backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
      },
    },
  ]}
  data-tw={"tw-text-black | lg:tw-bg-red-500"}
/> // tw import + short css
;"##### ; "3")]
#[test_case(r#####"<div css={tw`lg:tw-bg-red-500 max-width[100vw]`} />

// tw import + arbitrary property
;"#####, r#####"<div
  css={{
    maxWidth: "100vw",
    '@media (min-width: 1024px)': {
      '--tw-bg-opacity': "1",
      backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
    },
  }}
  data-tw={"lg:tw-bg-red-500 max-width[100vw]"}
/> // tw import + arbitrary property
;"##### ; "4")]
#[test_case(r#####"<div css={tw`lg:tw-bg-red-500 [max-width:100vw]`} />

// className should be ignored without the prefix
;"#####, r#####"<div
  css={{
    maxWidth: "100vw",
    '@media (min-width: 1024px)': {
      '--tw-bg-opacity': "1",
      backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
    },
  }}
  data-tw={"lg:tw-bg-red-500 [max-width:100vw]"}
/> // className should be ignored without the prefix
;"##### ; "5")]
#[test_case(r#####"<div className="block" />

// className should be converted with a prefix
;"#####, r#####"<div className="block" /> // className should be converted with a prefix
;"##### ; "6")]
#[test_case(r#####"<div className="tw-block" />

// group
;"#####, r#####"<div
  css={{
    display: "block",
  }}
  data-tw="tw-block"
/> // group
;"##### ; "7")]
#[test_case(r#####"<div tw="hover:(lg:tw-bg-red-500)" />
;"#####, r#####"<div
  css={{
    '@media (min-width: 1024px)': {
      ':hover': {
        '--tw-bg-opacity': "1",
        backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
      },
    },
  }}
  data-tw="hover:(lg:tw-bg-red-500)"
/>
;"##### ; "8")]
#[test_case(r#####"<div tw="hover:(lg:tw-bg-red-500 max-width[100vw])" />
;"#####, r#####"<div
  css={{
    ':hover': {
      maxWidth: "100vw",
    },
    '@media (min-width: 1024px)': {
      ':hover': {
        '--tw-bg-opacity': "1",
        backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
      },
    },
  }}
  data-tw="hover:(lg:tw-bg-red-500 max-width[100vw])"
/>
;"##### ; "9")]
#[test_case(r#####"<div tw="hover:(lg:tw-bg-red-500 [max-width:100vw])" />
;"#####, r#####"<div
  css={{
    ':hover': {
      maxWidth: "100vw",
    },
    '@media (min-width: 1024px)': {
      ':hover': {
        '--tw-bg-opacity': "1",
        backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
      },
    },
  }}
  data-tw="hover:(lg:tw-bg-red-500 [max-width:100vw])"
/>
;"##### ; "10")]
#[test_case(r#####"<div css={tw`hover:(lg:tw-bg-red-500)`} />
;"#####, r#####"<div
  css={{
    '@media (min-width: 1024px)': {
      ':hover': {
        '--tw-bg-opacity': "1",
        backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
      },
    },
  }}
  data-tw={"hover:(lg:tw-bg-red-500)"}
/>
;"##### ; "11")]
#[test_case(r#####"<div css={tw`hover:(lg:tw-bg-red-500 max-width[100vw])`} />
;"#####, r#####"<div
  css={{
    ':hover': {
      maxWidth: "100vw",
    },
    '@media (min-width: 1024px)': {
      ':hover': {
        '--tw-bg-opacity': "1",
        backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
      },
    },
  }}
  data-tw={"hover:(lg:tw-bg-red-500 max-width[100vw])"}
/>
;"##### ; "12")]
#[test_case(r#####"<div css={tw`hover:(lg:tw-bg-red-500 [max-width:100vw])`} />

// custom plugin classes
;"#####, r#####"<div
  css={{
    ':hover': {
      maxWidth: "100vw",
    },
    '@media (min-width: 1024px)': {
      ':hover': {
        '--tw-bg-opacity': "1",
        backgroundColor: "rgb(239 68 68 / var(--tw-bg-opacity))",
      },
    },
  }}
  data-tw={"hover:(lg:tw-bg-red-500 [max-width:100vw])"}
/> // custom plugin classes
;"##### ; "13")]
#[test_case(r#####"<div tw="tw-plugin-class" />
;"#####, r#####"<div
  css={{
    content: "working",
  }}
  data-tw="tw-plugin-class"
/>
;"##### ; "14")]
#[test_case(r#####"<div tw="tw-test-1" />
;"#####, r#####"<div
  css={{
    background: "5px",
    '.tw-a-class & .tw-some-class': {
      margin: "10px",
    },
    '.tw-a-class & > *': {
      margin: "20px",
    },
  }}
  data-tw="tw-test-1"
/>
;"##### ; "15")]
#[test_case(r#####"<div tw="tw-test-2" />"#####, r#####"<div
  css={{
    '.tw-a-class & .tw-some-class': {
      margin: "10px",
    },
    '.tw-a-class & > *': {
      margin: "20px",
    },
  }}
  data-tw="tw-test-2"
/>"##### ; "16")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
