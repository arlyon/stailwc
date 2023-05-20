use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`first-letter:block`"#####, r#####"({
  '::first-letter': {
    display: "block",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`first-line:block`"#####, r#####"({
  '::first-line': {
    display: "block",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`marker:block`"#####, r#####"({
  '& *::marker': {
    display: "block",
  },
  '::marker': {
    display: "block",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`selection:block`"#####, r#####"({
  '& *::selection': {
    display: "block",
  },
  '::selection': {
    display: "block",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`file:block`"#####, r#####"({
  '::file-selector-button': {
    display: "block",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`placeholder:block`"#####, r#####"({
  '::placeholder': {
    display: "block",
  },
})
;"##### ; "5")]
#[test_case(r#####"tw`backdrop:block`"#####, r#####"({
  '::backdrop': {
    display: "block",
  },
})
;"##### ; "6")]
#[test_case(r#####"tw`before:block`"#####, r#####"({
  '::before': {
    content: "var(--tw-content)",
    display: "block",
  },
})
;"##### ; "7")]
#[test_case(r#####"tw`after:block`"#####, r#####"({
  '::after': {
    content: "var(--tw-content)",
    display: "block",
  },
})
;"##### ; "8")]
#[test_case(r#####"tw`before:(content block)`"#####, r#####"({
  '::before': {
    content: "var(--tw-content)",
    display: "block",
    '--tw-content': "",
  },
})
;"##### ; "9")]
#[test_case(r#####"tw`after:(content block)`"#####, r#####"({
  '::after': {
    content: "var(--tw-content)",
    display: "block",
    '--tw-content': "",
  },
}) // Positional

;"##### ; "10")]
#[test_case(r#####"tw`first:block`"#####, r#####"({
  ':first-child': {
    display: "block",
  },
})
;"##### ; "11")]
#[test_case(r#####"tw`last:block`"#####, r#####"({
  ':last-child': {
    display: "block",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`only:block`"#####, r#####"({
  ':only-child': {
    display: "block",
  },
})
;"##### ; "13")]
#[test_case(r#####"tw`odd:block`"#####, r#####"({
  ':nth-child(odd)': {
    display: "block",
  },
})
;"##### ; "14")]
#[test_case(r#####"tw`even:block`"#####, r#####"({
  ':nth-child(even)': {
    display: "block",
  },
})
;"##### ; "15")]
#[test_case(r#####"tw`first-of-type:block`"#####, r#####"({
  ':first-of-type': {
    display: "block",
  },
})
;"##### ; "16")]
#[test_case(r#####"tw`last-of-type:block`"#####, r#####"({
  ':last-of-type': {
    display: "block",
  },
})
;"##### ; "17")]
#[test_case(r#####"tw`only-of-type:block`"#####, r#####"({
  ':only-of-type': {
    display: "block",
  },
}) // State

;"##### ; "18")]
#[test_case(r#####"tw`visited:block`"#####, r#####"({
  ':visited': {
    display: "block",
  },
})
;"##### ; "19")]
#[test_case(r#####"tw`target:block`"#####, r#####"({
  ':target': {
    display: "block",
  },
})
;"##### ; "20")]
#[test_case(r#####"tw`open:block`"#####, r#####"({
  '&[open]': {
    display: "block",
  },
}) // Forms

