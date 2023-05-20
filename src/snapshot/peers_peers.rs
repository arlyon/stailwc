use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"tw`peer-first:block`"#####, r#####"({
  '.peer:first-child ~ &': {
    display: "block",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`peer-last:block`"#####, r#####"({
  '.peer:last-child ~ &': {
    display: "block",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`peer-only:block`"#####, r#####"({
  '.peer:only-child ~ &': {
    display: "block",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`peer-odd:block`"#####, r#####"({
  '.peer:nth-child(odd) ~ &': {
    display: "block",
  },
})
;"##### ; "4")]
#[test_case(r#####"tw`peer-even:block`"#####, r#####"({
  '.peer:nth-child(even) ~ &': {
    display: "block",
  },
})
;"##### ; "5")]
#[test_case(r#####"tw`peer-first-of-type:block`"#####, r#####"({
  '.peer:first-of-type ~ &': {
    display: "block",
  },
})
;"##### ; "6")]
#[test_case(r#####"tw`peer-last-of-type:block`"#####, r#####"({
  '.peer:last-of-type ~ &': {
    display: "block",
  },
})
;"##### ; "7")]
#[test_case(r#####"tw`peer-only-of-type:block`"#####, r#####"({
  '.peer:only-of-type ~ &': {
    display: "block",
  },
}) // State

;"##### ; "8")]
#[test_case(r#####"tw`peer-visited:block`"#####, r#####"({
  '.peer:visited ~ &': {
    display: "block",
  },
})
;"##### ; "9")]
#[test_case(r#####"tw`peer-target:block`"#####, r#####"({
  '.peer:target ~ &': {
    display: "block",
  },
})
;"##### ; "10")]
#[test_case(r#####"tw`peer-open:block`"#####, r#####"({
  '.peer[open] ~ &': {
    display: "block",
  },
}) // Forms

;"##### ; "11")]
#[test_case(r#####"tw`peer-default:block`"#####, r#####"({
  '.peer:default ~ &': {
    display: "block",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`peer-checked:block`"#####, r#####"({
  '.peer:checked ~ &': {
    display: "block",
  },
})
;"##### ; "13")]
#[test_case(r#####"tw`peer-indeterminate:block`"#####, r#####"({
  '.peer:indeterminate ~ &': {
    display: "block",
  },
})
;"##### ; "14")]
#[test_case(r#####"tw`peer-placeholder-shown:block`"#####, r#####"({
  '.peer:placeholder-shown ~ &': {
    display: "block",
  },
})
;"##### ; "15")]
#[test_case(r#####"tw`peer-autofill:block`"#####, r#####"({
  '.peer:autofill ~ &': {
    display: "block",
  },
})
;"##### ; "16")]
#[test_case(r#####"tw`peer-optional:block`"#####, r#####"({
  '.peer:optional ~ &': {
    display: "block",
  },
})
;"##### ; "17")]
#[test_case(r#####"tw`peer-required:block`"#####, r#####"({
  '.peer:required ~ &': {
    display: "block",
  },
})
;"##### ; "18")]
#[test_case(r#####"tw`peer-valid:block`"#####, r#####"({
  '.peer:valid ~ &': {
    display: "block",
  },
})
;"##### ; "19")]
#[test_case(r#####"tw`peer-invalid:block`"#####, r#####"({
  '.peer:invalid ~ &': {
    display: "block",
  },
})
;"##### ; "20")]
#[test_case(r#####"tw`peer-in-range:block`"#####, r#####"({
  '.peer:in-range ~ &': {
    display: "block",
  },
})
;"##### ; "21")]
#[test_case(r#####"tw`peer-out-of-range:block`"#####, r#####"({
  '.peer:out-of-range ~ &': {
    display: "block",
  },
})
;"##### ; "22")]
#[test_case(r#####"tw`peer-read-only:block`"#####, r#####"({
  '.peer:read-only ~ &': {
    display: "block",
  },
}) // Content

;"##### ; "23")]
#[test_case(r#####"tw`peer-empty:block`"#####, r#####"({
  '.peer:empty ~ &': {
    display: "block",
  },
}) // Interactive

;"##### ; "24")]
#[test_case(r#####"tw`peer-focus-within:block`"#####, r#####"({
  '.peer:focus-within ~ &': {
    display: "block",
  },
})
;"##### ; "25")]
#[test_case(r#####"tw`peer-hover:block`"#####, r#####"({
  '.peer:hover ~ &': {
    display: "block",
  },
})
;"##### ; "26")]
#[test_case(r#####"tw`peer-focus:block`"#####, r#####"({
  '.peer:focus ~ &': {
    display: "block",
  },
})
;"##### ; "27")]
#[test_case(r#####"tw`peer-focus-visible:block`"#####, r#####"({
  '.peer:focus-visible ~ &': {
    display: "block",
  },
})
;"##### ; "28")]
#[test_case(r#####"tw`peer-active:block`"#####, r#####"({
  '.peer:active ~ &': {
    display: "block",
  },
})
;"##### ; "29")]
#[test_case(r#####"tw`peer-enabled:block`"#####, r#####"({
  '.peer:enabled ~ &': {
    display: "block",
  },
})
;"##### ; "30")]
#[test_case(r#####"tw`peer-disabled:block`"#####, r#####"({
  '.peer:disabled ~ &': {
    display: "block",
  },
}) // Twin custom

