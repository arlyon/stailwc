use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`before:valid:rtl:motion-safe:contrast-more:dark:print:portrait:any-pointer-fine:block`"#####, r#####"({
  '@media (any-pointer: fine)': {
    '@media (orientation: portrait)': {
      '@media print': {
        '@media (prefers-color-scheme: dark)': {
          '@media (prefers-contrast: more)': {
            '@media (prefers-reduced-motion: no-preference)': {
              ':is([dir="rtl"] &:valid)::before': {
                content: "var(--tw-content)",
                display: "block",
              },
            },
          },
        },
      },
    },
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`any-pointer-fine:portrait:print:dark:contrast-more:motion-safe:rtl:valid:before:mt-5`"#####, r#####"({
  '@media (prefers-reduced-motion: no-preference)': {
    '@media (prefers-contrast: more)': {
      '@media (prefers-color-scheme: dark)': {
        '@media print': {
          '@media (orientation: portrait)': {
            '@media (any-pointer: fine)': {
              ':is([dir="rtl"] &):valid::before': {
                content: "var(--tw-content)",
                marginTop: "1.25rem",
              },
            },
          },
        },
      },
    },
  },
})"##### ; "2")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