;"##### ; "21")]
#[test_case(r#####"tw`default:block`"#####, r#####"({
  ':default': {
    display: "block",
  },
})
;"##### ; "22")]
#[test_case(r#####"tw`checked:block`"#####, r#####"({
  ':checked': {
    display: "block",
  },
})
;"##### ; "23")]
#[test_case(r#####"tw`indeterminate:block`"#####, r#####"({
  ':indeterminate': {
    display: "block",
  },
})
;"##### ; "24")]
#[test_case(r#####"tw`placeholder-shown:block`"#####, r#####"({
  ':placeholder-shown': {
    display: "block",
  },
})
;"##### ; "25")]
#[test_case(r#####"tw`autofill:block`"#####, r#####"({
  ':autofill': {
    display: "block",
  },
})
;"##### ; "26")]
#[test_case(r#####"tw`optional:block`"#####, r#####"({
  ':optional': {
    display: "block",
  },
})
;"##### ; "27")]
#[test_case(r#####"tw`required:block`"#####, r#####"({
  ':required': {
    display: "block",
  },
})
;"##### ; "28")]
#[test_case(r#####"tw`valid:block`"#####, r#####"({
  ':valid': {
    display: "block",
  },
})
;"##### ; "29")]
#[test_case(r#####"tw`invalid:block`"#####, r#####"({
  ':invalid': {
    display: "block",
  },
})
;"##### ; "30")]
#[test_case(r#####"tw`in-range:block`"#####, r#####"({
  ':in-range': {
    display: "block",
  },
})
;"##### ; "31")]
#[test_case(r#####"tw`out-of-range:block`"#####, r#####"({
  ':out-of-range': {
    display: "block",
  },
})
;"##### ; "32")]
#[test_case(r#####"tw`read-only:block`"#####, r#####"({
  ':read-only': {
    display: "block",
  },
}) // Content

;"##### ; "33")]
#[test_case(r#####"tw`empty:block`"#####, r#####"({
  ':empty': {
    display: "block",
  },
}) // Interactive

;"##### ; "34")]
#[test_case(r#####"tw`focus-within:block`"#####, r#####"({
  ':focus-within': {
    display: "block",
  },
})
;"##### ; "35")]
#[test_case(r#####"tw`hover:block`"#####, r#####"({
  ':hover': {
    display: "block",
  },
})
;"##### ; "36")]
#[test_case(r#####"tw`focus:block`"#####, r#####"({
  ':focus': {
    display: "block",
  },
})
;"##### ; "37")]
#[test_case(r#####"tw`focus-visible:block`"#####, r#####"({
  ':focus-visible': {
    display: "block",
  },
})
;"##### ; "38")]
#[test_case(r#####"tw`active:block`"#####, r#####"({
  ':active': {
    display: "block",
  },
})
;"##### ; "39")]
#[test_case(r#####"tw`enabled:block`"#####, r#####"({
  ':enabled': {
    display: "block",
  },
})
;"##### ; "40")]
#[test_case(r#####"tw`disabled:block`"#####, r#####"({
  ':disabled': {
    display: "block",
  },
}) // Twin custom

;"##### ; "41")]
#[test_case(r#####"tw`all:block`"#####, r#####"({
  '& *': {
    display: "block",
  },
})
;"##### ; "42")]
#[test_case(r#####"tw`all-child:block`"#####, r#####"({
  '> *': {
    display: "block",
  },
})
;"##### ; "43")]
#[test_case(r#####"tw`sibling:block`"#####, r#####"({
  '& ~ *': {
    display: "block",
  },
})
;"##### ; "44")]
#[test_case(r#####"tw`hocus:block`"#####, r#####"({
  ':hover': {
    display: "block",
  },
  ':focus': {
    display: "block",
  },
})
;"##### ; "45")]
#[test_case(r#####"tw`link:block`"#####, r#####"({
  ':link': {
    display: "block",
  },
})
;"##### ; "46")]
#[test_case(r#####"tw`read-write:block`"#####, r#####"({
  ':read-write': {
    display: "block",
  },
})
;"##### ; "47")]
#[test_case(r#####"tw`svg:block`"#####, r#####"({
  '& svg': {
    display: "block",
  },
})
;"##### ; "48")]
#[test_case(r#####"tw`even-of-type:block`"#####, r#####"({
  ':nth-of-type(even)': {
    display: "block",
  },
})
;"##### ; "49")]
#[test_case(r#####"tw`odd-of-type:block`"#####, r#####"({
  ':nth-of-type(odd)': {
    display: "block",
  },
}) // Not versions of the above
// Positional

