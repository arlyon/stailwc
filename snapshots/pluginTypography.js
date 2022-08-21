
// From @tailwindcss/typography
tw`prose sm:prose-sm lg:prose-lg xl:prose-xl`

// From tailwindcss-typography
tw`rich-text`

      ↓ ↓ ↓ ↓ ↓ ↓

// From @tailwindcss/typography
({
  color: 'var(--tw-prose-body)',
  maxWidth: '65ch',
  '--tw-prose-body': '#374151',
  '--tw-prose-headings': '#111827',
  '--tw-prose-lead': '#4b5563',
  '--tw-prose-links': '#111827',
  '--tw-prose-bold': '#111827',
  '--tw-prose-counters': '#6b7280',
  '--tw-prose-bullets': '#d1d5db',
  '--tw-prose-hr': '#e5e7eb',
  '--tw-prose-quotes': '#111827',
  '--tw-prose-quote-borders': '#e5e7eb',
  '--tw-prose-captions': '#6b7280',
  '--tw-prose-code': '#111827',
  '--tw-prose-pre-code': '#e5e7eb',
  '--tw-prose-pre-bg': '#1f2937',
  '--tw-prose-th-borders': '#d1d5db',
  '--tw-prose-td-borders': '#e5e7eb',
  '--tw-prose-invert-body': '#d1d5db',
  '--tw-prose-invert-headings': '#fff',
  '--tw-prose-invert-lead': '#9ca3af',
  '--tw-prose-invert-links': '#fff',
  '--tw-prose-invert-bold': '#fff',
  '--tw-prose-invert-counters': '#9ca3af',
  '--tw-prose-invert-bullets': '#4b5563',
  '--tw-prose-invert-hr': '#374151',
  '--tw-prose-invert-quotes': '#f3f4f6',
  '--tw-prose-invert-quote-borders': '#374151',
  '--tw-prose-invert-captions': '#9ca3af',
  '--tw-prose-invert-code': '#fff',
  '--tw-prose-invert-pre-code': '#d1d5db',
  '--tw-prose-invert-pre-bg': 'rgb(0 0 0 / 50%)',
  '--tw-prose-invert-th-borders': '#4b5563',
  '--tw-prose-invert-td-borders': '#374151',
  fontSize: '1rem',
  lineHeight: '1.75',
  '& :where([class~="lead"]):not(:where([class~="not-prose"] *))': {
    color: 'var(--tw-prose-lead)',
    fontSize: '1.25em',
    lineHeight: '1.6',
    marginTop: '1.2em',
    marginBottom: '1.2em',
  },
  '& :where(a):not(:where([class~="not-prose"] *))': {
    color: 'var(--tw-prose-links)',
    textDecoration: 'underline',
    fontWeight: '500',
  },
  '& :where(strong):not(:where([class~="not-prose"] *))': {
    color: 'var(--tw-prose-bold)',
    fontWeight: '600',
  },
  '& :where(ol):not(:where([class~="not-prose"] *))': {
    listStyleType: 'decimal',
    paddingLeft: '1.625em',
  },
  '& :where(ol[type="A"]):not(:where([class~="not-prose"] *))': {
    listStyleType: 'upper-alpha',
  },
  '& :where(ol[type="a"]):not(:where([class~="not-prose"] *))': {
    listStyleType: 'lower-alpha',
  },
  '& :where(ol[type="A" s]):not(:where([class~="not-prose"] *))': {
    listStyleType: 'upper-alpha',
  },
  '& :where(ol[type="a" s]):not(:where([class~="not-prose"] *))': {
    listStyleType: 'lower-alpha',
  },
  '& :where(ol[type="I"]):not(:where([class~="not-prose"] *))': {
    listStyleType: 'upper-roman',
  },
  '& :where(ol[type="i"]):not(:where([class~="not-prose"] *))': {
    listStyleType: 'lower-roman',
  },
  '& :where(ol[type="I" s]):not(:where([class~="not-prose"] *))': {
    listStyleType: 'upper-roman',
  },
  '& :where(ol[type="i" s]):not(:where([class~="not-prose"] *))': {
    listStyleType: 'lower-roman',
  },
  '& :where(ol[type="1"]):not(:where([class~="not-prose"] *))': {
    listStyleType: 'decimal',
  },
  '& :where(ul):not(:where([class~="not-prose"] *))': {
    listStyleType: 'disc',
    paddingLeft: '1.625em',
  },
  '& :where(ol > li):not(:where([class~="not-prose"] *))::marker': {
    fontWeight: '400',
    color: 'var(--tw-prose-counters)',
  },
  '& :where(ul > li):not(:where([class~="not-prose"] *))::marker': {
    color: 'var(--tw-prose-bullets)',
  },
  '& :where(hr):not(:where([class~="not-prose"] *))': {
    borderColor: 'var(--tw-prose-hr)',
    borderTopWidth: '1px',
    marginTop: '3em',
    marginBottom: '3em',
  },
  '& :where(blockquote):not(:where([class~="not-prose"] *))': {
    fontWeight: '500',
    fontStyle: 'italic',
    color: 'var(--tw-prose-quotes)',
    borderLeftWidth: '0.25rem',
    borderLeftColor: 'var(--tw-prose-quote-borders)',
    quotes: '"\\201C""\\201D""\\2018""\\2019"',
    marginTop: '1.6em',
    marginBottom: '1.6em',
    paddingLeft: '1em',
  },
  '& :where(blockquote p:first-of-type):not(:where([class~="not-prose"] *))::before':
    {
      content: 'open-quote',
    },
  '& :where(blockquote p:last-of-type):not(:where([class~="not-prose"] *))::after':
    {
      content: 'close-quote',
    },
  '& :where(h1):not(:where([class~="not-prose"] *))': {
    color: 'var(--tw-prose-headings)',
    fontWeight: '800',
    fontSize: '2.25em',
    marginTop: '0',
    marginBottom: '0.8888889em',
    lineHeight: '1.1111111',
  },
  '& :where(h1 strong):not(:where([class~="not-prose"] *))': {
    fontWeight: '900',
  },
  '& :where(h2):not(:where([class~="not-prose"] *))': {
    color: 'var(--tw-prose-headings)',
    fontWeight: '700',
    fontSize: '1.5em',
    marginTop: '2em',
    marginBottom: '1em',
    lineHeight: '1.3333333',
  },
  '& :where(h2 strong):not(:where([class~="not-prose"] *))': {
    fontWeight: '800',
  },
  '& :where(h3):not(:where([class~="not-prose"] *))': {
    color: 'var(--tw-prose-headings)',
    fontWeight: '600',
    fontSize: '1.25em',
    marginTop: '1.6em',
    marginBottom: '0.6em',
    lineHeight: '1.6',
  },
  '& :where(h3 strong):not(:where([class~="not-prose"] *))': {
    fontWeight: '700',
  },
  '& :where(h4):not(:where([class~="not-prose"] *))': {
    color: 'var(--tw-prose-headings)',
    fontWeight: '600',
    marginTop: '1.5em',
    marginBottom: '0.5em',
    lineHeight: '1.5',
  },
  '& :where(h4 strong):not(:where([class~="not-prose"] *))': {
    fontWeight: '700',
  },
  '& :where(figure > *):not(:where([class~="not-prose"] *))': {
    marginTop: '0',
    marginBottom: '0',
  },
  '& :where(figcaption):not(:where([class~="not-prose"] *))': {
    color: 'var(--tw-prose-captions)',
    fontSize: '0.875em',
    lineHeight: '1.4285714',
    marginTop: '0.8571429em',
  },
  '& :where(code):not(:where([class~="not-prose"] *))': {
    color: 'var(--tw-prose-code)',
    fontWeight: '600',
    fontSize: '0.875em',
  },
  '& :where(code):not(:where([class~="not-prose"] *))::before': {
    content: '"`"',
  },
  '& :where(code):not(:where([class~="not-prose"] *))::after': {
    content: '"`"',
  },
  '& :where(a code):not(:where([class~="not-prose"] *))': {
    color: 'var(--tw-prose-links)',
  },
  '& :where(pre):not(:where([class~="not-prose"] *))': {
    color: 'var(--tw-prose-pre-code)',
    backgroundColor: 'var(--tw-prose-pre-bg)',
    overflowX: 'auto',
    fontWeight: '400',
    fontSize: '0.875em',
    lineHeight: '1.7142857',
    marginTop: '1.7142857em',
    marginBottom: '1.7142857em',
    borderRadius: '0.375rem',
    paddingTop: '0.8571429em',
    paddingRight: '1.1428571em',
    paddingBottom: '0.8571429em',
    paddingLeft: '1.1428571em',
  },
  '& :where(pre code):not(:where([class~="not-prose"] *))': {
    backgroundColor: 'transparent',
    borderWidth: '0',
    borderRadius: '0',
    padding: '0',
    fontWeight: 'inherit',
    color: 'inherit',
    fontSize: 'inherit',
    fontFamily: 'inherit',
    lineHeight: 'inherit',
  },
  '& :where(pre code):not(:where([class~="not-prose"] *))::before': {
    content: 'none',
  },
  '& :where(pre code):not(:where([class~="not-prose"] *))::after': {
    content: 'none',
  },
  '& :where(table):not(:where([class~="not-prose"] *))': {
    width: '100%',
    tableLayout: 'auto',
    textAlign: 'left',
    marginTop: '2em',
    marginBottom: '2em',
    fontSize: '0.875em',
    lineHeight: '1.7142857',
  },
  '& :where(thead):not(:where([class~="not-prose"] *))': {
    borderBottomWidth: '1px',
    borderBottomColor: 'var(--tw-prose-th-borders)',
  },
  '& :where(thead th):not(:where([class~="not-prose"] *))': {
    color: 'var(--tw-prose-headings)',
    fontWeight: '600',
    verticalAlign: 'bottom',
    paddingRight: '0.5714286em',
    paddingBottom: '0.5714286em',
    paddingLeft: '0.5714286em',
  },
  '& :where(tbody tr):not(:where([class~="not-prose"] *))': {
    borderBottomWidth: '1px',
    borderBottomColor: 'var(--tw-prose-td-borders)',
  },
  '& :where(tbody tr:last-child):not(:where([class~="not-prose"] *))': {
    borderBottomWidth: '0',
  },
  '& :where(tbody td):not(:where([class~="not-prose"] *))': {
    verticalAlign: 'baseline',
    paddingTop: '0.5714286em',
    paddingRight: '0.5714286em',
    paddingBottom: '0.5714286em',
    paddingLeft: '0.5714286em',
  },
  '& :where(p):not(:where([class~="not-prose"] *))': {
    marginTop: '1.25em',
    marginBottom: '1.25em',
  },
  '& :where(img):not(:where([class~="not-prose"] *))': {
    marginTop: '2em',
    marginBottom: '2em',
  },
  '& :where(video):not(:where([class~="not-prose"] *))': {
    marginTop: '2em',
    marginBottom: '2em',
  },
  '& :where(figure):not(:where([class~="not-prose"] *))': {
    marginTop: '2em',
    marginBottom: '2em',
  },
  '& :where(h2 code):not(:where([class~="not-prose"] *))': {
    fontSize: '0.875em',
  },
  '& :where(h3 code):not(:where([class~="not-prose"] *))': {
    fontSize: '0.9em',
  },
  '& :where(li):not(:where([class~="not-prose"] *))': {
    marginTop: '0.5em',
    marginBottom: '0.5em',
  },
  '& :where(ol > li):not(:where([class~="not-prose"] *))': {
    paddingLeft: '0.375em',
  },
  '& :where(ul > li):not(:where([class~="not-prose"] *))': {
    paddingLeft: '0.375em',
  },
  '& > :where(ul > li p):not(:where([class~="not-prose"] *))': {
    marginTop: '0.75em',
    marginBottom: '0.75em',
  },
  '& > :where(ul > li > *:first-child):not(:where([class~="not-prose"] *))': {
    marginTop: '1.25em',
  },
  '& > :where(ul > li > *:last-child):not(:where([class~="not-prose"] *))': {
    marginBottom: '1.25em',
  },
  '& > :where(ol > li > *:first-child):not(:where([class~="not-prose"] *))': {
    marginTop: '1.25em',
  },
  '& > :where(ol > li > *:last-child):not(:where([class~="not-prose"] *))': {
    marginBottom: '1.25em',
  },
  '& :where(ul ul, ul ol, ol ul, ol ol):not(:where([class~="not-prose"] *))': {
    marginTop: '0.75em',
    marginBottom: '0.75em',
  },
  '& :where(hr + *):not(:where([class~="not-prose"] *))': {
    marginTop: '0',
  },
  '& :where(h2 + *):not(:where([class~="not-prose"] *))': {
    marginTop: '0',
  },
  '& :where(h3 + *):not(:where([class~="not-prose"] *))': {
    marginTop: '0',
  },
  '& :where(h4 + *):not(:where([class~="not-prose"] *))': {
    marginTop: '0',
  },
  '& :where(thead th:first-child):not(:where([class~="not-prose"] *))': {
    paddingLeft: '0',
  },
  '& :where(thead th:last-child):not(:where([class~="not-prose"] *))': {
    paddingRight: '0',
  },
  '& :where(tbody td:first-child):not(:where([class~="not-prose"] *))': {
    paddingLeft: '0',
  },
  '& :where(tbody td:last-child):not(:where([class~="not-prose"] *))': {
    paddingRight: '0',
  },
  '& > :where(:first-child):not(:where([class~="not-prose"] *))': {
    marginTop: '0',
  },
  '& > :where(:last-child):not(:where([class~="not-prose"] *))': {
    marginBottom: '0',
  },
  '@media (min-width: 640px)': {
    fontSize: '0.875rem',
    lineHeight: '1.7142857',
    '& :where(p):not(:where([class~="not-prose"] *))': {
      marginTop: '1.1428571em',
      marginBottom: '1.1428571em',
    },
    '& :where([class~="lead"]):not(:where([class~="not-prose"] *))': {
      fontSize: '1.2857143em',
      lineHeight: '1.5555556',
      marginTop: '0.8888889em',
      marginBottom: '0.8888889em',
    },
    '& :where(blockquote):not(:where([class~="not-prose"] *))': {
      marginTop: '1.3333333em',
      marginBottom: '1.3333333em',
      paddingLeft: '1.1111111em',
    },
    '& :where(h1):not(:where([class~="not-prose"] *))': {
      fontSize: '2.1428571em',
      marginTop: '0',
      marginBottom: '0.8em',
      lineHeight: '1.2',
    },
    '& :where(h2):not(:where([class~="not-prose"] *))': {
      fontSize: '1.4285714em',
      marginTop: '1.6em',
      marginBottom: '0.8em',
      lineHeight: '1.4',
    },
    '& :where(h3):not(:where([class~="not-prose"] *))': {
      fontSize: '1.2857143em',
      marginTop: '1.5555556em',
      marginBottom: '0.4444444em',
      lineHeight: '1.5555556',
    },
    '& :where(h4):not(:where([class~="not-prose"] *))': {
      marginTop: '1.4285714em',
      marginBottom: '0.5714286em',
      lineHeight: '1.4285714',
    },
    '& :where(img):not(:where([class~="not-prose"] *))': {
      marginTop: '1.7142857em',
      marginBottom: '1.7142857em',
    },
    '& :where(video):not(:where([class~="not-prose"] *))': {
      marginTop: '1.7142857em',
      marginBottom: '1.7142857em',
    },
    '& :where(figure):not(:where([class~="not-prose"] *))': {
      marginTop: '1.7142857em',
      marginBottom: '1.7142857em',
    },
    '& :where(figure > *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
      marginBottom: '0',
    },
    '& :where(figcaption):not(:where([class~="not-prose"] *))': {
      fontSize: '0.8571429em',
      lineHeight: '1.3333333',
      marginTop: '0.6666667em',
    },
    '& :where(code):not(:where([class~="not-prose"] *))': {
      fontSize: '0.8571429em',
    },
    '& :where(h2 code):not(:where([class~="not-prose"] *))': {
      fontSize: '0.9em',
    },
    '& :where(h3 code):not(:where([class~="not-prose"] *))': {
      fontSize: '0.8888889em',
    },
    '& :where(pre):not(:where([class~="not-prose"] *))': {
      fontSize: '0.8571429em',
      lineHeight: '1.6666667',
      marginTop: '1.6666667em',
      marginBottom: '1.6666667em',
      borderRadius: '0.25rem',
      paddingTop: '0.6666667em',
      paddingRight: '1em',
      paddingBottom: '0.6666667em',
      paddingLeft: '1em',
    },
    '& :where(ol):not(:where([class~="not-prose"] *))': {
      paddingLeft: '1.5714286em',
    },
    '& :where(ul):not(:where([class~="not-prose"] *))': {
      paddingLeft: '1.5714286em',
    },
    '& :where(li):not(:where([class~="not-prose"] *))': {
      marginTop: '0.2857143em',
      marginBottom: '0.2857143em',
    },
    '& :where(ol > li):not(:where([class~="not-prose"] *))': {
      paddingLeft: '0.4285714em',
    },
    '& :where(ul > li):not(:where([class~="not-prose"] *))': {
      paddingLeft: '0.4285714em',
    },
    '& > :where(ul > li p):not(:where([class~="not-prose"] *))': {
      marginTop: '0.5714286em',
      marginBottom: '0.5714286em',
    },
    '& > :where(ul > li > *:first-child):not(:where([class~="not-prose"] *))': {
      marginTop: '1.1428571em',
    },
    '& > :where(ul > li > *:last-child):not(:where([class~="not-prose"] *))': {
      marginBottom: '1.1428571em',
    },
    '& > :where(ol > li > *:first-child):not(:where([class~="not-prose"] *))': {
      marginTop: '1.1428571em',
    },
    '& > :where(ol > li > *:last-child):not(:where([class~="not-prose"] *))': {
      marginBottom: '1.1428571em',
    },
    '& :where(ul ul, ul ol, ol ul, ol ol):not(:where([class~="not-prose"] *))':
      {
        marginTop: '0.5714286em',
        marginBottom: '0.5714286em',
      },
    '& :where(hr):not(:where([class~="not-prose"] *))': {
      marginTop: '2.8571429em',
      marginBottom: '2.8571429em',
    },
    '& :where(hr + *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& :where(h2 + *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& :where(h3 + *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& :where(h4 + *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& :where(table):not(:where([class~="not-prose"] *))': {
      fontSize: '0.8571429em',
      lineHeight: '1.5',
    },
    '& :where(thead th):not(:where([class~="not-prose"] *))': {
      paddingRight: '1em',
      paddingBottom: '0.6666667em',
      paddingLeft: '1em',
    },
    '& :where(thead th:first-child):not(:where([class~="not-prose"] *))': {
      paddingLeft: '0',
    },
    '& :where(thead th:last-child):not(:where([class~="not-prose"] *))': {
      paddingRight: '0',
    },
    '& :where(tbody td):not(:where([class~="not-prose"] *))': {
      paddingTop: '0.6666667em',
      paddingRight: '1em',
      paddingBottom: '0.6666667em',
      paddingLeft: '1em',
    },
    '& :where(tbody td:first-child):not(:where([class~="not-prose"] *))': {
      paddingLeft: '0',
    },
    '& :where(tbody td:last-child):not(:where([class~="not-prose"] *))': {
      paddingRight: '0',
    },
    '& > :where(:first-child):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& > :where(:last-child):not(:where([class~="not-prose"] *))': {
      marginBottom: '0',
    },
  },
  '@media (min-width: 1024px)': {
    fontSize: '1.125rem',
    lineHeight: '1.7777778',
    '& :where(p):not(:where([class~="not-prose"] *))': {
      marginTop: '1.3333333em',
      marginBottom: '1.3333333em',
    },
    '& :where([class~="lead"]):not(:where([class~="not-prose"] *))': {
      fontSize: '1.2222222em',
      lineHeight: '1.4545455',
      marginTop: '1.0909091em',
      marginBottom: '1.0909091em',
    },
    '& :where(blockquote):not(:where([class~="not-prose"] *))': {
      marginTop: '1.6666667em',
      marginBottom: '1.6666667em',
      paddingLeft: '1em',
    },
    '& :where(h1):not(:where([class~="not-prose"] *))': {
      fontSize: '2.6666667em',
      marginTop: '0',
      marginBottom: '0.8333333em',
      lineHeight: '1',
    },
    '& :where(h2):not(:where([class~="not-prose"] *))': {
      fontSize: '1.6666667em',
      marginTop: '1.8666667em',
      marginBottom: '1.0666667em',
      lineHeight: '1.3333333',
    },
    '& :where(h3):not(:where([class~="not-prose"] *))': {
      fontSize: '1.3333333em',
      marginTop: '1.6666667em',
      marginBottom: '0.6666667em',
      lineHeight: '1.5',
    },
    '& :where(h4):not(:where([class~="not-prose"] *))': {
      marginTop: '1.7777778em',
      marginBottom: '0.4444444em',
      lineHeight: '1.5555556',
    },
    '& :where(img):not(:where([class~="not-prose"] *))': {
      marginTop: '1.7777778em',
      marginBottom: '1.7777778em',
    },
    '& :where(video):not(:where([class~="not-prose"] *))': {
      marginTop: '1.7777778em',
      marginBottom: '1.7777778em',
    },
    '& :where(figure):not(:where([class~="not-prose"] *))': {
      marginTop: '1.7777778em',
      marginBottom: '1.7777778em',
    },
    '& :where(figure > *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
      marginBottom: '0',
    },
    '& :where(figcaption):not(:where([class~="not-prose"] *))': {
      fontSize: '0.8888889em',
      lineHeight: '1.5',
      marginTop: '1em',
    },
    '& :where(code):not(:where([class~="not-prose"] *))': {
      fontSize: '0.8888889em',
    },
    '& :where(h2 code):not(:where([class~="not-prose"] *))': {
      fontSize: '0.8666667em',
    },
    '& :where(h3 code):not(:where([class~="not-prose"] *))': {
      fontSize: '0.875em',
    },
    '& :where(pre):not(:where([class~="not-prose"] *))': {
      fontSize: '0.8888889em',
      lineHeight: '1.75',
      marginTop: '2em',
      marginBottom: '2em',
      borderRadius: '0.375rem',
      paddingTop: '1em',
      paddingRight: '1.5em',
      paddingBottom: '1em',
      paddingLeft: '1.5em',
    },
    '& :where(ol):not(:where([class~="not-prose"] *))': {
      paddingLeft: '1.5555556em',
    },
    '& :where(ul):not(:where([class~="not-prose"] *))': {
      paddingLeft: '1.5555556em',
    },
    '& :where(li):not(:where([class~="not-prose"] *))': {
      marginTop: '0.6666667em',
      marginBottom: '0.6666667em',
    },
    '& :where(ol > li):not(:where([class~="not-prose"] *))': {
      paddingLeft: '0.4444444em',
    },
    '& :where(ul > li):not(:where([class~="not-prose"] *))': {
      paddingLeft: '0.4444444em',
    },
    '& > :where(ul > li p):not(:where([class~="not-prose"] *))': {
      marginTop: '0.8888889em',
      marginBottom: '0.8888889em',
    },
    '& > :where(ul > li > *:first-child):not(:where([class~="not-prose"] *))': {
      marginTop: '1.3333333em',
    },
    '& > :where(ul > li > *:last-child):not(:where([class~="not-prose"] *))': {
      marginBottom: '1.3333333em',
    },
    '& > :where(ol > li > *:first-child):not(:where([class~="not-prose"] *))': {
      marginTop: '1.3333333em',
    },
    '& > :where(ol > li > *:last-child):not(:where([class~="not-prose"] *))': {
      marginBottom: '1.3333333em',
    },
    '& :where(ul ul, ul ol, ol ul, ol ol):not(:where([class~="not-prose"] *))':
      {
        marginTop: '0.8888889em',
        marginBottom: '0.8888889em',
      },
    '& :where(hr):not(:where([class~="not-prose"] *))': {
      marginTop: '3.1111111em',
      marginBottom: '3.1111111em',
    },
    '& :where(hr + *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& :where(h2 + *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& :where(h3 + *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& :where(h4 + *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& :where(table):not(:where([class~="not-prose"] *))': {
      fontSize: '0.8888889em',
      lineHeight: '1.5',
    },
    '& :where(thead th):not(:where([class~="not-prose"] *))': {
      paddingRight: '0.75em',
      paddingBottom: '0.75em',
      paddingLeft: '0.75em',
    },
    '& :where(thead th:first-child):not(:where([class~="not-prose"] *))': {
      paddingLeft: '0',
    },
    '& :where(thead th:last-child):not(:where([class~="not-prose"] *))': {
      paddingRight: '0',
    },
    '& :where(tbody td):not(:where([class~="not-prose"] *))': {
      paddingTop: '0.75em',
      paddingRight: '0.75em',
      paddingBottom: '0.75em',
      paddingLeft: '0.75em',
    },
    '& :where(tbody td:first-child):not(:where([class~="not-prose"] *))': {
      paddingLeft: '0',
    },
    '& :where(tbody td:last-child):not(:where([class~="not-prose"] *))': {
      paddingRight: '0',
    },
    '& > :where(:first-child):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& > :where(:last-child):not(:where([class~="not-prose"] *))': {
      marginBottom: '0',
    },
  },
  '@media (min-width: 1280px)': {
    fontSize: '1.25rem',
    lineHeight: '1.8',
    '& :where(p):not(:where([class~="not-prose"] *))': {
      marginTop: '1.2em',
      marginBottom: '1.2em',
    },
    '& :where([class~="lead"]):not(:where([class~="not-prose"] *))': {
      fontSize: '1.2em',
      lineHeight: '1.5',
      marginTop: '1em',
      marginBottom: '1em',
    },
    '& :where(blockquote):not(:where([class~="not-prose"] *))': {
      marginTop: '1.6em',
      marginBottom: '1.6em',
      paddingLeft: '1.0666667em',
    },
    '& :where(h1):not(:where([class~="not-prose"] *))': {
      fontSize: '2.8em',
      marginTop: '0',
      marginBottom: '0.8571429em',
      lineHeight: '1',
    },
    '& :where(h2):not(:where([class~="not-prose"] *))': {
      fontSize: '1.8em',
      marginTop: '1.5555556em',
      marginBottom: '0.8888889em',
      lineHeight: '1.1111111',
    },
    '& :where(h3):not(:where([class~="not-prose"] *))': {
      fontSize: '1.5em',
      marginTop: '1.6em',
      marginBottom: '0.6666667em',
      lineHeight: '1.3333333',
    },
    '& :where(h4):not(:where([class~="not-prose"] *))': {
      marginTop: '1.8em',
      marginBottom: '0.6em',
      lineHeight: '1.6',
    },
    '& :where(img):not(:where([class~="not-prose"] *))': {
      marginTop: '2em',
      marginBottom: '2em',
    },
    '& :where(video):not(:where([class~="not-prose"] *))': {
      marginTop: '2em',
      marginBottom: '2em',
    },
    '& :where(figure):not(:where([class~="not-prose"] *))': {
      marginTop: '2em',
      marginBottom: '2em',
    },
    '& :where(figure > *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
      marginBottom: '0',
    },
    '& :where(figcaption):not(:where([class~="not-prose"] *))': {
      fontSize: '0.9em',
      lineHeight: '1.5555556',
      marginTop: '1em',
    },
    '& :where(code):not(:where([class~="not-prose"] *))': {
      fontSize: '0.9em',
    },
    '& :where(h2 code):not(:where([class~="not-prose"] *))': {
      fontSize: '0.8611111em',
    },
    '& :where(h3 code):not(:where([class~="not-prose"] *))': {
      fontSize: '0.9em',
    },
    '& :where(pre):not(:where([class~="not-prose"] *))': {
      fontSize: '0.9em',
      lineHeight: '1.7777778',
      marginTop: '2em',
      marginBottom: '2em',
      borderRadius: '0.5rem',
      paddingTop: '1.1111111em',
      paddingRight: '1.3333333em',
      paddingBottom: '1.1111111em',
      paddingLeft: '1.3333333em',
    },
    '& :where(ol):not(:where([class~="not-prose"] *))': {
      paddingLeft: '1.6em',
    },
    '& :where(ul):not(:where([class~="not-prose"] *))': {
      paddingLeft: '1.6em',
    },
    '& :where(li):not(:where([class~="not-prose"] *))': {
      marginTop: '0.6em',
      marginBottom: '0.6em',
    },
    '& :where(ol > li):not(:where([class~="not-prose"] *))': {
      paddingLeft: '0.4em',
    },
    '& :where(ul > li):not(:where([class~="not-prose"] *))': {
      paddingLeft: '0.4em',
    },
    '& > :where(ul > li p):not(:where([class~="not-prose"] *))': {
      marginTop: '0.8em',
      marginBottom: '0.8em',
    },
    '& > :where(ul > li > *:first-child):not(:where([class~="not-prose"] *))': {
      marginTop: '1.2em',
    },
    '& > :where(ul > li > *:last-child):not(:where([class~="not-prose"] *))': {
      marginBottom: '1.2em',
    },
    '& > :where(ol > li > *:first-child):not(:where([class~="not-prose"] *))': {
      marginTop: '1.2em',
    },
    '& > :where(ol > li > *:last-child):not(:where([class~="not-prose"] *))': {
      marginBottom: '1.2em',
    },
    '& :where(ul ul, ul ol, ol ul, ol ol):not(:where([class~="not-prose"] *))':
      {
        marginTop: '0.8em',
        marginBottom: '0.8em',
      },
    '& :where(hr):not(:where([class~="not-prose"] *))': {
      marginTop: '2.8em',
      marginBottom: '2.8em',
    },
    '& :where(hr + *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& :where(h2 + *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& :where(h3 + *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& :where(h4 + *):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& :where(table):not(:where([class~="not-prose"] *))': {
      fontSize: '0.9em',
      lineHeight: '1.5555556',
    },
    '& :where(thead th):not(:where([class~="not-prose"] *))': {
      paddingRight: '0.6666667em',
      paddingBottom: '0.8888889em',
      paddingLeft: '0.6666667em',
    },
    '& :where(thead th:first-child):not(:where([class~="not-prose"] *))': {
      paddingLeft: '0',
    },
    '& :where(thead th:last-child):not(:where([class~="not-prose"] *))': {
      paddingRight: '0',
    },
    '& :where(tbody td):not(:where([class~="not-prose"] *))': {
      paddingTop: '0.8888889em',
      paddingRight: '0.6666667em',
      paddingBottom: '0.8888889em',
      paddingLeft: '0.6666667em',
    },
    '& :where(tbody td:first-child):not(:where([class~="not-prose"] *))': {
      paddingLeft: '0',
    },
    '& :where(tbody td:last-child):not(:where([class~="not-prose"] *))': {
      paddingRight: '0',
    },
    '& > :where(:first-child):not(:where([class~="not-prose"] *))': {
      marginTop: '0',
    },
    '& > :where(:last-child):not(:where([class~="not-prose"] *))': {
      marginBottom: '0',
    },
  },
}) // From tailwindcss-typography

({
  fontWeight: '400',
  fontSize: '1rem',
  lineHeight: '1.625',
  '& > * + *': {
    marginTop: '1em',
  },
  '& h1': {
    fontWeight: '700',
    fontSize: '3rem',
  },
  '& h1 lineHeight': {
    lineHeight: '1',
  },
  '& a': {
    fontWeight: '700',
    color: '#60a5fa',
  },
  '& a:hover': {
    color: '#2563eb',
    textDecoration: 'underline',
  },
  '& a:focus': {
    color: '#2563eb',
    textDecoration: 'underline',
  },
  '& a:active': {
    color: '#ea580c',
  },
  '& b': {
    fontWeight: '700',
  },
  '& strong': {
    fontWeight: '700',
  },
  '& i': {
    fontStyle: 'italic',
  },
  '& em': {
    fontStyle: 'italic',
  },
  '@media (min-width: 640px)': {
    '& h1': {
      fontSize: '3.75rem',
    },
    '& h1 lineHeight': {
      lineHeight: '1',
    },
  },
})


