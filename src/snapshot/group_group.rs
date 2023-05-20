use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`group-first:block`"#####, r#####"({
  '.group:first-child &': {
    display: "block",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`group-last:block`"#####, r#####"({
  '.group:last-child &': {
    display: "block",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`group-only:block`"#####, r#####"({
  '.group:only-child &': {
    display: "block",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`group-odd:block`"#####, r#####"({
  '.group:nth-child(odd) &': {
    display: "block",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`group-even:block`"#####, r#####"({
  '.group:nth-child(even) &': {
    display: "block",
  },
})
;"##### ; "5")]
#[test_case(r#####"tw`group-first-of-type:block`"#####, r#####"({
  '.group:first-of-type &': {
    display: "block",
  },
})
;"##### ; "6")]
#[test_case(r#####"tw`group-last-of-type:block`"#####, r#####"({
  '.group:last-of-type &': {
    display: "block",
  },
})
;"##### ; "7")]
#[test_case(r#####"tw`group-only-of-type:block`"#####, r#####"({
  '.group:only-of-type &': {
    display: "block",
  },
}) // State

;"##### ; "8")]
#[test_case(r#####"tw`group-visited:block`"#####, r#####"({
  '.group:visited &': {
    display: "block",
  },
})
;"##### ; "9")]
#[test_case(r#####"tw`group-target:block`"#####, r#####"({
  '.group:target &': {
    display: "block",
  },
})
;"##### ; "10")]
#[test_case(r#####"tw`group-open:block`"#####, r#####"({
  '.group[open] &': {
    display: "block",
  },
}) // Forms

;"##### ; "11")]
#[test_case(r#####"tw`group-default:block`"#####, r#####"({
  '.group:default &': {
    display: "block",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`group-checked:block`"#####, r#####"({
  '.group:checked &': {
    display: "block",
  },
})
;"##### ; "13")]
#[test_case(r#####"tw`group-indeterminate:block`"#####, r#####"({
  '.group:indeterminate &': {
    display: "block",
  },
})
;"##### ; "14")]
#[test_case(r#####"tw`group-placeholder-shown:block`"#####, r#####"({
  '.group:placeholder-shown &': {
    display: "block",
  },
})
;"##### ; "15")]
#[test_case(r#####"tw`group-autofill:block`"#####, r#####"({
  '.group:autofill &': {
    display: "block",
  },
})
;"##### ; "16")]
#[test_case(r#####"tw`group-optional:block`"#####, r#####"({
  '.group:optional &': {
    display: "block",
  },
})
;"##### ; "17")]
#[test_case(r#####"tw`group-required:block`"#####, r#####"({
  '.group:required &': {
    display: "block",
  },
})
;"##### ; "18")]
#[test_case(r#####"tw`group-valid:block`"#####, r#####"({
  '.group:valid &': {
    display: "block",
  },
})
;"##### ; "19")]
#[test_case(r#####"tw`group-invalid:block`"#####, r#####"({
  '.group:invalid &': {
    display: "block",
  },
})
;"##### ; "20")]
#[test_case(r#####"tw`group-in-range:block`"#####, r#####"({
  '.group:in-range &': {
    display: "block",
  },
})
;"##### ; "21")]
#[test_case(r#####"tw`group-out-of-range:block`"#####, r#####"({
  '.group:out-of-range &': {
    display: "block",
  },
})
;"##### ; "22")]
#[test_case(r#####"tw`group-read-only:block`"#####, r#####"({
  '.group:read-only &': {
    display: "block",
  },
}) // Content

;"##### ; "23")]
#[test_case(r#####"tw`group-empty:block`"#####, r#####"({
  '.group:empty &': {
    display: "block",
  },
}) // Interactive

;"##### ; "24")]
#[test_case(r#####"tw`group-focus-within:block`"#####, r#####"({
  '.group:focus-within &': {
    display: "block",
  },
})
;"##### ; "25")]
#[test_case(r#####"tw`group-hover:block`"#####, r#####"({
  '.group:hover &': {
    display: "block",
  },
})
;"##### ; "26")]
#[test_case(r#####"tw`group-focus:block`"#####, r#####"({
  '.group:focus &': {
    display: "block",
  },
})
;"##### ; "27")]
#[test_case(r#####"tw`group-focus-visible:block`"#####, r#####"({
  '.group:focus-visible &': {
    display: "block",
  },
})
;"##### ; "28")]
#[test_case(r#####"tw`group-active:block`"#####, r#####"({
  '.group:active &': {
    display: "block",
  },
})
;"##### ; "29")]
#[test_case(r#####"tw`group-enabled:block`"#####, r#####"({
  '.group:enabled &': {
    display: "block",
  },
})
;"##### ; "30")]
#[test_case(r#####"tw`group-disabled:block`"#####, r#####"({
  '.group:disabled &': {
    display: "block",
  },
}) // Twin custom