;"##### ; "50")]
#[test_case(r#####"tw`not-first:block`"#####, r#####"({
  ':not(:first-child)': {
    display: "block",
  },
})
;"##### ; "51")]
#[test_case(r#####"tw`not-last:block`"#####, r#####"({
  ':not(:last-child)': {
    display: "block",
  },
})
;"##### ; "52")]
#[test_case(r#####"tw`not-only:block`"#####, r#####"({
  ':not(:only-child)': {
    display: "block",
  },
})
;"##### ; "53")]
#[test_case(r#####"tw`not-odd:block`"#####, r#####"({
  ':not(:nth-child(odd))': {
    display: "block",
  },
})
;"##### ; "54")]
#[test_case(r#####"tw`not-even:block`"#####, r#####"({
  ':not(:nth-child(even))': {
    display: "block",
  },
})
;"##### ; "55")]
#[test_case(r#####"tw`not-first-of-type:block`"#####, r#####"({
  ':not(:first-of-type)': {
    display: "block",
  },
})
;"##### ; "56")]
#[test_case(r#####"tw`not-last-of-type:block`"#####, r#####"({
  ':not(:last-of-type)': {
    display: "block",
  },
})
;"##### ; "57")]
#[test_case(r#####"tw`not-only-of-type:block`"#####, r#####"({
  ':not(:only-of-type)': {
    display: "block",
  },
}) // State

;"##### ; "58")]
#[test_case(r#####"tw`not-target:block`"#####, r#####"({
  ':not(:target)': {
    display: "block",
  },
})
;"##### ; "59")]
#[test_case(r#####"tw`not-open:block`"#####, r#####"({
  ':not([open])': {
    display: "block",
  },
}) // Forms

;"##### ; "60")]
#[test_case(r#####"tw`not-default:block`"#####, r#####"({
  ':not(:default)': {
    display: "block",
  },
})
;"##### ; "61")]
#[test_case(r#####"tw`not-checked:block`"#####, r#####"({
  ':not(:checked)': {
    display: "block",
  },
})
;"##### ; "62")]
#[test_case(r#####"tw`not-indeterminate:block`"#####, r#####"({
  ':not(:indeterminate)': {
    display: "block",
  },
})
;"##### ; "63")]
#[test_case(r#####"tw`not-placeholder-shown:block`"#####, r#####"({
  ':not(:placeholder-shown)': {
    display: "block",
  },
})
;"##### ; "64")]
#[test_case(r#####"tw`not-autofill:block`"#####, r#####"({
  ':not(:autofill)': {
    display: "block",
  },
})
;"##### ; "65")]
#[test_case(r#####"tw`not-optional:block`"#####, r#####"({
  ':not(:optional)': {
    display: "block",
  },
})
;"##### ; "66")]
#[test_case(r#####"tw`not-required:block`"#####, r#####"({
  ':not(:required)': {
    display: "block",
  },
})
;"##### ; "67")]
#[test_case(r#####"tw`not-valid:block`"#####, r#####"({
  ':not(:valid)': {
    display: "block",
  },
})
;"##### ; "68")]
#[test_case(r#####"tw`not-invalid:block`"#####, r#####"({
  ':not(:invalid)': {
    display: "block",
  },
})
;"##### ; "69")]
#[test_case(r#####"tw`not-in-range:block`"#####, r#####"({
  ':not(:in-range)': {
    display: "block",
  },
})
;"##### ; "70")]
#[test_case(r#####"tw`not-out-of-range:block`"#####, r#####"({
  ':not(:out-of-range)': {
    display: "block",
  },
})
;"##### ; "71")]
#[test_case(r#####"tw`not-read-only:block`"#####, r#####"({
  ':not(:read-only)': {
    display: "block",
  },
}) // Content

;"##### ; "72")]
#[test_case(r#####"tw`not-empty:block`"#####, r#####"({
  ':not(:empty)': {
    display: "block",
  },
}) // Interactive

