
// https://tailwindcss.com/docs/font-variant-numeric
tw`normal-nums`
tw`ordinal`
tw`slashed-zero`
tw`lining-nums`
tw`oldstyle-nums`
tw`proportional-nums`
tw`tabular-nums`
tw`diagonal-fractions`
tw`stacked-fractions`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/font-variant-numeric
({
  fontVariantNumeric: 'normal',
});
({
  '--tw-ordinal': 'ordinal',
  fontVariantNumeric:
    'var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)',
});
({
  '--tw-slashed-zero': 'slashed-zero',
  fontVariantNumeric:
    'var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)',
});
({
  '--tw-numeric-figure': 'lining-nums',
  fontVariantNumeric:
    'var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)',
});
({
  '--tw-numeric-figure': 'oldstyle-nums',
  fontVariantNumeric:
    'var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)',
});
({
  '--tw-numeric-spacing': 'proportional-nums',
  fontVariantNumeric:
    'var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)',
});
({
  '--tw-numeric-spacing': 'tabular-nums',
  fontVariantNumeric:
    'var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)',
});
({
  '--tw-numeric-fraction': 'diagonal-fractions',
  fontVariantNumeric:
    'var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)',
});
({
  '--tw-numeric-fraction': 'stacked-fractions',
  fontVariantNumeric:
    'var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)',
})