;"##### ; "31")]
#[test_case(r#####"tw`group-all:block`"#####, r#####"({
  '.group * &': {
    display: "block",
  },
})
;"##### ; "32")]
#[test_case(r#####"tw`group-all-child:block`"#####, r#####"({
  '.group > * &': {
    display: "block",
  },
})
;"##### ; "33")]
#[test_case(r#####"tw`group-sibling:block`"#####, r#####"({
  '.group ~ * &': {
    display: "block",
  },
})
;"##### ; "34")]
#[test_case(r#####"tw`group-hocus:block`"#####, r#####"({
  '.group:hover &': {
    display: "block",
  },
  '.group:focus &': {
    display: "block",
  },
})
;"##### ; "35")]
#[test_case(r#####"tw`group-link:block`"#####, r#####"({
  '.group:link &': {
    display: "block",
  },
})
;"##### ; "36")]
#[test_case(r#####"tw`group-read-write:block`"#####, r#####"({
  '.group:read-write &': {
    display: "block",
  },
})
;"##### ; "37")]
#[test_case(r#####"tw`group-svg:block`"#####, r#####"({
  '.group svg &': {
    display: "block",
  },
})
;"##### ; "38")]
#[test_case(r#####"tw`group-even-of-type:block`"#####, r#####"({
  '.group:nth-of-type(even) &': {
    display: "block",
  },
})
;"##### ; "39")]
#[test_case(r#####"tw`group-odd-of-type:block`"#####, r#####"({
  '.group:nth-of-type(odd) &': {
    display: "block",
  },
}) // Not versions of the above
// Positional

;"##### ; "40")]
#[test_case(r#####"tw`group-not-first:block`"#####, r#####"({
  '.group:not(:first-child) &': {
    display: "block",
  },
})
;"##### ; "41")]
#[test_case(r#####"tw`group-not-last:block`"#####, r#####"({
  '.group:not(:last-child) &': {
    display: "block",
  },
})
;"##### ; "42")]
#[test_case(r#####"tw`group-not-only:block`"#####, r#####"({
  '.group:not(:only-child) &': {
    display: "block",
  },
})
;"##### ; "43")]
#[test_case(r#####"tw`group-not-odd:block`"#####, r#####"({
  '.group:not(:nth-child(odd)) &': {
    display: "block",
  },
})
;"##### ; "44")]
#[test_case(r#####"tw`group-not-even:block`"#####, r#####"({
  '.group:not(:nth-child(even)) &': {
    display: "block",
  },
})
;"##### ; "45")]
#[test_case(r#####"tw`group-not-first-of-type:block`"#####, r#####"({
  '.group:not(:first-of-type) &': {
    display: "block",
  },
})
;"##### ; "46")]
#[test_case(r#####"tw`group-not-last-of-type:block`"#####, r#####"({
  '.group:not(:last-of-type) &': {
    display: "block",
  },
})
;"##### ; "47")]
#[test_case(r#####"tw`group-not-only-of-type:block`"#####, r#####"({
  '.group:not(:only-of-type) &': {
    display: "block",
  },
}) // State

;"##### ; "48")]
#[test_case(r#####"tw`group-not-target:block`"#####, r#####"({
  '.group:not(:target) &': {
    display: "block",
  },
})
;"##### ; "49")]
#[test_case(r#####"tw`group-not-open:block`"#####, r#####"({
  '.group:not([open]) &': {
    display: "block",
  },
}) // Forms

