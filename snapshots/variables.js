
// Test a user created css class with a css variable as a rule property
tw`css-class-with-variable-as-rule-property`

// Test negative css variables
tw`-mx-gutter-1/2`

      ↓ ↓ ↓ ↓ ↓ ↓

// Test a user created css class with a css variable as a rule property
({
  '--some-css-variable-as-rule-prop': 'blue',
}) // Test negative css variables

({
  marginLeft: 'calc(var(--gutter-half) * -1)',
  marginRight: 'calc(var(--gutter-half) * -1)',
})


