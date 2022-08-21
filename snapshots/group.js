
// Positional
tw`group-first:block`
tw`group-last:block`
tw`group-only:block`
tw`group-odd:block`
tw`group-even:block`
tw`group-first-of-type:block`
tw`group-last-of-type:block`
tw`group-only-of-type:block`

// State
tw`group-visited:block`
tw`group-target:block`
tw`group-open:block`

// Forms
tw`group-default:block`
tw`group-checked:block`
tw`group-indeterminate:block`
tw`group-placeholder-shown:block`
tw`group-autofill:block`
tw`group-optional:block`
tw`group-required:block`
tw`group-valid:block`
tw`group-invalid:block`
tw`group-in-range:block`
tw`group-out-of-range:block`
tw`group-read-only:block`

// Content
tw`group-empty:block`

// Interactive
tw`group-focus-within:block`
tw`group-hover:block`
tw`group-focus:block`
tw`group-focus-visible:block`
tw`group-active:block`
tw`group-enabled:block`
tw`group-disabled:block`

// Twin custom
tw`group-all:block`
tw`group-all-child:block`
tw`group-sibling:block`
tw`group-hocus:block`
tw`group-link:block`
tw`group-read-write:block`
tw`group-svg:block`
tw`group-even-of-type:block`
tw`group-odd-of-type:block`

// Not versions of the above

// Positional
tw`group-not-first:block`
tw`group-not-last:block`
tw`group-not-only:block`
tw`group-not-odd:block`
tw`group-not-even:block`
tw`group-not-first-of-type:block`
tw`group-not-last-of-type:block`
tw`group-not-only-of-type:block`

// State
tw`group-not-visited:block`
tw`group-not-target:block`
tw`group-not-open:block`

// Forms
tw`group-not-default:block`
tw`group-not-checked:block`
tw`group-not-indeterminate:block`
tw`group-not-placeholder-shown:block`
tw`group-not-autofill:block`
tw`group-not-optional:block`
tw`group-not-required:block`
tw`group-not-valid:block`
tw`group-not-invalid:block`
tw`group-not-in-range:block`
tw`group-not-out-of-range:block`
tw`group-not-read-only:block`

// Content
tw`group-not-empty:block`

// Interactive
tw`group-not-focus-within:block`
tw`group-not-hover:block`
tw`group-not-focus:block`
tw`group-not-focus-visible:block`
tw`group-not-active:block`
tw`group-not-enabled:block`
tw`group-not-disabled:block`

// Twin custom
tw`group-not-all:block`
tw`group-not-all-child:block`
tw`group-not-sibling:block`
tw`group-not-hocus:block`
tw`group-not-link:block`
tw`group-not-read-write:block`
tw`group-not-svg:block`
tw`group-not-even-of-type:block`
tw`group-not-odd-of-type:block`

      ↓ ↓ ↓ ↓ ↓ ↓

// Positional
({
  '.group:first-child &': {
    display: 'block',
  },
});
({
  '.group:last-child &': {
    display: 'block',
  },
});
({
  '.group:only-child &': {
    display: 'block',
  },
});
({
  '.group:nth-child(odd) &': {
    display: 'block',
  },
});
({
  '.group:nth-child(even) &': {
    display: 'block',
  },
});
({
  '.group:first-of-type &': {
    display: 'block',
  },
});
({
  '.group:last-of-type &': {
    display: 'block',
  },
});
({
  '.group:only-of-type &': {
    display: 'block',
  },
}) // State

({
  '.group:visited &': {
    display: 'block',
  },
});
({
  '.group:target &': {
    display: 'block',
  },
});
({
  '.group[open] &': {
    display: 'block',
  },
}) // Forms

({
  '.group:default &': {
    display: 'block',
  },
});
({
  '.group:checked &': {
    display: 'block',
  },
});
({
  '.group:indeterminate &': {
    display: 'block',
  },
});
({
  '.group:placeholder-shown &': {
    display: 'block',
  },
});
({
  '.group:autofill &': {
    display: 'block',
  },
});
({
  '.group:optional &': {
    display: 'block',
  },
});
({
  '.group:required &': {
    display: 'block',
  },
});
({
  '.group:valid &': {
    display: 'block',
  },
});
({
  '.group:invalid &': {
    display: 'block',
  },
});
({
  '.group:in-range &': {
    display: 'block',
  },
});
({
  '.group:out-of-range &': {
    display: 'block',
  },
});
({
  '.group:read-only &': {
    display: 'block',
  },
}) // Content