;"##### ; "73")]
#[test_case(r#####"tw`not-focus-within:block`"#####, r#####"({
  ':not(:focus-within)': {
    display: "block",
  },
})
;"##### ; "74")]
#[test_case(r#####"tw`not-hover:block`"#####, r#####"({
  ':not(:hover)': {
    display: "block",
  },
})
;"##### ; "75")]
#[test_case(r#####"tw`not-focus:block`"#####, r#####"({
  ':not(:focus)': {
    display: "block",
  },
})
;"##### ; "76")]
#[test_case(r#####"tw`not-focus-visible:block`"#####, r#####"({
  ':not(:focus-visible)': {
    display: "block",
  },
})
;"##### ; "77")]
#[test_case(r#####"tw`not-active:block`"#####, r#####"({
  ':not(:active)': {
    display: "block",
  },
})
;"##### ; "78")]
#[test_case(r#####"tw`not-enabled:block`"#####, r#####"({
  ':not(:enabled)': {
    display: "block",
  },
})
;"##### ; "79")]
#[test_case(r#####"tw`not-disabled:block`"#####, r#####"({
  ':not(:disabled)': {
    display: "block",
  },
}) // Twin custom

;"##### ; "80")]
#[test_case(r#####"tw`not-all:block`"#####, r#####"({
  ':not(*)': {
    display: "block",
  },
})
;"##### ; "81")]
#[test_case(r#####"tw`not-all-child:block`"#####, r#####"({
  ':not(> *)': {
    display: "block",
  },
})
;"##### ; "82")]
#[test_case(r#####"tw`not-sibling:block`"#####, r#####"({
  ':not(~ *)': {
    display: "block",
  },
})
;"##### ; "83")]
#[test_case(r#####"tw`not-hocus:block`"#####, r#####"({
  ':not(:hover)': {
    display: "block",
  },
  ':not(:focus)': {
    display: "block",
  },
})
;"##### ; "84")]
#[test_case(r#####"tw`not-link:block`"#####, r#####"({
  ':not(:link)': {
    display: "block",
  },
})
;"##### ; "85")]
#[test_case(r#####"tw`not-read-write:block`"#####, r#####"({
  ':not(:read-write)': {
    display: "block",
  },
})
;"##### ; "86")]
#[test_case(r#####"tw`not-svg:block`"#####, r#####"({
  ':not(svg)': {
    display: "block",
  },
})
;"##### ; "87")]
#[test_case(r#####"tw`not-even-of-type:block`"#####, r#####"({
  ':not(:nth-of-type(even))': {
    display: "block",
  },
})
;"##### ; "88")]
#[test_case(r#####"tw`not-odd-of-type:block`"#####, r#####"({
  ':not(:nth-of-type(odd))': {
    display: "block",
  },
}) //

