use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"<div css={tw`// comment`} />

// multiline
;"#####, r#####"<div css={{}} data-tw={"// comment"} /> // multiline
;"##### ; "0")]
#[test_case(r#####"<div css={tw`/* comment */`} />

// mixture
;"#####, r#####"<div css={{}} data-tw={"/* comment */"} /> // mixture
;"##### ; "1")]
#[test_case(r#####"<div
  css={tw`// comment  
/*
multline
comment
*/
block
// comment
`}
/>

// multiline comment
;"#####, r#####"<div
  css={{
    display: "block",
  }}
  data-tw={"// comment /* multline comment */ block // comment"}
/> // multiline comment
;"##### ; "2")]
#[test_case(r#####"<div
  css={tw`/*  block  
comment too
*/`}
/>

// singleline comment with class
;"#####, r#####"<div css={{}} data-tw={"/* block comment too */"} /> // singleline comment with class
;"##### ; "3")]
#[test_case(r#####"<div
  css={tw`// a comment
block
`}
/>

// multiline comment out a singleline comment with class
;"#####, r#####"<div
  css={{
    display: "block",
  }}
  data-tw={"// a comment block"}
/> // multiline comment out a singleline comment with class
;"##### ; "4")]
#[test_case(r#####"<div
  css={tw`/*
// comment */
block
`}
/>

// mixture with single and multiline on same line
;"#####, r#####"<div
  css={{
    display: "block",
  }}
  data-tw={"/* // comment */ block"}
/> // mixture with single and multiline on same line
;"##### ; "5")]
#[test_case(r#####"<div
  css={tw`// hi
// ho /*
hum
*/`}
/>

// comment in variant group and consecutive strings
;"#####, r#####"<div css={{}} data-tw={"// hi // ho /* hum */"} /> // comment in variant group and consecutive strings
;"##### ; "6")]
#[test_case(r#####"<div css={tw`md:(text-xl/* text-yellow-500 */font-black)`} />

// break right bracket
;"#####, r#####"<div
  css={{
    '@media (min-width: 768px)': {
      fontSize: "1.25rem",
      lineHeight: "1.75rem",
      fontWeight: "900",
    },
  }}
  data-tw={"md:(text-xl/* text-yellow-500 */font-black)"}
/> // break right bracket
;"##### ; "7")]
#[test_case(r#####"<div
  css={tw`2xl:(// ####@@@@ 
  [background:/*start*/rgb(191, 201/*inner*/, 211)])`}
/>

// comments within multiline comment
;"#####, r#####"<div
  css={{
    '@media (min-width: 1536px)': {
      background: "rgb(191, 201, 211)",
    },
  }}
  data-tw={
    "2xl:(// ####@@@@ [background:/*start*/rgb(191, 201/*inner*/, 211)])"
  }
/> // comments within multiline comment
;"##### ; "8")]
#[test_case(r#####"<div
  css={tw`relative
  lg:(
    /***
    helloworld
    /****/
    //***
    flex
    text-5xl
    border-yellow-500
    /****/
)!`}
/>"#####, r#####"<div
  css={{
    position: "relative",
    '@media (min-width: 1024px)': {
      display: "flex !important",
      '--tw-border-opacity': "1 !important",
      borderColor: "rgb(234 179 8 / var(--tw-border-opacity)) !important",
      fontSize: "3rem !important",
      lineHeight: "1 !important",
    },
  }}
  data-tw={
    "relative lg:( /*** helloworld /****/ //*** flex text-5xl border-yellow-500 /****/ )!"
  }
/>"##### ; "9")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