({
  '.group:empty &': {
    display: 'block',
  },
}) // Interactive

({
  '.group:focus-within &': {
    display: 'block',
  },
});
({
  '.group:hover &': {
    display: 'block',
  },
});
({
  '.group:focus &': {
    display: 'block',
  },
});
({
  '.group:focus-visible &': {
    display: 'block',
  },
});
({
  '.group:active &': {
    display: 'block',
  },
});
({
  '.group:enabled &': {
    display: 'block',
  },
});
({
  '.group:disabled &': {
    display: 'block',
  },
}) // Twin custom

({
  '.group * &': {
    display: 'block',
  },
});
({
  '.group > * &': {
    display: 'block',
  },
});
({
  '.group ~ * &': {
    display: 'block',
  },
});
({
  '.group:hover &, .group:focus &': {
    display: 'block',
  },
});
({
  '.group:link &': {
    display: 'block',
  },
});
({
  '.group:read-write &': {
    display: 'block',
  },
});
({
  '.group svg &': {
    display: 'block',
  },
});
({
  '.group:nth-of-type(even) &': {
    display: 'block',
  },
});
({
  '.group:nth-of-type(odd) &': {
    display: 'block',
  },
}) // Not versions of the above
// Positional

({
  '.group:not(:first-child) &': {
    display: 'block',
  },
});
({
  '.group:not(:last-child) &': {
    display: 'block',
  },
});
({
  '.group:not(:only-child) &': {
    display: 'block',
  },
});
({
  '.group:not(:nth-child(odd)) &': {
    display: 'block',
  },
});
({
  '.group:not(:nth-child(even)) &': {
    display: 'block',
  },
});
({
  '.group:not(:first-of-type) &': {
    display: 'block',
  },
});
({
  '.group:not(:last-of-type) &': {
    display: 'block',
  },
});
({
  '.group:not(:only-of-type) &': {
    display: 'block',
  },
}) // State

({
  '.group:not(:visited) &': {
    display: 'block',
  },
});
({
  '.group:not(:target) &': {
    display: 'block',
  },
});
({
  '.group:not([open]) &': {
    display: 'block',
  },
}) // Forms

({
  '.group:not(:default) &': {
    display: 'block',
  },
});
({
  '.group:not(:checked) &': {
    display: 'block',
  },
});
({
  '.group:not(:indeterminate) &': {
    display: 'block',
  },
});
({
  '.group:not(:placeholder-shown) &': {
    display: 'block',
  },
});
({
  '.group:not(:autofill) &': {
    display: 'block',
  },
});
({
  '.group:not(:optional) &': {
    display: 'block',
  },
});
({
  '.group:not(:required) &': {
    display: 'block',
  },
});
({
  '.group:not(:valid) &': {
    display: 'block',
  },
});
({
  '.group:not(:invalid) &': {
    display: 'block',
  },
});
({
  '.group:not(:in-range) &': {
    display: 'block',
  },
});
({
  '.group:not(:out-of-range) &': {
    display: 'block',
  },
});
({
  '.group:not(:read-only) &': {
    display: 'block',
  },
}) // Content

({
  '.group:not(:empty) &': {
    display: 'block',
  },
}) // Interactive

({
  '.group:not(:focus-within) &': {
    display: 'block',
  },
});
({
  '.group:not(:hover) &': {
    display: 'block',
  },
});
({
  '.group:not(:focus) &': {
    display: 'block',
  },
});
({
  '.group:not(:focus-visible) &': {
    display: 'block',
  },
});
({
  '.group:not(:active) &': {
    display: 'block',
  },
});
({
  '.group:not(:enabled) &': {
    display: 'block',
  },
});
({
  '.group:not(:disabled) &': {
    display: 'block',
  },
}) // Twin custom

({
  '.group:not(*) &': {
    display: 'block',
  },
});
({
  '.group:not(> *) &': {
    display: 'block',
  },
});
({
  '.group:not(~ *) &': {
    display: 'block',
  },
});
({
  '.group:not(:hover) &, .group:not(:focus) &': {
    display: 'block',
  },
});
({
  '.group:not(:link) &': {
    display: 'block',
  },
});
({
  '.group:not(:read-write) &': {
    display: 'block',
  },
});
({
  '.group:not(svg) &': {
    display: 'block',
  },
});
({
  '.group:not(:nth-of-type(even)) &': {
    display: 'block',
  },
});
({
  '.group:not(:nth-of-type(odd)) &': {
    display: 'block',
  },
})