;"##### ; "89")]
#[test_case(r#####"tw`ltr:block`"#####, r#####"({
  ':is([dir="ltr"] &)': {
    display: "block",
  },
})
;"##### ; "90")]
#[test_case(r#####"tw`rtl:block`"#####, r#####"({
  ':is([dir="rtl"] &)': {
    display: "block",
  },
})
;"##### ; "91")]
#[test_case(r#####"tw`motion-safe:block`"#####, r#####"({
  '@media (prefers-reduced-motion: no-preference)': {
    display: "block",
  },
})
;"##### ; "92")]
#[test_case(r#####"tw`motion-reduce:block`"#####, r#####"({
  '@media (prefers-reduced-motion: reduce)': {
    display: "block",
  },
})
;"##### ; "93")]
#[test_case(r#####"tw`dark:block`"#####, r#####"({
  '@media (prefers-color-scheme: dark)': {
    display: "block",
  },
})
;"##### ; "94")]
#[test_case(r#####"tw`light:block`"#####, r#####"({
  '@media (prefers-color-scheme: light)': {
    display: "block",
  },
})
;"##### ; "95")]
#[test_case(r#####"tw`dark:sm:block`"#####, r#####"({
  '@media (min-width: 640px)': {
    '@media (prefers-color-scheme: dark)': {
      display: "block",
    },
  },
})
;"##### ; "96")]
#[test_case(r#####"tw`light:sm:block`"#####, r#####"({
  '@media (min-width: 640px)': {
    '@media (prefers-color-scheme: light)': {
      display: "block",
    },
  },
})
;"##### ; "97")]
#[test_case(r#####"tw`dark:group-hover:sm:block`"#####, r#####"({
  '@media (min-width: 640px)': {
    '@media (prefers-color-scheme: dark)': {
      '.group:hover &': {
        display: "block",
      },
    },
  },
})
;"##### ; "98")]
#[test_case(r#####"tw`light:group-hocus:sm:block`"#####, r#####"({
  '@media (min-width: 640px)': {
    '@media (prefers-color-scheme: light)': {
      '.group:hover &': {
        display: "block",
      },
      '.group:focus &': {
        display: "block",
      },
    },
  },
})
;"##### ; "99")]
#[test_case(r#####"tw`print:block`"#####, r#####"({
  '@media print': {
    display: "block",
  },
})
;"##### ; "100")]
#[test_case(r#####"tw`screen:block`"#####, r#####"({
  '@media screen': {
    display: "block",
  },
})
;"##### ; "101")]
#[test_case(r#####"tw`portrait:block`"#####, r#####"({
  '@media (orientation: portrait)': {
    display: "block",
  },
})
;"##### ; "102")]
#[test_case(r#####"tw`landscape:block`"#####, r#####"({
  '@media (orientation: landscape)': {
    display: "block",
  },
})
;"##### ; "103")]
#[test_case(r#####"tw`contrast-more:block`"#####, r#####"({
  '@media (prefers-contrast: more)': {
    display: "block",
  },
})
;"##### ; "104")]
#[test_case(r#####"tw`contrast-less:block`"#####, r#####"({
  '@media (prefers-contrast: less)': {
    display: "block",
  },
})
;"##### ; "105")]
#[test_case(r#####"tw`any-pointer-none:block`"#####, r#####"({
  '@media (any-pointer: none)': {
    display: "block",
  },
})
;"##### ; "106")]
#[test_case(r#####"tw`any-pointer-fine:block`"#####, r#####"({
  '@media (any-pointer: fine)': {
    display: "block",
  },
})
;"##### ; "107")]
#[test_case(r#####"tw`any-pointer-coarse:block`"#####, r#####"({
  '@media (any-pointer: coarse)': {
    display: "block",
  },
})
;"##### ; "108")]
#[test_case(r#####"tw`pointer-none:block`"#####, r#####"({
  '@media (pointer: none)': {
    display: "block",
  },
})
;"##### ; "109")]
#[test_case(r#####"tw`pointer-fine:block`"#####, r#####"({
  '@media (pointer: fine)': {
    display: "block",
  },
})
;"##### ; "110")]
#[test_case(r#####"tw`pointer-coarse:block`"#####, r#####"({
  '@media (pointer: coarse)': {
    display: "block",
  },
})
;"##### ; "111")]
#[test_case(r#####"tw`any-hover-none:block`"#####, r#####"({
  '@media (any-hover: none)': {
    display: "block",
  },
})
;"##### ; "112")]
#[test_case(r#####"tw`any-hover:block`"#####, r#####"({
  '@media (any-hover: hover)': {
    display: "block",
  },
})
;"##### ; "113")]
#[test_case(r#####"tw`can-hover:block`"#####, r#####"({
  '@media (hover: hover)': {
    display: "block",
  },
})
;"##### ; "114")]
#[test_case(r#####"tw`cant-hover:block`"#####, r#####"({
  '@media (hover: none)': {
    display: "block",
  },
}) // Arbitrary values

;"##### ; "115")]
#[test_case(r#####"tw`first:inset-[50px]`"#####, r#####"({
  ':first-child': {
    inset: "50px",
  },
})
;"##### ; "116")]
#[test_case(r#####"tw`md:text-[red]`"#####, r#####"({
  '@media (min-width: 768px)': {
    '--tw-text-opacity': "1",
    color: "rgb(255 0 0 / var(--tw-text-opacity))",
  },
}) // Random

;"##### ; "117")]
#[test_case(r#####"tw`xl:placeholder-red-500! first:md:block sm:disabled:flex`"#####, r#####"({
  '@media (min-width: 640px)': {
    ':disabled': {
      display: "flex",
    },
  },
  '@media (min-width: 768px)': {
    ':first-child': {
      display: "block",
    },
  },
  '@media (min-width: 1280px)': {
    '::placeholder': {
      '--tw-placeholder-opacity': "1 !important",
      color: "rgb(239 68 68 / var(--tw-placeholder-opacity)) !important",
    },
  },
})"##### ; "118")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