;"##### ; "31")]
#[test_case(r#####"tw`peer-all:block`"#####, r#####"({
  '.peer * ~ &': {
    display: "block",
  },
})
;"##### ; "32")]
#[test_case(r#####"tw`peer-all-child:block`"#####, r#####"({
  '.peer > * ~ &': {
    display: "block",
  },
})
;"##### ; "33")]
#[test_case(r#####"tw`peer-sibling:block`"#####, r#####"({
  '.peer ~ * ~ &': {
    display: "block",
  },
})
;"##### ; "34")]
#[test_case(r#####"tw`peer-hocus:block`"#####, r#####"({
  '.peer:hover ~ &': {
    display: "block",
  },
  '.peer:focus ~ &': {
    display: "block",
  },
})
;"##### ; "35")]
#[test_case(r#####"tw`peer-link:block`"#####, r#####"({
  '.peer:link ~ &': {
    display: "block",
  },
})
;"##### ; "36")]
#[test_case(r#####"tw`peer-read-write:block`"#####, r#####"({
  '.peer:read-write ~ &': {
    display: "block",
  },
})
;"##### ; "37")]
#[test_case(r#####"tw`peer-svg:block`"#####, r#####"({
  '.peer svg ~ &': {
    display: "block",
  },
})
;"##### ; "38")]
#[test_case(r#####"tw`peer-even-of-type:block`"#####, r#####"({
  '.peer:nth-of-type(even) ~ &': {
    display: "block",
  },
})
;"##### ; "39")]
#[test_case(r#####"tw`peer-odd-of-type:block`"#####, r#####"({
  '.peer:nth-of-type(odd) ~ &': {
    display: "block",
  },
}) // Not versions of the above
// Positional

;"##### ; "40")]
#[test_case(r#####"tw`peer-not-first:block`"#####, r#####"({
  '.peer:not(:first-child) ~ &': {
    display: "block",
  },
})
;"##### ; "41")]
#[test_case(r#####"tw`peer-not-last:block`"#####, r#####"({
  '.peer:not(:last-child) ~ &': {
    display: "block",
  },
})
;"##### ; "42")]
#[test_case(r#####"tw`peer-not-only:block`"#####, r#####"({
  '.peer:not(:only-child) ~ &': {
    display: "block",
  },
})
;"##### ; "43")]
#[test_case(r#####"tw`peer-not-odd:block`"#####, r#####"({
  '.peer:not(:nth-child(odd)) ~ &': {
    display: "block",
  },
})
;"##### ; "44")]
#[test_case(r#####"tw`peer-not-even:block`"#####, r#####"({
  '.peer:not(:nth-child(even)) ~ &': {
    display: "block",
  },
})
;"##### ; "45")]
#[test_case(r#####"tw`peer-not-first-of-type:block`"#####, r#####"({
  '.peer:not(:first-of-type) ~ &': {
    display: "block",
  },
})
;"##### ; "46")]
#[test_case(r#####"tw`peer-not-last-of-type:block`"#####, r#####"({
  '.peer:not(:last-of-type) ~ &': {
    display: "block",
  },
})
;"##### ; "47")]
#[test_case(r#####"tw`peer-not-only-of-type:block`"#####, r#####"({
  '.peer:not(:only-of-type) ~ &': {
    display: "block",
  },
}) // State

;"##### ; "48")]
#[test_case(r#####"tw`peer-not-target:block`"#####, r#####"({
  '.peer:not(:target) ~ &': {
    display: "block",
  },
})
;"##### ; "49")]
#[test_case(r#####"tw`peer-not-open:block`"#####, r#####"({
  '.peer:not([open]) ~ &': {
    display: "block",
  },
}) // Forms

