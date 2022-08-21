

import tw from "./macro"

// Pseudo element variants
tw`first-letter:block`
tw`first-line:block`
tw`marker:block`
tw`selection:block`
tw`file:block`
tw`placeholder:block`
tw`backdrop:block`
tw`before:block`
tw`after:block`
tw`before:(content block)`
tw`after:(content block)`

// Positional
tw`first:block`
tw`last:block`
tw`only:block`
tw`odd:block`
tw`even:block`
tw`first-of-type:block`
tw`last-of-type:block`
tw`only-of-type:block`

// State
tw`visited:block`
tw`target:block`
tw`open:block`

// Forms
tw`default:block`
tw`checked:block`
tw`indeterminate:block`
tw`placeholder-shown:block`
tw`autofill:block`
tw`optional:block`
tw`required:block`
tw`valid:block`
tw`invalid:block`
tw`in-range:block`
tw`out-of-range:block`
tw`read-only:block`

// Content
tw`empty:block`

// Interactive
tw`focus-within:block`
tw`hover:block`
tw`focus:block`
tw`focus-visible:block`
tw`active:block`
tw`enabled:block`
tw`disabled:block`

// Twin custom
tw`all:block`
tw`all-child:block`
tw`sibling:block`
tw`hocus:block`
tw`link:block`
tw`read-write:block`
tw`svg:block`
tw`even-of-type:block`
tw`odd-of-type:block`

// Not versions of the above

// Positional
tw`not-first:block`
tw`not-last:block`
tw`not-only:block`
tw`not-odd:block`
tw`not-even:block`
tw`not-first-of-type:block`
tw`not-last-of-type:block`
tw`not-only-of-type:block`

// State
tw`not-visited:block`
tw`not-target:block`
tw`not-open:block`

// Forms
tw`not-default:block`
tw`not-checked:block`
tw`not-indeterminate:block`
tw`not-placeholder-shown:block`
tw`not-autofill:block`
tw`not-optional:block`
tw`not-required:block`
tw`not-valid:block`
tw`not-invalid:block`
tw`not-in-range:block`
tw`not-out-of-range:block`
tw`not-read-only:block`

// Content
tw`not-empty:block`

// Interactive
tw`not-focus-within:block`
tw`not-hover:block`
tw`not-focus:block`
tw`not-focus-visible:block`
tw`not-active:block`
tw`not-enabled:block`
tw`not-disabled:block`

// Twin custom
tw`not-all:block`
tw`not-all-child:block`
tw`not-sibling:block`
tw`not-hocus:block`
tw`not-link:block`
tw`not-read-write:block`
tw`not-svg:block`
tw`not-even-of-type:block`
tw`not-odd-of-type:block`

tw`ltr:block`
tw`rtl:block`

tw`motion-safe:block`
tw`motion-reduce:block`

tw`dark:block`
tw`light:block`
tw`dark:sm:block`
tw`light:sm:block`
tw`dark:group-hover:sm:block`
tw`light:group-hocus:sm:block`

tw`print:block`
tw`screen:block`

tw`portrait:block`
tw`landscape:block`
tw`contrast-more:block`
tw`contrast-less:block`

tw`any-pointer-none:block`
tw`any-pointer-fine:block`
tw`any-pointer-coarse:block`

tw`pointer-none:block`
tw`pointer-fine:block`
tw`pointer-coarse:block`

tw`any-hover-none:block`
tw`any-hover:block`

tw`can-hover:block`
tw`cant-hover:block`

// Arbitrary values
tw`first:inset-[50px]`
tw`md:text-[red]`

// Random
tw`xl:placeholder-red-500! first:md:block sm:disabled:flex`

      ↓ ↓ ↓ ↓ ↓ ↓

// Pseudo element variants
({
  "::first-letter": {
    display: "block",
  },
});
({
  "::first-line": {
    display: "block",
  },
});
({
  "*::marker, ::marker": {
    display: "block",
  },
});
({
  "*::selection, ::selection": {
    display: "block",
  },
});
({
  "::file-selector-button": {
    display: "block",
  },
});
({
  "::placeholder": {
    display: "block",
  },
});
({
  "::backdrop": {
    display: "block",
  },
});
({
  ":before": {
    content: "''",
    display: "block",
  },
});
({
  ":after": {
    content: "''",
    display: "block",
  },
});
({
  ":before": {
    content: "''",
    display: "block",
  },
});
({
  ":after": {
    content: "''",
    display: "block",
  },
}) // Positional

