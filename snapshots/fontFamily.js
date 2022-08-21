
// https://tailwindcss.com/docs/font-family
tw`font-sans`
tw`font-serif`
tw`font-mono`

tw`font-['Open Sans']`

tw`font-[generic-name:fantasy]`
tw`font-[family-name:'this and that', this]`

      ↓ ↓ ↓ ↓ ↓ ↓

// https://tailwindcss.com/docs/font-family
({
  fontFamily:
    'ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"',
});
({
  fontFamily: 'ui-serif, Georgia, Cambria, "Times New Roman", Times, serif',
});
({
  fontFamily:
    'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
});
({
  fontFamily: "'Open Sans'",
});
({
  fontFamily: 'fantasy',
});
({
  fontFamily: "'this and that', this",
})