;"##### ; "50")]
#[test_case(r#####"tw`group-not-default:block`"#####, r#####"({
  '.group:not(:default) &': {
    display: "block",
  },
})
;"##### ; "51")]
#[test_case(r#####"tw`group-not-checked:block`"#####, r#####"({
  '.group:not(:checked) &': {
    display: "block",
  },
})
;"##### ; "52")]
#[test_case(r#####"tw`group-not-indeterminate:block`"#####, r#####"({
  '.group:not(:indeterminate) &': {
    display: "block",
  },
})
;"##### ; "53")]
#[test_case(r#####"tw`group-not-placeholder-shown:block`"#####, r#####"({
  '.group:not(:placeholder-shown) &': {
    display: "block",
  },
})
;"##### ; "54")]
#[test_case(r#####"tw`group-not-autofill:block`"#####, r#####"({
  '.group:not(:autofill) &': {
    display: "block",
  },
})
;"##### ; "55")]
#[test_case(r#####"tw`group-not-optional:block`"#####, r#####"({
  '.group:not(:optional) &': {
    display: "block",
  },
})
;"##### ; "56")]
#[test_case(r#####"tw`group-not-required:block`"#####, r#####"({
  '.group:not(:required) &': {
    display: "block",
  },
})
;"##### ; "57")]
#[test_case(r#####"tw`group-not-valid:block`"#####, r#####"({
  '.group:not(:valid) &': {
    display: "block",
  },
})
;"##### ; "58")]
#[test_case(r#####"tw`group-not-invalid:block`"#####, r#####"({
  '.group:not(:invalid) &': {
    display: "block",
  },
})
;"##### ; "59")]
#[test_case(r#####"tw`group-not-in-range:block`"#####, r#####"({
  '.group:not(:in-range) &': {
    display: "block",
  },
})
;"##### ; "60")]
#[test_case(r#####"tw`group-not-out-of-range:block`"#####, r#####"({
  '.group:not(:out-of-range) &': {
    display: "block",
  },
})
;"##### ; "61")]
#[test_case(r#####"tw`group-not-read-only:block`"#####, r#####"({
  '.group:not(:read-only) &': {
    display: "block",
  },
}) // Content

;"##### ; "62")]
#[test_case(r#####"tw`group-not-empty:block`"#####, r#####"({
  '.group:not(:empty) &': {
    display: "block",
  },
}) // Interactive

;"##### ; "63")]
#[test_case(r#####"tw`group-not-focus-within:block`"#####, r#####"({
  '.group:not(:focus-within) &': {
    display: "block",
  },
})
;"##### ; "64")]
#[test_case(r#####"tw`group-not-hover:block`"#####, r#####"({
  '.group:not(:hover) &': {
    display: "block",
  },
})
;"##### ; "65")]
#[test_case(r#####"tw`group-not-focus:block`"#####, r#####"({
  '.group:not(:focus) &': {
    display: "block",
  },
})
;"##### ; "66")]
#[test_case(r#####"tw`group-not-focus-visible:block`"#####, r#####"({
  '.group:not(:focus-visible) &': {
    display: "block",
  },
})
;"##### ; "67")]
#[test_case(r#####"tw`group-not-active:block`"#####, r#####"({
  '.group:not(:active) &': {
    display: "block",
  },
})
;"##### ; "68")]
#[test_case(r#####"tw`group-not-enabled:block`"#####, r#####"({
  '.group:not(:enabled) &': {
    display: "block",
  },
})
;"##### ; "69")]
#[test_case(r#####"tw`group-not-disabled:block`"#####, r#####"({
  '.group:not(:disabled) &': {
    display: "block",
  },
}) // Twin custom

;"##### ; "70")]
#[test_case(r#####"tw`group-not-all:block`"#####, r#####"({
  '.group:not(*) &': {
    display: "block",
  },
})
;"##### ; "71")]
#[test_case(r#####"tw`group-not-all-child:block`"#####, r#####"({
  '.group:not(> *) &': {
    display: "block",
  },
})
;"##### ; "72")]
#[test_case(r#####"tw`group-not-sibling:block`"#####, r#####"({
  '.group:not(~ *) &': {
    display: "block",
  },
})
;"##### ; "73")]
#[test_case(r#####"tw`group-not-hocus:block`"#####, r#####"({
  '.group:not(:hover) &': {
    display: "block",
  },
  '.group:not(:focus) &': {
    display: "block",
  },
})
;"##### ; "74")]
#[test_case(r#####"tw`group-not-link:block`"#####, r#####"({
  '.group:not(:link) &': {
    display: "block",
  },
})
;"##### ; "75")]
#[test_case(r#####"tw`group-not-read-write:block`"#####, r#####"({
  '.group:not(:read-write) &': {
    display: "block",
  },
})
;"##### ; "76")]
#[test_case(r#####"tw`group-not-svg:block`"#####, r#####"({
  '.group:not(svg) &': {
    display: "block",
  },
})
;"##### ; "77")]
#[test_case(r#####"tw`group-not-even-of-type:block`"#####, r#####"({
  '.group:not(:nth-of-type(even)) &': {
    display: "block",
  },
})
;"##### ; "78")]
#[test_case(r#####"tw`group-not-odd-of-type:block`"#####, r#####"({
  '.group:not(:nth-of-type(odd)) &': {
    display: "block",
  },
})"##### ; "79")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