({
  ":first-child": {
    display: "block",
  },
});
({
  ":last-child": {
    display: "block",
  },
});
({
  ":only-child": {
    display: "block",
  },
});
({
  ":nth-child(odd)": {
    display: "block",
  },
});
({
  ":nth-child(even)": {
    display: "block",
  },
});
({
  ":first-of-type": {
    display: "block",
  },
});
({
  ":last-of-type": {
    display: "block",
  },
});
({
  ":only-of-type": {
    display: "block",
  },
}) // State

({
  ":visited": {
    display: "block",
  },
});
({
  ":target": {
    display: "block",
  },
});
({
  "[open]": {
    display: "block",
  },
}) // Forms

({
  ":default": {
    display: "block",
  },
});
({
  ":checked": {
    display: "block",
  },
});
({
  ":indeterminate": {
    display: "block",
  },
});
({
  ":placeholder-shown": {
    display: "block",
  },
});
({
  ":autofill": {
    display: "block",
  },
});
({
  ":optional": {
    display: "block",
  },
});
({
  ":required": {
    display: "block",
  },
});
({
  ":valid": {
    display: "block",
  },
});
({
  ":invalid": {
    display: "block",
  },
});
({
  ":in-range": {
    display: "block",
  },
});
({
  ":out-of-range": {
    display: "block",
  },
});
({
  ":read-only": {
    display: "block",
  },
}) // Content

({
  ":empty": {
    display: "block",
  },
}) // Interactive

({
  ":focus-within": {
    display: "block",
  },
});
({
  ":hover": {
    display: "block",
  },
});
({
  ":focus": {
    display: "block",
  },
});
({
  ":focus-visible": {
    display: "block",
  },
});
({
  ":active": {
    display: "block",
  },
});
({
  ":enabled": {
    display: "block",
  },
});
({
  ":disabled": {
    display: "block",
  },
}) // Twin custom

({
  "*": {
    display: "block",
  },
});
({
  "> *": {
    display: "block",
  },
});
({
  "~ *": {
    display: "block",
  },
});
({
  ":hover, :focus": {
    display: "block",
  },
});
({
  ":link": {
    display: "block",
  },
});
({
  ":read-write": {
    display: "block",
  },
});
({
  svg: {
    display: "block",
  },
});
({
  ":nth-of-type(even)": {
    display: "block",
  },
});
({
  ":nth-of-type(odd)": {
    display: "block",
  },
}) // Not versions of the above
// Positional

({
  ":not(:first-child)": {
    display: "block",
  },
});
({
  ":not(:last-child)": {
    display: "block",
  },
});
({
  ":not(:only-child)": {
    display: "block",
  },
});
({
  ":not(:nth-child(odd))": {
    display: "block",
  },
});
({
  ":not(:nth-child(even))": {
    display: "block",
  },
});
({
  ":not(:first-of-type)": {
    display: "block",
  },
});
({
  ":not(:last-of-type)": {
    display: "block",
  },
});
({
  ":not(:only-of-type)": {
    display: "block",
  },
}) // State

({
  ":not(:visited)": {
    display: "block",
  },
});
({
  ":not(:target)": {
    display: "block",
  },
});
({
  ":not([open])": {
    display: "block",
  },
}) // Forms

({
  ":not(:default)": {
    display: "block",
  },
});
({
  ":not(:checked)": {
    display: "block",
  },
});
({
  ":not(:indeterminate)": {
    display: "block",
  },
});
({
  ":not(:placeholder-shown)": {
    display: "block",
  },
});
({
  ":not(:autofill)": {
    display: "block",
  },
});
({
  ":not(:optional)": {
    display: "block",
  },
});
({
  ":not(:required)": {
    display: "block",
  },
});
({
  ":not(:valid)": {
    display: "block",
  },
});
({
  ":not(:invalid)": {
    display: "block",
  },
});
({
  ":not(:in-range)": {
    display: "block",
  },
});
({
  ":not(:out-of-range)": {
    display: "block",
  },
});
({
  ":not(:read-only)": {
    display: "block",
  },
}) // Content

