
// Positional
tw`peer-first:block`
tw`peer-last:block`
tw`peer-only:block`
tw`peer-odd:block`
tw`peer-even:block`
tw`peer-first-of-type:block`
tw`peer-last-of-type:block`
tw`peer-only-of-type:block`

// State
tw`peer-visited:block`
tw`peer-target:block`
tw`peer-open:block`

// Forms
tw`peer-default:block`
tw`peer-checked:block`
tw`peer-indeterminate:block`
tw`peer-placeholder-shown:block`
tw`peer-autofill:block`
tw`peer-optional:block`
tw`peer-required:block`
tw`peer-valid:block`
tw`peer-invalid:block`
tw`peer-in-range:block`
tw`peer-out-of-range:block`
tw`peer-read-only:block`

// Content
tw`peer-empty:block`

// Interactive
tw`peer-focus-within:block`
tw`peer-hover:block`
tw`peer-focus:block`
tw`peer-focus-visible:block`
tw`peer-active:block`
tw`peer-enabled:block`
tw`peer-disabled:block`

// Twin custom
tw`peer-all:block`
tw`peer-all-child:block`
tw`peer-sibling:block`
tw`peer-hocus:block`
tw`peer-link:block`
tw`peer-read-write:block`
tw`peer-svg:block`
tw`peer-even-of-type:block`
tw`peer-odd-of-type:block`

// Not versions of the above

// Positional
tw`peer-not-first:block`
tw`peer-not-last:block`
tw`peer-not-only:block`
tw`peer-not-odd:block`
tw`peer-not-even:block`
tw`peer-not-first-of-type:block`
tw`peer-not-last-of-type:block`
tw`peer-not-only-of-type:block`

// State
tw`peer-not-visited:block`
tw`peer-not-target:block`
tw`peer-not-open:block`

// Forms
tw`peer-not-default:block`
tw`peer-not-checked:block`
tw`peer-not-indeterminate:block`
tw`peer-not-placeholder-shown:block`
tw`peer-not-autofill:block`
tw`peer-not-optional:block`
tw`peer-not-required:block`
tw`peer-not-valid:block`
tw`peer-not-invalid:block`
tw`peer-not-in-range:block`
tw`peer-not-out-of-range:block`
tw`peer-not-read-only:block`

// Content
tw`peer-not-empty:block`

// Interactive
tw`peer-not-focus-within:block`
tw`peer-not-hover:block`
tw`peer-not-focus:block`
tw`peer-not-focus-visible:block`
tw`peer-not-active:block`
tw`peer-not-enabled:block`
tw`peer-not-disabled:block`

// Twin custom
tw`peer-not-all:block`
tw`peer-not-all-child:block`
tw`peer-not-sibling:block`
tw`peer-not-hocus:block`
tw`peer-not-link:block`
tw`peer-not-read-write:block`
tw`peer-not-svg:block`
tw`peer-not-even-of-type:block`
tw`peer-not-odd-of-type:block`

      ↓ ↓ ↓ ↓ ↓ ↓

// Positional
({
  '.peer:first-child ~ &': {
    display: 'block',
  },
});
({
  '.peer:last-child ~ &': {
    display: 'block',
  },
});
({
  '.peer:only-child ~ &': {
    display: 'block',
  },
});
({
  '.peer:nth-child(odd) ~ &': {
    display: 'block',
  },
});
({
  '.peer:nth-child(even) ~ &': {
    display: 'block',
  },
});
({
  '.peer:first-of-type ~ &': {
    display: 'block',
  },
});
({
  '.peer:last-of-type ~ &': {
    display: 'block',
  },
});
({
  '.peer:only-of-type ~ &': {
    display: 'block',
  },
}) // State

({
  '.peer:visited ~ &': {
    display: 'block',
  },
});
({
  '.peer:target ~ &': {
    display: 'block',
  },
});
({
  '.peer[open] ~ &': {
    display: 'block',
  },
}) // Forms

({
  '.peer:default ~ &': {
    display: 'block',
  },
});
({
  '.peer:checked ~ &': {
    display: 'block',
  },
});
({
  '.peer:indeterminate ~ &': {
    display: 'block',
  },
});
({
  '.peer:placeholder-shown ~ &': {
    display: 'block',
  },
});
({
  '.peer:autofill ~ &': {
    display: 'block',
  },
});
({
  '.peer:optional ~ &': {
    display: 'block',
  },
});
({
  '.peer:required ~ &': {
    display: 'block',
  },
});
({
  '.peer:valid ~ &': {
    display: 'block',
  },
});
({
  '.peer:invalid ~ &': {
    display: 'block',
  },
});
({
  '.peer:in-range ~ &': {
    display: 'block',
  },
});
({
  '.peer:out-of-range ~ &': {
    display: 'block',
  },
});
({
  '.peer:read-only ~ &': {
    display: 'block',
  },
}) // Content

({
  '.peer:empty ~ &': {
    display: 'block',
  },
}) // Interactive

({
  '.peer:focus-within ~ &': {
    display: 'block',
  },
});
({
  '.peer:hover ~ &': {
    display: 'block',
  },
});
({
  '.peer:focus ~ &': {
    display: 'block',
  },
});
({
  '.peer:focus-visible ~ &': {
    display: 'block',
  },
});
({
  '.peer:active ~ &': {
    display: 'block',
  },
});
({
  '.peer:enabled ~ &': {
    display: 'block',
  },
});
({
  '.peer:disabled ~ &': {
    display: 'block',
  },
}) // Twin custom

({
  '.peer * ~ &': {
    display: 'block',
  },
});
({
  '.peer > * ~ &': {
    display: 'block',
  },
});
({
  '.peer ~ * ~ &': {
    display: 'block',
  },
});
({
  '.peer:hover ~ &, .peer:focus ~ &': {
    display: 'block',
  },
});
({
  '.peer:link ~ &': {
    display: 'block',
  },
});
({
  '.peer:read-write ~ &': {
    display: 'block',
  },
});
({
  '.peer svg ~ &': {
    display: 'block',
  },
});
({
  '.peer:nth-of-type(even) ~ &': {
    display: 'block',
  },
});
({
  '.peer:nth-of-type(odd) ~ &': {
    display: 'block',
  },
}) // Not versions of the above
// Positional

({
  '.peer:not(:first-child) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:last-child) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:only-child) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:nth-child(odd)) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:nth-child(even)) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:first-of-type) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:last-of-type) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:only-of-type) ~ &': {
    display: 'block',
  },
}) // State

({
  '.peer:not(:visited) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:target) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not([open]) ~ &': {
    display: 'block',
  },
}) // Forms

({
  '.peer:not(:default) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:checked) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:indeterminate) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:placeholder-shown) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:autofill) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:optional) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:required) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:valid) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:invalid) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:in-range) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:out-of-range) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:read-only) ~ &': {
    display: 'block',
  },
}) // Content

({
  '.peer:not(:empty) ~ &': {
    display: 'block',
  },
}) // Interactive

({
  '.peer:not(:focus-within) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:hover) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:focus) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:focus-visible) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:active) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:enabled) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:disabled) ~ &': {
    display: 'block',
  },
}) // Twin custom

({
  '.peer:not(*) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(> *) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(~ *) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:hover) ~ &, .peer:not(:focus) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:link) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:read-write) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(svg) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:nth-of-type(even)) ~ &': {
    display: 'block',
  },
});
({
  '.peer:not(:nth-of-type(odd)) ~ &': {
    display: 'block',
  },
})