;"##### ; "50")]
#[test_case(r#####"tw`peer-not-default:block`"#####, r#####"({
  '.peer:not(:default) ~ &': {
    display: "block",
  },
})
;"##### ; "51")]
#[test_case(r#####"tw`peer-not-checked:block`"#####, r#####"({
  '.peer:not(:checked) ~ &': {
    display: "block",
  },
})
;"##### ; "52")]
#[test_case(r#####"tw`peer-not-indeterminate:block`"#####, r#####"({
  '.peer:not(:indeterminate) ~ &': {
    display: "block",
  },
})
;"##### ; "53")]
#[test_case(r#####"tw`peer-not-placeholder-shown:block`"#####, r#####"({
  '.peer:not(:placeholder-shown) ~ &': {
    display: "block",
  },
})
;"##### ; "54")]
#[test_case(r#####"tw`peer-not-autofill:block`"#####, r#####"({
  '.peer:not(:autofill) ~ &': {
    display: "block",
  },
})
;"##### ; "55")]
#[test_case(r#####"tw`peer-not-optional:block`"#####, r#####"({
  '.peer:not(:optional) ~ &': {
    display: "block",
  },
})
;"##### ; "56")]
#[test_case(r#####"tw`peer-not-required:block`"#####, r#####"({
  '.peer:not(:required) ~ &': {
    display: "block",
  },
})
;"##### ; "57")]
#[test_case(r#####"tw`peer-not-valid:block`"#####, r#####"({
  '.peer:not(:valid) ~ &': {
    display: "block",
  },
})
;"##### ; "58")]
#[test_case(r#####"tw`peer-not-invalid:block`"#####, r#####"({
  '.peer:not(:invalid) ~ &': {
    display: "block",
  },
})
;"##### ; "59")]
#[test_case(r#####"tw`peer-not-in-range:block`"#####, r#####"({
  '.peer:not(:in-range) ~ &': {
    display: "block",
  },
})
;"##### ; "60")]
#[test_case(r#####"tw`peer-not-out-of-range:block`"#####, r#####"({
  '.peer:not(:out-of-range) ~ &': {
    display: "block",
  },
})
;"##### ; "61")]
#[test_case(r#####"tw`peer-not-read-only:block`"#####, r#####"({
  '.peer:not(:read-only) ~ &': {
    display: "block",
  },
}) // Content

;"##### ; "62")]
#[test_case(r#####"tw`peer-not-empty:block`"#####, r#####"({
  '.peer:not(:empty) ~ &': {
    display: "block",
  },
}) // Interactive

;"##### ; "63")]
#[test_case(r#####"tw`peer-not-focus-within:block`"#####, r#####"({
  '.peer:not(:focus-within) ~ &': {
    display: "block",
  },
})
;"##### ; "64")]
#[test_case(r#####"tw`peer-not-hover:block`"#####, r#####"({
  '.peer:not(:hover) ~ &': {
    display: "block",
  },
})
;"##### ; "65")]
#[test_case(r#####"tw`peer-not-focus:block`"#####, r#####"({
  '.peer:not(:focus) ~ &': {
    display: "block",
  },
})
;"##### ; "66")]
#[test_case(r#####"tw`peer-not-focus-visible:block`"#####, r#####"({
  '.peer:not(:focus-visible) ~ &': {
    display: "block",
  },
})
;"##### ; "67")]
#[test_case(r#####"tw`peer-not-active:block`"#####, r#####"({
  '.peer:not(:active) ~ &': {
    display: "block",
  },
})
;"##### ; "68")]
#[test_case(r#####"tw`peer-not-enabled:block`"#####, r#####"({
  '.peer:not(:enabled) ~ &': {
    display: "block",
  },
})
;"##### ; "69")]
#[test_case(r#####"tw`peer-not-disabled:block`"#####, r#####"({
  '.peer:not(:disabled) ~ &': {
    display: "block",
  },
}) // Twin custom

;"##### ; "70")]
#[test_case(r#####"tw`peer-not-all:block`"#####, r#####"({
  '.peer:not(*) ~ &': {
    display: "block",
  },
})
;"##### ; "71")]
#[test_case(r#####"tw`peer-not-all-child:block`"#####, r#####"({
  '.peer:not(> *) ~ &': {
    display: "block",
  },
})
;"##### ; "72")]
#[test_case(r#####"tw`peer-not-sibling:block`"#####, r#####"({
  '.peer:not(~ *) ~ &': {
    display: "block",
  },
})
;"##### ; "73")]
#[test_case(r#####"tw`peer-not-hocus:block`"#####, r#####"({
  '.peer:not(:hover) ~ &': {
    display: "block",
  },
  '.peer:not(:focus) ~ &': {
    display: "block",
  },
})
;"##### ; "74")]
#[test_case(r#####"tw`peer-not-link:block`"#####, r#####"({
  '.peer:not(:link) ~ &': {
    display: "block",
  },
})
;"##### ; "75")]
#[test_case(r#####"tw`peer-not-read-write:block`"#####, r#####"({
  '.peer:not(:read-write) ~ &': {
    display: "block",
  },
})
;"##### ; "76")]
#[test_case(r#####"tw`peer-not-svg:block`"#####, r#####"({
  '.peer:not(svg) ~ &': {
    display: "block",
  },
})
;"##### ; "77")]
#[test_case(r#####"tw`peer-not-even-of-type:block`"#####, r#####"({
  '.peer:not(:nth-of-type(even)) ~ &': {
    display: "block",
  },
})
;"##### ; "78")]
#[test_case(r#####"tw`peer-not-odd-of-type:block`"#####, r#####"({
  '.peer:not(:nth-of-type(odd)) ~ &': {
    display: "block",
  },
})"##### ; "79")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