({
  ":not(:empty)": {
    display: "block",
  },
}) // Interactive

({
  ":not(:focus-within)": {
    display: "block",
  },
});
({
  ":not(:hover)": {
    display: "block",
  },
});
({
  ":not(:focus)": {
    display: "block",
  },
});
({
  ":not(:focus-visible)": {
    display: "block",
  },
});
({
  ":not(:active)": {
    display: "block",
  },
});
({
  ":not(:enabled)": {
    display: "block",
  },
});
({
  ":not(:disabled)": {
    display: "block",
  },
}) // Twin custom

({
  ":not(*)": {
    display: "block",
  },
});
({
  ":not(> *)": {
    display: "block",
  },
});
({
  ":not(~ *)": {
    display: "block",
  },
});
({
  ":not(:hover), :not(:focus)": {
    display: "block",
  },
});
({
  ":not(:link)": {
    display: "block",
  },
});
({
  ":not(:read-write)": {
    display: "block",
  },
});
({
  ":not(svg)": {
    display: "block",
  },
});
({
  ":not(:nth-of-type(even))": {
    display: "block",
  },
});
({
  ":not(:nth-of-type(odd))": {
    display: "block",
  },
});
({
  "[dir='ltr'] &": {
    display: "block",
  },
});
({
  "[dir='rtl'] &": {
    display: "block",
  },
});
({
  "@media (prefers-reduced-motion: no-preference)": {
    display: "block",
  },
});
({
  "@media (prefers-reduced-motion: reduce)": {
    display: "block",
  },
});
({
  "@media (prefers-color-scheme: dark)": {
    display: "block",
  },
});
({
  "@media (prefers-color-scheme: light)": {
    display: "block",
  },
});
({
  "@media (prefers-color-scheme: dark)": {
    "@media (min-width: 640px)": {
      display: "block",
    },
  },
});
({
  "@media (prefers-color-scheme: light)": {
    "@media (min-width: 640px)": {
      display: "block",
    },
  },
});
({
  "@media (prefers-color-scheme: dark)": {
    "@media (min-width: 640px)": {
      ".group:hover &": {
        display: "block",
      },
    },
  },
});
({
  "@media (prefers-color-scheme: light)": {
    "@media (min-width: 640px)": {
      ".group:hover &, .group:focus &": {
        display: "block",
      },
    },
  },
});
({
  "@media print": {
    display: "block",
  },
});
({
  "@media screen": {
    display: "block",
  },
});
({
  "@media (orientation: portrait)": {
    display: "block",
  },
});
({
  "@media (orientation: landscape)": {
    display: "block",
  },
});
({
  "@media (prefers-contrast: more)": {
    display: "block",
  },
});
({
  "@media (prefers-contrast: less)": {
    display: "block",
  },
});
({
  "@media (any-pointer: none)": {
    display: "block",
  },
});
({
  "@media (any-pointer: fine)": {
    display: "block",
  },
});
({
  "@media (any-pointer: coarse)": {
    display: "block",
  },
});
({
  "@media (pointer: none)": {
    display: "block",
  },
});
({
  "@media (pointer: fine)": {
    display: "block",
  },
});
({
  "@media (pointer: coarse)": {
    display: "block",
  },
});
({
  "@media (any-hover: none)": {
    display: "block",
  },
});
({
  "@media (any-hover: hover)": {
    display: "block",
  },
});
({
  "@media (hover: hover)": {
    display: "block",
  },
});
({
  "@media (hover: none)": {
    display: "block",
  },
}) // Arbitrary values

({
  ":first-child": {
    top: "50px",
    right: "50px",
    bottom: "50px",
    left: "50px",
  },
});
({
  "@media (min-width: 768px)": {
    "--tw-text-opacity": "1",
    color: "rgb(255 0 0 / var(--tw-text-opacity))",
  },
}) // Random

({
  "@media (min-width: 768px)": {
    ":first-child": {
      display: "block",
    },
  },
  "@media (min-width: 640px)": {
    ":disabled": {
      display: "flex",
    },
  },
  "@media (min-width: 1280px)": {
    "::placeholder": {
      "--tw-placeholder-opacity": "1",
      color: "rgb(239 68 68 / var(--tw-placeholder-opacity)) !important",
    },
  },
})


